#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Software Pulse Register"]
    pub swpulse: SWPULSE,
    #[doc = "0x04 - Software Level Register"]
    pub swlevel: SWLEVEL,
    #[doc = "0x08 - I/O Routing Register"]
    pub route: ROUTE,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Channel Control Register"]
    pub ch0_ctrl: CH0_CTRL,
    #[doc = "0x14 - Channel Control Register"]
    pub ch1_ctrl: CH1_CTRL,
    #[doc = "0x18 - Channel Control Register"]
    pub ch2_ctrl: CH2_CTRL,
    #[doc = "0x1c - Channel Control Register"]
    pub ch3_ctrl: CH3_CTRL,
    #[doc = "0x20 - Channel Control Register"]
    pub ch4_ctrl: CH4_CTRL,
    #[doc = "0x24 - Channel Control Register"]
    pub ch5_ctrl: CH5_CTRL,
    #[doc = "0x28 - Channel Control Register"]
    pub ch6_ctrl: CH6_CTRL,
    #[doc = "0x2c - Channel Control Register"]
    pub ch7_ctrl: CH7_CTRL,
    #[doc = "0x30 - Channel Control Register"]
    pub ch8_ctrl: CH8_CTRL,
    #[doc = "0x34 - Channel Control Register"]
    pub ch9_ctrl: CH9_CTRL,
    #[doc = "0x38 - Channel Control Register"]
    pub ch10_ctrl: CH10_CTRL,
    #[doc = "0x3c - Channel Control Register"]
    pub ch11_ctrl: CH11_CTRL,
}
#[doc = "SWPULSE (w) register accessor: an alias for `Reg<SWPULSE_SPEC>`"]
pub type SWPULSE = crate::Reg<swpulse::SWPULSE_SPEC>;
#[doc = "Software Pulse Register"]
pub mod swpulse;
#[doc = "SWLEVEL (rw) register accessor: an alias for `Reg<SWLEVEL_SPEC>`"]
pub type SWLEVEL = crate::Reg<swlevel::SWLEVEL_SPEC>;
#[doc = "Software Level Register"]
pub mod swlevel;
#[doc = "ROUTE (rw) register accessor: an alias for `Reg<ROUTE_SPEC>`"]
pub type ROUTE = crate::Reg<route::ROUTE_SPEC>;
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "CH0_CTRL (rw) register accessor: an alias for `Reg<CH0_CTRL_SPEC>`"]
pub type CH0_CTRL = crate::Reg<ch0_ctrl::CH0_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch0_ctrl;
#[doc = "CH1_CTRL (rw) register accessor: an alias for `Reg<CH1_CTRL_SPEC>`"]
pub type CH1_CTRL = crate::Reg<ch1_ctrl::CH1_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch1_ctrl;
#[doc = "CH2_CTRL (rw) register accessor: an alias for `Reg<CH2_CTRL_SPEC>`"]
pub type CH2_CTRL = crate::Reg<ch2_ctrl::CH2_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch2_ctrl;
#[doc = "CH3_CTRL (rw) register accessor: an alias for `Reg<CH3_CTRL_SPEC>`"]
pub type CH3_CTRL = crate::Reg<ch3_ctrl::CH3_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch3_ctrl;
#[doc = "CH4_CTRL (rw) register accessor: an alias for `Reg<CH4_CTRL_SPEC>`"]
pub type CH4_CTRL = crate::Reg<ch4_ctrl::CH4_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch4_ctrl;
#[doc = "CH5_CTRL (rw) register accessor: an alias for `Reg<CH5_CTRL_SPEC>`"]
pub type CH5_CTRL = crate::Reg<ch5_ctrl::CH5_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch5_ctrl;
#[doc = "CH6_CTRL (rw) register accessor: an alias for `Reg<CH6_CTRL_SPEC>`"]
pub type CH6_CTRL = crate::Reg<ch6_ctrl::CH6_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch6_ctrl;
#[doc = "CH7_CTRL (rw) register accessor: an alias for `Reg<CH7_CTRL_SPEC>`"]
pub type CH7_CTRL = crate::Reg<ch7_ctrl::CH7_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch7_ctrl;
#[doc = "CH8_CTRL (rw) register accessor: an alias for `Reg<CH8_CTRL_SPEC>`"]
pub type CH8_CTRL = crate::Reg<ch8_ctrl::CH8_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch8_ctrl;
#[doc = "CH9_CTRL (rw) register accessor: an alias for `Reg<CH9_CTRL_SPEC>`"]
pub type CH9_CTRL = crate::Reg<ch9_ctrl::CH9_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch9_ctrl;
#[doc = "CH10_CTRL (rw) register accessor: an alias for `Reg<CH10_CTRL_SPEC>`"]
pub type CH10_CTRL = crate::Reg<ch10_ctrl::CH10_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch10_ctrl;
#[doc = "CH11_CTRL (rw) register accessor: an alias for `Reg<CH11_CTRL_SPEC>`"]
pub type CH11_CTRL = crate::Reg<ch11_ctrl::CH11_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch11_ctrl;
