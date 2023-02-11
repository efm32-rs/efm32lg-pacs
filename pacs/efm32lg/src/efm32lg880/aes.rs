#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x08 - Status Register"]
    pub status: STATUS,
    #[doc = "0x0c - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x10 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x14 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x18 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x1c - DATA Register"]
    pub data: DATA,
    #[doc = "0x20 - XORDATA Register"]
    pub xordata: XORDATA,
    _reserved9: [u8; 0x0c],
    #[doc = "0x30 - KEY Low Register"]
    pub keyla: KEYLA,
    #[doc = "0x34 - KEY Low Register"]
    pub keylb: KEYLB,
    #[doc = "0x38 - KEY Low Register"]
    pub keylc: KEYLC,
    #[doc = "0x3c - KEY Low Register"]
    pub keyld: KEYLD,
    #[doc = "0x40 - KEY High Register"]
    pub keyha: KEYHA,
    #[doc = "0x44 - KEY High Register"]
    pub keyhb: KEYHB,
    #[doc = "0x48 - KEY High Register"]
    pub keyhc: KEYHC,
    #[doc = "0x4c - KEY High Register"]
    pub keyhd: KEYHD,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "CMD (w) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "IEN (rw) register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "IF (r) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "IFS (w) register accessor: an alias for `Reg<IFS_SPEC>`"]
pub type IFS = crate::Reg<ifs::IFS_SPEC>;
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "IFC (w) register accessor: an alias for `Reg<IFC_SPEC>`"]
pub type IFC = crate::Reg<ifc::IFC_SPEC>;
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "DATA (rw) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "DATA Register"]
pub mod data;
#[doc = "XORDATA (rw) register accessor: an alias for `Reg<XORDATA_SPEC>`"]
pub type XORDATA = crate::Reg<xordata::XORDATA_SPEC>;
#[doc = "XORDATA Register"]
pub mod xordata;
#[doc = "KEYLA (rw) register accessor: an alias for `Reg<KEYLA_SPEC>`"]
pub type KEYLA = crate::Reg<keyla::KEYLA_SPEC>;
#[doc = "KEY Low Register"]
pub mod keyla;
#[doc = "KEYLB (rw) register accessor: an alias for `Reg<KEYLB_SPEC>`"]
pub type KEYLB = crate::Reg<keylb::KEYLB_SPEC>;
#[doc = "KEY Low Register"]
pub mod keylb;
#[doc = "KEYLC (rw) register accessor: an alias for `Reg<KEYLC_SPEC>`"]
pub type KEYLC = crate::Reg<keylc::KEYLC_SPEC>;
#[doc = "KEY Low Register"]
pub mod keylc;
#[doc = "KEYLD (rw) register accessor: an alias for `Reg<KEYLD_SPEC>`"]
pub type KEYLD = crate::Reg<keyld::KEYLD_SPEC>;
#[doc = "KEY Low Register"]
pub mod keyld;
#[doc = "KEYHA (rw) register accessor: an alias for `Reg<KEYHA_SPEC>`"]
pub type KEYHA = crate::Reg<keyha::KEYHA_SPEC>;
#[doc = "KEY High Register"]
pub mod keyha;
#[doc = "KEYHB (rw) register accessor: an alias for `Reg<KEYHB_SPEC>`"]
pub type KEYHB = crate::Reg<keyhb::KEYHB_SPEC>;
#[doc = "KEY High Register"]
pub mod keyhb;
#[doc = "KEYHC (rw) register accessor: an alias for `Reg<KEYHC_SPEC>`"]
pub type KEYHC = crate::Reg<keyhc::KEYHC_SPEC>;
#[doc = "KEY High Register"]
pub mod keyhc;
#[doc = "KEYHD (rw) register accessor: an alias for `Reg<KEYHD_SPEC>`"]
pub type KEYHD = crate::Reg<keyhd::KEYHD_SPEC>;
#[doc = "KEY High Register"]
pub mod keyhd;
