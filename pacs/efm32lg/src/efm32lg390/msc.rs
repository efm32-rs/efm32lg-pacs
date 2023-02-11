#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Memory System Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Read Control Register"]
    pub readctrl: READCTRL,
    #[doc = "0x08 - Write Control Register"]
    pub writectrl: WRITECTRL,
    #[doc = "0x0c - Write Command Register"]
    pub writecmd: WRITECMD,
    #[doc = "0x10 - Page Erase/Write Address Buffer"]
    pub addrb: ADDRB,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - Write Data Register"]
    pub wdata: WDATA,
    #[doc = "0x1c - Status Register"]
    pub status: STATUS,
    _reserved7: [u8; 0x0c],
    #[doc = "0x2c - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x30 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x34 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x38 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x3c - Configuration Lock Register"]
    pub lock: LOCK,
    #[doc = "0x40 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x44 - Cache Hits Performance Counter"]
    pub cachehits: CACHEHITS,
    #[doc = "0x48 - Cache Misses Performance Counter"]
    pub cachemisses: CACHEMISSES,
    _reserved15: [u8; 0x04],
    #[doc = "0x50 - Flash Write and Erase Timebase"]
    pub timebase: TIMEBASE,
    #[doc = "0x54 - Mass Erase Lock Register"]
    pub masslock: MASSLOCK,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Memory System Control Register"]
pub mod ctrl;
#[doc = "READCTRL (rw) register accessor: an alias for `Reg<READCTRL_SPEC>`"]
pub type READCTRL = crate::Reg<readctrl::READCTRL_SPEC>;
#[doc = "Read Control Register"]
pub mod readctrl;
#[doc = "WRITECTRL (rw) register accessor: an alias for `Reg<WRITECTRL_SPEC>`"]
pub type WRITECTRL = crate::Reg<writectrl::WRITECTRL_SPEC>;
#[doc = "Write Control Register"]
pub mod writectrl;
#[doc = "WRITECMD (w) register accessor: an alias for `Reg<WRITECMD_SPEC>`"]
pub type WRITECMD = crate::Reg<writecmd::WRITECMD_SPEC>;
#[doc = "Write Command Register"]
pub mod writecmd;
#[doc = "ADDRB (rw) register accessor: an alias for `Reg<ADDRB_SPEC>`"]
pub type ADDRB = crate::Reg<addrb::ADDRB_SPEC>;
#[doc = "Page Erase/Write Address Buffer"]
pub mod addrb;
#[doc = "WDATA (rw) register accessor: an alias for `Reg<WDATA_SPEC>`"]
pub type WDATA = crate::Reg<wdata::WDATA_SPEC>;
#[doc = "Write Data Register"]
pub mod wdata;
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
#[doc = "LOCK (rw) register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Configuration Lock Register"]
pub mod lock;
#[doc = "CMD (w) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "CACHEHITS (r) register accessor: an alias for `Reg<CACHEHITS_SPEC>`"]
pub type CACHEHITS = crate::Reg<cachehits::CACHEHITS_SPEC>;
#[doc = "Cache Hits Performance Counter"]
pub mod cachehits;
#[doc = "CACHEMISSES (r) register accessor: an alias for `Reg<CACHEMISSES_SPEC>`"]
pub type CACHEMISSES = crate::Reg<cachemisses::CACHEMISSES_SPEC>;
#[doc = "Cache Misses Performance Counter"]
pub mod cachemisses;
#[doc = "TIMEBASE (rw) register accessor: an alias for `Reg<TIMEBASE_SPEC>`"]
pub type TIMEBASE = crate::Reg<timebase::TIMEBASE_SPEC>;
#[doc = "Flash Write and Erase Timebase"]
pub mod timebase;
#[doc = "MASSLOCK (rw) register accessor: an alias for `Reg<MASSLOCK_SPEC>`"]
pub type MASSLOCK = crate::Reg<masslock::MASSLOCK_SPEC>;
#[doc = "Mass Erase Lock Register"]
pub mod masslock;
