#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Configuration Lock Register"]
    pub lock: LOCK,
    _reserved2: [u8; 0x18],
    #[doc = "0x24 - Auxiliary Control Register"]
    pub auxctrl: AUXCTRL,
    _reserved3: [u8; 0x04],
    #[doc = "0x2c - Energy mode 4 configuration register"]
    pub em4conf: EM4CONF,
    #[doc = "0x30 - Backup Power configuration register"]
    pub buctrl: BUCTRL,
    #[doc = "0x34 - Power connection configuration register"]
    pub pwrconf: PWRCONF,
    #[doc = "0x38 - Backup mode inactive configuration register"]
    pub buinact: BUINACT,
    #[doc = "0x3c - Backup mode active configuration register"]
    pub buact: BUACT,
    #[doc = "0x40 - Status register"]
    pub status: STATUS,
    #[doc = "0x44 - I/O Routing Register"]
    pub route: ROUTE,
    #[doc = "0x48 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x4c - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x50 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x54 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x58 - BU_VIN Backup BOD calibration"]
    pub bubodbuvincal: BUBODBUVINCAL,
    #[doc = "0x5c - Unregulated power Backup BOD calibration"]
    pub bubodunregcal: BUBODUNREGCAL,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "LOCK (rw) register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Configuration Lock Register"]
pub mod lock;
#[doc = "AUXCTRL (rw) register accessor: an alias for `Reg<AUXCTRL_SPEC>`"]
pub type AUXCTRL = crate::Reg<auxctrl::AUXCTRL_SPEC>;
#[doc = "Auxiliary Control Register"]
pub mod auxctrl;
#[doc = "EM4CONF (rw) register accessor: an alias for `Reg<EM4CONF_SPEC>`"]
pub type EM4CONF = crate::Reg<em4conf::EM4CONF_SPEC>;
#[doc = "Energy mode 4 configuration register"]
pub mod em4conf;
#[doc = "BUCTRL (rw) register accessor: an alias for `Reg<BUCTRL_SPEC>`"]
pub type BUCTRL = crate::Reg<buctrl::BUCTRL_SPEC>;
#[doc = "Backup Power configuration register"]
pub mod buctrl;
#[doc = "PWRCONF (rw) register accessor: an alias for `Reg<PWRCONF_SPEC>`"]
pub type PWRCONF = crate::Reg<pwrconf::PWRCONF_SPEC>;
#[doc = "Power connection configuration register"]
pub mod pwrconf;
#[doc = "BUINACT (rw) register accessor: an alias for `Reg<BUINACT_SPEC>`"]
pub type BUINACT = crate::Reg<buinact::BUINACT_SPEC>;
#[doc = "Backup mode inactive configuration register"]
pub mod buinact;
#[doc = "BUACT (rw) register accessor: an alias for `Reg<BUACT_SPEC>`"]
pub type BUACT = crate::Reg<buact::BUACT_SPEC>;
#[doc = "Backup mode active configuration register"]
pub mod buact;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status register"]
pub mod status;
#[doc = "ROUTE (rw) register accessor: an alias for `Reg<ROUTE_SPEC>`"]
pub type ROUTE = crate::Reg<route::ROUTE_SPEC>;
#[doc = "I/O Routing Register"]
pub mod route;
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
#[doc = "BUBODBUVINCAL (rw) register accessor: an alias for `Reg<BUBODBUVINCAL_SPEC>`"]
pub type BUBODBUVINCAL = crate::Reg<bubodbuvincal::BUBODBUVINCAL_SPEC>;
#[doc = "BU_VIN Backup BOD calibration"]
pub mod bubodbuvincal;
#[doc = "BUBODUNREGCAL (rw) register accessor: an alias for `Reg<BUBODUNREGCAL_SPEC>`"]
pub type BUBODUNREGCAL = crate::Reg<bubodunregcal::BUBODUNREGCAL_SPEC>;
#[doc = "Unregulated power Backup BOD calibration"]
pub mod bubodunregcal;
