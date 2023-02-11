#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Timing Control Register"]
    pub timctrl: TIMCTRL,
    #[doc = "0x08 - Peripheral Control Register"]
    pub perctrl: PERCTRL,
    #[doc = "0x0c - Decoder control Register"]
    pub decctrl: DECCTRL,
    #[doc = "0x10 - Bias Control Register"]
    pub biasctrl: BIASCTRL,
    #[doc = "0x14 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x18 - Channel enable Register"]
    pub chen: CHEN,
    #[doc = "0x1c - Scan result register"]
    pub scanres: SCANRES,
    #[doc = "0x20 - Status Register"]
    pub status: STATUS,
    #[doc = "0x24 - Result buffer pointers"]
    pub ptr: PTR,
    #[doc = "0x28 - Result buffer data register"]
    pub bufdata: BUFDATA,
    #[doc = "0x2c - Current channel index"]
    pub curch: CURCH,
    #[doc = "0x30 - Current decoder state"]
    pub decstate: DECSTATE,
    #[doc = "0x34 - Decoder input register"]
    pub sensorstate: SENSORSTATE,
    #[doc = "0x38 - GPIO Idle phase configuration"]
    pub idleconf: IDLECONF,
    #[doc = "0x3c - Alternative excite pin configuration"]
    pub altexconf: ALTEXCONF,
    #[doc = "0x40 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x44 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x48 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x4c - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x50 - Synchronization Busy Register"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x54 - I/O Routing Register"]
    pub route: ROUTE,
    #[doc = "0x58 - LESENSE RAM power-down register"]
    pub powerdown: POWERDOWN,
    _reserved23: [u8; 0x01a4],
    #[doc = "0x200 - State transition configuration A"]
    pub st0_tconfa: ST0_TCONFA,
    #[doc = "0x204 - State transition configuration B"]
    pub st0_tconfb: ST0_TCONFB,
    #[doc = "0x208 - State transition configuration A"]
    pub st1_tconfa: ST1_TCONFA,
    #[doc = "0x20c - State transition configuration B"]
    pub st1_tconfb: ST1_TCONFB,
    #[doc = "0x210 - State transition configuration A"]
    pub st2_tconfa: ST2_TCONFA,
    #[doc = "0x214 - State transition configuration B"]
    pub st2_tconfb: ST2_TCONFB,
    #[doc = "0x218 - State transition configuration A"]
    pub st3_tconfa: ST3_TCONFA,
    #[doc = "0x21c - State transition configuration B"]
    pub st3_tconfb: ST3_TCONFB,
    #[doc = "0x220 - State transition configuration A"]
    pub st4_tconfa: ST4_TCONFA,
    #[doc = "0x224 - State transition configuration B"]
    pub st4_tconfb: ST4_TCONFB,
    #[doc = "0x228 - State transition configuration A"]
    pub st5_tconfa: ST5_TCONFA,
    #[doc = "0x22c - State transition configuration B"]
    pub st5_tconfb: ST5_TCONFB,
    #[doc = "0x230 - State transition configuration A"]
    pub st6_tconfa: ST6_TCONFA,
    #[doc = "0x234 - State transition configuration B"]
    pub st6_tconfb: ST6_TCONFB,
    #[doc = "0x238 - State transition configuration A"]
    pub st7_tconfa: ST7_TCONFA,
    #[doc = "0x23c - State transition configuration B"]
    pub st7_tconfb: ST7_TCONFB,
    #[doc = "0x240 - State transition configuration A"]
    pub st8_tconfa: ST8_TCONFA,
    #[doc = "0x244 - State transition configuration B"]
    pub st8_tconfb: ST8_TCONFB,
    #[doc = "0x248 - State transition configuration A"]
    pub st9_tconfa: ST9_TCONFA,
    #[doc = "0x24c - State transition configuration B"]
    pub st9_tconfb: ST9_TCONFB,
    #[doc = "0x250 - State transition configuration A"]
    pub st10_tconfa: ST10_TCONFA,
    #[doc = "0x254 - State transition configuration B"]
    pub st10_tconfb: ST10_TCONFB,
    #[doc = "0x258 - State transition configuration A"]
    pub st11_tconfa: ST11_TCONFA,
    #[doc = "0x25c - State transition configuration B"]
    pub st11_tconfb: ST11_TCONFB,
    #[doc = "0x260 - State transition configuration A"]
    pub st12_tconfa: ST12_TCONFA,
    #[doc = "0x264 - State transition configuration B"]
    pub st12_tconfb: ST12_TCONFB,
    #[doc = "0x268 - State transition configuration A"]
    pub st13_tconfa: ST13_TCONFA,
    #[doc = "0x26c - State transition configuration B"]
    pub st13_tconfb: ST13_TCONFB,
    #[doc = "0x270 - State transition configuration A"]
    pub st14_tconfa: ST14_TCONFA,
    #[doc = "0x274 - State transition configuration B"]
    pub st14_tconfb: ST14_TCONFB,
    #[doc = "0x278 - State transition configuration A"]
    pub st15_tconfa: ST15_TCONFA,
    #[doc = "0x27c - State transition configuration B"]
    pub st15_tconfb: ST15_TCONFB,
    #[doc = "0x280 - Scan results"]
    pub buf0_data: BUF0_DATA,
    #[doc = "0x284 - Scan results"]
    pub buf1_data: BUF1_DATA,
    #[doc = "0x288 - Scan results"]
    pub buf2_data: BUF2_DATA,
    #[doc = "0x28c - Scan results"]
    pub buf3_data: BUF3_DATA,
    #[doc = "0x290 - Scan results"]
    pub buf4_data: BUF4_DATA,
    #[doc = "0x294 - Scan results"]
    pub buf5_data: BUF5_DATA,
    #[doc = "0x298 - Scan results"]
    pub buf6_data: BUF6_DATA,
    #[doc = "0x29c - Scan results"]
    pub buf7_data: BUF7_DATA,
    #[doc = "0x2a0 - Scan results"]
    pub buf8_data: BUF8_DATA,
    #[doc = "0x2a4 - Scan results"]
    pub buf9_data: BUF9_DATA,
    #[doc = "0x2a8 - Scan results"]
    pub buf10_data: BUF10_DATA,
    #[doc = "0x2ac - Scan results"]
    pub buf11_data: BUF11_DATA,
    #[doc = "0x2b0 - Scan results"]
    pub buf12_data: BUF12_DATA,
    #[doc = "0x2b4 - Scan results"]
    pub buf13_data: BUF13_DATA,
    #[doc = "0x2b8 - Scan results"]
    pub buf14_data: BUF14_DATA,
    #[doc = "0x2bc - Scan results"]
    pub buf15_data: BUF15_DATA,
    #[doc = "0x2c0 - Scan configuration"]
    pub ch0_timing: CH0_TIMING,
    #[doc = "0x2c4 - Scan configuration"]
    pub ch0_interact: CH0_INTERACT,
    #[doc = "0x2c8 - Scan configuration"]
    pub ch0_eval: CH0_EVAL,
    _reserved74: [u8; 0x04],
    #[doc = "0x2d0 - Scan configuration"]
    pub ch1_timing: CH1_TIMING,
    #[doc = "0x2d4 - Scan configuration"]
    pub ch1_interact: CH1_INTERACT,
    #[doc = "0x2d8 - Scan configuration"]
    pub ch1_eval: CH1_EVAL,
    _reserved77: [u8; 0x04],
    #[doc = "0x2e0 - Scan configuration"]
    pub ch2_timing: CH2_TIMING,
    #[doc = "0x2e4 - Scan configuration"]
    pub ch2_interact: CH2_INTERACT,
    #[doc = "0x2e8 - Scan configuration"]
    pub ch2_eval: CH2_EVAL,
    _reserved80: [u8; 0x04],
    #[doc = "0x2f0 - Scan configuration"]
    pub ch3_timing: CH3_TIMING,
    #[doc = "0x2f4 - Scan configuration"]
    pub ch3_interact: CH3_INTERACT,
    #[doc = "0x2f8 - Scan configuration"]
    pub ch3_eval: CH3_EVAL,
    _reserved83: [u8; 0x04],
    #[doc = "0x300 - Scan configuration"]
    pub ch4_timing: CH4_TIMING,
    #[doc = "0x304 - Scan configuration"]
    pub ch4_interact: CH4_INTERACT,
    #[doc = "0x308 - Scan configuration"]
    pub ch4_eval: CH4_EVAL,
    _reserved86: [u8; 0x04],
    #[doc = "0x310 - Scan configuration"]
    pub ch5_timing: CH5_TIMING,
    #[doc = "0x314 - Scan configuration"]
    pub ch5_interact: CH5_INTERACT,
    #[doc = "0x318 - Scan configuration"]
    pub ch5_eval: CH5_EVAL,
    _reserved89: [u8; 0x04],
    #[doc = "0x320 - Scan configuration"]
    pub ch6_timing: CH6_TIMING,
    #[doc = "0x324 - Scan configuration"]
    pub ch6_interact: CH6_INTERACT,
    #[doc = "0x328 - Scan configuration"]
    pub ch6_eval: CH6_EVAL,
    _reserved92: [u8; 0x04],
    #[doc = "0x330 - Scan configuration"]
    pub ch7_timing: CH7_TIMING,
    #[doc = "0x334 - Scan configuration"]
    pub ch7_interact: CH7_INTERACT,
    #[doc = "0x338 - Scan configuration"]
    pub ch7_eval: CH7_EVAL,
    _reserved95: [u8; 0x04],
    #[doc = "0x340 - Scan configuration"]
    pub ch8_timing: CH8_TIMING,
    #[doc = "0x344 - Scan configuration"]
    pub ch8_interact: CH8_INTERACT,
    #[doc = "0x348 - Scan configuration"]
    pub ch8_eval: CH8_EVAL,
    _reserved98: [u8; 0x04],
    #[doc = "0x350 - Scan configuration"]
    pub ch9_timing: CH9_TIMING,
    #[doc = "0x354 - Scan configuration"]
    pub ch9_interact: CH9_INTERACT,
    #[doc = "0x358 - Scan configuration"]
    pub ch9_eval: CH9_EVAL,
    _reserved101: [u8; 0x04],
    #[doc = "0x360 - Scan configuration"]
    pub ch10_timing: CH10_TIMING,
    #[doc = "0x364 - Scan configuration"]
    pub ch10_interact: CH10_INTERACT,
    #[doc = "0x368 - Scan configuration"]
    pub ch10_eval: CH10_EVAL,
    _reserved104: [u8; 0x04],
    #[doc = "0x370 - Scan configuration"]
    pub ch11_timing: CH11_TIMING,
    #[doc = "0x374 - Scan configuration"]
    pub ch11_interact: CH11_INTERACT,
    #[doc = "0x378 - Scan configuration"]
    pub ch11_eval: CH11_EVAL,
    _reserved107: [u8; 0x04],
    #[doc = "0x380 - Scan configuration"]
    pub ch12_timing: CH12_TIMING,
    #[doc = "0x384 - Scan configuration"]
    pub ch12_interact: CH12_INTERACT,
    #[doc = "0x388 - Scan configuration"]
    pub ch12_eval: CH12_EVAL,
    _reserved110: [u8; 0x04],
    #[doc = "0x390 - Scan configuration"]
    pub ch13_timing: CH13_TIMING,
    #[doc = "0x394 - Scan configuration"]
    pub ch13_interact: CH13_INTERACT,
    #[doc = "0x398 - Scan configuration"]
    pub ch13_eval: CH13_EVAL,
    _reserved113: [u8; 0x04],
    #[doc = "0x3a0 - Scan configuration"]
    pub ch14_timing: CH14_TIMING,
    #[doc = "0x3a4 - Scan configuration"]
    pub ch14_interact: CH14_INTERACT,
    #[doc = "0x3a8 - Scan configuration"]
    pub ch14_eval: CH14_EVAL,
    _reserved116: [u8; 0x04],
    #[doc = "0x3b0 - Scan configuration"]
    pub ch15_timing: CH15_TIMING,
    #[doc = "0x3b4 - Scan configuration"]
    pub ch15_interact: CH15_INTERACT,
    #[doc = "0x3b8 - Scan configuration"]
    pub ch15_eval: CH15_EVAL,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "TIMCTRL (rw) register accessor: an alias for `Reg<TIMCTRL_SPEC>`"]
pub type TIMCTRL = crate::Reg<timctrl::TIMCTRL_SPEC>;
#[doc = "Timing Control Register"]
pub mod timctrl;
#[doc = "PERCTRL (rw) register accessor: an alias for `Reg<PERCTRL_SPEC>`"]
pub type PERCTRL = crate::Reg<perctrl::PERCTRL_SPEC>;
#[doc = "Peripheral Control Register"]
pub mod perctrl;
#[doc = "DECCTRL (rw) register accessor: an alias for `Reg<DECCTRL_SPEC>`"]
pub type DECCTRL = crate::Reg<decctrl::DECCTRL_SPEC>;
#[doc = "Decoder control Register"]
pub mod decctrl;
#[doc = "BIASCTRL (rw) register accessor: an alias for `Reg<BIASCTRL_SPEC>`"]
pub type BIASCTRL = crate::Reg<biasctrl::BIASCTRL_SPEC>;
#[doc = "Bias Control Register"]
pub mod biasctrl;
#[doc = "CMD (w) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "CHEN (rw) register accessor: an alias for `Reg<CHEN_SPEC>`"]
pub type CHEN = crate::Reg<chen::CHEN_SPEC>;
#[doc = "Channel enable Register"]
pub mod chen;
#[doc = "SCANRES (r) register accessor: an alias for `Reg<SCANRES_SPEC>`"]
pub type SCANRES = crate::Reg<scanres::SCANRES_SPEC>;
#[doc = "Scan result register"]
pub mod scanres;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "PTR (r) register accessor: an alias for `Reg<PTR_SPEC>`"]
pub type PTR = crate::Reg<ptr::PTR_SPEC>;
#[doc = "Result buffer pointers"]
pub mod ptr;
#[doc = "BUFDATA (r) register accessor: an alias for `Reg<BUFDATA_SPEC>`"]
pub type BUFDATA = crate::Reg<bufdata::BUFDATA_SPEC>;
#[doc = "Result buffer data register"]
pub mod bufdata;
#[doc = "CURCH (r) register accessor: an alias for `Reg<CURCH_SPEC>`"]
pub type CURCH = crate::Reg<curch::CURCH_SPEC>;
#[doc = "Current channel index"]
pub mod curch;
#[doc = "DECSTATE (rw) register accessor: an alias for `Reg<DECSTATE_SPEC>`"]
pub type DECSTATE = crate::Reg<decstate::DECSTATE_SPEC>;
#[doc = "Current decoder state"]
pub mod decstate;
#[doc = "SENSORSTATE (rw) register accessor: an alias for `Reg<SENSORSTATE_SPEC>`"]
pub type SENSORSTATE = crate::Reg<sensorstate::SENSORSTATE_SPEC>;
#[doc = "Decoder input register"]
pub mod sensorstate;
#[doc = "IDLECONF (rw) register accessor: an alias for `Reg<IDLECONF_SPEC>`"]
pub type IDLECONF = crate::Reg<idleconf::IDLECONF_SPEC>;
#[doc = "GPIO Idle phase configuration"]
pub mod idleconf;
#[doc = "ALTEXCONF (rw) register accessor: an alias for `Reg<ALTEXCONF_SPEC>`"]
pub type ALTEXCONF = crate::Reg<altexconf::ALTEXCONF_SPEC>;
#[doc = "Alternative excite pin configuration"]
pub mod altexconf;
#[doc = "IF (r) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "IFC (w) register accessor: an alias for `Reg<IFC_SPEC>`"]
pub type IFC = crate::Reg<ifc::IFC_SPEC>;
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "IFS (w) register accessor: an alias for `Reg<IFS_SPEC>`"]
pub type IFS = crate::Reg<ifs::IFS_SPEC>;
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "IEN (rw) register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "SYNCBUSY (r) register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "ROUTE (rw) register accessor: an alias for `Reg<ROUTE_SPEC>`"]
pub type ROUTE = crate::Reg<route::ROUTE_SPEC>;
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "POWERDOWN (rw) register accessor: an alias for `Reg<POWERDOWN_SPEC>`"]
pub type POWERDOWN = crate::Reg<powerdown::POWERDOWN_SPEC>;
#[doc = "LESENSE RAM power-down register"]
pub mod powerdown;
#[doc = "ST0_TCONFA (rw) register accessor: an alias for `Reg<ST0_TCONFA_SPEC>`"]
pub type ST0_TCONFA = crate::Reg<st0_tconfa::ST0_TCONFA_SPEC>;
#[doc = "State transition configuration A"]
pub mod st0_tconfa;
#[doc = "ST0_TCONFB (rw) register accessor: an alias for `Reg<ST0_TCONFB_SPEC>`"]
pub type ST0_TCONFB = crate::Reg<st0_tconfb::ST0_TCONFB_SPEC>;
#[doc = "State transition configuration B"]
pub mod st0_tconfb;
#[doc = "ST1_TCONFA (rw) register accessor: an alias for `Reg<ST1_TCONFA_SPEC>`"]
pub type ST1_TCONFA = crate::Reg<st1_tconfa::ST1_TCONFA_SPEC>;
#[doc = "State transition configuration A"]
pub mod st1_tconfa;
#[doc = "ST1_TCONFB (rw) register accessor: an alias for `Reg<ST1_TCONFB_SPEC>`"]
pub type ST1_TCONFB = crate::Reg<st1_tconfb::ST1_TCONFB_SPEC>;
#[doc = "State transition configuration B"]
pub mod st1_tconfb;
#[doc = "ST2_TCONFA (rw) register accessor: an alias for `Reg<ST2_TCONFA_SPEC>`"]
pub type ST2_TCONFA = crate::Reg<st2_tconfa::ST2_TCONFA_SPEC>;
#[doc = "State transition configuration A"]
pub mod st2_tconfa;
#[doc = "ST2_TCONFB (rw) register accessor: an alias for `Reg<ST2_TCONFB_SPEC>`"]
pub type ST2_TCONFB = crate::Reg<st2_tconfb::ST2_TCONFB_SPEC>;
#[doc = "State transition configuration B"]
pub mod st2_tconfb;
#[doc = "ST3_TCONFA (rw) register accessor: an alias for `Reg<ST3_TCONFA_SPEC>`"]
pub type ST3_TCONFA = crate::Reg<st3_tconfa::ST3_TCONFA_SPEC>;
#[doc = "State transition configuration A"]
pub mod st3_tconfa;
#[doc = "ST3_TCONFB (rw) register accessor: an alias for `Reg<ST3_TCONFB_SPEC>`"]
pub type ST3_TCONFB = crate::Reg<st3_tconfb::ST3_TCONFB_SPEC>;
#[doc = "State transition configuration B"]
pub mod st3_tconfb;
#[doc = "ST4_TCONFA (rw) register accessor: an alias for `Reg<ST4_TCONFA_SPEC>`"]
pub type ST4_TCONFA = crate::Reg<st4_tconfa::ST4_TCONFA_SPEC>;
#[doc = "State transition configuration A"]
pub mod st4_tconfa;
#[doc = "ST4_TCONFB (rw) register accessor: an alias for `Reg<ST4_TCONFB_SPEC>`"]
pub type ST4_TCONFB = crate::Reg<st4_tconfb::ST4_TCONFB_SPEC>;
#[doc = "State transition configuration B"]
pub mod st4_tconfb;
#[doc = "ST5_TCONFA (rw) register accessor: an alias for `Reg<ST5_TCONFA_SPEC>`"]
pub type ST5_TCONFA = crate::Reg<st5_tconfa::ST5_TCONFA_SPEC>;
#[doc = "State transition configuration A"]
pub mod st5_tconfa;
#[doc = "ST5_TCONFB (rw) register accessor: an alias for `Reg<ST5_TCONFB_SPEC>`"]
pub type ST5_TCONFB = crate::Reg<st5_tconfb::ST5_TCONFB_SPEC>;
#[doc = "State transition configuration B"]
pub mod st5_tconfb;
#[doc = "ST6_TCONFA (rw) register accessor: an alias for `Reg<ST6_TCONFA_SPEC>`"]
pub type ST6_TCONFA = crate::Reg<st6_tconfa::ST6_TCONFA_SPEC>;
#[doc = "State transition configuration A"]
pub mod st6_tconfa;
#[doc = "ST6_TCONFB (rw) register accessor: an alias for `Reg<ST6_TCONFB_SPEC>`"]
pub type ST6_TCONFB = crate::Reg<st6_tconfb::ST6_TCONFB_SPEC>;
#[doc = "State transition configuration B"]
pub mod st6_tconfb;
#[doc = "ST7_TCONFA (rw) register accessor: an alias for `Reg<ST7_TCONFA_SPEC>`"]
pub type ST7_TCONFA = crate::Reg<st7_tconfa::ST7_TCONFA_SPEC>;
#[doc = "State transition configuration A"]
pub mod st7_tconfa;
#[doc = "ST7_TCONFB (rw) register accessor: an alias for `Reg<ST7_TCONFB_SPEC>`"]
pub type ST7_TCONFB = crate::Reg<st7_tconfb::ST7_TCONFB_SPEC>;
#[doc = "State transition configuration B"]
pub mod st7_tconfb;
#[doc = "ST8_TCONFA (rw) register accessor: an alias for `Reg<ST8_TCONFA_SPEC>`"]
pub type ST8_TCONFA = crate::Reg<st8_tconfa::ST8_TCONFA_SPEC>;
#[doc = "State transition configuration A"]
pub mod st8_tconfa;
#[doc = "ST8_TCONFB (rw) register accessor: an alias for `Reg<ST8_TCONFB_SPEC>`"]
pub type ST8_TCONFB = crate::Reg<st8_tconfb::ST8_TCONFB_SPEC>;
#[doc = "State transition configuration B"]
pub mod st8_tconfb;
#[doc = "ST9_TCONFA (rw) register accessor: an alias for `Reg<ST9_TCONFA_SPEC>`"]
pub type ST9_TCONFA = crate::Reg<st9_tconfa::ST9_TCONFA_SPEC>;
#[doc = "State transition configuration A"]
pub mod st9_tconfa;
#[doc = "ST9_TCONFB (rw) register accessor: an alias for `Reg<ST9_TCONFB_SPEC>`"]
pub type ST9_TCONFB = crate::Reg<st9_tconfb::ST9_TCONFB_SPEC>;
#[doc = "State transition configuration B"]
pub mod st9_tconfb;
#[doc = "ST10_TCONFA (rw) register accessor: an alias for `Reg<ST10_TCONFA_SPEC>`"]
pub type ST10_TCONFA = crate::Reg<st10_tconfa::ST10_TCONFA_SPEC>;
#[doc = "State transition configuration A"]
pub mod st10_tconfa;
#[doc = "ST10_TCONFB (rw) register accessor: an alias for `Reg<ST10_TCONFB_SPEC>`"]
pub type ST10_TCONFB = crate::Reg<st10_tconfb::ST10_TCONFB_SPEC>;
#[doc = "State transition configuration B"]
pub mod st10_tconfb;
#[doc = "ST11_TCONFA (rw) register accessor: an alias for `Reg<ST11_TCONFA_SPEC>`"]
pub type ST11_TCONFA = crate::Reg<st11_tconfa::ST11_TCONFA_SPEC>;
#[doc = "State transition configuration A"]
pub mod st11_tconfa;
#[doc = "ST11_TCONFB (rw) register accessor: an alias for `Reg<ST11_TCONFB_SPEC>`"]
pub type ST11_TCONFB = crate::Reg<st11_tconfb::ST11_TCONFB_SPEC>;
#[doc = "State transition configuration B"]
pub mod st11_tconfb;
#[doc = "ST12_TCONFA (rw) register accessor: an alias for `Reg<ST12_TCONFA_SPEC>`"]
pub type ST12_TCONFA = crate::Reg<st12_tconfa::ST12_TCONFA_SPEC>;
#[doc = "State transition configuration A"]
pub mod st12_tconfa;
#[doc = "ST12_TCONFB (rw) register accessor: an alias for `Reg<ST12_TCONFB_SPEC>`"]
pub type ST12_TCONFB = crate::Reg<st12_tconfb::ST12_TCONFB_SPEC>;
#[doc = "State transition configuration B"]
pub mod st12_tconfb;
#[doc = "ST13_TCONFA (rw) register accessor: an alias for `Reg<ST13_TCONFA_SPEC>`"]
pub type ST13_TCONFA = crate::Reg<st13_tconfa::ST13_TCONFA_SPEC>;
#[doc = "State transition configuration A"]
pub mod st13_tconfa;
#[doc = "ST13_TCONFB (rw) register accessor: an alias for `Reg<ST13_TCONFB_SPEC>`"]
pub type ST13_TCONFB = crate::Reg<st13_tconfb::ST13_TCONFB_SPEC>;
#[doc = "State transition configuration B"]
pub mod st13_tconfb;
#[doc = "ST14_TCONFA (rw) register accessor: an alias for `Reg<ST14_TCONFA_SPEC>`"]
pub type ST14_TCONFA = crate::Reg<st14_tconfa::ST14_TCONFA_SPEC>;
#[doc = "State transition configuration A"]
pub mod st14_tconfa;
#[doc = "ST14_TCONFB (rw) register accessor: an alias for `Reg<ST14_TCONFB_SPEC>`"]
pub type ST14_TCONFB = crate::Reg<st14_tconfb::ST14_TCONFB_SPEC>;
#[doc = "State transition configuration B"]
pub mod st14_tconfb;
#[doc = "ST15_TCONFA (rw) register accessor: an alias for `Reg<ST15_TCONFA_SPEC>`"]
pub type ST15_TCONFA = crate::Reg<st15_tconfa::ST15_TCONFA_SPEC>;
#[doc = "State transition configuration A"]
pub mod st15_tconfa;
#[doc = "ST15_TCONFB (rw) register accessor: an alias for `Reg<ST15_TCONFB_SPEC>`"]
pub type ST15_TCONFB = crate::Reg<st15_tconfb::ST15_TCONFB_SPEC>;
#[doc = "State transition configuration B"]
pub mod st15_tconfb;
#[doc = "BUF0_DATA (rw) register accessor: an alias for `Reg<BUF0_DATA_SPEC>`"]
pub type BUF0_DATA = crate::Reg<buf0_data::BUF0_DATA_SPEC>;
#[doc = "Scan results"]
pub mod buf0_data;
#[doc = "BUF1_DATA (rw) register accessor: an alias for `Reg<BUF1_DATA_SPEC>`"]
pub type BUF1_DATA = crate::Reg<buf1_data::BUF1_DATA_SPEC>;
#[doc = "Scan results"]
pub mod buf1_data;
#[doc = "BUF2_DATA (rw) register accessor: an alias for `Reg<BUF2_DATA_SPEC>`"]
pub type BUF2_DATA = crate::Reg<buf2_data::BUF2_DATA_SPEC>;
#[doc = "Scan results"]
pub mod buf2_data;
#[doc = "BUF3_DATA (rw) register accessor: an alias for `Reg<BUF3_DATA_SPEC>`"]
pub type BUF3_DATA = crate::Reg<buf3_data::BUF3_DATA_SPEC>;
#[doc = "Scan results"]
pub mod buf3_data;
#[doc = "BUF4_DATA (rw) register accessor: an alias for `Reg<BUF4_DATA_SPEC>`"]
pub type BUF4_DATA = crate::Reg<buf4_data::BUF4_DATA_SPEC>;
#[doc = "Scan results"]
pub mod buf4_data;
#[doc = "BUF5_DATA (rw) register accessor: an alias for `Reg<BUF5_DATA_SPEC>`"]
pub type BUF5_DATA = crate::Reg<buf5_data::BUF5_DATA_SPEC>;
#[doc = "Scan results"]
pub mod buf5_data;
#[doc = "BUF6_DATA (rw) register accessor: an alias for `Reg<BUF6_DATA_SPEC>`"]
pub type BUF6_DATA = crate::Reg<buf6_data::BUF6_DATA_SPEC>;
#[doc = "Scan results"]
pub mod buf6_data;
#[doc = "BUF7_DATA (rw) register accessor: an alias for `Reg<BUF7_DATA_SPEC>`"]
pub type BUF7_DATA = crate::Reg<buf7_data::BUF7_DATA_SPEC>;
#[doc = "Scan results"]
pub mod buf7_data;
#[doc = "BUF8_DATA (rw) register accessor: an alias for `Reg<BUF8_DATA_SPEC>`"]
pub type BUF8_DATA = crate::Reg<buf8_data::BUF8_DATA_SPEC>;
#[doc = "Scan results"]
pub mod buf8_data;
#[doc = "BUF9_DATA (rw) register accessor: an alias for `Reg<BUF9_DATA_SPEC>`"]
pub type BUF9_DATA = crate::Reg<buf9_data::BUF9_DATA_SPEC>;
#[doc = "Scan results"]
pub mod buf9_data;
#[doc = "BUF10_DATA (rw) register accessor: an alias for `Reg<BUF10_DATA_SPEC>`"]
pub type BUF10_DATA = crate::Reg<buf10_data::BUF10_DATA_SPEC>;
#[doc = "Scan results"]
pub mod buf10_data;
#[doc = "BUF11_DATA (rw) register accessor: an alias for `Reg<BUF11_DATA_SPEC>`"]
pub type BUF11_DATA = crate::Reg<buf11_data::BUF11_DATA_SPEC>;
#[doc = "Scan results"]
pub mod buf11_data;
#[doc = "BUF12_DATA (rw) register accessor: an alias for `Reg<BUF12_DATA_SPEC>`"]
pub type BUF12_DATA = crate::Reg<buf12_data::BUF12_DATA_SPEC>;
#[doc = "Scan results"]
pub mod buf12_data;
#[doc = "BUF13_DATA (rw) register accessor: an alias for `Reg<BUF13_DATA_SPEC>`"]
pub type BUF13_DATA = crate::Reg<buf13_data::BUF13_DATA_SPEC>;
#[doc = "Scan results"]
pub mod buf13_data;
#[doc = "BUF14_DATA (rw) register accessor: an alias for `Reg<BUF14_DATA_SPEC>`"]
pub type BUF14_DATA = crate::Reg<buf14_data::BUF14_DATA_SPEC>;
#[doc = "Scan results"]
pub mod buf14_data;
#[doc = "BUF15_DATA (rw) register accessor: an alias for `Reg<BUF15_DATA_SPEC>`"]
pub type BUF15_DATA = crate::Reg<buf15_data::BUF15_DATA_SPEC>;
#[doc = "Scan results"]
pub mod buf15_data;
#[doc = "CH0_TIMING (rw) register accessor: an alias for `Reg<CH0_TIMING_SPEC>`"]
pub type CH0_TIMING = crate::Reg<ch0_timing::CH0_TIMING_SPEC>;
#[doc = "Scan configuration"]
pub mod ch0_timing;
#[doc = "CH0_INTERACT (rw) register accessor: an alias for `Reg<CH0_INTERACT_SPEC>`"]
pub type CH0_INTERACT = crate::Reg<ch0_interact::CH0_INTERACT_SPEC>;
#[doc = "Scan configuration"]
pub mod ch0_interact;
#[doc = "CH0_EVAL (rw) register accessor: an alias for `Reg<CH0_EVAL_SPEC>`"]
pub type CH0_EVAL = crate::Reg<ch0_eval::CH0_EVAL_SPEC>;
#[doc = "Scan configuration"]
pub mod ch0_eval;
#[doc = "CH1_TIMING (rw) register accessor: an alias for `Reg<CH1_TIMING_SPEC>`"]
pub type CH1_TIMING = crate::Reg<ch1_timing::CH1_TIMING_SPEC>;
#[doc = "Scan configuration"]
pub mod ch1_timing;
#[doc = "CH1_INTERACT (rw) register accessor: an alias for `Reg<CH1_INTERACT_SPEC>`"]
pub type CH1_INTERACT = crate::Reg<ch1_interact::CH1_INTERACT_SPEC>;
#[doc = "Scan configuration"]
pub mod ch1_interact;
#[doc = "CH1_EVAL (rw) register accessor: an alias for `Reg<CH1_EVAL_SPEC>`"]
pub type CH1_EVAL = crate::Reg<ch1_eval::CH1_EVAL_SPEC>;
#[doc = "Scan configuration"]
pub mod ch1_eval;
#[doc = "CH2_TIMING (rw) register accessor: an alias for `Reg<CH2_TIMING_SPEC>`"]
pub type CH2_TIMING = crate::Reg<ch2_timing::CH2_TIMING_SPEC>;
#[doc = "Scan configuration"]
pub mod ch2_timing;
#[doc = "CH2_INTERACT (rw) register accessor: an alias for `Reg<CH2_INTERACT_SPEC>`"]
pub type CH2_INTERACT = crate::Reg<ch2_interact::CH2_INTERACT_SPEC>;
#[doc = "Scan configuration"]
pub mod ch2_interact;
#[doc = "CH2_EVAL (rw) register accessor: an alias for `Reg<CH2_EVAL_SPEC>`"]
pub type CH2_EVAL = crate::Reg<ch2_eval::CH2_EVAL_SPEC>;
#[doc = "Scan configuration"]
pub mod ch2_eval;
#[doc = "CH3_TIMING (rw) register accessor: an alias for `Reg<CH3_TIMING_SPEC>`"]
pub type CH3_TIMING = crate::Reg<ch3_timing::CH3_TIMING_SPEC>;
#[doc = "Scan configuration"]
pub mod ch3_timing;
#[doc = "CH3_INTERACT (rw) register accessor: an alias for `Reg<CH3_INTERACT_SPEC>`"]
pub type CH3_INTERACT = crate::Reg<ch3_interact::CH3_INTERACT_SPEC>;
#[doc = "Scan configuration"]
pub mod ch3_interact;
#[doc = "CH3_EVAL (rw) register accessor: an alias for `Reg<CH3_EVAL_SPEC>`"]
pub type CH3_EVAL = crate::Reg<ch3_eval::CH3_EVAL_SPEC>;
#[doc = "Scan configuration"]
pub mod ch3_eval;
#[doc = "CH4_TIMING (rw) register accessor: an alias for `Reg<CH4_TIMING_SPEC>`"]
pub type CH4_TIMING = crate::Reg<ch4_timing::CH4_TIMING_SPEC>;
#[doc = "Scan configuration"]
pub mod ch4_timing;
#[doc = "CH4_INTERACT (rw) register accessor: an alias for `Reg<CH4_INTERACT_SPEC>`"]
pub type CH4_INTERACT = crate::Reg<ch4_interact::CH4_INTERACT_SPEC>;
#[doc = "Scan configuration"]
pub mod ch4_interact;
#[doc = "CH4_EVAL (rw) register accessor: an alias for `Reg<CH4_EVAL_SPEC>`"]
pub type CH4_EVAL = crate::Reg<ch4_eval::CH4_EVAL_SPEC>;
#[doc = "Scan configuration"]
pub mod ch4_eval;
#[doc = "CH5_TIMING (rw) register accessor: an alias for `Reg<CH5_TIMING_SPEC>`"]
pub type CH5_TIMING = crate::Reg<ch5_timing::CH5_TIMING_SPEC>;
#[doc = "Scan configuration"]
pub mod ch5_timing;
#[doc = "CH5_INTERACT (rw) register accessor: an alias for `Reg<CH5_INTERACT_SPEC>`"]
pub type CH5_INTERACT = crate::Reg<ch5_interact::CH5_INTERACT_SPEC>;
#[doc = "Scan configuration"]
pub mod ch5_interact;
#[doc = "CH5_EVAL (rw) register accessor: an alias for `Reg<CH5_EVAL_SPEC>`"]
pub type CH5_EVAL = crate::Reg<ch5_eval::CH5_EVAL_SPEC>;
#[doc = "Scan configuration"]
pub mod ch5_eval;
#[doc = "CH6_TIMING (rw) register accessor: an alias for `Reg<CH6_TIMING_SPEC>`"]
pub type CH6_TIMING = crate::Reg<ch6_timing::CH6_TIMING_SPEC>;
#[doc = "Scan configuration"]
pub mod ch6_timing;
#[doc = "CH6_INTERACT (rw) register accessor: an alias for `Reg<CH6_INTERACT_SPEC>`"]
pub type CH6_INTERACT = crate::Reg<ch6_interact::CH6_INTERACT_SPEC>;
#[doc = "Scan configuration"]
pub mod ch6_interact;
#[doc = "CH6_EVAL (rw) register accessor: an alias for `Reg<CH6_EVAL_SPEC>`"]
pub type CH6_EVAL = crate::Reg<ch6_eval::CH6_EVAL_SPEC>;
#[doc = "Scan configuration"]
pub mod ch6_eval;
#[doc = "CH7_TIMING (rw) register accessor: an alias for `Reg<CH7_TIMING_SPEC>`"]
pub type CH7_TIMING = crate::Reg<ch7_timing::CH7_TIMING_SPEC>;
#[doc = "Scan configuration"]
pub mod ch7_timing;
#[doc = "CH7_INTERACT (rw) register accessor: an alias for `Reg<CH7_INTERACT_SPEC>`"]
pub type CH7_INTERACT = crate::Reg<ch7_interact::CH7_INTERACT_SPEC>;
#[doc = "Scan configuration"]
pub mod ch7_interact;
#[doc = "CH7_EVAL (rw) register accessor: an alias for `Reg<CH7_EVAL_SPEC>`"]
pub type CH7_EVAL = crate::Reg<ch7_eval::CH7_EVAL_SPEC>;
#[doc = "Scan configuration"]
pub mod ch7_eval;
#[doc = "CH8_TIMING (rw) register accessor: an alias for `Reg<CH8_TIMING_SPEC>`"]
pub type CH8_TIMING = crate::Reg<ch8_timing::CH8_TIMING_SPEC>;
#[doc = "Scan configuration"]
pub mod ch8_timing;
#[doc = "CH8_INTERACT (rw) register accessor: an alias for `Reg<CH8_INTERACT_SPEC>`"]
pub type CH8_INTERACT = crate::Reg<ch8_interact::CH8_INTERACT_SPEC>;
#[doc = "Scan configuration"]
pub mod ch8_interact;
#[doc = "CH8_EVAL (rw) register accessor: an alias for `Reg<CH8_EVAL_SPEC>`"]
pub type CH8_EVAL = crate::Reg<ch8_eval::CH8_EVAL_SPEC>;
#[doc = "Scan configuration"]
pub mod ch8_eval;
#[doc = "CH9_TIMING (rw) register accessor: an alias for `Reg<CH9_TIMING_SPEC>`"]
pub type CH9_TIMING = crate::Reg<ch9_timing::CH9_TIMING_SPEC>;
#[doc = "Scan configuration"]
pub mod ch9_timing;
#[doc = "CH9_INTERACT (rw) register accessor: an alias for `Reg<CH9_INTERACT_SPEC>`"]
pub type CH9_INTERACT = crate::Reg<ch9_interact::CH9_INTERACT_SPEC>;
#[doc = "Scan configuration"]
pub mod ch9_interact;
#[doc = "CH9_EVAL (rw) register accessor: an alias for `Reg<CH9_EVAL_SPEC>`"]
pub type CH9_EVAL = crate::Reg<ch9_eval::CH9_EVAL_SPEC>;
#[doc = "Scan configuration"]
pub mod ch9_eval;
#[doc = "CH10_TIMING (rw) register accessor: an alias for `Reg<CH10_TIMING_SPEC>`"]
pub type CH10_TIMING = crate::Reg<ch10_timing::CH10_TIMING_SPEC>;
#[doc = "Scan configuration"]
pub mod ch10_timing;
#[doc = "CH10_INTERACT (rw) register accessor: an alias for `Reg<CH10_INTERACT_SPEC>`"]
pub type CH10_INTERACT = crate::Reg<ch10_interact::CH10_INTERACT_SPEC>;
#[doc = "Scan configuration"]
pub mod ch10_interact;
#[doc = "CH10_EVAL (rw) register accessor: an alias for `Reg<CH10_EVAL_SPEC>`"]
pub type CH10_EVAL = crate::Reg<ch10_eval::CH10_EVAL_SPEC>;
#[doc = "Scan configuration"]
pub mod ch10_eval;
#[doc = "CH11_TIMING (rw) register accessor: an alias for `Reg<CH11_TIMING_SPEC>`"]
pub type CH11_TIMING = crate::Reg<ch11_timing::CH11_TIMING_SPEC>;
#[doc = "Scan configuration"]
pub mod ch11_timing;
#[doc = "CH11_INTERACT (rw) register accessor: an alias for `Reg<CH11_INTERACT_SPEC>`"]
pub type CH11_INTERACT = crate::Reg<ch11_interact::CH11_INTERACT_SPEC>;
#[doc = "Scan configuration"]
pub mod ch11_interact;
#[doc = "CH11_EVAL (rw) register accessor: an alias for `Reg<CH11_EVAL_SPEC>`"]
pub type CH11_EVAL = crate::Reg<ch11_eval::CH11_EVAL_SPEC>;
#[doc = "Scan configuration"]
pub mod ch11_eval;
#[doc = "CH12_TIMING (rw) register accessor: an alias for `Reg<CH12_TIMING_SPEC>`"]
pub type CH12_TIMING = crate::Reg<ch12_timing::CH12_TIMING_SPEC>;
#[doc = "Scan configuration"]
pub mod ch12_timing;
#[doc = "CH12_INTERACT (rw) register accessor: an alias for `Reg<CH12_INTERACT_SPEC>`"]
pub type CH12_INTERACT = crate::Reg<ch12_interact::CH12_INTERACT_SPEC>;
#[doc = "Scan configuration"]
pub mod ch12_interact;
#[doc = "CH12_EVAL (rw) register accessor: an alias for `Reg<CH12_EVAL_SPEC>`"]
pub type CH12_EVAL = crate::Reg<ch12_eval::CH12_EVAL_SPEC>;
#[doc = "Scan configuration"]
pub mod ch12_eval;
#[doc = "CH13_TIMING (rw) register accessor: an alias for `Reg<CH13_TIMING_SPEC>`"]
pub type CH13_TIMING = crate::Reg<ch13_timing::CH13_TIMING_SPEC>;
#[doc = "Scan configuration"]
pub mod ch13_timing;
#[doc = "CH13_INTERACT (rw) register accessor: an alias for `Reg<CH13_INTERACT_SPEC>`"]
pub type CH13_INTERACT = crate::Reg<ch13_interact::CH13_INTERACT_SPEC>;
#[doc = "Scan configuration"]
pub mod ch13_interact;
#[doc = "CH13_EVAL (rw) register accessor: an alias for `Reg<CH13_EVAL_SPEC>`"]
pub type CH13_EVAL = crate::Reg<ch13_eval::CH13_EVAL_SPEC>;
#[doc = "Scan configuration"]
pub mod ch13_eval;
#[doc = "CH14_TIMING (rw) register accessor: an alias for `Reg<CH14_TIMING_SPEC>`"]
pub type CH14_TIMING = crate::Reg<ch14_timing::CH14_TIMING_SPEC>;
#[doc = "Scan configuration"]
pub mod ch14_timing;
#[doc = "CH14_INTERACT (rw) register accessor: an alias for `Reg<CH14_INTERACT_SPEC>`"]
pub type CH14_INTERACT = crate::Reg<ch14_interact::CH14_INTERACT_SPEC>;
#[doc = "Scan configuration"]
pub mod ch14_interact;
#[doc = "CH14_EVAL (rw) register accessor: an alias for `Reg<CH14_EVAL_SPEC>`"]
pub type CH14_EVAL = crate::Reg<ch14_eval::CH14_EVAL_SPEC>;
#[doc = "Scan configuration"]
pub mod ch14_eval;
#[doc = "CH15_TIMING (rw) register accessor: an alias for `Reg<CH15_TIMING_SPEC>`"]
pub type CH15_TIMING = crate::Reg<ch15_timing::CH15_TIMING_SPEC>;
#[doc = "Scan configuration"]
pub mod ch15_timing;
#[doc = "CH15_INTERACT (rw) register accessor: an alias for `Reg<CH15_INTERACT_SPEC>`"]
pub type CH15_INTERACT = crate::Reg<ch15_interact::CH15_INTERACT_SPEC>;
#[doc = "Scan configuration"]
pub mod ch15_interact;
#[doc = "CH15_EVAL (rw) register accessor: an alias for `Reg<CH15_EVAL_SPEC>`"]
pub type CH15_EVAL = crate::Reg<ch15_eval::CH15_EVAL_SPEC>;
#[doc = "Scan configuration"]
pub mod ch15_eval;
