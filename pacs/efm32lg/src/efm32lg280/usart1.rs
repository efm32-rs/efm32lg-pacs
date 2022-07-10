#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - USART Frame Format Register"]
    pub frame: crate::Reg<frame::FRAME_SPEC>,
    #[doc = "0x08 - USART Trigger Control register"]
    pub trigctrl: crate::Reg<trigctrl::TRIGCTRL_SPEC>,
    #[doc = "0x0c - Command Register"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x10 - USART Status Register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x14 - Clock Control Register"]
    pub clkdiv: crate::Reg<clkdiv::CLKDIV_SPEC>,
    #[doc = "0x18 - RX Buffer Data Extended Register"]
    pub rxdatax: crate::Reg<rxdatax::RXDATAX_SPEC>,
    #[doc = "0x1c - RX Buffer Data Register"]
    pub rxdata: crate::Reg<rxdata::RXDATA_SPEC>,
    #[doc = "0x20 - RX Buffer Double Data Extended Register"]
    pub rxdoublex: crate::Reg<rxdoublex::RXDOUBLEX_SPEC>,
    #[doc = "0x24 - RX FIFO Double Data Register"]
    pub rxdouble: crate::Reg<rxdouble::RXDOUBLE_SPEC>,
    #[doc = "0x28 - RX Buffer Data Extended Peek Register"]
    pub rxdataxp: crate::Reg<rxdataxp::RXDATAXP_SPEC>,
    #[doc = "0x2c - RX Buffer Double Data Extended Peek Register"]
    pub rxdoublexp: crate::Reg<rxdoublexp::RXDOUBLEXP_SPEC>,
    #[doc = "0x30 - TX Buffer Data Extended Register"]
    pub txdatax: crate::Reg<txdatax::TXDATAX_SPEC>,
    #[doc = "0x34 - TX Buffer Data Register"]
    pub txdata: crate::Reg<txdata::TXDATA_SPEC>,
    #[doc = "0x38 - TX Buffer Double Data Extended Register"]
    pub txdoublex: crate::Reg<txdoublex::TXDOUBLEX_SPEC>,
    #[doc = "0x3c - TX Buffer Double Data Register"]
    pub txdouble: crate::Reg<txdouble::TXDOUBLE_SPEC>,
    #[doc = "0x40 - Interrupt Flag Register"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x44 - Interrupt Flag Set Register"]
    pub ifs: crate::Reg<ifs::IFS_SPEC>,
    #[doc = "0x48 - Interrupt Flag Clear Register"]
    pub ifc: crate::Reg<ifc::IFC_SPEC>,
    #[doc = "0x4c - Interrupt Enable Register"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    #[doc = "0x50 - IrDA Control Register"]
    pub irctrl: crate::Reg<irctrl::IRCTRL_SPEC>,
    #[doc = "0x54 - I/O Routing Register"]
    pub route: crate::Reg<route::ROUTE_SPEC>,
    #[doc = "0x58 - USART Input Register"]
    pub input: crate::Reg<input::INPUT_SPEC>,
    #[doc = "0x5c - I2S Control Register"]
    pub i2sctrl: crate::Reg<i2sctrl::I2SCTRL_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "FRAME register accessor: an alias for `Reg<FRAME_SPEC>`"]
pub type FRAME = crate::Reg<frame::FRAME_SPEC>;
#[doc = "USART Frame Format Register"]
pub mod frame;
#[doc = "TRIGCTRL register accessor: an alias for `Reg<TRIGCTRL_SPEC>`"]
pub type TRIGCTRL = crate::Reg<trigctrl::TRIGCTRL_SPEC>;
#[doc = "USART Trigger Control register"]
pub mod trigctrl;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "USART Status Register"]
pub mod status;
#[doc = "CLKDIV register accessor: an alias for `Reg<CLKDIV_SPEC>`"]
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
#[doc = "Clock Control Register"]
pub mod clkdiv;
#[doc = "RXDATAX register accessor: an alias for `Reg<RXDATAX_SPEC>`"]
pub type RXDATAX = crate::Reg<rxdatax::RXDATAX_SPEC>;
#[doc = "RX Buffer Data Extended Register"]
pub mod rxdatax;
#[doc = "RXDATA register accessor: an alias for `Reg<RXDATA_SPEC>`"]
pub type RXDATA = crate::Reg<rxdata::RXDATA_SPEC>;
#[doc = "RX Buffer Data Register"]
pub mod rxdata;
#[doc = "RXDOUBLEX register accessor: an alias for `Reg<RXDOUBLEX_SPEC>`"]
pub type RXDOUBLEX = crate::Reg<rxdoublex::RXDOUBLEX_SPEC>;
#[doc = "RX Buffer Double Data Extended Register"]
pub mod rxdoublex;
#[doc = "RXDOUBLE register accessor: an alias for `Reg<RXDOUBLE_SPEC>`"]
pub type RXDOUBLE = crate::Reg<rxdouble::RXDOUBLE_SPEC>;
#[doc = "RX FIFO Double Data Register"]
pub mod rxdouble;
#[doc = "RXDATAXP register accessor: an alias for `Reg<RXDATAXP_SPEC>`"]
pub type RXDATAXP = crate::Reg<rxdataxp::RXDATAXP_SPEC>;
#[doc = "RX Buffer Data Extended Peek Register"]
pub mod rxdataxp;
#[doc = "RXDOUBLEXP register accessor: an alias for `Reg<RXDOUBLEXP_SPEC>`"]
pub type RXDOUBLEXP = crate::Reg<rxdoublexp::RXDOUBLEXP_SPEC>;
#[doc = "RX Buffer Double Data Extended Peek Register"]
pub mod rxdoublexp;
#[doc = "TXDATAX register accessor: an alias for `Reg<TXDATAX_SPEC>`"]
pub type TXDATAX = crate::Reg<txdatax::TXDATAX_SPEC>;
#[doc = "TX Buffer Data Extended Register"]
pub mod txdatax;
#[doc = "TXDATA register accessor: an alias for `Reg<TXDATA_SPEC>`"]
pub type TXDATA = crate::Reg<txdata::TXDATA_SPEC>;
#[doc = "TX Buffer Data Register"]
pub mod txdata;
#[doc = "TXDOUBLEX register accessor: an alias for `Reg<TXDOUBLEX_SPEC>`"]
pub type TXDOUBLEX = crate::Reg<txdoublex::TXDOUBLEX_SPEC>;
#[doc = "TX Buffer Double Data Extended Register"]
pub mod txdoublex;
#[doc = "TXDOUBLE register accessor: an alias for `Reg<TXDOUBLE_SPEC>`"]
pub type TXDOUBLE = crate::Reg<txdouble::TXDOUBLE_SPEC>;
#[doc = "TX Buffer Double Data Register"]
pub mod txdouble;
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
#[doc = "IRCTRL register accessor: an alias for `Reg<IRCTRL_SPEC>`"]
pub type IRCTRL = crate::Reg<irctrl::IRCTRL_SPEC>;
#[doc = "IrDA Control Register"]
pub mod irctrl;
#[doc = "ROUTE register accessor: an alias for `Reg<ROUTE_SPEC>`"]
pub type ROUTE = crate::Reg<route::ROUTE_SPEC>;
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "INPUT register accessor: an alias for `Reg<INPUT_SPEC>`"]
pub type INPUT = crate::Reg<input::INPUT_SPEC>;
#[doc = "USART Input Register"]
pub mod input;
#[doc = "I2SCTRL register accessor: an alias for `Reg<I2SCTRL_SPEC>`"]
pub type I2SCTRL = crate::Reg<i2sctrl::I2SCTRL_SPEC>;
#[doc = "I2S Control Register"]
pub mod i2sctrl;
