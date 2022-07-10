#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Command Register"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x08 - Status Register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x0c - Interrupt Enable Register"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    #[doc = "0x10 - Interrupt Flag Register"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x14 - Interrupt Flag Set Register"]
    pub ifs: crate::Reg<ifs::IFS_SPEC>,
    #[doc = "0x18 - Interrupt Flag Clear Register"]
    pub ifc: crate::Reg<ifc::IFC_SPEC>,
    #[doc = "0x1c - DATA Register"]
    pub data: crate::Reg<data::DATA_SPEC>,
    #[doc = "0x20 - XORDATA Register"]
    pub xordata: crate::Reg<xordata::XORDATA_SPEC>,
    _reserved9: [u8; 0x0c],
    #[doc = "0x30 - KEY Low Register"]
    pub keyla: crate::Reg<keyla::KEYLA_SPEC>,
    #[doc = "0x34 - KEY Low Register"]
    pub keylb: crate::Reg<keylb::KEYLB_SPEC>,
    #[doc = "0x38 - KEY Low Register"]
    pub keylc: crate::Reg<keylc::KEYLC_SPEC>,
    #[doc = "0x3c - KEY Low Register"]
    pub keyld: crate::Reg<keyld::KEYLD_SPEC>,
    #[doc = "0x40 - KEY High Register"]
    pub keyha: crate::Reg<keyha::KEYHA_SPEC>,
    #[doc = "0x44 - KEY High Register"]
    pub keyhb: crate::Reg<keyhb::KEYHB_SPEC>,
    #[doc = "0x48 - KEY High Register"]
    pub keyhc: crate::Reg<keyhc::KEYHC_SPEC>,
    #[doc = "0x4c - KEY High Register"]
    pub keyhd: crate::Reg<keyhd::KEYHD_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "IEN register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "IF register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "IFS register accessor: an alias for `Reg<IFS_SPEC>`"]
pub type IFS = crate::Reg<ifs::IFS_SPEC>;
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "IFC register accessor: an alias for `Reg<IFC_SPEC>`"]
pub type IFC = crate::Reg<ifc::IFC_SPEC>;
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "DATA Register"]
pub mod data;
#[doc = "XORDATA register accessor: an alias for `Reg<XORDATA_SPEC>`"]
pub type XORDATA = crate::Reg<xordata::XORDATA_SPEC>;
#[doc = "XORDATA Register"]
pub mod xordata;
#[doc = "KEYLA register accessor: an alias for `Reg<KEYLA_SPEC>`"]
pub type KEYLA = crate::Reg<keyla::KEYLA_SPEC>;
#[doc = "KEY Low Register"]
pub mod keyla;
#[doc = "KEYLB register accessor: an alias for `Reg<KEYLB_SPEC>`"]
pub type KEYLB = crate::Reg<keylb::KEYLB_SPEC>;
#[doc = "KEY Low Register"]
pub mod keylb;
#[doc = "KEYLC register accessor: an alias for `Reg<KEYLC_SPEC>`"]
pub type KEYLC = crate::Reg<keylc::KEYLC_SPEC>;
#[doc = "KEY Low Register"]
pub mod keylc;
#[doc = "KEYLD register accessor: an alias for `Reg<KEYLD_SPEC>`"]
pub type KEYLD = crate::Reg<keyld::KEYLD_SPEC>;
#[doc = "KEY Low Register"]
pub mod keyld;
#[doc = "KEYHA register accessor: an alias for `Reg<KEYHA_SPEC>`"]
pub type KEYHA = crate::Reg<keyha::KEYHA_SPEC>;
#[doc = "KEY High Register"]
pub mod keyha;
#[doc = "KEYHB register accessor: an alias for `Reg<KEYHB_SPEC>`"]
pub type KEYHB = crate::Reg<keyhb::KEYHB_SPEC>;
#[doc = "KEY High Register"]
pub mod keyhb;
#[doc = "KEYHC register accessor: an alias for `Reg<KEYHC_SPEC>`"]
pub type KEYHC = crate::Reg<keyhc::KEYHC_SPEC>;
#[doc = "KEY High Register"]
pub mod keyhc;
#[doc = "KEYHD register accessor: an alias for `Reg<KEYHD_SPEC>`"]
pub type KEYHD = crate::Reg<keyhd::KEYHD_SPEC>;
#[doc = "KEY High Register"]
pub mod keyhd;
