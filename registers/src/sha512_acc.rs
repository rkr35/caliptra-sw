// Licensed under the Apache-2.0 license.
//
// generated by caliptra_registers_generator with rtl-caliptra repo at 374ed44ec6c106a86ecced6e2329fa12ddbd9e42
//
#![allow(clippy::erasing_op)]
#![allow(clippy::identity_op)]
#[derive(Clone, Copy)]
pub struct RegisterBlock(*mut u32);
impl RegisterBlock {
    /// # Safety
    ///
    /// The caller is responsible for ensuring that ptr is valid for
    /// volatile reads and writes at any of the offsets in this register
    /// block.
    pub unsafe fn new(ptr: *mut u32) -> Self {
        Self(ptr)
    }
    pub fn sha512_acc_csr() -> Self {
        unsafe { Self::new(0x30021000 as *mut u32) }
    }
    /// SHA lock register for SHA access, reading 0 will set the lock, Write 1 to clear the lock
    /// [br]Caliptra Access: RW
    /// [br]SOC Access:      RW
    ///
    /// Read value: [`sha512_acc::regs::LockReadVal`]; Write value: [`sha512_acc::regs::LockWriteVal`]
    pub fn lock(&self) -> ureg::RegRef<crate::sha512_acc::meta::Lock> {
        unsafe { ureg::RegRef::new(self.0.wrapping_add(0 / core::mem::size_of::<u32>())) }
    }
    /// Stores the user that locked the SHA
    /// [br]Caliptra Access: RO
    /// [br]SOC Access:      RO
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn user(&self) -> ureg::RegRef<crate::sha512_acc::meta::User> {
        unsafe { ureg::RegRef::new(self.0.wrapping_add(4 / core::mem::size_of::<u32>())) }
    }
    /// Stores the requested mode for the SHA to execute.
    /// SHA Supports both SHA384 and SHA512 modes of operation.
    /// SHA Supports streaming mode - SHA is computed on a stream of incoming data to datain register.
    ///             mailbox mode - SHA is computed on LENGTH bytes of data stored in the mailbox from START_ADDRESS.
    /// [br]Caliptra Access: RW
    /// [br]SOC Access:      RW
    ///
    /// Read value: [`sha512_acc::regs::ModeReadVal`]; Write value: [`sha512_acc::regs::ModeWriteVal`]
    pub fn mode(&self) -> ureg::RegRef<crate::sha512_acc::meta::Mode> {
        unsafe { ureg::RegRef::new(self.0.wrapping_add(8 / core::mem::size_of::<u32>())) }
    }
    /// The start address for FW controlled SHA performed on data stored in the mailbox.
    /// Start Address must be dword aligned.
    /// [br]Caliptra Access: RW
    /// [br]SOC Access:      RW
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn start_address(&self) -> ureg::RegRef<crate::sha512_acc::meta::StartAddress> {
        unsafe { ureg::RegRef::new(self.0.wrapping_add(0xc / core::mem::size_of::<u32>())) }
    }
    /// The length of data to be processed in bytes.
    /// [br]Caliptra Access: RW
    /// [br]SOC Access:      RW
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn dlen(&self) -> ureg::RegRef<crate::sha512_acc::meta::Dlen> {
        unsafe { ureg::RegRef::new(self.0.wrapping_add(0x10 / core::mem::size_of::<u32>())) }
    }
    /// Data in register for SHA Streaming function
    /// [br]Caliptra Access: RW
    /// [br]SOC Access:      RW
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn datain(&self) -> ureg::RegRef<crate::sha512_acc::meta::Datain> {
        unsafe { ureg::RegRef::new(self.0.wrapping_add(0x14 / core::mem::size_of::<u32>())) }
    }
    /// For Streaming Function, indicates that the initiator is done streaming.
    /// For the Mailbox SHA Function, indicates that the SHA can begin execution.
    /// [br]Caliptra Access: RW
    /// [br]SOC Access:      RW
    ///
    /// Read value: [`sha512_acc::regs::ExecuteReadVal`]; Write value: [`sha512_acc::regs::ExecuteWriteVal`]
    pub fn execute(&self) -> ureg::RegRef<crate::sha512_acc::meta::Execute> {
        unsafe { ureg::RegRef::new(self.0.wrapping_add(0x18 / core::mem::size_of::<u32>())) }
    }
    /// Status register indicating when the requested function is complete
    /// [br]Caliptra Access: RO
    /// [br]SOC Access:      RO
    ///
    /// Read value: [`sha512_acc::regs::StatusReadVal`]; Write value: [`sha512_acc::regs::StatusWriteVal`]
    pub fn status(&self) -> ureg::RegRef<crate::sha512_acc::meta::Status> {
        unsafe { ureg::RegRef::new(self.0.wrapping_add(0x1c / core::mem::size_of::<u32>())) }
    }
    /// 16 32-bit registers storing the 512-bit digest output in
    /// big-endian representation.
    /// [br]Caliptra Access: RO
    /// [br]SOC Access:      RO
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn digest(&self) -> ureg::Array<16, ureg::RegRef<crate::sha512_acc::meta::Digest>> {
        unsafe { ureg::Array::new(self.0.wrapping_add(0x20 / core::mem::size_of::<u32>())) }
    }
}
pub mod regs {
    //! Types that represent the values held by registers.
    #[derive(Clone, Copy)]
    pub struct ExecuteReadVal(u32);
    impl ExecuteReadVal {
        ///
        #[inline(always)]
        pub fn execute(&self) -> bool {
            ((self.0 >> 0) & 1) != 0
        }
        /// Construct a WriteVal that can be used to modify the contents of this register value.
        pub fn modify(self) -> ExecuteWriteVal {
            ExecuteWriteVal(self.0)
        }
    }
    impl From<u32> for ExecuteReadVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<ExecuteReadVal> for u32 {
        fn from(val: ExecuteReadVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct ExecuteWriteVal(u32);
    impl ExecuteWriteVal {
        ///
        #[inline(always)]
        pub fn execute(self, val: bool) -> Self {
            Self((self.0 & !(1 << 0)) | (u32::from(val) << 0))
        }
    }
    impl From<u32> for ExecuteWriteVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<ExecuteWriteVal> for u32 {
        fn from(val: ExecuteWriteVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct LockReadVal(u32);
    impl LockReadVal {
        ///
        #[inline(always)]
        pub fn lock(&self) -> bool {
            ((self.0 >> 0) & 1) != 0
        }
        /// Construct a WriteVal that can be used to modify the contents of this register value.
        pub fn modify(self) -> LockWriteVal {
            LockWriteVal(self.0)
        }
    }
    impl From<u32> for LockReadVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<LockReadVal> for u32 {
        fn from(val: LockReadVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct LockWriteVal(u32);
    impl LockWriteVal {
        ///
        #[inline(always)]
        pub fn lock(self, val: bool) -> Self {
            Self((self.0 & !(1 << 0)) | (u32::from(val) << 0))
        }
    }
    impl From<u32> for LockWriteVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<LockWriteVal> for u32 {
        fn from(val: LockWriteVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct ModeReadVal(u32);
    impl ModeReadVal {
        ///
        #[inline(always)]
        pub fn mode(&self) -> super::enums::ShaCmdE {
            super::enums::ShaCmdE::try_from((self.0 >> 0) & 3).unwrap()
        }
        /// Default behavior assumes that data in mailbox is little endian,
        /// When set to 0, data from mailbox will be swizzled from little to big endian at the byte level.
        /// When set to 1, data from the mailbox will be loaded into SHA as-is.
        /// [br]Caliptra Access: RW
        /// [br]SOC Access:      RW
        #[inline(always)]
        pub fn endian_toggle(&self) -> bool {
            ((self.0 >> 2) & 1) != 0
        }
        /// Construct a WriteVal that can be used to modify the contents of this register value.
        pub fn modify(self) -> ModeWriteVal {
            ModeWriteVal(self.0)
        }
    }
    impl From<u32> for ModeReadVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<ModeReadVal> for u32 {
        fn from(val: ModeReadVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct ModeWriteVal(u32);
    impl ModeWriteVal {
        ///
        #[inline(always)]
        pub fn mode(
            self,
            f: impl FnOnce(super::enums::selector::ShaCmdESelector) -> super::enums::ShaCmdE,
        ) -> Self {
            Self(
                (self.0 & !(3 << 0))
                    | (u32::from(f(super::enums::selector::ShaCmdESelector())) << 0),
            )
        }
        /// Default behavior assumes that data in mailbox is little endian,
        /// When set to 0, data from mailbox will be swizzled from little to big endian at the byte level.
        /// When set to 1, data from the mailbox will be loaded into SHA as-is.
        /// [br]Caliptra Access: RW
        /// [br]SOC Access:      RW
        #[inline(always)]
        pub fn endian_toggle(self, val: bool) -> Self {
            Self((self.0 & !(1 << 2)) | (u32::from(val) << 2))
        }
    }
    impl From<u32> for ModeWriteVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<ModeWriteVal> for u32 {
        fn from(val: ModeWriteVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct StatusReadVal(u32);
    impl StatusReadVal {
        /// Valid bit, indicating that the digest is complete
        #[inline(always)]
        pub fn valid(&self) -> bool {
            ((self.0 >> 0) & 1) != 0
        }
    }
    impl From<u32> for StatusReadVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<StatusReadVal> for u32 {
        fn from(val: StatusReadVal) -> u32 {
            val.0
        }
    }
}
pub mod enums {
    //! Enumerations used by some register fields.
    #[derive(Clone, Copy, Eq, PartialEq)]
    #[repr(u32)]
    pub enum ShaCmdE {
        ShaStream384 = 0,
        ShaStream512 = 1,
        ShaMbox384 = 2,
        ShaMbox512 = 3,
    }
    impl ShaCmdE {
        #[inline(always)]
        pub fn sha_stream_384(&self) -> bool {
            *self == Self::ShaStream384
        }
        #[inline(always)]
        pub fn sha_stream_512(&self) -> bool {
            *self == Self::ShaStream512
        }
        #[inline(always)]
        pub fn sha_mbox_384(&self) -> bool {
            *self == Self::ShaMbox384
        }
        #[inline(always)]
        pub fn sha_mbox_512(&self) -> bool {
            *self == Self::ShaMbox512
        }
    }
    impl TryFrom<u32> for ShaCmdE {
        type Error = ();
        #[inline(always)]
        fn try_from(val: u32) -> Result<ShaCmdE, ()> {
            match val {
                0 => Ok(Self::ShaStream384),
                1 => Ok(Self::ShaStream512),
                2 => Ok(Self::ShaMbox384),
                3 => Ok(Self::ShaMbox512),
                _ => Err(()),
            }
        }
    }
    impl From<ShaCmdE> for u32 {
        fn from(val: ShaCmdE) -> Self {
            val as u32
        }
    }
    pub mod selector {
        pub struct ShaCmdESelector();
        impl ShaCmdESelector {
            #[inline(always)]
            pub fn sha_stream_384(&self) -> super::ShaCmdE {
                super::ShaCmdE::ShaStream384
            }
            #[inline(always)]
            pub fn sha_stream_512(&self) -> super::ShaCmdE {
                super::ShaCmdE::ShaStream512
            }
            #[inline(always)]
            pub fn sha_mbox_384(&self) -> super::ShaCmdE {
                super::ShaCmdE::ShaMbox384
            }
            #[inline(always)]
            pub fn sha_mbox_512(&self) -> super::ShaCmdE {
                super::ShaCmdE::ShaMbox512
            }
        }
    }
}
pub mod meta {
    //! Additional metadata needed by ureg.
    #[derive(Clone, Copy)]
    pub struct Lock();
    impl ureg::RegType for Lock {
        type Raw = u32;
    }
    impl ureg::ReadableReg for Lock {
        type ReadVal = crate::sha512_acc::regs::LockReadVal;
    }
    impl ureg::WritableReg for Lock {
        type WriteVal = crate::sha512_acc::regs::LockWriteVal;
    }
    impl ureg::ResettableReg for Lock {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct User();
    impl ureg::RegType for User {
        type Raw = u32;
    }
    impl ureg::ReadableReg for User {
        type ReadVal = u32;
    }
    #[derive(Clone, Copy)]
    pub struct Mode();
    impl ureg::RegType for Mode {
        type Raw = u32;
    }
    impl ureg::ReadableReg for Mode {
        type ReadVal = crate::sha512_acc::regs::ModeReadVal;
    }
    impl ureg::WritableReg for Mode {
        type WriteVal = crate::sha512_acc::regs::ModeWriteVal;
    }
    impl ureg::ResettableReg for Mode {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct StartAddress();
    impl ureg::RegType for StartAddress {
        type Raw = u32;
    }
    impl ureg::ReadableReg for StartAddress {
        type ReadVal = u32;
    }
    impl ureg::WritableReg for StartAddress {
        type WriteVal = u32;
    }
    impl ureg::ResettableReg for StartAddress {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct Dlen();
    impl ureg::RegType for Dlen {
        type Raw = u32;
    }
    impl ureg::ReadableReg for Dlen {
        type ReadVal = u32;
    }
    impl ureg::WritableReg for Dlen {
        type WriteVal = u32;
    }
    impl ureg::ResettableReg for Dlen {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct Datain();
    impl ureg::RegType for Datain {
        type Raw = u32;
    }
    impl ureg::ReadableReg for Datain {
        type ReadVal = u32;
    }
    impl ureg::WritableReg for Datain {
        type WriteVal = u32;
    }
    impl ureg::ResettableReg for Datain {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct Execute();
    impl ureg::RegType for Execute {
        type Raw = u32;
    }
    impl ureg::ReadableReg for Execute {
        type ReadVal = crate::sha512_acc::regs::ExecuteReadVal;
    }
    impl ureg::WritableReg for Execute {
        type WriteVal = crate::sha512_acc::regs::ExecuteWriteVal;
    }
    impl ureg::ResettableReg for Execute {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct Status();
    impl ureg::RegType for Status {
        type Raw = u32;
    }
    impl ureg::ReadableReg for Status {
        type ReadVal = crate::sha512_acc::regs::StatusReadVal;
    }
    #[derive(Clone, Copy)]
    pub struct Digest();
    impl ureg::RegType for Digest {
        type Raw = u32;
    }
    impl ureg::ReadableReg for Digest {
        type ReadVal = u32;
    }
}
