#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Command Register"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x08 - Status Register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x0c - Single Sample Control Register"]
    pub singlectrl: crate::Reg<singlectrl::SINGLECTRL_SPEC>,
    #[doc = "0x10 - Scan Control Register"]
    pub scanctrl: crate::Reg<scanctrl::SCANCTRL_SPEC>,
    #[doc = "0x14 - Interrupt Enable Register"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    #[doc = "0x18 - Interrupt Flag Register"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x1c - Interrupt Flag Set Register"]
    pub ifs: crate::Reg<ifs::IFS_SPEC>,
    #[doc = "0x20 - Interrupt Flag Clear Register"]
    pub ifc: crate::Reg<ifc::IFC_SPEC>,
    #[doc = "0x24 - Single Conversion Result Data"]
    pub singledata: crate::Reg<singledata::SINGLEDATA_SPEC>,
    #[doc = "0x28 - Scan Conversion Result Data"]
    pub scandata: crate::Reg<scandata::SCANDATA_SPEC>,
    #[doc = "0x2c - Single Conversion Result Data Peek Register"]
    pub singledatap: crate::Reg<singledatap::SINGLEDATAP_SPEC>,
    #[doc = "0x30 - Scan Sequence Result Data Peek Register"]
    pub scandatap: crate::Reg<scandatap::SCANDATAP_SPEC>,
    #[doc = "0x34 - Calibration Register"]
    pub cal: crate::Reg<cal::CAL_SPEC>,
    _reserved14: [u8; 0x04],
    #[doc = "0x3c - Bias Programming Register"]
    pub biasprog: crate::Reg<biasprog::BIASPROG_SPEC>,
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
#[doc = "SINGLECTRL register accessor: an alias for `Reg<SINGLECTRL_SPEC>`"]
pub type SINGLECTRL = crate::Reg<singlectrl::SINGLECTRL_SPEC>;
#[doc = "Single Sample Control Register"]
pub mod singlectrl;
#[doc = "SCANCTRL register accessor: an alias for `Reg<SCANCTRL_SPEC>`"]
pub type SCANCTRL = crate::Reg<scanctrl::SCANCTRL_SPEC>;
#[doc = "Scan Control Register"]
pub mod scanctrl;
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
#[doc = "SINGLEDATA register accessor: an alias for `Reg<SINGLEDATA_SPEC>`"]
pub type SINGLEDATA = crate::Reg<singledata::SINGLEDATA_SPEC>;
#[doc = "Single Conversion Result Data"]
pub mod singledata;
#[doc = "SCANDATA register accessor: an alias for `Reg<SCANDATA_SPEC>`"]
pub type SCANDATA = crate::Reg<scandata::SCANDATA_SPEC>;
#[doc = "Scan Conversion Result Data"]
pub mod scandata;
#[doc = "SINGLEDATAP register accessor: an alias for `Reg<SINGLEDATAP_SPEC>`"]
pub type SINGLEDATAP = crate::Reg<singledatap::SINGLEDATAP_SPEC>;
#[doc = "Single Conversion Result Data Peek Register"]
pub mod singledatap;
#[doc = "SCANDATAP register accessor: an alias for `Reg<SCANDATAP_SPEC>`"]
pub type SCANDATAP = crate::Reg<scandatap::SCANDATAP_SPEC>;
#[doc = "Scan Sequence Result Data Peek Register"]
pub mod scandatap;
#[doc = "CAL register accessor: an alias for `Reg<CAL_SPEC>`"]
pub type CAL = crate::Reg<cal::CAL_SPEC>;
#[doc = "Calibration Register"]
pub mod cal;
#[doc = "BIASPROG register accessor: an alias for `Reg<BIASPROG_SPEC>`"]
pub type BIASPROG = crate::Reg<biasprog::BIASPROG_SPEC>;
#[doc = "Bias Programming Register"]
pub mod biasprog;
