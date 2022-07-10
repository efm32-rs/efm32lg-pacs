#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Reset Cause Register"]
    pub rstcause: crate::Reg<rstcause::RSTCAUSE_SPEC>,
    #[doc = "0x08 - Command Register"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "RSTCAUSE register accessor: an alias for `Reg<RSTCAUSE_SPEC>`"]
pub type RSTCAUSE = crate::Reg<rstcause::RSTCAUSE_SPEC>;
#[doc = "Reset Cause Register"]
pub mod rstcause;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
