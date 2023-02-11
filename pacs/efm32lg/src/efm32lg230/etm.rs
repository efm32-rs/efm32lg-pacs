#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Main Control Register"]
    pub etmcr: ETMCR,
    #[doc = "0x04 - Configuration Code Register"]
    pub etmccr: ETMCCR,
    #[doc = "0x08 - ETM Trigger Event Register"]
    pub etmtrigger: ETMTRIGGER,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - ETM Status Register"]
    pub etmsr: ETMSR,
    #[doc = "0x14 - ETM System Configuration Register"]
    pub etmscr: ETMSCR,
    _reserved5: [u8; 0x08],
    #[doc = "0x20 - ETM TraceEnable Event Register"]
    pub etmteevr: ETMTEEVR,
    #[doc = "0x24 - ETM Trace control Register"]
    pub etmtecr1: ETMTECR1,
    _reserved7: [u8; 0x04],
    #[doc = "0x2c - ETM Fifo Full Level Register"]
    pub etmfflr: ETMFFLR,
    _reserved8: [u8; 0x0110],
    #[doc = "0x140 - Counter Reload Value"]
    pub etmcntrldvr1: ETMCNTRLDVR1,
    _reserved9: [u8; 0x9c],
    #[doc = "0x1e0 - Synchronisation Frequency Register"]
    pub etmsyncfr: ETMSYNCFR,
    #[doc = "0x1e4 - ID Register"]
    pub etmidr: ETMIDR,
    #[doc = "0x1e8 - Configuration Code Extension Register"]
    pub etmccer: ETMCCER,
    _reserved12: [u8; 0x04],
    #[doc = "0x1f0 - TraceEnable Start/Stop EmbeddedICE Control Register"]
    pub etmtesseicr: ETMTESSEICR,
    _reserved13: [u8; 0x04],
    #[doc = "0x1f8 - Timestamp Event Register"]
    pub etmtsevr: ETMTSEVR,
    _reserved14: [u8; 0x04],
    #[doc = "0x200 - CoreSight Trace ID Register"]
    pub etmtraceidr: ETMTRACEIDR,
    _reserved15: [u8; 0x04],
    #[doc = "0x208 - ETM ID Register 2"]
    pub etmidr2: ETMIDR2,
    _reserved16: [u8; 0x0108],
    #[doc = "0x314 - Device Power-down Status Register"]
    pub etmpdsr: ETMPDSR,
    _reserved17: [u8; 0x0bc8],
    #[doc = "0xee0 - Integration Test Miscellaneous Inputs Register"]
    pub etmiscin: ETMISCIN,
    _reserved18: [u8; 0x04],
    #[doc = "0xee8 - Integration Test Trigger Out Register"]
    pub ittrigout: ITTRIGOUT,
    _reserved19: [u8; 0x04],
    #[doc = "0xef0 - ETM Integration Test ATB Control 2 Register"]
    pub etmitatbctr2: ETMITATBCTR2,
    _reserved20: [u8; 0x04],
    #[doc = "0xef8 - ETM Integration Test ATB Control 0 Register"]
    pub etmitatbctr0: ETMITATBCTR0,
    _reserved21: [u8; 0x04],
    #[doc = "0xf00 - ETM Integration Control Register"]
    pub etmitctrl: ETMITCTRL,
    _reserved22: [u8; 0x9c],
    #[doc = "0xfa0 - ETM Claim Tag Set Register"]
    pub etmclaimset: ETMCLAIMSET,
    #[doc = "0xfa4 - ETM Claim Tag Clear Register"]
    pub etmclaimclr: ETMCLAIMCLR,
    _reserved24: [u8; 0x08],
    #[doc = "0xfb0 - ETM Lock Access Register"]
    pub etmlar: ETMLAR,
    #[doc = "0xfb4 - Lock Status Register"]
    pub etmlsr: ETMLSR,
    #[doc = "0xfb8 - ETM Authentication Status Register"]
    pub etmauthstatus: ETMAUTHSTATUS,
    _reserved27: [u8; 0x10],
    #[doc = "0xfcc - CoreSight Device Type Register"]
    pub etmdevtype: ETMDEVTYPE,
    #[doc = "0xfd0 - Peripheral ID4 Register"]
    pub etmpidr4: ETMPIDR4,
    #[doc = "0xfd4 - Peripheral ID5 Register"]
    pub etmpidr5: ETMPIDR5,
    #[doc = "0xfd8 - Peripheral ID6 Register"]
    pub etmpidr6: ETMPIDR6,
    #[doc = "0xfdc - Peripheral ID7 Register"]
    pub etmpidr7: ETMPIDR7,
    #[doc = "0xfe0 - Peripheral ID0 Register"]
    pub etmpidr0: ETMPIDR0,
    #[doc = "0xfe4 - Peripheral ID1 Register"]
    pub etmpidr1: ETMPIDR1,
    #[doc = "0xfe8 - Peripheral ID2 Register"]
    pub etmpidr2: ETMPIDR2,
    #[doc = "0xfec - Peripheral ID3 Register"]
    pub etmpidr3: ETMPIDR3,
    #[doc = "0xff0 - Component ID0 Register"]
    pub etmcidr0: ETMCIDR0,
    #[doc = "0xff4 - Component ID1 Register"]
    pub etmcidr1: ETMCIDR1,
    #[doc = "0xff8 - Component ID2 Register"]
    pub etmcidr2: ETMCIDR2,
    #[doc = "0xffc - Component ID3 Register"]
    pub etmcidr3: ETMCIDR3,
}
#[doc = "ETMCR (rw) register accessor: an alias for `Reg<ETMCR_SPEC>`"]
pub type ETMCR = crate::Reg<etmcr::ETMCR_SPEC>;
#[doc = "Main Control Register"]
pub mod etmcr;
#[doc = "ETMCCR (r) register accessor: an alias for `Reg<ETMCCR_SPEC>`"]
pub type ETMCCR = crate::Reg<etmccr::ETMCCR_SPEC>;
#[doc = "Configuration Code Register"]
pub mod etmccr;
#[doc = "ETMTRIGGER (rw) register accessor: an alias for `Reg<ETMTRIGGER_SPEC>`"]
pub type ETMTRIGGER = crate::Reg<etmtrigger::ETMTRIGGER_SPEC>;
#[doc = "ETM Trigger Event Register"]
pub mod etmtrigger;
#[doc = "ETMSR (rw) register accessor: an alias for `Reg<ETMSR_SPEC>`"]
pub type ETMSR = crate::Reg<etmsr::ETMSR_SPEC>;
#[doc = "ETM Status Register"]
pub mod etmsr;
#[doc = "ETMSCR (r) register accessor: an alias for `Reg<ETMSCR_SPEC>`"]
pub type ETMSCR = crate::Reg<etmscr::ETMSCR_SPEC>;
#[doc = "ETM System Configuration Register"]
pub mod etmscr;
#[doc = "ETMTEEVR (rw) register accessor: an alias for `Reg<ETMTEEVR_SPEC>`"]
pub type ETMTEEVR = crate::Reg<etmteevr::ETMTEEVR_SPEC>;
#[doc = "ETM TraceEnable Event Register"]
pub mod etmteevr;
#[doc = "ETMTECR1 (rw) register accessor: an alias for `Reg<ETMTECR1_SPEC>`"]
pub type ETMTECR1 = crate::Reg<etmtecr1::ETMTECR1_SPEC>;
#[doc = "ETM Trace control Register"]
pub mod etmtecr1;
#[doc = "ETMFFLR (rw) register accessor: an alias for `Reg<ETMFFLR_SPEC>`"]
pub type ETMFFLR = crate::Reg<etmfflr::ETMFFLR_SPEC>;
#[doc = "ETM Fifo Full Level Register"]
pub mod etmfflr;
#[doc = "ETMCNTRLDVR1 (rw) register accessor: an alias for `Reg<ETMCNTRLDVR1_SPEC>`"]
pub type ETMCNTRLDVR1 = crate::Reg<etmcntrldvr1::ETMCNTRLDVR1_SPEC>;
#[doc = "Counter Reload Value"]
pub mod etmcntrldvr1;
#[doc = "ETMSYNCFR (rw) register accessor: an alias for `Reg<ETMSYNCFR_SPEC>`"]
pub type ETMSYNCFR = crate::Reg<etmsyncfr::ETMSYNCFR_SPEC>;
#[doc = "Synchronisation Frequency Register"]
pub mod etmsyncfr;
#[doc = "ETMIDR (r) register accessor: an alias for `Reg<ETMIDR_SPEC>`"]
pub type ETMIDR = crate::Reg<etmidr::ETMIDR_SPEC>;
#[doc = "ID Register"]
pub mod etmidr;
#[doc = "ETMCCER (r) register accessor: an alias for `Reg<ETMCCER_SPEC>`"]
pub type ETMCCER = crate::Reg<etmccer::ETMCCER_SPEC>;
#[doc = "Configuration Code Extension Register"]
pub mod etmccer;
#[doc = "ETMTESSEICR (rw) register accessor: an alias for `Reg<ETMTESSEICR_SPEC>`"]
pub type ETMTESSEICR = crate::Reg<etmtesseicr::ETMTESSEICR_SPEC>;
#[doc = "TraceEnable Start/Stop EmbeddedICE Control Register"]
pub mod etmtesseicr;
#[doc = "ETMTSEVR (rw) register accessor: an alias for `Reg<ETMTSEVR_SPEC>`"]
pub type ETMTSEVR = crate::Reg<etmtsevr::ETMTSEVR_SPEC>;
#[doc = "Timestamp Event Register"]
pub mod etmtsevr;
#[doc = "ETMTRACEIDR (rw) register accessor: an alias for `Reg<ETMTRACEIDR_SPEC>`"]
pub type ETMTRACEIDR = crate::Reg<etmtraceidr::ETMTRACEIDR_SPEC>;
#[doc = "CoreSight Trace ID Register"]
pub mod etmtraceidr;
#[doc = "ETMIDR2 (r) register accessor: an alias for `Reg<ETMIDR2_SPEC>`"]
pub type ETMIDR2 = crate::Reg<etmidr2::ETMIDR2_SPEC>;
#[doc = "ETM ID Register 2"]
pub mod etmidr2;
#[doc = "ETMPDSR (r) register accessor: an alias for `Reg<ETMPDSR_SPEC>`"]
pub type ETMPDSR = crate::Reg<etmpdsr::ETMPDSR_SPEC>;
#[doc = "Device Power-down Status Register"]
pub mod etmpdsr;
#[doc = "ETMISCIN (rw) register accessor: an alias for `Reg<ETMISCIN_SPEC>`"]
pub type ETMISCIN = crate::Reg<etmiscin::ETMISCIN_SPEC>;
#[doc = "Integration Test Miscellaneous Inputs Register"]
pub mod etmiscin;
#[doc = "ITTRIGOUT (w) register accessor: an alias for `Reg<ITTRIGOUT_SPEC>`"]
pub type ITTRIGOUT = crate::Reg<ittrigout::ITTRIGOUT_SPEC>;
#[doc = "Integration Test Trigger Out Register"]
pub mod ittrigout;
#[doc = "ETMITATBCTR2 (r) register accessor: an alias for `Reg<ETMITATBCTR2_SPEC>`"]
pub type ETMITATBCTR2 = crate::Reg<etmitatbctr2::ETMITATBCTR2_SPEC>;
#[doc = "ETM Integration Test ATB Control 2 Register"]
pub mod etmitatbctr2;
#[doc = "ETMITATBCTR0 (w) register accessor: an alias for `Reg<ETMITATBCTR0_SPEC>`"]
pub type ETMITATBCTR0 = crate::Reg<etmitatbctr0::ETMITATBCTR0_SPEC>;
#[doc = "ETM Integration Test ATB Control 0 Register"]
pub mod etmitatbctr0;
#[doc = "ETMITCTRL (rw) register accessor: an alias for `Reg<ETMITCTRL_SPEC>`"]
pub type ETMITCTRL = crate::Reg<etmitctrl::ETMITCTRL_SPEC>;
#[doc = "ETM Integration Control Register"]
pub mod etmitctrl;
#[doc = "ETMCLAIMSET (rw) register accessor: an alias for `Reg<ETMCLAIMSET_SPEC>`"]
pub type ETMCLAIMSET = crate::Reg<etmclaimset::ETMCLAIMSET_SPEC>;
#[doc = "ETM Claim Tag Set Register"]
pub mod etmclaimset;
#[doc = "ETMCLAIMCLR (rw) register accessor: an alias for `Reg<ETMCLAIMCLR_SPEC>`"]
pub type ETMCLAIMCLR = crate::Reg<etmclaimclr::ETMCLAIMCLR_SPEC>;
#[doc = "ETM Claim Tag Clear Register"]
pub mod etmclaimclr;
#[doc = "ETMLAR (rw) register accessor: an alias for `Reg<ETMLAR_SPEC>`"]
pub type ETMLAR = crate::Reg<etmlar::ETMLAR_SPEC>;
#[doc = "ETM Lock Access Register"]
pub mod etmlar;
#[doc = "ETMLSR (r) register accessor: an alias for `Reg<ETMLSR_SPEC>`"]
pub type ETMLSR = crate::Reg<etmlsr::ETMLSR_SPEC>;
#[doc = "Lock Status Register"]
pub mod etmlsr;
#[doc = "ETMAUTHSTATUS (r) register accessor: an alias for `Reg<ETMAUTHSTATUS_SPEC>`"]
pub type ETMAUTHSTATUS = crate::Reg<etmauthstatus::ETMAUTHSTATUS_SPEC>;
#[doc = "ETM Authentication Status Register"]
pub mod etmauthstatus;
#[doc = "ETMDEVTYPE (r) register accessor: an alias for `Reg<ETMDEVTYPE_SPEC>`"]
pub type ETMDEVTYPE = crate::Reg<etmdevtype::ETMDEVTYPE_SPEC>;
#[doc = "CoreSight Device Type Register"]
pub mod etmdevtype;
#[doc = "ETMPIDR4 (r) register accessor: an alias for `Reg<ETMPIDR4_SPEC>`"]
pub type ETMPIDR4 = crate::Reg<etmpidr4::ETMPIDR4_SPEC>;
#[doc = "Peripheral ID4 Register"]
pub mod etmpidr4;
#[doc = "ETMPIDR5 (w) register accessor: an alias for `Reg<ETMPIDR5_SPEC>`"]
pub type ETMPIDR5 = crate::Reg<etmpidr5::ETMPIDR5_SPEC>;
#[doc = "Peripheral ID5 Register"]
pub mod etmpidr5;
#[doc = "ETMPIDR6 (w) register accessor: an alias for `Reg<ETMPIDR6_SPEC>`"]
pub type ETMPIDR6 = crate::Reg<etmpidr6::ETMPIDR6_SPEC>;
#[doc = "Peripheral ID6 Register"]
pub mod etmpidr6;
#[doc = "ETMPIDR7 (w) register accessor: an alias for `Reg<ETMPIDR7_SPEC>`"]
pub type ETMPIDR7 = crate::Reg<etmpidr7::ETMPIDR7_SPEC>;
#[doc = "Peripheral ID7 Register"]
pub mod etmpidr7;
#[doc = "ETMPIDR0 (r) register accessor: an alias for `Reg<ETMPIDR0_SPEC>`"]
pub type ETMPIDR0 = crate::Reg<etmpidr0::ETMPIDR0_SPEC>;
#[doc = "Peripheral ID0 Register"]
pub mod etmpidr0;
#[doc = "ETMPIDR1 (r) register accessor: an alias for `Reg<ETMPIDR1_SPEC>`"]
pub type ETMPIDR1 = crate::Reg<etmpidr1::ETMPIDR1_SPEC>;
#[doc = "Peripheral ID1 Register"]
pub mod etmpidr1;
#[doc = "ETMPIDR2 (r) register accessor: an alias for `Reg<ETMPIDR2_SPEC>`"]
pub type ETMPIDR2 = crate::Reg<etmpidr2::ETMPIDR2_SPEC>;
#[doc = "Peripheral ID2 Register"]
pub mod etmpidr2;
#[doc = "ETMPIDR3 (r) register accessor: an alias for `Reg<ETMPIDR3_SPEC>`"]
pub type ETMPIDR3 = crate::Reg<etmpidr3::ETMPIDR3_SPEC>;
#[doc = "Peripheral ID3 Register"]
pub mod etmpidr3;
#[doc = "ETMCIDR0 (r) register accessor: an alias for `Reg<ETMCIDR0_SPEC>`"]
pub type ETMCIDR0 = crate::Reg<etmcidr0::ETMCIDR0_SPEC>;
#[doc = "Component ID0 Register"]
pub mod etmcidr0;
#[doc = "ETMCIDR1 (r) register accessor: an alias for `Reg<ETMCIDR1_SPEC>`"]
pub type ETMCIDR1 = crate::Reg<etmcidr1::ETMCIDR1_SPEC>;
#[doc = "Component ID1 Register"]
pub mod etmcidr1;
#[doc = "ETMCIDR2 (r) register accessor: an alias for `Reg<ETMCIDR2_SPEC>`"]
pub type ETMCIDR2 = crate::Reg<etmcidr2::ETMCIDR2_SPEC>;
#[doc = "Component ID2 Register"]
pub mod etmcidr2;
#[doc = "ETMCIDR3 (r) register accessor: an alias for `Reg<ETMCIDR3_SPEC>`"]
pub type ETMCIDR3 = crate::Reg<etmcidr3::ETMCIDR3_SPEC>;
#[doc = "Component ID3 Register"]
pub mod etmcidr3;
