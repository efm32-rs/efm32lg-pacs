#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Low power mode configuration"]
    pub lpmode: crate::Reg<lpmode::LPMODE_SPEC>,
    #[doc = "0x08 - Counter Value Register"]
    pub cnt: crate::Reg<cnt::CNT_SPEC>,
    #[doc = "0x0c - Counter Compare Value"]
    pub comp0: crate::Reg<comp0::COMP0_SPEC>,
    #[doc = "0x10 - Backup mode timestamp"]
    pub timestamp: crate::Reg<timestamp::TIMESTAMP_SPEC>,
    #[doc = "0x14 - LFXO"]
    pub lfxofdet: crate::Reg<lfxofdet::LFXOFDET_SPEC>,
    #[doc = "0x18 - Status Register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x1c - Command Register"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x20 - Retention RAM power-down Register"]
    pub powerdown: crate::Reg<powerdown::POWERDOWN_SPEC>,
    #[doc = "0x24 - Configuration Lock Register"]
    pub lock: crate::Reg<lock::LOCK_SPEC>,
    #[doc = "0x28 - Interrupt Flag Register"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x2c - Interrupt Flag Set Register"]
    pub ifs: crate::Reg<ifs::IFS_SPEC>,
    #[doc = "0x30 - Interrupt Flag Clear Register"]
    pub ifc: crate::Reg<ifc::IFC_SPEC>,
    #[doc = "0x34 - Interrupt Enable Register"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    #[doc = "0x38 - Freeze Register"]
    pub freeze: crate::Reg<freeze::FREEZE_SPEC>,
    #[doc = "0x3c - Synchronization Busy Register"]
    pub syncbusy: crate::Reg<syncbusy::SYNCBUSY_SPEC>,
    _reserved16: [u8; 0xc0],
    #[doc = "0x100 - Retention Register"]
    pub ret0_reg: crate::Reg<ret0_reg::RET0_REG_SPEC>,
    #[doc = "0x104 - Retention Register"]
    pub ret1_reg: crate::Reg<ret1_reg::RET1_REG_SPEC>,
    #[doc = "0x108 - Retention Register"]
    pub ret2_reg: crate::Reg<ret2_reg::RET2_REG_SPEC>,
    #[doc = "0x10c - Retention Register"]
    pub ret3_reg: crate::Reg<ret3_reg::RET3_REG_SPEC>,
    #[doc = "0x110 - Retention Register"]
    pub ret4_reg: crate::Reg<ret4_reg::RET4_REG_SPEC>,
    #[doc = "0x114 - Retention Register"]
    pub ret5_reg: crate::Reg<ret5_reg::RET5_REG_SPEC>,
    #[doc = "0x118 - Retention Register"]
    pub ret6_reg: crate::Reg<ret6_reg::RET6_REG_SPEC>,
    #[doc = "0x11c - Retention Register"]
    pub ret7_reg: crate::Reg<ret7_reg::RET7_REG_SPEC>,
    #[doc = "0x120 - Retention Register"]
    pub ret8_reg: crate::Reg<ret8_reg::RET8_REG_SPEC>,
    #[doc = "0x124 - Retention Register"]
    pub ret9_reg: crate::Reg<ret9_reg::RET9_REG_SPEC>,
    #[doc = "0x128 - Retention Register"]
    pub ret10_reg: crate::Reg<ret10_reg::RET10_REG_SPEC>,
    #[doc = "0x12c - Retention Register"]
    pub ret11_reg: crate::Reg<ret11_reg::RET11_REG_SPEC>,
    #[doc = "0x130 - Retention Register"]
    pub ret12_reg: crate::Reg<ret12_reg::RET12_REG_SPEC>,
    #[doc = "0x134 - Retention Register"]
    pub ret13_reg: crate::Reg<ret13_reg::RET13_REG_SPEC>,
    #[doc = "0x138 - Retention Register"]
    pub ret14_reg: crate::Reg<ret14_reg::RET14_REG_SPEC>,
    #[doc = "0x13c - Retention Register"]
    pub ret15_reg: crate::Reg<ret15_reg::RET15_REG_SPEC>,
    #[doc = "0x140 - Retention Register"]
    pub ret16_reg: crate::Reg<ret16_reg::RET16_REG_SPEC>,
    #[doc = "0x144 - Retention Register"]
    pub ret17_reg: crate::Reg<ret17_reg::RET17_REG_SPEC>,
    #[doc = "0x148 - Retention Register"]
    pub ret18_reg: crate::Reg<ret18_reg::RET18_REG_SPEC>,
    #[doc = "0x14c - Retention Register"]
    pub ret19_reg: crate::Reg<ret19_reg::RET19_REG_SPEC>,
    #[doc = "0x150 - Retention Register"]
    pub ret20_reg: crate::Reg<ret20_reg::RET20_REG_SPEC>,
    #[doc = "0x154 - Retention Register"]
    pub ret21_reg: crate::Reg<ret21_reg::RET21_REG_SPEC>,
    #[doc = "0x158 - Retention Register"]
    pub ret22_reg: crate::Reg<ret22_reg::RET22_REG_SPEC>,
    #[doc = "0x15c - Retention Register"]
    pub ret23_reg: crate::Reg<ret23_reg::RET23_REG_SPEC>,
    #[doc = "0x160 - Retention Register"]
    pub ret24_reg: crate::Reg<ret24_reg::RET24_REG_SPEC>,
    #[doc = "0x164 - Retention Register"]
    pub ret25_reg: crate::Reg<ret25_reg::RET25_REG_SPEC>,
    #[doc = "0x168 - Retention Register"]
    pub ret26_reg: crate::Reg<ret26_reg::RET26_REG_SPEC>,
    #[doc = "0x16c - Retention Register"]
    pub ret27_reg: crate::Reg<ret27_reg::RET27_REG_SPEC>,
    #[doc = "0x170 - Retention Register"]
    pub ret28_reg: crate::Reg<ret28_reg::RET28_REG_SPEC>,
    #[doc = "0x174 - Retention Register"]
    pub ret29_reg: crate::Reg<ret29_reg::RET29_REG_SPEC>,
    #[doc = "0x178 - Retention Register"]
    pub ret30_reg: crate::Reg<ret30_reg::RET30_REG_SPEC>,
    #[doc = "0x17c - Retention Register"]
    pub ret31_reg: crate::Reg<ret31_reg::RET31_REG_SPEC>,
    #[doc = "0x180 - Retention Register"]
    pub ret32_reg: crate::Reg<ret32_reg::RET32_REG_SPEC>,
    #[doc = "0x184 - Retention Register"]
    pub ret33_reg: crate::Reg<ret33_reg::RET33_REG_SPEC>,
    #[doc = "0x188 - Retention Register"]
    pub ret34_reg: crate::Reg<ret34_reg::RET34_REG_SPEC>,
    #[doc = "0x18c - Retention Register"]
    pub ret35_reg: crate::Reg<ret35_reg::RET35_REG_SPEC>,
    #[doc = "0x190 - Retention Register"]
    pub ret36_reg: crate::Reg<ret36_reg::RET36_REG_SPEC>,
    #[doc = "0x194 - Retention Register"]
    pub ret37_reg: crate::Reg<ret37_reg::RET37_REG_SPEC>,
    #[doc = "0x198 - Retention Register"]
    pub ret38_reg: crate::Reg<ret38_reg::RET38_REG_SPEC>,
    #[doc = "0x19c - Retention Register"]
    pub ret39_reg: crate::Reg<ret39_reg::RET39_REG_SPEC>,
    #[doc = "0x1a0 - Retention Register"]
    pub ret40_reg: crate::Reg<ret40_reg::RET40_REG_SPEC>,
    #[doc = "0x1a4 - Retention Register"]
    pub ret41_reg: crate::Reg<ret41_reg::RET41_REG_SPEC>,
    #[doc = "0x1a8 - Retention Register"]
    pub ret42_reg: crate::Reg<ret42_reg::RET42_REG_SPEC>,
    #[doc = "0x1ac - Retention Register"]
    pub ret43_reg: crate::Reg<ret43_reg::RET43_REG_SPEC>,
    #[doc = "0x1b0 - Retention Register"]
    pub ret44_reg: crate::Reg<ret44_reg::RET44_REG_SPEC>,
    #[doc = "0x1b4 - Retention Register"]
    pub ret45_reg: crate::Reg<ret45_reg::RET45_REG_SPEC>,
    #[doc = "0x1b8 - Retention Register"]
    pub ret46_reg: crate::Reg<ret46_reg::RET46_REG_SPEC>,
    #[doc = "0x1bc - Retention Register"]
    pub ret47_reg: crate::Reg<ret47_reg::RET47_REG_SPEC>,
    #[doc = "0x1c0 - Retention Register"]
    pub ret48_reg: crate::Reg<ret48_reg::RET48_REG_SPEC>,
    #[doc = "0x1c4 - Retention Register"]
    pub ret49_reg: crate::Reg<ret49_reg::RET49_REG_SPEC>,
    #[doc = "0x1c8 - Retention Register"]
    pub ret50_reg: crate::Reg<ret50_reg::RET50_REG_SPEC>,
    #[doc = "0x1cc - Retention Register"]
    pub ret51_reg: crate::Reg<ret51_reg::RET51_REG_SPEC>,
    #[doc = "0x1d0 - Retention Register"]
    pub ret52_reg: crate::Reg<ret52_reg::RET52_REG_SPEC>,
    #[doc = "0x1d4 - Retention Register"]
    pub ret53_reg: crate::Reg<ret53_reg::RET53_REG_SPEC>,
    #[doc = "0x1d8 - Retention Register"]
    pub ret54_reg: crate::Reg<ret54_reg::RET54_REG_SPEC>,
    #[doc = "0x1dc - Retention Register"]
    pub ret55_reg: crate::Reg<ret55_reg::RET55_REG_SPEC>,
    #[doc = "0x1e0 - Retention Register"]
    pub ret56_reg: crate::Reg<ret56_reg::RET56_REG_SPEC>,
    #[doc = "0x1e4 - Retention Register"]
    pub ret57_reg: crate::Reg<ret57_reg::RET57_REG_SPEC>,
    #[doc = "0x1e8 - Retention Register"]
    pub ret58_reg: crate::Reg<ret58_reg::RET58_REG_SPEC>,
    #[doc = "0x1ec - Retention Register"]
    pub ret59_reg: crate::Reg<ret59_reg::RET59_REG_SPEC>,
    #[doc = "0x1f0 - Retention Register"]
    pub ret60_reg: crate::Reg<ret60_reg::RET60_REG_SPEC>,
    #[doc = "0x1f4 - Retention Register"]
    pub ret61_reg: crate::Reg<ret61_reg::RET61_REG_SPEC>,
    #[doc = "0x1f8 - Retention Register"]
    pub ret62_reg: crate::Reg<ret62_reg::RET62_REG_SPEC>,
    #[doc = "0x1fc - Retention Register"]
    pub ret63_reg: crate::Reg<ret63_reg::RET63_REG_SPEC>,
    #[doc = "0x200 - Retention Register"]
    pub ret64_reg: crate::Reg<ret64_reg::RET64_REG_SPEC>,
    #[doc = "0x204 - Retention Register"]
    pub ret65_reg: crate::Reg<ret65_reg::RET65_REG_SPEC>,
    #[doc = "0x208 - Retention Register"]
    pub ret66_reg: crate::Reg<ret66_reg::RET66_REG_SPEC>,
    #[doc = "0x20c - Retention Register"]
    pub ret67_reg: crate::Reg<ret67_reg::RET67_REG_SPEC>,
    #[doc = "0x210 - Retention Register"]
    pub ret68_reg: crate::Reg<ret68_reg::RET68_REG_SPEC>,
    #[doc = "0x214 - Retention Register"]
    pub ret69_reg: crate::Reg<ret69_reg::RET69_REG_SPEC>,
    #[doc = "0x218 - Retention Register"]
    pub ret70_reg: crate::Reg<ret70_reg::RET70_REG_SPEC>,
    #[doc = "0x21c - Retention Register"]
    pub ret71_reg: crate::Reg<ret71_reg::RET71_REG_SPEC>,
    #[doc = "0x220 - Retention Register"]
    pub ret72_reg: crate::Reg<ret72_reg::RET72_REG_SPEC>,
    #[doc = "0x224 - Retention Register"]
    pub ret73_reg: crate::Reg<ret73_reg::RET73_REG_SPEC>,
    #[doc = "0x228 - Retention Register"]
    pub ret74_reg: crate::Reg<ret74_reg::RET74_REG_SPEC>,
    #[doc = "0x22c - Retention Register"]
    pub ret75_reg: crate::Reg<ret75_reg::RET75_REG_SPEC>,
    #[doc = "0x230 - Retention Register"]
    pub ret76_reg: crate::Reg<ret76_reg::RET76_REG_SPEC>,
    #[doc = "0x234 - Retention Register"]
    pub ret77_reg: crate::Reg<ret77_reg::RET77_REG_SPEC>,
    #[doc = "0x238 - Retention Register"]
    pub ret78_reg: crate::Reg<ret78_reg::RET78_REG_SPEC>,
    #[doc = "0x23c - Retention Register"]
    pub ret79_reg: crate::Reg<ret79_reg::RET79_REG_SPEC>,
    #[doc = "0x240 - Retention Register"]
    pub ret80_reg: crate::Reg<ret80_reg::RET80_REG_SPEC>,
    #[doc = "0x244 - Retention Register"]
    pub ret81_reg: crate::Reg<ret81_reg::RET81_REG_SPEC>,
    #[doc = "0x248 - Retention Register"]
    pub ret82_reg: crate::Reg<ret82_reg::RET82_REG_SPEC>,
    #[doc = "0x24c - Retention Register"]
    pub ret83_reg: crate::Reg<ret83_reg::RET83_REG_SPEC>,
    #[doc = "0x250 - Retention Register"]
    pub ret84_reg: crate::Reg<ret84_reg::RET84_REG_SPEC>,
    #[doc = "0x254 - Retention Register"]
    pub ret85_reg: crate::Reg<ret85_reg::RET85_REG_SPEC>,
    #[doc = "0x258 - Retention Register"]
    pub ret86_reg: crate::Reg<ret86_reg::RET86_REG_SPEC>,
    #[doc = "0x25c - Retention Register"]
    pub ret87_reg: crate::Reg<ret87_reg::RET87_REG_SPEC>,
    #[doc = "0x260 - Retention Register"]
    pub ret88_reg: crate::Reg<ret88_reg::RET88_REG_SPEC>,
    #[doc = "0x264 - Retention Register"]
    pub ret89_reg: crate::Reg<ret89_reg::RET89_REG_SPEC>,
    #[doc = "0x268 - Retention Register"]
    pub ret90_reg: crate::Reg<ret90_reg::RET90_REG_SPEC>,
    #[doc = "0x26c - Retention Register"]
    pub ret91_reg: crate::Reg<ret91_reg::RET91_REG_SPEC>,
    #[doc = "0x270 - Retention Register"]
    pub ret92_reg: crate::Reg<ret92_reg::RET92_REG_SPEC>,
    #[doc = "0x274 - Retention Register"]
    pub ret93_reg: crate::Reg<ret93_reg::RET93_REG_SPEC>,
    #[doc = "0x278 - Retention Register"]
    pub ret94_reg: crate::Reg<ret94_reg::RET94_REG_SPEC>,
    #[doc = "0x27c - Retention Register"]
    pub ret95_reg: crate::Reg<ret95_reg::RET95_REG_SPEC>,
    #[doc = "0x280 - Retention Register"]
    pub ret96_reg: crate::Reg<ret96_reg::RET96_REG_SPEC>,
    #[doc = "0x284 - Retention Register"]
    pub ret97_reg: crate::Reg<ret97_reg::RET97_REG_SPEC>,
    #[doc = "0x288 - Retention Register"]
    pub ret98_reg: crate::Reg<ret98_reg::RET98_REG_SPEC>,
    #[doc = "0x28c - Retention Register"]
    pub ret99_reg: crate::Reg<ret99_reg::RET99_REG_SPEC>,
    #[doc = "0x290 - Retention Register"]
    pub ret100_reg: crate::Reg<ret100_reg::RET100_REG_SPEC>,
    #[doc = "0x294 - Retention Register"]
    pub ret101_reg: crate::Reg<ret101_reg::RET101_REG_SPEC>,
    #[doc = "0x298 - Retention Register"]
    pub ret102_reg: crate::Reg<ret102_reg::RET102_REG_SPEC>,
    #[doc = "0x29c - Retention Register"]
    pub ret103_reg: crate::Reg<ret103_reg::RET103_REG_SPEC>,
    #[doc = "0x2a0 - Retention Register"]
    pub ret104_reg: crate::Reg<ret104_reg::RET104_REG_SPEC>,
    #[doc = "0x2a4 - Retention Register"]
    pub ret105_reg: crate::Reg<ret105_reg::RET105_REG_SPEC>,
    #[doc = "0x2a8 - Retention Register"]
    pub ret106_reg: crate::Reg<ret106_reg::RET106_REG_SPEC>,
    #[doc = "0x2ac - Retention Register"]
    pub ret107_reg: crate::Reg<ret107_reg::RET107_REG_SPEC>,
    #[doc = "0x2b0 - Retention Register"]
    pub ret108_reg: crate::Reg<ret108_reg::RET108_REG_SPEC>,
    #[doc = "0x2b4 - Retention Register"]
    pub ret109_reg: crate::Reg<ret109_reg::RET109_REG_SPEC>,
    #[doc = "0x2b8 - Retention Register"]
    pub ret110_reg: crate::Reg<ret110_reg::RET110_REG_SPEC>,
    #[doc = "0x2bc - Retention Register"]
    pub ret111_reg: crate::Reg<ret111_reg::RET111_REG_SPEC>,
    #[doc = "0x2c0 - Retention Register"]
    pub ret112_reg: crate::Reg<ret112_reg::RET112_REG_SPEC>,
    #[doc = "0x2c4 - Retention Register"]
    pub ret113_reg: crate::Reg<ret113_reg::RET113_REG_SPEC>,
    #[doc = "0x2c8 - Retention Register"]
    pub ret114_reg: crate::Reg<ret114_reg::RET114_REG_SPEC>,
    #[doc = "0x2cc - Retention Register"]
    pub ret115_reg: crate::Reg<ret115_reg::RET115_REG_SPEC>,
    #[doc = "0x2d0 - Retention Register"]
    pub ret116_reg: crate::Reg<ret116_reg::RET116_REG_SPEC>,
    #[doc = "0x2d4 - Retention Register"]
    pub ret117_reg: crate::Reg<ret117_reg::RET117_REG_SPEC>,
    #[doc = "0x2d8 - Retention Register"]
    pub ret118_reg: crate::Reg<ret118_reg::RET118_REG_SPEC>,
    #[doc = "0x2dc - Retention Register"]
    pub ret119_reg: crate::Reg<ret119_reg::RET119_REG_SPEC>,
    #[doc = "0x2e0 - Retention Register"]
    pub ret120_reg: crate::Reg<ret120_reg::RET120_REG_SPEC>,
    #[doc = "0x2e4 - Retention Register"]
    pub ret121_reg: crate::Reg<ret121_reg::RET121_REG_SPEC>,
    #[doc = "0x2e8 - Retention Register"]
    pub ret122_reg: crate::Reg<ret122_reg::RET122_REG_SPEC>,
    #[doc = "0x2ec - Retention Register"]
    pub ret123_reg: crate::Reg<ret123_reg::RET123_REG_SPEC>,
    #[doc = "0x2f0 - Retention Register"]
    pub ret124_reg: crate::Reg<ret124_reg::RET124_REG_SPEC>,
    #[doc = "0x2f4 - Retention Register"]
    pub ret125_reg: crate::Reg<ret125_reg::RET125_REG_SPEC>,
    #[doc = "0x2f8 - Retention Register"]
    pub ret126_reg: crate::Reg<ret126_reg::RET126_REG_SPEC>,
    #[doc = "0x2fc - Retention Register"]
    pub ret127_reg: crate::Reg<ret127_reg::RET127_REG_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "LPMODE register accessor: an alias for `Reg<LPMODE_SPEC>`"]
pub type LPMODE = crate::Reg<lpmode::LPMODE_SPEC>;
#[doc = "Low power mode configuration"]
pub mod lpmode;
#[doc = "CNT register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Counter Value Register"]
pub mod cnt;
#[doc = "COMP0 register accessor: an alias for `Reg<COMP0_SPEC>`"]
pub type COMP0 = crate::Reg<comp0::COMP0_SPEC>;
#[doc = "Counter Compare Value"]
pub mod comp0;
#[doc = "TIMESTAMP register accessor: an alias for `Reg<TIMESTAMP_SPEC>`"]
pub type TIMESTAMP = crate::Reg<timestamp::TIMESTAMP_SPEC>;
#[doc = "Backup mode timestamp"]
pub mod timestamp;
#[doc = "LFXOFDET register accessor: an alias for `Reg<LFXOFDET_SPEC>`"]
pub type LFXOFDET = crate::Reg<lfxofdet::LFXOFDET_SPEC>;
#[doc = "LFXO"]
pub mod lfxofdet;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "POWERDOWN register accessor: an alias for `Reg<POWERDOWN_SPEC>`"]
pub type POWERDOWN = crate::Reg<powerdown::POWERDOWN_SPEC>;
#[doc = "Retention RAM power-down Register"]
pub mod powerdown;
#[doc = "LOCK register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Configuration Lock Register"]
pub mod lock;
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
#[doc = "FREEZE register accessor: an alias for `Reg<FREEZE_SPEC>`"]
pub type FREEZE = crate::Reg<freeze::FREEZE_SPEC>;
#[doc = "Freeze Register"]
pub mod freeze;
#[doc = "SYNCBUSY register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "RET0_REG register accessor: an alias for `Reg<RET0_REG_SPEC>`"]
pub type RET0_REG = crate::Reg<ret0_reg::RET0_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret0_reg;
#[doc = "RET1_REG register accessor: an alias for `Reg<RET1_REG_SPEC>`"]
pub type RET1_REG = crate::Reg<ret1_reg::RET1_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret1_reg;
#[doc = "RET2_REG register accessor: an alias for `Reg<RET2_REG_SPEC>`"]
pub type RET2_REG = crate::Reg<ret2_reg::RET2_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret2_reg;
#[doc = "RET3_REG register accessor: an alias for `Reg<RET3_REG_SPEC>`"]
pub type RET3_REG = crate::Reg<ret3_reg::RET3_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret3_reg;
#[doc = "RET4_REG register accessor: an alias for `Reg<RET4_REG_SPEC>`"]
pub type RET4_REG = crate::Reg<ret4_reg::RET4_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret4_reg;
#[doc = "RET5_REG register accessor: an alias for `Reg<RET5_REG_SPEC>`"]
pub type RET5_REG = crate::Reg<ret5_reg::RET5_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret5_reg;
#[doc = "RET6_REG register accessor: an alias for `Reg<RET6_REG_SPEC>`"]
pub type RET6_REG = crate::Reg<ret6_reg::RET6_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret6_reg;
#[doc = "RET7_REG register accessor: an alias for `Reg<RET7_REG_SPEC>`"]
pub type RET7_REG = crate::Reg<ret7_reg::RET7_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret7_reg;
#[doc = "RET8_REG register accessor: an alias for `Reg<RET8_REG_SPEC>`"]
pub type RET8_REG = crate::Reg<ret8_reg::RET8_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret8_reg;
#[doc = "RET9_REG register accessor: an alias for `Reg<RET9_REG_SPEC>`"]
pub type RET9_REG = crate::Reg<ret9_reg::RET9_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret9_reg;
#[doc = "RET10_REG register accessor: an alias for `Reg<RET10_REG_SPEC>`"]
pub type RET10_REG = crate::Reg<ret10_reg::RET10_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret10_reg;
#[doc = "RET11_REG register accessor: an alias for `Reg<RET11_REG_SPEC>`"]
pub type RET11_REG = crate::Reg<ret11_reg::RET11_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret11_reg;
#[doc = "RET12_REG register accessor: an alias for `Reg<RET12_REG_SPEC>`"]
pub type RET12_REG = crate::Reg<ret12_reg::RET12_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret12_reg;
#[doc = "RET13_REG register accessor: an alias for `Reg<RET13_REG_SPEC>`"]
pub type RET13_REG = crate::Reg<ret13_reg::RET13_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret13_reg;
#[doc = "RET14_REG register accessor: an alias for `Reg<RET14_REG_SPEC>`"]
pub type RET14_REG = crate::Reg<ret14_reg::RET14_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret14_reg;
#[doc = "RET15_REG register accessor: an alias for `Reg<RET15_REG_SPEC>`"]
pub type RET15_REG = crate::Reg<ret15_reg::RET15_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret15_reg;
#[doc = "RET16_REG register accessor: an alias for `Reg<RET16_REG_SPEC>`"]
pub type RET16_REG = crate::Reg<ret16_reg::RET16_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret16_reg;
#[doc = "RET17_REG register accessor: an alias for `Reg<RET17_REG_SPEC>`"]
pub type RET17_REG = crate::Reg<ret17_reg::RET17_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret17_reg;
#[doc = "RET18_REG register accessor: an alias for `Reg<RET18_REG_SPEC>`"]
pub type RET18_REG = crate::Reg<ret18_reg::RET18_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret18_reg;
#[doc = "RET19_REG register accessor: an alias for `Reg<RET19_REG_SPEC>`"]
pub type RET19_REG = crate::Reg<ret19_reg::RET19_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret19_reg;
#[doc = "RET20_REG register accessor: an alias for `Reg<RET20_REG_SPEC>`"]
pub type RET20_REG = crate::Reg<ret20_reg::RET20_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret20_reg;
#[doc = "RET21_REG register accessor: an alias for `Reg<RET21_REG_SPEC>`"]
pub type RET21_REG = crate::Reg<ret21_reg::RET21_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret21_reg;
#[doc = "RET22_REG register accessor: an alias for `Reg<RET22_REG_SPEC>`"]
pub type RET22_REG = crate::Reg<ret22_reg::RET22_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret22_reg;
#[doc = "RET23_REG register accessor: an alias for `Reg<RET23_REG_SPEC>`"]
pub type RET23_REG = crate::Reg<ret23_reg::RET23_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret23_reg;
#[doc = "RET24_REG register accessor: an alias for `Reg<RET24_REG_SPEC>`"]
pub type RET24_REG = crate::Reg<ret24_reg::RET24_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret24_reg;
#[doc = "RET25_REG register accessor: an alias for `Reg<RET25_REG_SPEC>`"]
pub type RET25_REG = crate::Reg<ret25_reg::RET25_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret25_reg;
#[doc = "RET26_REG register accessor: an alias for `Reg<RET26_REG_SPEC>`"]
pub type RET26_REG = crate::Reg<ret26_reg::RET26_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret26_reg;
#[doc = "RET27_REG register accessor: an alias for `Reg<RET27_REG_SPEC>`"]
pub type RET27_REG = crate::Reg<ret27_reg::RET27_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret27_reg;
#[doc = "RET28_REG register accessor: an alias for `Reg<RET28_REG_SPEC>`"]
pub type RET28_REG = crate::Reg<ret28_reg::RET28_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret28_reg;
#[doc = "RET29_REG register accessor: an alias for `Reg<RET29_REG_SPEC>`"]
pub type RET29_REG = crate::Reg<ret29_reg::RET29_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret29_reg;
#[doc = "RET30_REG register accessor: an alias for `Reg<RET30_REG_SPEC>`"]
pub type RET30_REG = crate::Reg<ret30_reg::RET30_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret30_reg;
#[doc = "RET31_REG register accessor: an alias for `Reg<RET31_REG_SPEC>`"]
pub type RET31_REG = crate::Reg<ret31_reg::RET31_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret31_reg;
#[doc = "RET32_REG register accessor: an alias for `Reg<RET32_REG_SPEC>`"]
pub type RET32_REG = crate::Reg<ret32_reg::RET32_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret32_reg;
#[doc = "RET33_REG register accessor: an alias for `Reg<RET33_REG_SPEC>`"]
pub type RET33_REG = crate::Reg<ret33_reg::RET33_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret33_reg;
#[doc = "RET34_REG register accessor: an alias for `Reg<RET34_REG_SPEC>`"]
pub type RET34_REG = crate::Reg<ret34_reg::RET34_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret34_reg;
#[doc = "RET35_REG register accessor: an alias for `Reg<RET35_REG_SPEC>`"]
pub type RET35_REG = crate::Reg<ret35_reg::RET35_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret35_reg;
#[doc = "RET36_REG register accessor: an alias for `Reg<RET36_REG_SPEC>`"]
pub type RET36_REG = crate::Reg<ret36_reg::RET36_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret36_reg;
#[doc = "RET37_REG register accessor: an alias for `Reg<RET37_REG_SPEC>`"]
pub type RET37_REG = crate::Reg<ret37_reg::RET37_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret37_reg;
#[doc = "RET38_REG register accessor: an alias for `Reg<RET38_REG_SPEC>`"]
pub type RET38_REG = crate::Reg<ret38_reg::RET38_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret38_reg;
#[doc = "RET39_REG register accessor: an alias for `Reg<RET39_REG_SPEC>`"]
pub type RET39_REG = crate::Reg<ret39_reg::RET39_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret39_reg;
#[doc = "RET40_REG register accessor: an alias for `Reg<RET40_REG_SPEC>`"]
pub type RET40_REG = crate::Reg<ret40_reg::RET40_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret40_reg;
#[doc = "RET41_REG register accessor: an alias for `Reg<RET41_REG_SPEC>`"]
pub type RET41_REG = crate::Reg<ret41_reg::RET41_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret41_reg;
#[doc = "RET42_REG register accessor: an alias for `Reg<RET42_REG_SPEC>`"]
pub type RET42_REG = crate::Reg<ret42_reg::RET42_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret42_reg;
#[doc = "RET43_REG register accessor: an alias for `Reg<RET43_REG_SPEC>`"]
pub type RET43_REG = crate::Reg<ret43_reg::RET43_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret43_reg;
#[doc = "RET44_REG register accessor: an alias for `Reg<RET44_REG_SPEC>`"]
pub type RET44_REG = crate::Reg<ret44_reg::RET44_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret44_reg;
#[doc = "RET45_REG register accessor: an alias for `Reg<RET45_REG_SPEC>`"]
pub type RET45_REG = crate::Reg<ret45_reg::RET45_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret45_reg;
#[doc = "RET46_REG register accessor: an alias for `Reg<RET46_REG_SPEC>`"]
pub type RET46_REG = crate::Reg<ret46_reg::RET46_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret46_reg;
#[doc = "RET47_REG register accessor: an alias for `Reg<RET47_REG_SPEC>`"]
pub type RET47_REG = crate::Reg<ret47_reg::RET47_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret47_reg;
#[doc = "RET48_REG register accessor: an alias for `Reg<RET48_REG_SPEC>`"]
pub type RET48_REG = crate::Reg<ret48_reg::RET48_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret48_reg;
#[doc = "RET49_REG register accessor: an alias for `Reg<RET49_REG_SPEC>`"]
pub type RET49_REG = crate::Reg<ret49_reg::RET49_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret49_reg;
#[doc = "RET50_REG register accessor: an alias for `Reg<RET50_REG_SPEC>`"]
pub type RET50_REG = crate::Reg<ret50_reg::RET50_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret50_reg;
#[doc = "RET51_REG register accessor: an alias for `Reg<RET51_REG_SPEC>`"]
pub type RET51_REG = crate::Reg<ret51_reg::RET51_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret51_reg;
#[doc = "RET52_REG register accessor: an alias for `Reg<RET52_REG_SPEC>`"]
pub type RET52_REG = crate::Reg<ret52_reg::RET52_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret52_reg;
#[doc = "RET53_REG register accessor: an alias for `Reg<RET53_REG_SPEC>`"]
pub type RET53_REG = crate::Reg<ret53_reg::RET53_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret53_reg;
#[doc = "RET54_REG register accessor: an alias for `Reg<RET54_REG_SPEC>`"]
pub type RET54_REG = crate::Reg<ret54_reg::RET54_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret54_reg;
#[doc = "RET55_REG register accessor: an alias for `Reg<RET55_REG_SPEC>`"]
pub type RET55_REG = crate::Reg<ret55_reg::RET55_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret55_reg;
#[doc = "RET56_REG register accessor: an alias for `Reg<RET56_REG_SPEC>`"]
pub type RET56_REG = crate::Reg<ret56_reg::RET56_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret56_reg;
#[doc = "RET57_REG register accessor: an alias for `Reg<RET57_REG_SPEC>`"]
pub type RET57_REG = crate::Reg<ret57_reg::RET57_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret57_reg;
#[doc = "RET58_REG register accessor: an alias for `Reg<RET58_REG_SPEC>`"]
pub type RET58_REG = crate::Reg<ret58_reg::RET58_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret58_reg;
#[doc = "RET59_REG register accessor: an alias for `Reg<RET59_REG_SPEC>`"]
pub type RET59_REG = crate::Reg<ret59_reg::RET59_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret59_reg;
#[doc = "RET60_REG register accessor: an alias for `Reg<RET60_REG_SPEC>`"]
pub type RET60_REG = crate::Reg<ret60_reg::RET60_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret60_reg;
#[doc = "RET61_REG register accessor: an alias for `Reg<RET61_REG_SPEC>`"]
pub type RET61_REG = crate::Reg<ret61_reg::RET61_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret61_reg;
#[doc = "RET62_REG register accessor: an alias for `Reg<RET62_REG_SPEC>`"]
pub type RET62_REG = crate::Reg<ret62_reg::RET62_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret62_reg;
#[doc = "RET63_REG register accessor: an alias for `Reg<RET63_REG_SPEC>`"]
pub type RET63_REG = crate::Reg<ret63_reg::RET63_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret63_reg;
#[doc = "RET64_REG register accessor: an alias for `Reg<RET64_REG_SPEC>`"]
pub type RET64_REG = crate::Reg<ret64_reg::RET64_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret64_reg;
#[doc = "RET65_REG register accessor: an alias for `Reg<RET65_REG_SPEC>`"]
pub type RET65_REG = crate::Reg<ret65_reg::RET65_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret65_reg;
#[doc = "RET66_REG register accessor: an alias for `Reg<RET66_REG_SPEC>`"]
pub type RET66_REG = crate::Reg<ret66_reg::RET66_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret66_reg;
#[doc = "RET67_REG register accessor: an alias for `Reg<RET67_REG_SPEC>`"]
pub type RET67_REG = crate::Reg<ret67_reg::RET67_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret67_reg;
#[doc = "RET68_REG register accessor: an alias for `Reg<RET68_REG_SPEC>`"]
pub type RET68_REG = crate::Reg<ret68_reg::RET68_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret68_reg;
#[doc = "RET69_REG register accessor: an alias for `Reg<RET69_REG_SPEC>`"]
pub type RET69_REG = crate::Reg<ret69_reg::RET69_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret69_reg;
#[doc = "RET70_REG register accessor: an alias for `Reg<RET70_REG_SPEC>`"]
pub type RET70_REG = crate::Reg<ret70_reg::RET70_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret70_reg;
#[doc = "RET71_REG register accessor: an alias for `Reg<RET71_REG_SPEC>`"]
pub type RET71_REG = crate::Reg<ret71_reg::RET71_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret71_reg;
#[doc = "RET72_REG register accessor: an alias for `Reg<RET72_REG_SPEC>`"]
pub type RET72_REG = crate::Reg<ret72_reg::RET72_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret72_reg;
#[doc = "RET73_REG register accessor: an alias for `Reg<RET73_REG_SPEC>`"]
pub type RET73_REG = crate::Reg<ret73_reg::RET73_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret73_reg;
#[doc = "RET74_REG register accessor: an alias for `Reg<RET74_REG_SPEC>`"]
pub type RET74_REG = crate::Reg<ret74_reg::RET74_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret74_reg;
#[doc = "RET75_REG register accessor: an alias for `Reg<RET75_REG_SPEC>`"]
pub type RET75_REG = crate::Reg<ret75_reg::RET75_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret75_reg;
#[doc = "RET76_REG register accessor: an alias for `Reg<RET76_REG_SPEC>`"]
pub type RET76_REG = crate::Reg<ret76_reg::RET76_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret76_reg;
#[doc = "RET77_REG register accessor: an alias for `Reg<RET77_REG_SPEC>`"]
pub type RET77_REG = crate::Reg<ret77_reg::RET77_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret77_reg;
#[doc = "RET78_REG register accessor: an alias for `Reg<RET78_REG_SPEC>`"]
pub type RET78_REG = crate::Reg<ret78_reg::RET78_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret78_reg;
#[doc = "RET79_REG register accessor: an alias for `Reg<RET79_REG_SPEC>`"]
pub type RET79_REG = crate::Reg<ret79_reg::RET79_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret79_reg;
#[doc = "RET80_REG register accessor: an alias for `Reg<RET80_REG_SPEC>`"]
pub type RET80_REG = crate::Reg<ret80_reg::RET80_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret80_reg;
#[doc = "RET81_REG register accessor: an alias for `Reg<RET81_REG_SPEC>`"]
pub type RET81_REG = crate::Reg<ret81_reg::RET81_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret81_reg;
#[doc = "RET82_REG register accessor: an alias for `Reg<RET82_REG_SPEC>`"]
pub type RET82_REG = crate::Reg<ret82_reg::RET82_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret82_reg;
#[doc = "RET83_REG register accessor: an alias for `Reg<RET83_REG_SPEC>`"]
pub type RET83_REG = crate::Reg<ret83_reg::RET83_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret83_reg;
#[doc = "RET84_REG register accessor: an alias for `Reg<RET84_REG_SPEC>`"]
pub type RET84_REG = crate::Reg<ret84_reg::RET84_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret84_reg;
#[doc = "RET85_REG register accessor: an alias for `Reg<RET85_REG_SPEC>`"]
pub type RET85_REG = crate::Reg<ret85_reg::RET85_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret85_reg;
#[doc = "RET86_REG register accessor: an alias for `Reg<RET86_REG_SPEC>`"]
pub type RET86_REG = crate::Reg<ret86_reg::RET86_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret86_reg;
#[doc = "RET87_REG register accessor: an alias for `Reg<RET87_REG_SPEC>`"]
pub type RET87_REG = crate::Reg<ret87_reg::RET87_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret87_reg;
#[doc = "RET88_REG register accessor: an alias for `Reg<RET88_REG_SPEC>`"]
pub type RET88_REG = crate::Reg<ret88_reg::RET88_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret88_reg;
#[doc = "RET89_REG register accessor: an alias for `Reg<RET89_REG_SPEC>`"]
pub type RET89_REG = crate::Reg<ret89_reg::RET89_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret89_reg;
#[doc = "RET90_REG register accessor: an alias for `Reg<RET90_REG_SPEC>`"]
pub type RET90_REG = crate::Reg<ret90_reg::RET90_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret90_reg;
#[doc = "RET91_REG register accessor: an alias for `Reg<RET91_REG_SPEC>`"]
pub type RET91_REG = crate::Reg<ret91_reg::RET91_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret91_reg;
#[doc = "RET92_REG register accessor: an alias for `Reg<RET92_REG_SPEC>`"]
pub type RET92_REG = crate::Reg<ret92_reg::RET92_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret92_reg;
#[doc = "RET93_REG register accessor: an alias for `Reg<RET93_REG_SPEC>`"]
pub type RET93_REG = crate::Reg<ret93_reg::RET93_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret93_reg;
#[doc = "RET94_REG register accessor: an alias for `Reg<RET94_REG_SPEC>`"]
pub type RET94_REG = crate::Reg<ret94_reg::RET94_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret94_reg;
#[doc = "RET95_REG register accessor: an alias for `Reg<RET95_REG_SPEC>`"]
pub type RET95_REG = crate::Reg<ret95_reg::RET95_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret95_reg;
#[doc = "RET96_REG register accessor: an alias for `Reg<RET96_REG_SPEC>`"]
pub type RET96_REG = crate::Reg<ret96_reg::RET96_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret96_reg;
#[doc = "RET97_REG register accessor: an alias for `Reg<RET97_REG_SPEC>`"]
pub type RET97_REG = crate::Reg<ret97_reg::RET97_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret97_reg;
#[doc = "RET98_REG register accessor: an alias for `Reg<RET98_REG_SPEC>`"]
pub type RET98_REG = crate::Reg<ret98_reg::RET98_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret98_reg;
#[doc = "RET99_REG register accessor: an alias for `Reg<RET99_REG_SPEC>`"]
pub type RET99_REG = crate::Reg<ret99_reg::RET99_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret99_reg;
#[doc = "RET100_REG register accessor: an alias for `Reg<RET100_REG_SPEC>`"]
pub type RET100_REG = crate::Reg<ret100_reg::RET100_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret100_reg;
#[doc = "RET101_REG register accessor: an alias for `Reg<RET101_REG_SPEC>`"]
pub type RET101_REG = crate::Reg<ret101_reg::RET101_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret101_reg;
#[doc = "RET102_REG register accessor: an alias for `Reg<RET102_REG_SPEC>`"]
pub type RET102_REG = crate::Reg<ret102_reg::RET102_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret102_reg;
#[doc = "RET103_REG register accessor: an alias for `Reg<RET103_REG_SPEC>`"]
pub type RET103_REG = crate::Reg<ret103_reg::RET103_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret103_reg;
#[doc = "RET104_REG register accessor: an alias for `Reg<RET104_REG_SPEC>`"]
pub type RET104_REG = crate::Reg<ret104_reg::RET104_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret104_reg;
#[doc = "RET105_REG register accessor: an alias for `Reg<RET105_REG_SPEC>`"]
pub type RET105_REG = crate::Reg<ret105_reg::RET105_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret105_reg;
#[doc = "RET106_REG register accessor: an alias for `Reg<RET106_REG_SPEC>`"]
pub type RET106_REG = crate::Reg<ret106_reg::RET106_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret106_reg;
#[doc = "RET107_REG register accessor: an alias for `Reg<RET107_REG_SPEC>`"]
pub type RET107_REG = crate::Reg<ret107_reg::RET107_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret107_reg;
#[doc = "RET108_REG register accessor: an alias for `Reg<RET108_REG_SPEC>`"]
pub type RET108_REG = crate::Reg<ret108_reg::RET108_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret108_reg;
#[doc = "RET109_REG register accessor: an alias for `Reg<RET109_REG_SPEC>`"]
pub type RET109_REG = crate::Reg<ret109_reg::RET109_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret109_reg;
#[doc = "RET110_REG register accessor: an alias for `Reg<RET110_REG_SPEC>`"]
pub type RET110_REG = crate::Reg<ret110_reg::RET110_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret110_reg;
#[doc = "RET111_REG register accessor: an alias for `Reg<RET111_REG_SPEC>`"]
pub type RET111_REG = crate::Reg<ret111_reg::RET111_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret111_reg;
#[doc = "RET112_REG register accessor: an alias for `Reg<RET112_REG_SPEC>`"]
pub type RET112_REG = crate::Reg<ret112_reg::RET112_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret112_reg;
#[doc = "RET113_REG register accessor: an alias for `Reg<RET113_REG_SPEC>`"]
pub type RET113_REG = crate::Reg<ret113_reg::RET113_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret113_reg;
#[doc = "RET114_REG register accessor: an alias for `Reg<RET114_REG_SPEC>`"]
pub type RET114_REG = crate::Reg<ret114_reg::RET114_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret114_reg;
#[doc = "RET115_REG register accessor: an alias for `Reg<RET115_REG_SPEC>`"]
pub type RET115_REG = crate::Reg<ret115_reg::RET115_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret115_reg;
#[doc = "RET116_REG register accessor: an alias for `Reg<RET116_REG_SPEC>`"]
pub type RET116_REG = crate::Reg<ret116_reg::RET116_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret116_reg;
#[doc = "RET117_REG register accessor: an alias for `Reg<RET117_REG_SPEC>`"]
pub type RET117_REG = crate::Reg<ret117_reg::RET117_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret117_reg;
#[doc = "RET118_REG register accessor: an alias for `Reg<RET118_REG_SPEC>`"]
pub type RET118_REG = crate::Reg<ret118_reg::RET118_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret118_reg;
#[doc = "RET119_REG register accessor: an alias for `Reg<RET119_REG_SPEC>`"]
pub type RET119_REG = crate::Reg<ret119_reg::RET119_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret119_reg;
#[doc = "RET120_REG register accessor: an alias for `Reg<RET120_REG_SPEC>`"]
pub type RET120_REG = crate::Reg<ret120_reg::RET120_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret120_reg;
#[doc = "RET121_REG register accessor: an alias for `Reg<RET121_REG_SPEC>`"]
pub type RET121_REG = crate::Reg<ret121_reg::RET121_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret121_reg;
#[doc = "RET122_REG register accessor: an alias for `Reg<RET122_REG_SPEC>`"]
pub type RET122_REG = crate::Reg<ret122_reg::RET122_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret122_reg;
#[doc = "RET123_REG register accessor: an alias for `Reg<RET123_REG_SPEC>`"]
pub type RET123_REG = crate::Reg<ret123_reg::RET123_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret123_reg;
#[doc = "RET124_REG register accessor: an alias for `Reg<RET124_REG_SPEC>`"]
pub type RET124_REG = crate::Reg<ret124_reg::RET124_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret124_reg;
#[doc = "RET125_REG register accessor: an alias for `Reg<RET125_REG_SPEC>`"]
pub type RET125_REG = crate::Reg<ret125_reg::RET125_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret125_reg;
#[doc = "RET126_REG register accessor: an alias for `Reg<RET126_REG_SPEC>`"]
pub type RET126_REG = crate::Reg<ret126_reg::RET126_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret126_reg;
#[doc = "RET127_REG register accessor: an alias for `Reg<RET127_REG_SPEC>`"]
pub type RET127_REG = crate::Reg<ret127_reg::RET127_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret127_reg;
