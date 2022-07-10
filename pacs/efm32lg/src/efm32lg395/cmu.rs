#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CMU Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - High Frequency Core Clock Division Register"]
    pub hfcoreclkdiv: crate::Reg<hfcoreclkdiv::HFCORECLKDIV_SPEC>,
    #[doc = "0x08 - High Frequency Peripheral Clock Division Register"]
    pub hfperclkdiv: crate::Reg<hfperclkdiv::HFPERCLKDIV_SPEC>,
    #[doc = "0x0c - HFRCO Control Register"]
    pub hfrcoctrl: crate::Reg<hfrcoctrl::HFRCOCTRL_SPEC>,
    #[doc = "0x10 - LFRCO Control Register"]
    pub lfrcoctrl: crate::Reg<lfrcoctrl::LFRCOCTRL_SPEC>,
    #[doc = "0x14 - AUXHFRCO Control Register"]
    pub auxhfrcoctrl: crate::Reg<auxhfrcoctrl::AUXHFRCOCTRL_SPEC>,
    #[doc = "0x18 - Calibration Control Register"]
    pub calctrl: crate::Reg<calctrl::CALCTRL_SPEC>,
    #[doc = "0x1c - Calibration Counter Register"]
    pub calcnt: crate::Reg<calcnt::CALCNT_SPEC>,
    #[doc = "0x20 - Oscillator Enable/Disable Command Register"]
    pub oscencmd: crate::Reg<oscencmd::OSCENCMD_SPEC>,
    #[doc = "0x24 - Command Register"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x28 - Low Frequency Clock Select Register"]
    pub lfclksel: crate::Reg<lfclksel::LFCLKSEL_SPEC>,
    #[doc = "0x2c - Status Register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x30 - Interrupt Flag Register"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x34 - Interrupt Flag Set Register"]
    pub ifs: crate::Reg<ifs::IFS_SPEC>,
    #[doc = "0x38 - Interrupt Flag Clear Register"]
    pub ifc: crate::Reg<ifc::IFC_SPEC>,
    #[doc = "0x3c - Interrupt Enable Register"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    #[doc = "0x40 - High Frequency Core Clock Enable Register 0"]
    pub hfcoreclken0: crate::Reg<hfcoreclken0::HFCORECLKEN0_SPEC>,
    #[doc = "0x44 - High Frequency Peripheral Clock Enable Register 0"]
    pub hfperclken0: crate::Reg<hfperclken0::HFPERCLKEN0_SPEC>,
    _reserved18: [u8; 0x08],
    #[doc = "0x50 - Synchronization Busy Register"]
    pub syncbusy: crate::Reg<syncbusy::SYNCBUSY_SPEC>,
    #[doc = "0x54 - Freeze Register"]
    pub freeze: crate::Reg<freeze::FREEZE_SPEC>,
    #[doc = "0x58 - Low Frequency A Clock Enable Register 0 (Async Reg)"]
    pub lfaclken0: crate::Reg<lfaclken0::LFACLKEN0_SPEC>,
    _reserved21: [u8; 0x04],
    #[doc = "0x60 - Low Frequency B Clock Enable Register 0 (Async Reg)"]
    pub lfbclken0: crate::Reg<lfbclken0::LFBCLKEN0_SPEC>,
    _reserved22: [u8; 0x04],
    #[doc = "0x68 - Low Frequency A Prescaler Register 0 (Async Reg)"]
    pub lfapresc0: crate::Reg<lfapresc0::LFAPRESC0_SPEC>,
    _reserved23: [u8; 0x04],
    #[doc = "0x70 - Low Frequency B Prescaler Register 0 (Async Reg)"]
    pub lfbpresc0: crate::Reg<lfbpresc0::LFBPRESC0_SPEC>,
    _reserved24: [u8; 0x04],
    #[doc = "0x78 - PCNT Control Register"]
    pub pcntctrl: crate::Reg<pcntctrl::PCNTCTRL_SPEC>,
    _reserved25: [u8; 0x04],
    #[doc = "0x80 - I/O Routing Register"]
    pub route: crate::Reg<route::ROUTE_SPEC>,
    #[doc = "0x84 - Configuration Lock Register"]
    pub lock: crate::Reg<lock::LOCK_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "CMU Control Register"]
pub mod ctrl;
#[doc = "HFCORECLKDIV register accessor: an alias for `Reg<HFCORECLKDIV_SPEC>`"]
pub type HFCORECLKDIV = crate::Reg<hfcoreclkdiv::HFCORECLKDIV_SPEC>;
#[doc = "High Frequency Core Clock Division Register"]
pub mod hfcoreclkdiv;
#[doc = "HFPERCLKDIV register accessor: an alias for `Reg<HFPERCLKDIV_SPEC>`"]
pub type HFPERCLKDIV = crate::Reg<hfperclkdiv::HFPERCLKDIV_SPEC>;
#[doc = "High Frequency Peripheral Clock Division Register"]
pub mod hfperclkdiv;
#[doc = "HFRCOCTRL register accessor: an alias for `Reg<HFRCOCTRL_SPEC>`"]
pub type HFRCOCTRL = crate::Reg<hfrcoctrl::HFRCOCTRL_SPEC>;
#[doc = "HFRCO Control Register"]
pub mod hfrcoctrl;
#[doc = "LFRCOCTRL register accessor: an alias for `Reg<LFRCOCTRL_SPEC>`"]
pub type LFRCOCTRL = crate::Reg<lfrcoctrl::LFRCOCTRL_SPEC>;
#[doc = "LFRCO Control Register"]
pub mod lfrcoctrl;
#[doc = "AUXHFRCOCTRL register accessor: an alias for `Reg<AUXHFRCOCTRL_SPEC>`"]
pub type AUXHFRCOCTRL = crate::Reg<auxhfrcoctrl::AUXHFRCOCTRL_SPEC>;
#[doc = "AUXHFRCO Control Register"]
pub mod auxhfrcoctrl;
#[doc = "CALCTRL register accessor: an alias for `Reg<CALCTRL_SPEC>`"]
pub type CALCTRL = crate::Reg<calctrl::CALCTRL_SPEC>;
#[doc = "Calibration Control Register"]
pub mod calctrl;
#[doc = "CALCNT register accessor: an alias for `Reg<CALCNT_SPEC>`"]
pub type CALCNT = crate::Reg<calcnt::CALCNT_SPEC>;
#[doc = "Calibration Counter Register"]
pub mod calcnt;
#[doc = "OSCENCMD register accessor: an alias for `Reg<OSCENCMD_SPEC>`"]
pub type OSCENCMD = crate::Reg<oscencmd::OSCENCMD_SPEC>;
#[doc = "Oscillator Enable/Disable Command Register"]
pub mod oscencmd;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "LFCLKSEL register accessor: an alias for `Reg<LFCLKSEL_SPEC>`"]
pub type LFCLKSEL = crate::Reg<lfclksel::LFCLKSEL_SPEC>;
#[doc = "Low Frequency Clock Select Register"]
pub mod lfclksel;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
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
#[doc = "HFCORECLKEN0 register accessor: an alias for `Reg<HFCORECLKEN0_SPEC>`"]
pub type HFCORECLKEN0 = crate::Reg<hfcoreclken0::HFCORECLKEN0_SPEC>;
#[doc = "High Frequency Core Clock Enable Register 0"]
pub mod hfcoreclken0;
#[doc = "HFPERCLKEN0 register accessor: an alias for `Reg<HFPERCLKEN0_SPEC>`"]
pub type HFPERCLKEN0 = crate::Reg<hfperclken0::HFPERCLKEN0_SPEC>;
#[doc = "High Frequency Peripheral Clock Enable Register 0"]
pub mod hfperclken0;
#[doc = "SYNCBUSY register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "FREEZE register accessor: an alias for `Reg<FREEZE_SPEC>`"]
pub type FREEZE = crate::Reg<freeze::FREEZE_SPEC>;
#[doc = "Freeze Register"]
pub mod freeze;
#[doc = "LFACLKEN0 register accessor: an alias for `Reg<LFACLKEN0_SPEC>`"]
pub type LFACLKEN0 = crate::Reg<lfaclken0::LFACLKEN0_SPEC>;
#[doc = "Low Frequency A Clock Enable Register 0 (Async Reg)"]
pub mod lfaclken0;
#[doc = "LFBCLKEN0 register accessor: an alias for `Reg<LFBCLKEN0_SPEC>`"]
pub type LFBCLKEN0 = crate::Reg<lfbclken0::LFBCLKEN0_SPEC>;
#[doc = "Low Frequency B Clock Enable Register 0 (Async Reg)"]
pub mod lfbclken0;
#[doc = "LFAPRESC0 register accessor: an alias for `Reg<LFAPRESC0_SPEC>`"]
pub type LFAPRESC0 = crate::Reg<lfapresc0::LFAPRESC0_SPEC>;
#[doc = "Low Frequency A Prescaler Register 0 (Async Reg)"]
pub mod lfapresc0;
#[doc = "LFBPRESC0 register accessor: an alias for `Reg<LFBPRESC0_SPEC>`"]
pub type LFBPRESC0 = crate::Reg<lfbpresc0::LFBPRESC0_SPEC>;
#[doc = "Low Frequency B Prescaler Register 0 (Async Reg)"]
pub mod lfbpresc0;
#[doc = "PCNTCTRL register accessor: an alias for `Reg<PCNTCTRL_SPEC>`"]
pub type PCNTCTRL = crate::Reg<pcntctrl::PCNTCTRL_SPEC>;
#[doc = "PCNT Control Register"]
pub mod pcntctrl;
#[doc = "ROUTE register accessor: an alias for `Reg<ROUTE_SPEC>`"]
pub type ROUTE = crate::Reg<route::ROUTE_SPEC>;
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "LOCK register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Configuration Lock Register"]
pub mod lock;
