#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Command Register"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x08 - Status Register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x0c - Clock Control Register"]
    pub clkdiv: crate::Reg<clkdiv::CLKDIV_SPEC>,
    #[doc = "0x10 - Start Frame Register"]
    pub startframe: crate::Reg<startframe::STARTFRAME_SPEC>,
    #[doc = "0x14 - Signal Frame Register"]
    pub sigframe: crate::Reg<sigframe::SIGFRAME_SPEC>,
    #[doc = "0x18 - Receive Buffer Data Extended Register"]
    pub rxdatax: crate::Reg<rxdatax::RXDATAX_SPEC>,
    #[doc = "0x1c - Receive Buffer Data Register"]
    pub rxdata: crate::Reg<rxdata::RXDATA_SPEC>,
    #[doc = "0x20 - Receive Buffer Data Extended Peek Register"]
    pub rxdataxp: crate::Reg<rxdataxp::RXDATAXP_SPEC>,
    #[doc = "0x24 - Transmit Buffer Data Extended Register"]
    pub txdatax: crate::Reg<txdatax::TXDATAX_SPEC>,
    #[doc = "0x28 - Transmit Buffer Data Register"]
    pub txdata: crate::Reg<txdata::TXDATA_SPEC>,
    #[doc = "0x2c - Interrupt Flag Register"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x30 - Interrupt Flag Set Register"]
    pub ifs: crate::Reg<ifs::IFS_SPEC>,
    #[doc = "0x34 - Interrupt Flag Clear Register"]
    pub ifc: crate::Reg<ifc::IFC_SPEC>,
    #[doc = "0x38 - Interrupt Enable Register"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    #[doc = "0x3c - Pulse Control Register"]
    pub pulsectrl: crate::Reg<pulsectrl::PULSECTRL_SPEC>,
    #[doc = "0x40 - Freeze Register"]
    pub freeze: crate::Reg<freeze::FREEZE_SPEC>,
    #[doc = "0x44 - Synchronization Busy Register"]
    pub syncbusy: crate::Reg<syncbusy::SYNCBUSY_SPEC>,
    _reserved18: [u8; 0x0c],
    #[doc = "0x54 - I/O Routing Register"]
    pub route: crate::Reg<route::ROUTE_SPEC>,
    _reserved19: [u8; 0x54],
    #[doc = "0xac - LEUART Input Register"]
    pub input: crate::Reg<input::INPUT_SPEC>,
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
#[doc = "CLKDIV register accessor: an alias for `Reg<CLKDIV_SPEC>`"]
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
#[doc = "Clock Control Register"]
pub mod clkdiv;
#[doc = "STARTFRAME register accessor: an alias for `Reg<STARTFRAME_SPEC>`"]
pub type STARTFRAME = crate::Reg<startframe::STARTFRAME_SPEC>;
#[doc = "Start Frame Register"]
pub mod startframe;
#[doc = "SIGFRAME register accessor: an alias for `Reg<SIGFRAME_SPEC>`"]
pub type SIGFRAME = crate::Reg<sigframe::SIGFRAME_SPEC>;
#[doc = "Signal Frame Register"]
pub mod sigframe;
#[doc = "RXDATAX register accessor: an alias for `Reg<RXDATAX_SPEC>`"]
pub type RXDATAX = crate::Reg<rxdatax::RXDATAX_SPEC>;
#[doc = "Receive Buffer Data Extended Register"]
pub mod rxdatax;
#[doc = "RXDATA register accessor: an alias for `Reg<RXDATA_SPEC>`"]
pub type RXDATA = crate::Reg<rxdata::RXDATA_SPEC>;
#[doc = "Receive Buffer Data Register"]
pub mod rxdata;
#[doc = "RXDATAXP register accessor: an alias for `Reg<RXDATAXP_SPEC>`"]
pub type RXDATAXP = crate::Reg<rxdataxp::RXDATAXP_SPEC>;
#[doc = "Receive Buffer Data Extended Peek Register"]
pub mod rxdataxp;
#[doc = "TXDATAX register accessor: an alias for `Reg<TXDATAX_SPEC>`"]
pub type TXDATAX = crate::Reg<txdatax::TXDATAX_SPEC>;
#[doc = "Transmit Buffer Data Extended Register"]
pub mod txdatax;
#[doc = "TXDATA register accessor: an alias for `Reg<TXDATA_SPEC>`"]
pub type TXDATA = crate::Reg<txdata::TXDATA_SPEC>;
#[doc = "Transmit Buffer Data Register"]
pub mod txdata;
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
#[doc = "IEN register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "PULSECTRL register accessor: an alias for `Reg<PULSECTRL_SPEC>`"]
pub type PULSECTRL = crate::Reg<pulsectrl::PULSECTRL_SPEC>;
#[doc = "Pulse Control Register"]
pub mod pulsectrl;
#[doc = "FREEZE register accessor: an alias for `Reg<FREEZE_SPEC>`"]
pub type FREEZE = crate::Reg<freeze::FREEZE_SPEC>;
#[doc = "Freeze Register"]
pub mod freeze;
#[doc = "SYNCBUSY register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "ROUTE register accessor: an alias for `Reg<ROUTE_SPEC>`"]
pub type ROUTE = crate::Reg<route::ROUTE_SPEC>;
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "INPUT register accessor: an alias for `Reg<INPUT_SPEC>`"]
pub type INPUT = crate::Reg<input::INPUT_SPEC>;
#[doc = "LEUART Input Register"]
pub mod input;
