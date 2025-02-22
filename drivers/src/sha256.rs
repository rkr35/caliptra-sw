/*++

Licensed under the Apache-2.0 license.

File Name:

    sha256.rs

Abstract:

    File contains API for SHA-256 Cryptography operations

--*/

use core::usize;

use crate::{array::Array4x16, wait, Array4x8, CaliptraError, CaliptraResult};
use caliptra_registers::sha256::Sha256Reg;

const SHA256_BLOCK_BYTE_SIZE: usize = 64;
const SHA256_BLOCK_LEN_OFFSET: usize = 56;
const SHA256_MAX_DATA_SIZE: usize = 1024 * 1024;

pub type Sha256Digest<'a> = &'a mut Array4x8;

pub struct Sha256 {
    sha256: Sha256Reg,
}

impl Sha256 {
    pub fn new(sha256: Sha256Reg) -> Self {
        Self { sha256 }
    }
    /// Initialize multi step digest operation
    ///
    /// # Returns
    ///
    /// * `Sha256Digest` - Object representing the digest operation
    pub fn digest_init<'a>(
        &'a mut self,
        digest: Sha256Digest<'a>,
    ) -> CaliptraResult<Sha256DigestOp<'a>> {
        let op = Sha256DigestOp {
            sha: self,
            state: Sha256DigestState::Init,
            buf: [0u8; SHA256_BLOCK_BYTE_SIZE],
            buf_idx: 0,
            data_size: 0,
            digest,
        };

        Ok(op)
    }

    /// Calculate the digest of the buffer
    ///
    /// # Arguments
    ///
    /// * `buf` - Buffer to calculate the digest over
    pub fn digest(&mut self, buf: &[u8]) -> CaliptraResult<Array4x8> {
        // Check if the buffer is not large
        if buf.len() > SHA256_MAX_DATA_SIZE {
            return Err(CaliptraError::DRIVER_SHA256_MAX_DATA);
        }

        let mut first = true;
        let mut bytes_remaining = buf.len();

        loop {
            let offset = buf.len() - bytes_remaining;
            match bytes_remaining {
                0..=63 => {
                    // PANIC-FREE: Use buf.get() instead if buf[] as the compiler
                    // cannot reason about `offset` parameter to optimize out
                    // the panic.
                    if let Some(slice) = buf.get(offset..) {
                        self.digest_partial_block(slice, first, buf.len())?;
                        break;
                    } else {
                        return Err(CaliptraError::DRIVER_SHA256_INVALID_SLICE);
                    }
                }
                _ => {
                    // PANIC-FREE: Use buf.get() instead if buf[] as the compiler
                    // cannot reason about `offset` parameter to optimize out
                    // the panic call.
                    if let Some(slice) = buf.get(offset..offset + SHA256_BLOCK_BYTE_SIZE) {
                        let block = <&[u8; SHA256_BLOCK_BYTE_SIZE]>::try_from(slice).unwrap();
                        self.digest_block(block, first)?;
                        bytes_remaining -= SHA256_BLOCK_BYTE_SIZE;
                        first = false;
                    } else {
                        return Err(CaliptraError::DRIVER_SHA256_INVALID_SLICE);
                    }
                }
            }
        }

        let sha256 = self.sha256.regs();
        Ok(Array4x8::read_from_reg(sha256.digest()))
    }

    /// Copy digest to buffer
    ///
    /// # Arguments
    ///
    /// * `buf` - Digest buffer
    fn copy_digest_to_buf(&mut self, buf: &mut Array4x8) -> CaliptraResult<()> {
        let sha256 = self.sha256.regs();
        *buf = Array4x8::read_from_reg(sha256.digest());
        Ok(())
    }

    /// Calculate the digest of the last block
    ///
    /// # Arguments
    ///
    /// * `slice` - Slice of buffer to digest
    /// * `first` - Flag indicating if this is the first buffer
    /// * `buf_size` - Total buffer size
    fn digest_partial_block(
        &mut self,
        slice: &[u8],
        first: bool,
        buf_size: usize,
    ) -> CaliptraResult<()> {
        /// Set block length
        fn set_block_len(buf_size: usize, block: &mut [u8; SHA256_BLOCK_BYTE_SIZE]) {
            let bit_len = (buf_size as u64) << 3;
            block[SHA256_BLOCK_LEN_OFFSET..].copy_from_slice(&bit_len.to_be_bytes());
        }

        // Construct the block
        let mut block = [0u8; SHA256_BLOCK_BYTE_SIZE];

        // PANIC-FREE: Following check optimizes the out of bounds
        // panic in copy_from_slice
        if slice.len() > block.len() - 1 {
            return Err(CaliptraError::DRIVER_SHA256_INDEX_OUT_OF_BOUNDS);
        }
        block[..slice.len()].copy_from_slice(slice);
        block[slice.len()] = 0b1000_0000;
        if slice.len() < SHA256_BLOCK_LEN_OFFSET {
            set_block_len(buf_size, &mut block);
        }

        // Calculate the digest of the op
        self.digest_block(&block, first)?;

        // Add a padding block if one is needed
        if slice.len() >= SHA256_BLOCK_LEN_OFFSET {
            block.fill(0);
            set_block_len(buf_size, &mut block);
            self.digest_block(&block, false)?;
        }

        Ok(())
    }

    /// Calculate digest of the full block
    ///
    /// # Arguments
    ///
    /// * `block`: Block to calculate the digest
    /// * `first` - Flag indicating if this is the first block
    fn digest_block(
        &mut self,
        block: &[u8; SHA256_BLOCK_BYTE_SIZE],
        first: bool,
    ) -> CaliptraResult<()> {
        let sha256 = self.sha256.regs_mut();
        Array4x16::from(block).write_to_reg(sha256.block());
        self.digest_op(first)
    }

    // Perform the digest operation in the hardware
    //
    // # Arguments
    //
    /// * `first` - Flag indicating if this is the first block
    fn digest_op(&mut self, first: bool) -> CaliptraResult<()> {
        let sha256 = self.sha256.regs_mut();

        // Wait for the hardware to be ready
        wait::until(|| sha256.status().read().ready());

        if first {
            // Submit the first block
            sha256.ctrl().write(|w| w.mode(true).init(true).next(false));
        } else {
            // Submit next block in existing hashing chain
            sha256.ctrl().write(|w| w.mode(true).init(false).next(true));
        }

        // Wait for the digest operation to finish
        wait::until(|| sha256.status().read().valid());

        Ok(())
    }
}

/// SHA-256 Digest state
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Sha256DigestState {
    /// Initial state
    Init,

    /// Pending state
    Pending,

    /// Final state
    Final,
}

/// Multi step SHA-256 digest operation
pub struct Sha256DigestOp<'a> {
    /// SHA-256 Engine
    sha: &'a mut Sha256,

    /// State
    state: Sha256DigestState,

    /// Staging buffer
    buf: [u8; SHA256_BLOCK_BYTE_SIZE],

    /// Current staging buffer index
    buf_idx: usize,

    /// Data size
    data_size: usize,

    /// Digest
    digest: Sha256Digest<'a>,
}

impl<'a> Sha256DigestOp<'a> {
    /// Update the digest with data
    ///
    /// # Arguments
    ///
    /// * `data` - Data to used to update the digest
    pub fn update(&mut self, data: &[u8]) -> CaliptraResult<()> {
        if self.state == Sha256DigestState::Final {
            return Err(CaliptraError::DRIVER_SHA256_INVALID_STATE);
        }

        if self.data_size + data.len() > SHA256_MAX_DATA_SIZE {
            return Err(CaliptraError::DRIVER_SHA256_MAX_DATA);
        }

        for byte in data {
            self.data_size += 1;

            // PANIC-FREE: Following check optimizes the out of bounds
            // panic in indexing the `buf`
            if self.buf_idx >= self.buf.len() {
                return Err(CaliptraError::DRIVER_SHA256_INDEX_OUT_OF_BOUNDS);
            }

            // Copy the data to the buffer
            self.buf[self.buf_idx] = *byte;
            self.buf_idx += 1;

            // If the buffer is full calculate the digest of accumulated data
            if self.buf_idx == self.buf.len() {
                self.sha.digest_block(&self.buf, self.is_first())?;
                self.reset_buf_state();
            }
        }

        Ok(())
    }

    /// Finalize the digest operations
    pub fn finalize(&mut self) -> CaliptraResult<()> {
        if self.state == Sha256DigestState::Final {
            return Err(CaliptraError::DRIVER_SHA256_INVALID_STATE);
        }

        if self.buf_idx > self.buf.len() {
            return Err(CaliptraError::DRIVER_SHA256_INVALID_SLICE);
        }

        // Calculate the digest of the final block
        let buf = &self.buf[..self.buf_idx];
        self.sha
            .digest_partial_block(buf, self.is_first(), self.data_size)?;

        // Set the state of the operation to final
        self.state = Sha256DigestState::Final;

        // Copy digest
        self.sha.copy_digest_to_buf(self.digest)?;

        Ok(())
    }

    /// Check if this the first digest operation
    fn is_first(&self) -> bool {
        self.state == Sha256DigestState::Init
    }

    /// Reset internal buffer state
    fn reset_buf_state(&mut self) {
        self.buf.fill(0);
        self.buf_idx = 0;
        self.state = Sha256DigestState::Pending;
    }
}
