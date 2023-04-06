// Licensed under the Apache-2.0 license.
//
// generated by caliptra_registers_generator with caliptra-rtl repo at c1c08303a2420f3005019b49f0a02a6caa554a69
//
#![allow(clippy::erasing_op)]
#![allow(clippy::identity_op)]
#[derive(Clone, Copy)]
pub struct RegisterBlock<TMmio: ureg::Mmio + core::borrow::Borrow<TMmio> = ureg::RealMmio> {
    ptr: *mut u32,
    mmio: TMmio,
}
impl RegisterBlock<ureg::RealMmio> {
    pub fn mbox_csr() -> Self {
        unsafe { Self::new(0x30020000 as *mut u32) }
    }
}
impl<TMmio: ureg::Mmio + core::default::Default> RegisterBlock<TMmio> {
    /// # Safety
    ///
    /// The caller is responsible for ensuring that ptr is valid for
    /// volatile reads and writes at any of the offsets in this register
    /// block.
    pub unsafe fn new(ptr: *mut u32) -> Self {
        Self {
            ptr,
            mmio: core::default::Default::default(),
        }
    }
}
impl<TMmio: ureg::Mmio> RegisterBlock<TMmio> {
    /// # Safety
    ///
    /// The caller is responsible for ensuring that ptr is valid for
    /// volatile reads and writes at any of the offsets in this register
    /// block.
    pub unsafe fn new_with_mmio(ptr: *mut u32, mmio: TMmio) -> Self {
        Self { ptr, mmio }
    }
    /// Mailbox lock register for mailbox access, reading 0 will set the lock
    /// [br]Caliptra Access: RO
    /// [br]SOC Access:      RO
    ///
    /// Read value: [`mbox::regs::LockReadVal`]; Write value: [`mbox::regs::LockWriteVal`]
    pub fn lock(&self) -> ureg::RegRef<crate::mbox::meta::Lock, &TMmio> {
        unsafe {
            ureg::RegRef::new_with_mmio(
                self.ptr.wrapping_add(0 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Stores the user that locked the mailbox
    /// [br]Caliptra Access: RO
    /// [br]SOC Access:      RO
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn user(&self) -> ureg::RegRef<crate::mbox::meta::User, &TMmio> {
        unsafe {
            ureg::RegRef::new_with_mmio(
                self.ptr.wrapping_add(4 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Command requested for data in mailbox
    /// [br]Caliptra Access: RW
    /// [br]SOC Access:      RW
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn cmd(&self) -> ureg::RegRef<crate::mbox::meta::Cmd, &TMmio> {
        unsafe {
            ureg::RegRef::new_with_mmio(
                self.ptr.wrapping_add(8 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Data length for mailbox access in bytes
    /// [br]Caliptra Access: RW
    /// [br]SOC Access:      RW
    /// [br]TAP Access [in debug/manuf mode]: RO
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn dlen(&self) -> ureg::RegRef<crate::mbox::meta::Dlen, &TMmio> {
        unsafe {
            ureg::RegRef::new_with_mmio(
                self.ptr.wrapping_add(0xc / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Data in register, write the next data to mailbox
    /// [br]Caliptra Access: RW
    /// [br]SOC Access:      RW
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn datain(&self) -> ureg::RegRef<crate::mbox::meta::Datain, &TMmio> {
        unsafe {
            ureg::RegRef::new_with_mmio(
                self.ptr.wrapping_add(0x10 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Data out register, read the next data from mailbox
    /// [br]Caliptra Access: RO
    /// [br]SOC Access:      RO
    /// [br]TAP Access [in debug/manuf mode]: RO
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn dataout(&self) -> ureg::RegRef<crate::mbox::meta::Dataout, &TMmio> {
        unsafe {
            ureg::RegRef::new_with_mmio(
                self.ptr.wrapping_add(0x14 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Mailbox execute register indicates to receiver that the sender is done
    /// [br]Caliptra Access: RW
    /// [br]SOC Access:      RW
    ///
    /// Read value: [`mbox::regs::ExecuteReadVal`]; Write value: [`mbox::regs::ExecuteWriteVal`]
    pub fn execute(&self) -> ureg::RegRef<crate::mbox::meta::Execute, &TMmio> {
        unsafe {
            ureg::RegRef::new_with_mmio(
                self.ptr.wrapping_add(0x18 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Status of the mailbox command
    ///
    /// Read value: [`mbox::regs::StatusReadVal`]; Write value: [`mbox::regs::StatusWriteVal`]
    pub fn status(&self) -> ureg::RegRef<crate::mbox::meta::Status, &TMmio> {
        unsafe {
            ureg::RegRef::new_with_mmio(
                self.ptr.wrapping_add(0x1c / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Capability for uC only to force unlock the mailbox.
    /// [br]Caliptra Access: RW
    /// [br]SOC Access:      RO
    ///
    /// Read value: [`mbox::regs::UnlockReadVal`]; Write value: [`mbox::regs::UnlockWriteVal`]
    pub fn unlock(&self) -> ureg::RegRef<crate::mbox::meta::Unlock, &TMmio> {
        unsafe {
            ureg::RegRef::new_with_mmio(
                self.ptr.wrapping_add(0x20 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
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
    pub struct StatusReadVal(u32);
    impl StatusReadVal {
        /// Indicates the status of mailbox command
        /// [br]Caliptra Access: RW
        /// [br]SOC Access:      RW
        /// [br]TAP Access [in debug/manuf mode]: RO
        #[inline(always)]
        pub fn status(&self) -> super::enums::MboxStatusE {
            super::enums::MboxStatusE::try_from((self.0 >> 0) & 0xf).unwrap()
        }
        /// Indicates a correctable ECC single-bit error was
        /// detected and corrected while reading dataout.
        /// Auto-clears when mbox_execute field is cleared.
        /// [br]Caliptra Access: RO
        /// [br]SOC Access:      RO
        /// [br]TAP Access [in debug/manuf mode]: RO
        #[inline(always)]
        pub fn ecc_single_error(&self) -> bool {
            ((self.0 >> 4) & 1) != 0
        }
        /// Indicates an uncorrectable ECC double-bit error
        /// was detected while reading dataout.
        /// Firmware developers are advised to set the command
        /// status to CMD_FAILURE in response.
        /// Auto-clears when mbox_execute field is cleared.
        /// [br]Caliptra Access: RO
        /// [br]SOC Access:      RO
        /// [br]TAP Access [in debug/manuf mode]: RO
        #[inline(always)]
        pub fn ecc_double_error(&self) -> bool {
            ((self.0 >> 5) & 1) != 0
        }
        /// Indicates the present state of the mailbox FSM
        /// [br]Caliptra Access: RO
        /// [br]SOC Access:      RO
        /// [br]TAP Access [in debug/manuf mode]: RO
        #[inline(always)]
        pub fn mbox_fsm_ps(&self) -> super::enums::MboxFsmE {
            super::enums::MboxFsmE::try_from((self.0 >> 6) & 7).unwrap()
        }
        /// Indicates that the current lock was acquired by the SoC
        /// [br]Caliptra Access: RO
        /// [br]SOC Access:      RO
        /// [br]TAP Access [in debug/manuf mode]: RO
        #[inline(always)]
        pub fn soc_has_lock(&self) -> bool {
            ((self.0 >> 9) & 1) != 0
        }
        /// Construct a WriteVal that can be used to modify the contents of this register value.
        pub fn modify(self) -> StatusWriteVal {
            StatusWriteVal(self.0)
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
    #[derive(Clone, Copy)]
    pub struct StatusWriteVal(u32);
    impl StatusWriteVal {
        /// Indicates the status of mailbox command
        /// [br]Caliptra Access: RW
        /// [br]SOC Access:      RW
        /// [br]TAP Access [in debug/manuf mode]: RO
        #[inline(always)]
        pub fn status(
            self,
            f: impl FnOnce(super::enums::selector::MboxStatusESelector) -> super::enums::MboxStatusE,
        ) -> Self {
            Self(
                (self.0 & !(0xf << 0))
                    | (u32::from(f(super::enums::selector::MboxStatusESelector())) << 0),
            )
        }
    }
    impl From<u32> for StatusWriteVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<StatusWriteVal> for u32 {
        fn from(val: StatusWriteVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct UnlockReadVal(u32);
    impl UnlockReadVal {
        ///
        #[inline(always)]
        pub fn unlock(&self) -> bool {
            ((self.0 >> 0) & 1) != 0
        }
        /// Construct a WriteVal that can be used to modify the contents of this register value.
        pub fn modify(self) -> UnlockWriteVal {
            UnlockWriteVal(self.0)
        }
    }
    impl From<u32> for UnlockReadVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<UnlockReadVal> for u32 {
        fn from(val: UnlockReadVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct UnlockWriteVal(u32);
    impl UnlockWriteVal {
        ///
        #[inline(always)]
        pub fn unlock(self, val: bool) -> Self {
            Self((self.0 & !(1 << 0)) | (u32::from(val) << 0))
        }
    }
    impl From<u32> for UnlockWriteVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<UnlockWriteVal> for u32 {
        fn from(val: UnlockWriteVal) -> u32 {
            val.0
        }
    }
}
pub mod enums {
    //! Enumerations used by some register fields.
    #[derive(Clone, Copy, Eq, PartialEq)]
    #[repr(u32)]
    pub enum MboxFsmE {
        MboxIdle = 0,
        MboxRdyForCmd = 1,
        MboxRdyForData = 2,
        MboxRdyForDlen = 3,
        MboxExecuteSoc = 4,
        Reserved5 = 5,
        MboxExecuteUc = 6,
        Reserved7 = 7,
    }
    impl MboxFsmE {
        #[inline(always)]
        pub fn mbox_idle(&self) -> bool {
            *self == Self::MboxIdle
        }
        #[inline(always)]
        pub fn mbox_rdy_for_cmd(&self) -> bool {
            *self == Self::MboxRdyForCmd
        }
        #[inline(always)]
        pub fn mbox_rdy_for_data(&self) -> bool {
            *self == Self::MboxRdyForData
        }
        #[inline(always)]
        pub fn mbox_rdy_for_dlen(&self) -> bool {
            *self == Self::MboxRdyForDlen
        }
        #[inline(always)]
        pub fn mbox_execute_soc(&self) -> bool {
            *self == Self::MboxExecuteSoc
        }
        #[inline(always)]
        pub fn mbox_execute_uc(&self) -> bool {
            *self == Self::MboxExecuteUc
        }
    }
    impl TryFrom<u32> for MboxFsmE {
        type Error = ();
        #[inline(always)]
        fn try_from(val: u32) -> Result<MboxFsmE, ()> {
            match val {
                0 => Ok(Self::MboxIdle),
                1 => Ok(Self::MboxRdyForCmd),
                2 => Ok(Self::MboxRdyForData),
                3 => Ok(Self::MboxRdyForDlen),
                4 => Ok(Self::MboxExecuteSoc),
                5 => Ok(Self::Reserved5),
                6 => Ok(Self::MboxExecuteUc),
                7 => Ok(Self::Reserved7),
                _ => Err(()),
            }
        }
    }
    impl From<MboxFsmE> for u32 {
        fn from(val: MboxFsmE) -> Self {
            val as u32
        }
    }
    #[derive(Clone, Copy, Eq, PartialEq)]
    #[repr(u32)]
    pub enum MboxStatusE {
        CmdBusy = 0,
        DataReady = 1,
        CmdComplete = 2,
        CmdFailure = 3,
        Reserved4 = 4,
        Reserved5 = 5,
        Reserved6 = 6,
        Reserved7 = 7,
        Reserved8 = 8,
        Reserved9 = 9,
        Reserved10 = 10,
        Reserved11 = 11,
        Reserved12 = 12,
        Reserved13 = 13,
        Reserved14 = 14,
        Reserved15 = 15,
    }
    impl MboxStatusE {
        #[inline(always)]
        pub fn cmd_busy(&self) -> bool {
            *self == Self::CmdBusy
        }
        #[inline(always)]
        pub fn data_ready(&self) -> bool {
            *self == Self::DataReady
        }
        #[inline(always)]
        pub fn cmd_complete(&self) -> bool {
            *self == Self::CmdComplete
        }
        #[inline(always)]
        pub fn cmd_failure(&self) -> bool {
            *self == Self::CmdFailure
        }
    }
    impl TryFrom<u32> for MboxStatusE {
        type Error = ();
        #[inline(always)]
        fn try_from(val: u32) -> Result<MboxStatusE, ()> {
            match val {
                0 => Ok(Self::CmdBusy),
                1 => Ok(Self::DataReady),
                2 => Ok(Self::CmdComplete),
                3 => Ok(Self::CmdFailure),
                4 => Ok(Self::Reserved4),
                5 => Ok(Self::Reserved5),
                6 => Ok(Self::Reserved6),
                7 => Ok(Self::Reserved7),
                8 => Ok(Self::Reserved8),
                9 => Ok(Self::Reserved9),
                10 => Ok(Self::Reserved10),
                11 => Ok(Self::Reserved11),
                12 => Ok(Self::Reserved12),
                13 => Ok(Self::Reserved13),
                14 => Ok(Self::Reserved14),
                15 => Ok(Self::Reserved15),
                _ => Err(()),
            }
        }
    }
    impl From<MboxStatusE> for u32 {
        fn from(val: MboxStatusE) -> Self {
            val as u32
        }
    }
    pub mod selector {
        pub struct MboxFsmESelector();
        impl MboxFsmESelector {
            #[inline(always)]
            pub fn mbox_idle(&self) -> super::MboxFsmE {
                super::MboxFsmE::MboxIdle
            }
            #[inline(always)]
            pub fn mbox_rdy_for_cmd(&self) -> super::MboxFsmE {
                super::MboxFsmE::MboxRdyForCmd
            }
            #[inline(always)]
            pub fn mbox_rdy_for_dlen(&self) -> super::MboxFsmE {
                super::MboxFsmE::MboxRdyForDlen
            }
            #[inline(always)]
            pub fn mbox_rdy_for_data(&self) -> super::MboxFsmE {
                super::MboxFsmE::MboxRdyForData
            }
            #[inline(always)]
            pub fn mbox_execute_uc(&self) -> super::MboxFsmE {
                super::MboxFsmE::MboxExecuteUc
            }
            #[inline(always)]
            pub fn mbox_execute_soc(&self) -> super::MboxFsmE {
                super::MboxFsmE::MboxExecuteSoc
            }
        }
        pub struct MboxStatusESelector();
        impl MboxStatusESelector {
            #[inline(always)]
            pub fn cmd_busy(&self) -> super::MboxStatusE {
                super::MboxStatusE::CmdBusy
            }
            #[inline(always)]
            pub fn data_ready(&self) -> super::MboxStatusE {
                super::MboxStatusE::DataReady
            }
            #[inline(always)]
            pub fn cmd_complete(&self) -> super::MboxStatusE {
                super::MboxStatusE::CmdComplete
            }
            #[inline(always)]
            pub fn cmd_failure(&self) -> super::MboxStatusE {
                super::MboxStatusE::CmdFailure
            }
        }
    }
}
pub mod meta {
    //! Additional metadata needed by ureg.
    pub type Lock = ureg::ReadOnlyReg32<crate::mbox::regs::LockReadVal>;
    pub type User = ureg::ReadOnlyReg32<u32>;
    pub type Cmd = ureg::ReadWriteReg32<0, u32, u32>;
    pub type Dlen = ureg::ReadWriteReg32<0, u32, u32>;
    pub type Datain = ureg::ReadWriteReg32<0, u32, u32>;
    pub type Dataout = ureg::ReadWriteReg32<0, u32, u32>;
    pub type Execute = ureg::ReadWriteReg32<
        0,
        crate::mbox::regs::ExecuteReadVal,
        crate::mbox::regs::ExecuteWriteVal,
    >;
    pub type Status = ureg::ReadWriteReg32<
        0,
        crate::mbox::regs::StatusReadVal,
        crate::mbox::regs::StatusWriteVal,
    >;
    pub type Unlock = ureg::ReadWriteReg32<
        0,
        crate::mbox::regs::UnlockReadVal,
        crate::mbox::regs::UnlockWriteVal,
    >;
}
