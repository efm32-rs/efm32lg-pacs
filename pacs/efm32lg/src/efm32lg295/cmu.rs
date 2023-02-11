#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CMU Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - High Frequency Core Clock Division Register"]
    pub hfcoreclkdiv: HFCORECLKDIV,
    #[doc = "0x08 - High Frequency Peripheral Clock Division Register"]
    pub hfperclkdiv: HFPERCLKDIV,
    #[doc = "0x0c - HFRCO Control Register"]
    pub hfrcoctrl: HFRCOCTRL,
    #[doc = "0x10 - LFRCO Control Register"]
    pub lfrcoctrl: LFRCOCTRL,
    #[doc = "0x14 - AUXHFRCO Control Register"]
    pub auxhfrcoctrl: AUXHFRCOCTRL,
    #[doc = "0x18 - Calibration Control Register"]
    pub calctrl: CALCTRL,
    #[doc = "0x1c - Calibration Counter Register"]
    pub calcnt: CALCNT,
    #[doc = "0x20 - Oscillator Enable/Disable Command Register"]
    pub oscencmd: OSCENCMD,
    #[doc = "0x24 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x28 - Low Frequency Clock Select Register"]
    pub lfclksel: LFCLKSEL,
    #[doc = "0x2c - Status Register"]
    pub status: STATUS,
    #[doc = "0x30 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x34 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x38 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x3c - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x40 - High Frequency Core Clock Enable Register 0"]
    pub hfcoreclken0: HFCORECLKEN0,
    #[doc = "0x44 - High Frequency Peripheral Clock Enable Register 0"]
    pub hfperclken0: HFPERCLKEN0,
    _reserved18: [u8; 0x08],
    #[doc = "0x50 - Synchronization Busy Register"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x54 - Freeze Register"]
    pub freeze: FREEZE,
    #[doc = "0x58 - Low Frequency A Clock Enable Register 0 (Async Reg)"]
    pub lfaclken0: LFACLKEN0,
    _reserved21: [u8; 0x04],
    #[doc = "0x60 - Low Frequency B Clock Enable Register 0 (Async Reg)"]
    pub lfbclken0: LFBCLKEN0,
    _reserved22: [u8; 0x04],
    #[doc = "0x68 - Low Frequency A Prescaler Register 0 (Async Reg)"]
    pub lfapresc0: LFAPRESC0,
    _reserved23: [u8; 0x04],
    #[doc = "0x70 - Low Frequency B Prescaler Register 0 (Async Reg)"]
    pub lfbpresc0: LFBPRESC0,
    _reserved24: [u8; 0x04],
    #[doc = "0x78 - PCNT Control Register"]
    pub pcntctrl: PCNTCTRL,
    _reserved25: [u8; 0x04],
    #[doc = "0x80 - I/O Routing Register"]
    pub route: ROUTE,
    #[doc = "0x84 - Configuration Lock Register"]
    pub lock: LOCK,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "CMU Control Register"]
pub mod ctrl;
#[doc = "HFCORECLKDIV (rw) register accessor: an alias for `Reg<HFCORECLKDIV_SPEC>`"]
pub type HFCORECLKDIV = crate::Reg<hfcoreclkdiv::HFCORECLKDIV_SPEC>;
#[doc = "High Frequency Core Clock Division Register"]
pub mod hfcoreclkdiv;
#[doc = "HFPERCLKDIV (rw) register accessor: an alias for `Reg<HFPERCLKDIV_SPEC>`"]
pub type HFPERCLKDIV = crate::Reg<hfperclkdiv::HFPERCLKDIV_SPEC>;
#[doc = "High Frequency Peripheral Clock Division Register"]
pub mod hfperclkdiv;
#[doc = "HFRCOCTRL (rw) register accessor: an alias for `Reg<HFRCOCTRL_SPEC>`"]
pub type HFRCOCTRL = crate::Reg<hfrcoctrl::HFRCOCTRL_SPEC>;
#[doc = "HFRCO Control Register"]
pub mod hfrcoctrl;
#[doc = "LFRCOCTRL (rw) register accessor: an alias for `Reg<LFRCOCTRL_SPEC>`"]
pub type LFRCOCTRL = crate::Reg<lfrcoctrl::LFRCOCTRL_SPEC>;
#[doc = "LFRCO Control Register"]
pub mod lfrcoctrl;
#[doc = "AUXHFRCOCTRL (rw) register accessor: an alias for `Reg<AUXHFRCOCTRL_SPEC>`"]
pub type AUXHFRCOCTRL = crate::Reg<auxhfrcoctrl::AUXHFRCOCTRL_SPEC>;
#[doc = "AUXHFRCO Control Register"]
pub mod auxhfrcoctrl;
#[doc = "CALCTRL (rw) register accessor: an alias for `Reg<CALCTRL_SPEC>`"]
pub type CALCTRL = crate::Reg<calctrl::CALCTRL_SPEC>;
#[doc = "Calibration Control Register"]
pub mod calctrl;
#[doc = "CALCNT (rw) register accessor: an alias for `Reg<CALCNT_SPEC>`"]
pub type CALCNT = crate::Reg<calcnt::CALCNT_SPEC>;
#[doc = "Calibration Counter Register"]
pub mod calcnt;
#[doc = "OSCENCMD (w) register accessor: an alias for `Reg<OSCENCMD_SPEC>`"]
pub type OSCENCMD = crate::Reg<oscencmd::OSCENCMD_SPEC>;
#[doc = "Oscillator Enable/Disable Command Register"]
pub mod oscencmd;
#[doc = "CMD (w) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "LFCLKSEL (rw) register accessor: an alias for `Reg<LFCLKSEL_SPEC>`"]
pub type LFCLKSEL = crate::Reg<lfclksel::LFCLKSEL_SPEC>;
#[doc = "Low Frequency Clock Select Register"]
pub mod lfclksel;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
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
#[doc = "HFCORECLKEN0 (rw) register accessor: an alias for `Reg<HFCORECLKEN0_SPEC>`"]
pub type HFCORECLKEN0 = crate::Reg<hfcoreclken0::HFCORECLKEN0_SPEC>;
#[doc = "High Frequency Core Clock Enable Register 0"]
pub mod hfcoreclken0;
#[doc = "HFPERCLKEN0 (rw) register accessor: an alias for `Reg<HFPERCLKEN0_SPEC>`"]
pub type HFPERCLKEN0 = crate::Reg<hfperclken0::HFPERCLKEN0_SPEC>;
#[doc = "High Frequency Peripheral Clock Enable Register 0"]
pub mod hfperclken0;
#[doc = "SYNCBUSY (r) register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "FREEZE (rw) register accessor: an alias for `Reg<FREEZE_SPEC>`"]
pub type FREEZE = crate::Reg<freeze::FREEZE_SPEC>;
#[doc = "Freeze Register"]
pub mod freeze;
#[doc = "LFACLKEN0 (rw) register accessor: an alias for `Reg<LFACLKEN0_SPEC>`"]
pub type LFACLKEN0 = crate::Reg<lfaclken0::LFACLKEN0_SPEC>;
#[doc = "Low Frequency A Clock Enable Register 0 (Async Reg)"]
pub mod lfaclken0;
#[doc = "LFBCLKEN0 (rw) register accessor: an alias for `Reg<LFBCLKEN0_SPEC>`"]
pub type LFBCLKEN0 = crate::Reg<lfbclken0::LFBCLKEN0_SPEC>;
#[doc = "Low Frequency B Clock Enable Register 0 (Async Reg)"]
pub mod lfbclken0;
#[doc = "LFAPRESC0 (rw) register accessor: an alias for `Reg<LFAPRESC0_SPEC>`"]
pub type LFAPRESC0 = crate::Reg<lfapresc0::LFAPRESC0_SPEC>;
#[doc = "Low Frequency A Prescaler Register 0 (Async Reg)"]
pub mod lfapresc0;
#[doc = "LFBPRESC0 (rw) register accessor: an alias for `Reg<LFBPRESC0_SPEC>`"]
pub type LFBPRESC0 = crate::Reg<lfbpresc0::LFBPRESC0_SPEC>;
#[doc = "Low Frequency B Prescaler Register 0 (Async Reg)"]
pub mod lfbpresc0;
#[doc = "PCNTCTRL (rw) register accessor: an alias for `Reg<PCNTCTRL_SPEC>`"]
pub type PCNTCTRL = crate::Reg<pcntctrl::PCNTCTRL_SPEC>;
#[doc = "PCNT Control Register"]
pub mod pcntctrl;
#[doc = "ROUTE (rw) register accessor: an alias for `Reg<ROUTE_SPEC>`"]
pub type ROUTE = crate::Reg<route::ROUTE_SPEC>;
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "LOCK (rw) register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Configuration Lock Register"]
pub mod lock;
