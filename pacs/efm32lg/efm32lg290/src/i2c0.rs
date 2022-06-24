#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Command Register"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x08 - State Register"]
    pub state: crate::Reg<state::STATE_SPEC>,
    #[doc = "0x0c - Status Register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x10 - Clock Division Register"]
    pub clkdiv: crate::Reg<clkdiv::CLKDIV_SPEC>,
    #[doc = "0x14 - Slave Address Register"]
    pub saddr: crate::Reg<saddr::SADDR_SPEC>,
    #[doc = "0x18 - Slave Address Mask Register"]
    pub saddrmask: crate::Reg<saddrmask::SADDRMASK_SPEC>,
    #[doc = "0x1c - Receive Buffer Data Register"]
    pub rxdata: crate::Reg<rxdata::RXDATA_SPEC>,
    #[doc = "0x20 - Receive Buffer Data Peek Register"]
    pub rxdatap: crate::Reg<rxdatap::RXDATAP_SPEC>,
    #[doc = "0x24 - Transmit Buffer Data Register"]
    pub txdata: crate::Reg<txdata::TXDATA_SPEC>,
    #[doc = "0x28 - Interrupt Flag Register"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x2c - Interrupt Flag Set Register"]
    pub ifs: crate::Reg<ifs::IFS_SPEC>,
    #[doc = "0x30 - Interrupt Flag Clear Register"]
    pub ifc: crate::Reg<ifc::IFC_SPEC>,
    #[doc = "0x34 - Interrupt Enable Register"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    #[doc = "0x38 - I/O Routing Register"]
    pub route: crate::Reg<route::ROUTE_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "STATE register accessor: an alias for `Reg<STATE_SPEC>`"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "State Register"]
pub mod state;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "CLKDIV register accessor: an alias for `Reg<CLKDIV_SPEC>`"]
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
#[doc = "Clock Division Register"]
pub mod clkdiv;
#[doc = "SADDR register accessor: an alias for `Reg<SADDR_SPEC>`"]
pub type SADDR = crate::Reg<saddr::SADDR_SPEC>;
#[doc = "Slave Address Register"]
pub mod saddr;
#[doc = "SADDRMASK register accessor: an alias for `Reg<SADDRMASK_SPEC>`"]
pub type SADDRMASK = crate::Reg<saddrmask::SADDRMASK_SPEC>;
#[doc = "Slave Address Mask Register"]
pub mod saddrmask;
#[doc = "RXDATA register accessor: an alias for `Reg<RXDATA_SPEC>`"]
pub type RXDATA = crate::Reg<rxdata::RXDATA_SPEC>;
#[doc = "Receive Buffer Data Register"]
pub mod rxdata;
#[doc = "RXDATAP register accessor: an alias for `Reg<RXDATAP_SPEC>`"]
pub type RXDATAP = crate::Reg<rxdatap::RXDATAP_SPEC>;
#[doc = "Receive Buffer Data Peek Register"]
pub mod rxdatap;
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
#[doc = "ROUTE register accessor: an alias for `Reg<ROUTE_SPEC>`"]
pub type ROUTE = crate::Reg<route::ROUTE_SPEC>;
#[doc = "I/O Routing Register"]
pub mod route;
