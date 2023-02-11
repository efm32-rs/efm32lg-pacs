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
    #[doc = "0x1c - Counter Top Value Register"]
    pub top: TOP,
    #[doc = "0x20 - Counter Top Value Buffer Register"]
    pub topb: TOPB,
    #[doc = "0x24 - Counter Value Register"]
    pub cnt: CNT,
    #[doc = "0x28 - I/O Routing Register"]
    pub route: ROUTE,
    _reserved11: [u8; 0x04],
    #[doc = "0x30 - CC Channel Control Register"]
    pub cc0_ctrl: CC0_CTRL,
    #[doc = "0x34 - CC Channel Value Register"]
    pub cc0_ccv: CC0_CCV,
    #[doc = "0x38 - CC Channel Value Peek Register"]
    pub cc0_ccvp: CC0_CCVP,
    #[doc = "0x3c - CC Channel Buffer Register"]
    pub cc0_ccvb: CC0_CCVB,
    #[doc = "0x40 - CC Channel Control Register"]
    pub cc1_ctrl: CC1_CTRL,
    #[doc = "0x44 - CC Channel Value Register"]
    pub cc1_ccv: CC1_CCV,
    #[doc = "0x48 - CC Channel Value Peek Register"]
    pub cc1_ccvp: CC1_CCVP,
    #[doc = "0x4c - CC Channel Buffer Register"]
    pub cc1_ccvb: CC1_CCVB,
    #[doc = "0x50 - CC Channel Control Register"]
    pub cc2_ctrl: CC2_CTRL,
    #[doc = "0x54 - CC Channel Value Register"]
    pub cc2_ccv: CC2_CCV,
    #[doc = "0x58 - CC Channel Value Peek Register"]
    pub cc2_ccvp: CC2_CCVP,
    #[doc = "0x5c - CC Channel Buffer Register"]
    pub cc2_ccvb: CC2_CCVB,
    _reserved23: [u8; 0x10],
    #[doc = "0x70 - DTI Control Register"]
    pub dtctrl: DTCTRL,
    #[doc = "0x74 - DTI Time Control Register"]
    pub dttime: DTTIME,
    #[doc = "0x78 - DTI Fault Configuration Register"]
    pub dtfc: DTFC,
    #[doc = "0x7c - DTI Output Generation Enable Register"]
    pub dtogen: DTOGEN,
    #[doc = "0x80 - DTI Fault Register"]
    pub dtfault: DTFAULT,
    #[doc = "0x84 - DTI Fault Clear Register"]
    pub dtfaultc: DTFAULTC,
    #[doc = "0x88 - DTI Configuration Lock Register"]
    pub dtlock: DTLOCK,
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
#[doc = "TOP (rw) register accessor: an alias for `Reg<TOP_SPEC>`"]
pub type TOP = crate::Reg<top::TOP_SPEC>;
#[doc = "Counter Top Value Register"]
pub mod top;
#[doc = "TOPB (rw) register accessor: an alias for `Reg<TOPB_SPEC>`"]
pub type TOPB = crate::Reg<topb::TOPB_SPEC>;
#[doc = "Counter Top Value Buffer Register"]
pub mod topb;
#[doc = "CNT (rw) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Counter Value Register"]
pub mod cnt;
#[doc = "ROUTE (rw) register accessor: an alias for `Reg<ROUTE_SPEC>`"]
pub type ROUTE = crate::Reg<route::ROUTE_SPEC>;
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "CC0_CTRL (rw) register accessor: an alias for `Reg<CC0_CTRL_SPEC>`"]
pub type CC0_CTRL = crate::Reg<cc0_ctrl::CC0_CTRL_SPEC>;
#[doc = "CC Channel Control Register"]
pub mod cc0_ctrl;
#[doc = "CC0_CCV (rw) register accessor: an alias for `Reg<CC0_CCV_SPEC>`"]
pub type CC0_CCV = crate::Reg<cc0_ccv::CC0_CCV_SPEC>;
#[doc = "CC Channel Value Register"]
pub mod cc0_ccv;
#[doc = "CC0_CCVP (r) register accessor: an alias for `Reg<CC0_CCVP_SPEC>`"]
pub type CC0_CCVP = crate::Reg<cc0_ccvp::CC0_CCVP_SPEC>;
#[doc = "CC Channel Value Peek Register"]
pub mod cc0_ccvp;
#[doc = "CC0_CCVB (rw) register accessor: an alias for `Reg<CC0_CCVB_SPEC>`"]
pub type CC0_CCVB = crate::Reg<cc0_ccvb::CC0_CCVB_SPEC>;
#[doc = "CC Channel Buffer Register"]
pub mod cc0_ccvb;
#[doc = "CC1_CTRL (rw) register accessor: an alias for `Reg<CC1_CTRL_SPEC>`"]
pub type CC1_CTRL = crate::Reg<cc1_ctrl::CC1_CTRL_SPEC>;
#[doc = "CC Channel Control Register"]
pub mod cc1_ctrl;
#[doc = "CC1_CCV (rw) register accessor: an alias for `Reg<CC1_CCV_SPEC>`"]
pub type CC1_CCV = crate::Reg<cc1_ccv::CC1_CCV_SPEC>;
#[doc = "CC Channel Value Register"]
pub mod cc1_ccv;
#[doc = "CC1_CCVP (r) register accessor: an alias for `Reg<CC1_CCVP_SPEC>`"]
pub type CC1_CCVP = crate::Reg<cc1_ccvp::CC1_CCVP_SPEC>;
#[doc = "CC Channel Value Peek Register"]
pub mod cc1_ccvp;
#[doc = "CC1_CCVB (rw) register accessor: an alias for `Reg<CC1_CCVB_SPEC>`"]
pub type CC1_CCVB = crate::Reg<cc1_ccvb::CC1_CCVB_SPEC>;
#[doc = "CC Channel Buffer Register"]
pub mod cc1_ccvb;
#[doc = "CC2_CTRL (rw) register accessor: an alias for `Reg<CC2_CTRL_SPEC>`"]
pub type CC2_CTRL = crate::Reg<cc2_ctrl::CC2_CTRL_SPEC>;
#[doc = "CC Channel Control Register"]
pub mod cc2_ctrl;
#[doc = "CC2_CCV (rw) register accessor: an alias for `Reg<CC2_CCV_SPEC>`"]
pub type CC2_CCV = crate::Reg<cc2_ccv::CC2_CCV_SPEC>;
#[doc = "CC Channel Value Register"]
pub mod cc2_ccv;
#[doc = "CC2_CCVP (r) register accessor: an alias for `Reg<CC2_CCVP_SPEC>`"]
pub type CC2_CCVP = crate::Reg<cc2_ccvp::CC2_CCVP_SPEC>;
#[doc = "CC Channel Value Peek Register"]
pub mod cc2_ccvp;
#[doc = "CC2_CCVB (rw) register accessor: an alias for `Reg<CC2_CCVB_SPEC>`"]
pub type CC2_CCVB = crate::Reg<cc2_ccvb::CC2_CCVB_SPEC>;
#[doc = "CC Channel Buffer Register"]
pub mod cc2_ccvb;
#[doc = "DTCTRL (rw) register accessor: an alias for `Reg<DTCTRL_SPEC>`"]
pub type DTCTRL = crate::Reg<dtctrl::DTCTRL_SPEC>;
#[doc = "DTI Control Register"]
pub mod dtctrl;
#[doc = "DTTIME (rw) register accessor: an alias for `Reg<DTTIME_SPEC>`"]
pub type DTTIME = crate::Reg<dttime::DTTIME_SPEC>;
#[doc = "DTI Time Control Register"]
pub mod dttime;
#[doc = "DTFC (rw) register accessor: an alias for `Reg<DTFC_SPEC>`"]
pub type DTFC = crate::Reg<dtfc::DTFC_SPEC>;
#[doc = "DTI Fault Configuration Register"]
pub mod dtfc;
#[doc = "DTOGEN (rw) register accessor: an alias for `Reg<DTOGEN_SPEC>`"]
pub type DTOGEN = crate::Reg<dtogen::DTOGEN_SPEC>;
#[doc = "DTI Output Generation Enable Register"]
pub mod dtogen;
#[doc = "DTFAULT (r) register accessor: an alias for `Reg<DTFAULT_SPEC>`"]
pub type DTFAULT = crate::Reg<dtfault::DTFAULT_SPEC>;
#[doc = "DTI Fault Register"]
pub mod dtfault;
#[doc = "DTFAULTC (w) register accessor: an alias for `Reg<DTFAULTC_SPEC>`"]
pub type DTFAULTC = crate::Reg<dtfaultc::DTFAULTC_SPEC>;
#[doc = "DTI Fault Clear Register"]
pub mod dtfaultc;
#[doc = "DTLOCK (rw) register accessor: an alias for `Reg<DTLOCK_SPEC>`"]
pub type DTLOCK = crate::Reg<dtlock::DTLOCK_SPEC>;
#[doc = "DTI Configuration Lock Register"]
pub mod dtlock;
