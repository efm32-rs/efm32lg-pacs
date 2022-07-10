#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Status Registers"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x04 - DMA Configuration Register"]
    pub config: crate::Reg<config::CONFIG_SPEC>,
    #[doc = "0x08 - Channel Control Data Base Pointer Register"]
    pub ctrlbase: crate::Reg<ctrlbase::CTRLBASE_SPEC>,
    #[doc = "0x0c - Channel Alternate Control Data Base Pointer Register"]
    pub altctrlbase: crate::Reg<altctrlbase::ALTCTRLBASE_SPEC>,
    #[doc = "0x10 - Channel Wait on Request Status Register"]
    pub chwaitstatus: crate::Reg<chwaitstatus::CHWAITSTATUS_SPEC>,
    #[doc = "0x14 - Channel Software Request Register"]
    pub chswreq: crate::Reg<chswreq::CHSWREQ_SPEC>,
    #[doc = "0x18 - Channel Useburst Set Register"]
    pub chusebursts: crate::Reg<chusebursts::CHUSEBURSTS_SPEC>,
    #[doc = "0x1c - Channel Useburst Clear Register"]
    pub chuseburstc: crate::Reg<chuseburstc::CHUSEBURSTC_SPEC>,
    #[doc = "0x20 - Channel Request Mask Set Register"]
    pub chreqmasks: crate::Reg<chreqmasks::CHREQMASKS_SPEC>,
    #[doc = "0x24 - Channel Request Mask Clear Register"]
    pub chreqmaskc: crate::Reg<chreqmaskc::CHREQMASKC_SPEC>,
    #[doc = "0x28 - Channel Enable Set Register"]
    pub chens: crate::Reg<chens::CHENS_SPEC>,
    #[doc = "0x2c - Channel Enable Clear Register"]
    pub chenc: crate::Reg<chenc::CHENC_SPEC>,
    #[doc = "0x30 - Channel Alternate Set Register"]
    pub chalts: crate::Reg<chalts::CHALTS_SPEC>,
    #[doc = "0x34 - Channel Alternate Clear Register"]
    pub chaltc: crate::Reg<chaltc::CHALTC_SPEC>,
    #[doc = "0x38 - Channel Priority Set Register"]
    pub chpris: crate::Reg<chpris::CHPRIS_SPEC>,
    #[doc = "0x3c - Channel Priority Clear Register"]
    pub chpric: crate::Reg<chpric::CHPRIC_SPEC>,
    _reserved16: [u8; 0x0c],
    #[doc = "0x4c - Bus Error Clear Register"]
    pub errorc: crate::Reg<errorc::ERRORC_SPEC>,
    _reserved17: [u8; 0x0dc0],
    #[doc = "0xe10 - Channel Request Status"]
    pub chreqstatus: crate::Reg<chreqstatus::CHREQSTATUS_SPEC>,
    _reserved18: [u8; 0x04],
    #[doc = "0xe18 - Channel Single Request Status"]
    pub chsreqstatus: crate::Reg<chsreqstatus::CHSREQSTATUS_SPEC>,
    _reserved19: [u8; 0x01e4],
    #[doc = "0x1000 - Interrupt Flag Register"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x1004 - Interrupt Flag Set Register"]
    pub ifs: crate::Reg<ifs::IFS_SPEC>,
    #[doc = "0x1008 - Interrupt Flag Clear Register"]
    pub ifc: crate::Reg<ifc::IFC_SPEC>,
    #[doc = "0x100c - Interrupt Enable register"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    #[doc = "0x1010 - DMA Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x1014 - DMA Retain Descriptor State"]
    pub rds: crate::Reg<rds::RDS_SPEC>,
    _reserved25: [u8; 0x08],
    #[doc = "0x1020 - Channel 0 Loop Register"]
    pub loop0: crate::Reg<loop0::LOOP0_SPEC>,
    #[doc = "0x1024 - Channel 1 Loop Register"]
    pub loop1: crate::Reg<loop1::LOOP1_SPEC>,
    _reserved27: [u8; 0x38],
    #[doc = "0x1060 - Channel 0 Rectangle Register"]
    pub rect0: crate::Reg<rect0::RECT0_SPEC>,
    _reserved28: [u8; 0x9c],
    #[doc = "0x1100 - Channel Control Register"]
    pub ch0_ctrl: crate::Reg<ch0_ctrl::CH0_CTRL_SPEC>,
    #[doc = "0x1104 - Channel Control Register"]
    pub ch1_ctrl: crate::Reg<ch1_ctrl::CH1_CTRL_SPEC>,
    #[doc = "0x1108 - Channel Control Register"]
    pub ch2_ctrl: crate::Reg<ch2_ctrl::CH2_CTRL_SPEC>,
    #[doc = "0x110c - Channel Control Register"]
    pub ch3_ctrl: crate::Reg<ch3_ctrl::CH3_CTRL_SPEC>,
    #[doc = "0x1110 - Channel Control Register"]
    pub ch4_ctrl: crate::Reg<ch4_ctrl::CH4_CTRL_SPEC>,
    #[doc = "0x1114 - Channel Control Register"]
    pub ch5_ctrl: crate::Reg<ch5_ctrl::CH5_CTRL_SPEC>,
    #[doc = "0x1118 - Channel Control Register"]
    pub ch6_ctrl: crate::Reg<ch6_ctrl::CH6_CTRL_SPEC>,
    #[doc = "0x111c - Channel Control Register"]
    pub ch7_ctrl: crate::Reg<ch7_ctrl::CH7_CTRL_SPEC>,
    #[doc = "0x1120 - Channel Control Register"]
    pub ch8_ctrl: crate::Reg<ch8_ctrl::CH8_CTRL_SPEC>,
    #[doc = "0x1124 - Channel Control Register"]
    pub ch9_ctrl: crate::Reg<ch9_ctrl::CH9_CTRL_SPEC>,
    #[doc = "0x1128 - Channel Control Register"]
    pub ch10_ctrl: crate::Reg<ch10_ctrl::CH10_CTRL_SPEC>,
    #[doc = "0x112c - Channel Control Register"]
    pub ch11_ctrl: crate::Reg<ch11_ctrl::CH11_CTRL_SPEC>,
}
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "DMA Status Registers"]
pub mod status;
#[doc = "CONFIG register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "DMA Configuration Register"]
pub mod config;
#[doc = "CTRLBASE register accessor: an alias for `Reg<CTRLBASE_SPEC>`"]
pub type CTRLBASE = crate::Reg<ctrlbase::CTRLBASE_SPEC>;
#[doc = "Channel Control Data Base Pointer Register"]
pub mod ctrlbase;
#[doc = "ALTCTRLBASE register accessor: an alias for `Reg<ALTCTRLBASE_SPEC>`"]
pub type ALTCTRLBASE = crate::Reg<altctrlbase::ALTCTRLBASE_SPEC>;
#[doc = "Channel Alternate Control Data Base Pointer Register"]
pub mod altctrlbase;
#[doc = "CHWAITSTATUS register accessor: an alias for `Reg<CHWAITSTATUS_SPEC>`"]
pub type CHWAITSTATUS = crate::Reg<chwaitstatus::CHWAITSTATUS_SPEC>;
#[doc = "Channel Wait on Request Status Register"]
pub mod chwaitstatus;
#[doc = "CHSWREQ register accessor: an alias for `Reg<CHSWREQ_SPEC>`"]
pub type CHSWREQ = crate::Reg<chswreq::CHSWREQ_SPEC>;
#[doc = "Channel Software Request Register"]
pub mod chswreq;
#[doc = "CHUSEBURSTS register accessor: an alias for `Reg<CHUSEBURSTS_SPEC>`"]
pub type CHUSEBURSTS = crate::Reg<chusebursts::CHUSEBURSTS_SPEC>;
#[doc = "Channel Useburst Set Register"]
pub mod chusebursts;
#[doc = "CHUSEBURSTC register accessor: an alias for `Reg<CHUSEBURSTC_SPEC>`"]
pub type CHUSEBURSTC = crate::Reg<chuseburstc::CHUSEBURSTC_SPEC>;
#[doc = "Channel Useburst Clear Register"]
pub mod chuseburstc;
#[doc = "CHREQMASKS register accessor: an alias for `Reg<CHREQMASKS_SPEC>`"]
pub type CHREQMASKS = crate::Reg<chreqmasks::CHREQMASKS_SPEC>;
#[doc = "Channel Request Mask Set Register"]
pub mod chreqmasks;
#[doc = "CHREQMASKC register accessor: an alias for `Reg<CHREQMASKC_SPEC>`"]
pub type CHREQMASKC = crate::Reg<chreqmaskc::CHREQMASKC_SPEC>;
#[doc = "Channel Request Mask Clear Register"]
pub mod chreqmaskc;
#[doc = "CHENS register accessor: an alias for `Reg<CHENS_SPEC>`"]
pub type CHENS = crate::Reg<chens::CHENS_SPEC>;
#[doc = "Channel Enable Set Register"]
pub mod chens;
#[doc = "CHENC register accessor: an alias for `Reg<CHENC_SPEC>`"]
pub type CHENC = crate::Reg<chenc::CHENC_SPEC>;
#[doc = "Channel Enable Clear Register"]
pub mod chenc;
#[doc = "CHALTS register accessor: an alias for `Reg<CHALTS_SPEC>`"]
pub type CHALTS = crate::Reg<chalts::CHALTS_SPEC>;
#[doc = "Channel Alternate Set Register"]
pub mod chalts;
#[doc = "CHALTC register accessor: an alias for `Reg<CHALTC_SPEC>`"]
pub type CHALTC = crate::Reg<chaltc::CHALTC_SPEC>;
#[doc = "Channel Alternate Clear Register"]
pub mod chaltc;
#[doc = "CHPRIS register accessor: an alias for `Reg<CHPRIS_SPEC>`"]
pub type CHPRIS = crate::Reg<chpris::CHPRIS_SPEC>;
#[doc = "Channel Priority Set Register"]
pub mod chpris;
#[doc = "CHPRIC register accessor: an alias for `Reg<CHPRIC_SPEC>`"]
pub type CHPRIC = crate::Reg<chpric::CHPRIC_SPEC>;
#[doc = "Channel Priority Clear Register"]
pub mod chpric;
#[doc = "ERRORC register accessor: an alias for `Reg<ERRORC_SPEC>`"]
pub type ERRORC = crate::Reg<errorc::ERRORC_SPEC>;
#[doc = "Bus Error Clear Register"]
pub mod errorc;
#[doc = "CHREQSTATUS register accessor: an alias for `Reg<CHREQSTATUS_SPEC>`"]
pub type CHREQSTATUS = crate::Reg<chreqstatus::CHREQSTATUS_SPEC>;
#[doc = "Channel Request Status"]
pub mod chreqstatus;
#[doc = "CHSREQSTATUS register accessor: an alias for `Reg<CHSREQSTATUS_SPEC>`"]
pub type CHSREQSTATUS = crate::Reg<chsreqstatus::CHSREQSTATUS_SPEC>;
#[doc = "Channel Single Request Status"]
pub mod chsreqstatus;
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
#[doc = "Interrupt Enable register"]
pub mod ien;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "DMA Control Register"]
pub mod ctrl;
#[doc = "RDS register accessor: an alias for `Reg<RDS_SPEC>`"]
pub type RDS = crate::Reg<rds::RDS_SPEC>;
#[doc = "DMA Retain Descriptor State"]
pub mod rds;
#[doc = "LOOP0 register accessor: an alias for `Reg<LOOP0_SPEC>`"]
pub type LOOP0 = crate::Reg<loop0::LOOP0_SPEC>;
#[doc = "Channel 0 Loop Register"]
pub mod loop0;
#[doc = "LOOP1 register accessor: an alias for `Reg<LOOP1_SPEC>`"]
pub type LOOP1 = crate::Reg<loop1::LOOP1_SPEC>;
#[doc = "Channel 1 Loop Register"]
pub mod loop1;
#[doc = "RECT0 register accessor: an alias for `Reg<RECT0_SPEC>`"]
pub type RECT0 = crate::Reg<rect0::RECT0_SPEC>;
#[doc = "Channel 0 Rectangle Register"]
pub mod rect0;
#[doc = "CH0_CTRL register accessor: an alias for `Reg<CH0_CTRL_SPEC>`"]
pub type CH0_CTRL = crate::Reg<ch0_ctrl::CH0_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch0_ctrl;
#[doc = "CH1_CTRL register accessor: an alias for `Reg<CH1_CTRL_SPEC>`"]
pub type CH1_CTRL = crate::Reg<ch1_ctrl::CH1_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch1_ctrl;
#[doc = "CH2_CTRL register accessor: an alias for `Reg<CH2_CTRL_SPEC>`"]
pub type CH2_CTRL = crate::Reg<ch2_ctrl::CH2_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch2_ctrl;
#[doc = "CH3_CTRL register accessor: an alias for `Reg<CH3_CTRL_SPEC>`"]
pub type CH3_CTRL = crate::Reg<ch3_ctrl::CH3_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch3_ctrl;
#[doc = "CH4_CTRL register accessor: an alias for `Reg<CH4_CTRL_SPEC>`"]
pub type CH4_CTRL = crate::Reg<ch4_ctrl::CH4_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch4_ctrl;
#[doc = "CH5_CTRL register accessor: an alias for `Reg<CH5_CTRL_SPEC>`"]
pub type CH5_CTRL = crate::Reg<ch5_ctrl::CH5_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch5_ctrl;
#[doc = "CH6_CTRL register accessor: an alias for `Reg<CH6_CTRL_SPEC>`"]
pub type CH6_CTRL = crate::Reg<ch6_ctrl::CH6_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch6_ctrl;
#[doc = "CH7_CTRL register accessor: an alias for `Reg<CH7_CTRL_SPEC>`"]
pub type CH7_CTRL = crate::Reg<ch7_ctrl::CH7_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch7_ctrl;
#[doc = "CH8_CTRL register accessor: an alias for `Reg<CH8_CTRL_SPEC>`"]
pub type CH8_CTRL = crate::Reg<ch8_ctrl::CH8_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch8_ctrl;
#[doc = "CH9_CTRL register accessor: an alias for `Reg<CH9_CTRL_SPEC>`"]
pub type CH9_CTRL = crate::Reg<ch9_ctrl::CH9_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch9_ctrl;
#[doc = "CH10_CTRL register accessor: an alias for `Reg<CH10_CTRL_SPEC>`"]
pub type CH10_CTRL = crate::Reg<ch10_ctrl::CH10_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch10_ctrl;
#[doc = "CH11_CTRL register accessor: an alias for `Reg<CH11_CTRL_SPEC>`"]
pub type CH11_CTRL = crate::Reg<ch11_ctrl::CH11_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch11_ctrl;
