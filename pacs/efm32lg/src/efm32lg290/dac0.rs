#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Status Register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x08 - Channel 0 Control Register"]
    pub ch0ctrl: crate::Reg<ch0ctrl::CH0CTRL_SPEC>,
    #[doc = "0x0c - Channel 1 Control Register"]
    pub ch1ctrl: crate::Reg<ch1ctrl::CH1CTRL_SPEC>,
    #[doc = "0x10 - Interrupt Enable Register"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    #[doc = "0x14 - Interrupt Flag Register"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x18 - Interrupt Flag Set Register"]
    pub ifs: crate::Reg<ifs::IFS_SPEC>,
    #[doc = "0x1c - Interrupt Flag Clear Register"]
    pub ifc: crate::Reg<ifc::IFC_SPEC>,
    #[doc = "0x20 - Channel 0 Data Register"]
    pub ch0data: crate::Reg<ch0data::CH0DATA_SPEC>,
    #[doc = "0x24 - Channel 1 Data Register"]
    pub ch1data: crate::Reg<ch1data::CH1DATA_SPEC>,
    #[doc = "0x28 - Combined Data Register"]
    pub combdata: crate::Reg<combdata::COMBDATA_SPEC>,
    #[doc = "0x2c - Calibration Register"]
    pub cal: crate::Reg<cal::CAL_SPEC>,
    #[doc = "0x30 - Bias Programming Register"]
    pub biasprog: crate::Reg<biasprog::BIASPROG_SPEC>,
    _reserved13: [u8; 0x20],
    #[doc = "0x54 - Operational Amplifier Control Register"]
    pub opactrl: crate::Reg<opactrl::OPACTRL_SPEC>,
    #[doc = "0x58 - Operational Amplifier Offset Register"]
    pub opaoffset: crate::Reg<opaoffset::OPAOFFSET_SPEC>,
    #[doc = "0x5c - Operational Amplifier Mux Configuration Register"]
    pub opa0mux: crate::Reg<opa0mux::OPA0MUX_SPEC>,
    #[doc = "0x60 - Operational Amplifier Mux Configuration Register"]
    pub opa1mux: crate::Reg<opa1mux::OPA1MUX_SPEC>,
    #[doc = "0x64 - Operational Amplifier Mux Configuration Register"]
    pub opa2mux: crate::Reg<opa2mux::OPA2MUX_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "CH0CTRL register accessor: an alias for `Reg<CH0CTRL_SPEC>`"]
pub type CH0CTRL = crate::Reg<ch0ctrl::CH0CTRL_SPEC>;
#[doc = "Channel 0 Control Register"]
pub mod ch0ctrl;
#[doc = "CH1CTRL register accessor: an alias for `Reg<CH1CTRL_SPEC>`"]
pub type CH1CTRL = crate::Reg<ch1ctrl::CH1CTRL_SPEC>;
#[doc = "Channel 1 Control Register"]
pub mod ch1ctrl;
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
#[doc = "CH0DATA register accessor: an alias for `Reg<CH0DATA_SPEC>`"]
pub type CH0DATA = crate::Reg<ch0data::CH0DATA_SPEC>;
#[doc = "Channel 0 Data Register"]
pub mod ch0data;
#[doc = "CH1DATA register accessor: an alias for `Reg<CH1DATA_SPEC>`"]
pub type CH1DATA = crate::Reg<ch1data::CH1DATA_SPEC>;
#[doc = "Channel 1 Data Register"]
pub mod ch1data;
#[doc = "COMBDATA register accessor: an alias for `Reg<COMBDATA_SPEC>`"]
pub type COMBDATA = crate::Reg<combdata::COMBDATA_SPEC>;
#[doc = "Combined Data Register"]
pub mod combdata;
#[doc = "CAL register accessor: an alias for `Reg<CAL_SPEC>`"]
pub type CAL = crate::Reg<cal::CAL_SPEC>;
#[doc = "Calibration Register"]
pub mod cal;
#[doc = "BIASPROG register accessor: an alias for `Reg<BIASPROG_SPEC>`"]
pub type BIASPROG = crate::Reg<biasprog::BIASPROG_SPEC>;
#[doc = "Bias Programming Register"]
pub mod biasprog;
#[doc = "OPACTRL register accessor: an alias for `Reg<OPACTRL_SPEC>`"]
pub type OPACTRL = crate::Reg<opactrl::OPACTRL_SPEC>;
#[doc = "Operational Amplifier Control Register"]
pub mod opactrl;
#[doc = "OPAOFFSET register accessor: an alias for `Reg<OPAOFFSET_SPEC>`"]
pub type OPAOFFSET = crate::Reg<opaoffset::OPAOFFSET_SPEC>;
#[doc = "Operational Amplifier Offset Register"]
pub mod opaoffset;
#[doc = "OPA0MUX register accessor: an alias for `Reg<OPA0MUX_SPEC>`"]
pub type OPA0MUX = crate::Reg<opa0mux::OPA0MUX_SPEC>;
#[doc = "Operational Amplifier Mux Configuration Register"]
pub mod opa0mux;
#[doc = "OPA1MUX register accessor: an alias for `Reg<OPA1MUX_SPEC>`"]
pub type OPA1MUX = crate::Reg<opa1mux::OPA1MUX_SPEC>;
#[doc = "Operational Amplifier Mux Configuration Register"]
pub mod opa1mux;
#[doc = "OPA2MUX register accessor: an alias for `Reg<OPA2MUX_SPEC>`"]
pub type OPA2MUX = crate::Reg<opa2mux::OPA2MUX_SPEC>;
#[doc = "Operational Amplifier Mux Configuration Register"]
pub mod opa2mux;
