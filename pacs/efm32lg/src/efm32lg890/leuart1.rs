#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x08 - Status Register"]
    pub status: STATUS,
    #[doc = "0x0c - Clock Control Register"]
    pub clkdiv: CLKDIV,
    #[doc = "0x10 - Start Frame Register"]
    pub startframe: STARTFRAME,
    #[doc = "0x14 - Signal Frame Register"]
    pub sigframe: SIGFRAME,
    #[doc = "0x18 - Receive Buffer Data Extended Register"]
    pub rxdatax: RXDATAX,
    #[doc = "0x1c - Receive Buffer Data Register"]
    pub rxdata: RXDATA,
    #[doc = "0x20 - Receive Buffer Data Extended Peek Register"]
    pub rxdataxp: RXDATAXP,
    #[doc = "0x24 - Transmit Buffer Data Extended Register"]
    pub txdatax: TXDATAX,
    #[doc = "0x28 - Transmit Buffer Data Register"]
    pub txdata: TXDATA,
    #[doc = "0x2c - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x30 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x34 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x38 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x3c - Pulse Control Register"]
    pub pulsectrl: PULSECTRL,
    #[doc = "0x40 - Freeze Register"]
    pub freeze: FREEZE,
    #[doc = "0x44 - Synchronization Busy Register"]
    pub syncbusy: SYNCBUSY,
    _reserved18: [u8; 0x0c],
    #[doc = "0x54 - I/O Routing Register"]
    pub route: ROUTE,
    _reserved19: [u8; 0x54],
    #[doc = "0xac - LEUART Input Register"]
    pub input: INPUT,
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
#[doc = "CLKDIV (rw) register accessor: an alias for `Reg<CLKDIV_SPEC>`"]
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
#[doc = "Clock Control Register"]
pub mod clkdiv;
#[doc = "STARTFRAME (rw) register accessor: an alias for `Reg<STARTFRAME_SPEC>`"]
pub type STARTFRAME = crate::Reg<startframe::STARTFRAME_SPEC>;
#[doc = "Start Frame Register"]
pub mod startframe;
#[doc = "SIGFRAME (rw) register accessor: an alias for `Reg<SIGFRAME_SPEC>`"]
pub type SIGFRAME = crate::Reg<sigframe::SIGFRAME_SPEC>;
#[doc = "Signal Frame Register"]
pub mod sigframe;
#[doc = "RXDATAX (r) register accessor: an alias for `Reg<RXDATAX_SPEC>`"]
pub type RXDATAX = crate::Reg<rxdatax::RXDATAX_SPEC>;
#[doc = "Receive Buffer Data Extended Register"]
pub mod rxdatax;
#[doc = "RXDATA (r) register accessor: an alias for `Reg<RXDATA_SPEC>`"]
pub type RXDATA = crate::Reg<rxdata::RXDATA_SPEC>;
#[doc = "Receive Buffer Data Register"]
pub mod rxdata;
#[doc = "RXDATAXP (r) register accessor: an alias for `Reg<RXDATAXP_SPEC>`"]
pub type RXDATAXP = crate::Reg<rxdataxp::RXDATAXP_SPEC>;
#[doc = "Receive Buffer Data Extended Peek Register"]
pub mod rxdataxp;
#[doc = "TXDATAX (w) register accessor: an alias for `Reg<TXDATAX_SPEC>`"]
pub type TXDATAX = crate::Reg<txdatax::TXDATAX_SPEC>;
#[doc = "Transmit Buffer Data Extended Register"]
pub mod txdatax;
#[doc = "TXDATA (w) register accessor: an alias for `Reg<TXDATA_SPEC>`"]
pub type TXDATA = crate::Reg<txdata::TXDATA_SPEC>;
#[doc = "Transmit Buffer Data Register"]
pub mod txdata;
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
#[doc = "IEN (rw) register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "PULSECTRL (rw) register accessor: an alias for `Reg<PULSECTRL_SPEC>`"]
pub type PULSECTRL = crate::Reg<pulsectrl::PULSECTRL_SPEC>;
#[doc = "Pulse Control Register"]
pub mod pulsectrl;
#[doc = "FREEZE (rw) register accessor: an alias for `Reg<FREEZE_SPEC>`"]
pub type FREEZE = crate::Reg<freeze::FREEZE_SPEC>;
#[doc = "Freeze Register"]
pub mod freeze;
#[doc = "SYNCBUSY (r) register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "ROUTE (rw) register accessor: an alias for `Reg<ROUTE_SPEC>`"]
pub type ROUTE = crate::Reg<route::ROUTE_SPEC>;
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "INPUT (rw) register accessor: an alias for `Reg<INPUT_SPEC>`"]
pub type INPUT = crate::Reg<input::INPUT_SPEC>;
#[doc = "LEUART Input Register"]
pub mod input;
