/*++

Licensed under the Apache-2.0 license.

File Name:

    ecc384.rs

Abstract:

    File contains API for ECC-384 Cryptography operations

--*/
use crate::kv_access::{KvAccess, KvAccessErr};
use crate::{
    array_concat3, wait, Array4x12, CaliptraError, CaliptraResult, KeyReadArgs, KeyWriteArgs,
};
use caliptra_registers::ecc::EccReg;
use zerocopy::{AsBytes, FromBytes};

/// ECC-384 Coordinate
pub type Ecc384Scalar = Array4x12;

/// ECC-384 Seed
#[derive(Debug, Copy, Clone)]
pub enum Ecc384Seed<'a> {
    /// Array
    Array4x12(&'a Ecc384Scalar),

    /// Key Vault Key
    Key(KeyReadArgs),
}

impl<'a> From<&'a Array4x12> for Ecc384Seed<'a> {
    /// Converts to this type from the input type.
    fn from(value: &'a Array4x12) -> Self {
        Self::Array4x12(value)
    }
}

impl From<KeyReadArgs> for Ecc384Seed<'_> {
    /// Converts to this type from the input type.
    fn from(value: KeyReadArgs) -> Self {
        Self::Key(value)
    }
}

/// ECC-384 Public Key output
#[derive(Debug)]
pub enum Ecc384PrivKeyOut<'a> {
    /// Array
    Array4x12(&'a mut Ecc384Scalar),

    /// Key Vault Key
    Key(KeyWriteArgs),
}

impl<'a> From<&'a mut Array4x12> for Ecc384PrivKeyOut<'a> {
    /// Converts to this type from the input type.
    fn from(value: &'a mut Array4x12) -> Self {
        Self::Array4x12(value)
    }
}

impl<'a> From<KeyWriteArgs> for Ecc384PrivKeyOut<'a> {
    /// Converts to this type from the input type.
    fn from(value: KeyWriteArgs) -> Self {
        Self::Key(value)
    }
}

/// ECC-384 Public Key input
#[derive(Debug, Copy, Clone)]
pub enum Ecc384PrivKeyIn<'a> {
    /// Array
    Array4x12(&'a Ecc384Scalar),

    /// Key Vault Key
    Key(KeyReadArgs),
}

impl<'a> From<&'a Array4x12> for Ecc384PrivKeyIn<'a> {
    /// Converts to this type from the input type.
    fn from(value: &'a Array4x12) -> Self {
        Self::Array4x12(value)
    }
}
impl From<KeyReadArgs> for Ecc384PrivKeyIn<'_> {
    /// Converts to this type from the input type.
    fn from(value: KeyReadArgs) -> Self {
        Self::Key(value)
    }
}

/// ECC-384 Public Key
#[repr(C)]
#[derive(AsBytes, FromBytes, Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Ecc384PubKey {
    /// X coordinate
    pub x: Ecc384Scalar,

    /// Y coordinate
    pub y: Ecc384Scalar,
}

impl Ecc384PubKey {
    /// Return DER formatted public key in uncompressed form
    #[inline(never)]
    pub fn to_der(&self) -> [u8; 97] {
        array_concat3([0x04], (&self.x).into(), (&self.y).into())
    }
}

/// ECC-384 Signature
#[repr(C)]
#[derive(Debug, Default, AsBytes, FromBytes, Copy, Clone, Eq, PartialEq)]
pub struct Ecc384Signature {
    /// Random point
    pub r: Ecc384Scalar,

    /// Proof
    pub s: Ecc384Scalar,
}

/// Elliptic Curve P-384 API
pub struct Ecc384 {
    ecc: EccReg,
}

impl Ecc384 {
    pub fn new(ecc: EccReg) -> Self {
        Self { ecc }
    }
    /// Generate ECC-384 Key Pair
    ///
    /// # Arguments
    ///
    /// * `seed` - Seed for deterministic ECC Key Pair generation
    /// * `nonce` - Nonce for deterministic ECC Key Pair generation
    /// * `priv_key` - Generate ECC-384 Private key
    ///
    /// # Returns
    ///
    /// * `Ecc384PubKey` - Generated ECC-384 Public Key
    pub fn key_pair(
        &mut self,
        seed: Ecc384Seed,
        nonce: &Array4x12,
        mut priv_key: Ecc384PrivKeyOut,
    ) -> CaliptraResult<Ecc384PubKey> {
        let ecc = self.ecc.regs_mut();

        // Wait for hardware ready
        wait::until(|| ecc.status().read().ready());

        // Configure hardware to route keys to user specified hardware blocks
        match &mut priv_key {
            Ecc384PrivKeyOut::Array4x12(arr) => {
                KvAccess::begin_copy_to_arr(ecc.kv_wr_pkey_status(), ecc.kv_wr_pkey_ctrl(), arr)?
            }
            Ecc384PrivKeyOut::Key(key) => {
                KvAccess::begin_copy_to_kv(ecc.kv_wr_pkey_status(), ecc.kv_wr_pkey_ctrl(), *key)?
            }
        }

        // Copy seed to the hardware
        match seed {
            Ecc384Seed::Array4x12(arr) => KvAccess::copy_from_arr(arr, ecc.seed())?,
            Ecc384Seed::Key(key) => {
                KvAccess::copy_from_kv(key, ecc.kv_rd_seed_status(), ecc.kv_rd_seed_ctrl())
                    .map_err(|err| err.into_read_seed_err())?
            }
        }

        // Copy nonce to the hardware
        KvAccess::copy_from_arr(nonce, ecc.nonce())?;

        // Program the command register for key generation
        ecc.ctrl().write(|w| w.ctrl(|w| w.keygen()));

        // Wait for command to complete
        wait::until(|| ecc.status().read().valid());

        // Copy the private key
        match &mut priv_key {
            Ecc384PrivKeyOut::Array4x12(arr) => KvAccess::end_copy_to_arr(ecc.privkey_out(), arr)?,
            Ecc384PrivKeyOut::Key(key) => KvAccess::end_copy_to_kv(ecc.kv_wr_pkey_status(), *key)
                .map_err(|err| err.into_write_priv_key_err())?,
        }

        let pub_key = Ecc384PubKey {
            x: Array4x12::read_from_reg(ecc.pubkey_x()),
            y: Array4x12::read_from_reg(ecc.pubkey_y()),
        };

        Ok(pub_key)
    }

    /// Sign the digest with specified private key
    ///
    /// # Arguments
    ///
    /// * `priv_key` - Private key
    /// * `digest` - Digest to sign
    ///
    /// # Returns
    ///
    /// * `Ecc384Signature` - Generate signature
    pub fn sign(
        &mut self,
        priv_key: Ecc384PrivKeyIn,
        data: &Ecc384Scalar,
    ) -> CaliptraResult<Ecc384Signature> {
        let ecc = self.ecc.regs_mut();

        // Wait for hardware ready
        wait::until(|| ecc.status().read().ready());

        // Copy private key
        match priv_key {
            Ecc384PrivKeyIn::Array4x12(arr) => KvAccess::copy_from_arr(arr, ecc.privkey_in())?,
            Ecc384PrivKeyIn::Key(key) => {
                KvAccess::copy_from_kv(key, ecc.kv_rd_pkey_status(), ecc.kv_rd_pkey_ctrl())
                    .map_err(|err| err.into_read_priv_key_err())?
            }
        }

        // Copy digest
        KvAccess::copy_from_arr(data, ecc.msg())?;

        // Program the command register
        ecc.ctrl().write(|w| w.ctrl(|w| w.signing()));

        // Wait for command to complete
        wait::until(|| ecc.status().read().valid());

        // Copy signature
        let signature = Ecc384Signature {
            r: Array4x12::read_from_reg(ecc.sign_r()),
            s: Array4x12::read_from_reg(ecc.sign_s()),
        };

        Ok(signature)
    }

    /// Verify signature with specified public key and digest
    ///
    /// # Arguments
    ///
    /// * `pub_key` - Public key
    /// * `digest` - digest to verify
    /// * `signature` - Signature to verify
    ///
    /// # Result
    ///
    /// *  `bool` - True if the signature verification passed else false
    pub fn verify(
        &mut self,
        pub_key: &Ecc384PubKey,
        digest: &Ecc384Scalar,
        signature: &Ecc384Signature,
    ) -> CaliptraResult<bool> {
        let ecc = self.ecc.regs_mut();

        // Wait for hardware ready
        wait::until(|| ecc.status().read().ready());

        // Copy public key to registers
        pub_key.x.write_to_reg(ecc.pubkey_x());
        pub_key.y.write_to_reg(ecc.pubkey_y());

        // Copy digest to registers
        digest.write_to_reg(ecc.msg());

        // Copy signature to registers
        signature.r.write_to_reg(ecc.sign_r());
        signature.s.write_to_reg(ecc.sign_s());

        // Program the command register
        ecc.ctrl().write(|w| w.ctrl(|w| w.verifying()));

        // Wait for command to complete
        wait::until(|| ecc.status().read().valid());

        // Copy the random value
        let verify_r = Array4x12::read_from_reg(ecc.verify_r());

        // compare the hardware generate `r` with one in signature
        if verify_r == signature.r {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

/// ECC-384 key access error trait
trait Ecc384KeyAccessErr {
    /// Convert to read seed operation error
    fn into_read_seed_err(self) -> CaliptraError;

    /// Convert to read data operation error
    fn into_read_data_err(self) -> CaliptraError;

    /// Convert to read private key operation error
    fn into_read_priv_key_err(self) -> CaliptraError;

    /// Convert to write private key operation error
    fn into_write_priv_key_err(self) -> CaliptraError;
}

impl Ecc384KeyAccessErr for KvAccessErr {
    /// Convert to read seed operation error
    fn into_read_seed_err(self) -> CaliptraError {
        match self {
            KvAccessErr::KeyRead => CaliptraError::DRIVER_ECC384_READ_SEED_KV_READ,
            KvAccessErr::KeyWrite => CaliptraError::DRIVER_ECC384_READ_SEED_KV_WRITE,
            KvAccessErr::Generic => CaliptraError::DRIVER_ECC384_READ_SEED_KV_UNKNOWN,
        }
    }

    /// Convert to read data operation error
    fn into_read_data_err(self) -> CaliptraError {
        match self {
            KvAccessErr::KeyRead => CaliptraError::DRIVER_ECC384_READ_DATA_KV_READ,
            KvAccessErr::KeyWrite => CaliptraError::DRIVER_ECC384_READ_DATA_KV_WRITE,
            KvAccessErr::Generic => CaliptraError::DRIVER_ECC384_READ_DATA_KV_UNKNOWN,
        }
    }

    /// Convert to reads private key operation error
    fn into_read_priv_key_err(self) -> CaliptraError {
        match self {
            KvAccessErr::KeyRead => CaliptraError::DRIVER_ECC384_READ_PRIV_KEY_KV_READ,
            KvAccessErr::KeyWrite => CaliptraError::DRIVER_ECC384_READ_PRIV_KEY_KV_WRITE,
            KvAccessErr::Generic => CaliptraError::DRIVER_ECC384_READ_PRIV_KEY_KV_UNKNOWN,
        }
    }

    /// Convert to write private key operation error
    fn into_write_priv_key_err(self) -> CaliptraError {
        match self {
            KvAccessErr::KeyRead => CaliptraError::DRIVER_ECC384_WRITE_PRIV_KEY_KV_READ,
            KvAccessErr::KeyWrite => CaliptraError::DRIVER_ECC384_WRITE_PRIV_KEY_KV_WRITE,
            KvAccessErr::Generic => CaliptraError::DRIVER_ECC384_READ_PRIV_KEY_KV_UNKNOWN,
        }
    }
}
