#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port Control Register"]
    pub pa_ctrl: crate::Reg<pa_ctrl::PA_CTRL_SPEC>,
    #[doc = "0x04 - Port Pin Mode Low Register"]
    pub pa_model: crate::Reg<pa_model::PA_MODEL_SPEC>,
    #[doc = "0x08 - Port Pin Mode High Register"]
    pub pa_modeh: crate::Reg<pa_modeh::PA_MODEH_SPEC>,
    #[doc = "0x0c - Port Data Out Register"]
    pub pa_dout: crate::Reg<pa_dout::PA_DOUT_SPEC>,
    #[doc = "0x10 - Port Data Out Set Register"]
    pub pa_doutset: crate::Reg<pa_doutset::PA_DOUTSET_SPEC>,
    #[doc = "0x14 - Port Data Out Clear Register"]
    pub pa_doutclr: crate::Reg<pa_doutclr::PA_DOUTCLR_SPEC>,
    #[doc = "0x18 - Port Data Out Toggle Register"]
    pub pa_douttgl: crate::Reg<pa_douttgl::PA_DOUTTGL_SPEC>,
    #[doc = "0x1c - Port Data In Register"]
    pub pa_din: crate::Reg<pa_din::PA_DIN_SPEC>,
    #[doc = "0x20 - Port Unlocked Pins Register"]
    pub pa_pinlockn: crate::Reg<pa_pinlockn::PA_PINLOCKN_SPEC>,
    #[doc = "0x24 - Port Control Register"]
    pub pb_ctrl: crate::Reg<pb_ctrl::PB_CTRL_SPEC>,
    #[doc = "0x28 - Port Pin Mode Low Register"]
    pub pb_model: crate::Reg<pb_model::PB_MODEL_SPEC>,
    #[doc = "0x2c - Port Pin Mode High Register"]
    pub pb_modeh: crate::Reg<pb_modeh::PB_MODEH_SPEC>,
    #[doc = "0x30 - Port Data Out Register"]
    pub pb_dout: crate::Reg<pb_dout::PB_DOUT_SPEC>,
    #[doc = "0x34 - Port Data Out Set Register"]
    pub pb_doutset: crate::Reg<pb_doutset::PB_DOUTSET_SPEC>,
    #[doc = "0x38 - Port Data Out Clear Register"]
    pub pb_doutclr: crate::Reg<pb_doutclr::PB_DOUTCLR_SPEC>,
    #[doc = "0x3c - Port Data Out Toggle Register"]
    pub pb_douttgl: crate::Reg<pb_douttgl::PB_DOUTTGL_SPEC>,
    #[doc = "0x40 - Port Data In Register"]
    pub pb_din: crate::Reg<pb_din::PB_DIN_SPEC>,
    #[doc = "0x44 - Port Unlocked Pins Register"]
    pub pb_pinlockn: crate::Reg<pb_pinlockn::PB_PINLOCKN_SPEC>,
    #[doc = "0x48 - Port Control Register"]
    pub pc_ctrl: crate::Reg<pc_ctrl::PC_CTRL_SPEC>,
    #[doc = "0x4c - Port Pin Mode Low Register"]
    pub pc_model: crate::Reg<pc_model::PC_MODEL_SPEC>,
    #[doc = "0x50 - Port Pin Mode High Register"]
    pub pc_modeh: crate::Reg<pc_modeh::PC_MODEH_SPEC>,
    #[doc = "0x54 - Port Data Out Register"]
    pub pc_dout: crate::Reg<pc_dout::PC_DOUT_SPEC>,
    #[doc = "0x58 - Port Data Out Set Register"]
    pub pc_doutset: crate::Reg<pc_doutset::PC_DOUTSET_SPEC>,
    #[doc = "0x5c - Port Data Out Clear Register"]
    pub pc_doutclr: crate::Reg<pc_doutclr::PC_DOUTCLR_SPEC>,
    #[doc = "0x60 - Port Data Out Toggle Register"]
    pub pc_douttgl: crate::Reg<pc_douttgl::PC_DOUTTGL_SPEC>,
    #[doc = "0x64 - Port Data In Register"]
    pub pc_din: crate::Reg<pc_din::PC_DIN_SPEC>,
    #[doc = "0x68 - Port Unlocked Pins Register"]
    pub pc_pinlockn: crate::Reg<pc_pinlockn::PC_PINLOCKN_SPEC>,
    #[doc = "0x6c - Port Control Register"]
    pub pd_ctrl: crate::Reg<pd_ctrl::PD_CTRL_SPEC>,
    #[doc = "0x70 - Port Pin Mode Low Register"]
    pub pd_model: crate::Reg<pd_model::PD_MODEL_SPEC>,
    #[doc = "0x74 - Port Pin Mode High Register"]
    pub pd_modeh: crate::Reg<pd_modeh::PD_MODEH_SPEC>,
    #[doc = "0x78 - Port Data Out Register"]
    pub pd_dout: crate::Reg<pd_dout::PD_DOUT_SPEC>,
    #[doc = "0x7c - Port Data Out Set Register"]
    pub pd_doutset: crate::Reg<pd_doutset::PD_DOUTSET_SPEC>,
    #[doc = "0x80 - Port Data Out Clear Register"]
    pub pd_doutclr: crate::Reg<pd_doutclr::PD_DOUTCLR_SPEC>,
    #[doc = "0x84 - Port Data Out Toggle Register"]
    pub pd_douttgl: crate::Reg<pd_douttgl::PD_DOUTTGL_SPEC>,
    #[doc = "0x88 - Port Data In Register"]
    pub pd_din: crate::Reg<pd_din::PD_DIN_SPEC>,
    #[doc = "0x8c - Port Unlocked Pins Register"]
    pub pd_pinlockn: crate::Reg<pd_pinlockn::PD_PINLOCKN_SPEC>,
    #[doc = "0x90 - Port Control Register"]
    pub pe_ctrl: crate::Reg<pe_ctrl::PE_CTRL_SPEC>,
    #[doc = "0x94 - Port Pin Mode Low Register"]
    pub pe_model: crate::Reg<pe_model::PE_MODEL_SPEC>,
    #[doc = "0x98 - Port Pin Mode High Register"]
    pub pe_modeh: crate::Reg<pe_modeh::PE_MODEH_SPEC>,
    #[doc = "0x9c - Port Data Out Register"]
    pub pe_dout: crate::Reg<pe_dout::PE_DOUT_SPEC>,
    #[doc = "0xa0 - Port Data Out Set Register"]
    pub pe_doutset: crate::Reg<pe_doutset::PE_DOUTSET_SPEC>,
    #[doc = "0xa4 - Port Data Out Clear Register"]
    pub pe_doutclr: crate::Reg<pe_doutclr::PE_DOUTCLR_SPEC>,
    #[doc = "0xa8 - Port Data Out Toggle Register"]
    pub pe_douttgl: crate::Reg<pe_douttgl::PE_DOUTTGL_SPEC>,
    #[doc = "0xac - Port Data In Register"]
    pub pe_din: crate::Reg<pe_din::PE_DIN_SPEC>,
    #[doc = "0xb0 - Port Unlocked Pins Register"]
    pub pe_pinlockn: crate::Reg<pe_pinlockn::PE_PINLOCKN_SPEC>,
    #[doc = "0xb4 - Port Control Register"]
    pub pf_ctrl: crate::Reg<pf_ctrl::PF_CTRL_SPEC>,
    #[doc = "0xb8 - Port Pin Mode Low Register"]
    pub pf_model: crate::Reg<pf_model::PF_MODEL_SPEC>,
    #[doc = "0xbc - Port Pin Mode High Register"]
    pub pf_modeh: crate::Reg<pf_modeh::PF_MODEH_SPEC>,
    #[doc = "0xc0 - Port Data Out Register"]
    pub pf_dout: crate::Reg<pf_dout::PF_DOUT_SPEC>,
    #[doc = "0xc4 - Port Data Out Set Register"]
    pub pf_doutset: crate::Reg<pf_doutset::PF_DOUTSET_SPEC>,
    #[doc = "0xc8 - Port Data Out Clear Register"]
    pub pf_doutclr: crate::Reg<pf_doutclr::PF_DOUTCLR_SPEC>,
    #[doc = "0xcc - Port Data Out Toggle Register"]
    pub pf_douttgl: crate::Reg<pf_douttgl::PF_DOUTTGL_SPEC>,
    #[doc = "0xd0 - Port Data In Register"]
    pub pf_din: crate::Reg<pf_din::PF_DIN_SPEC>,
    #[doc = "0xd4 - Port Unlocked Pins Register"]
    pub pf_pinlockn: crate::Reg<pf_pinlockn::PF_PINLOCKN_SPEC>,
    _reserved54: [u8; 0x28],
    #[doc = "0x100 - External Interrupt Port Select Low Register"]
    pub extipsell: crate::Reg<extipsell::EXTIPSELL_SPEC>,
    #[doc = "0x104 - External Interrupt Port Select High Register"]
    pub extipselh: crate::Reg<extipselh::EXTIPSELH_SPEC>,
    #[doc = "0x108 - External Interrupt Rising Edge Trigger Register"]
    pub extirise: crate::Reg<extirise::EXTIRISE_SPEC>,
    #[doc = "0x10c - External Interrupt Falling Edge Trigger Register"]
    pub extifall: crate::Reg<extifall::EXTIFALL_SPEC>,
    #[doc = "0x110 - Interrupt Enable Register"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    #[doc = "0x114 - Interrupt Flag Register"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x118 - Interrupt Flag Set Register"]
    pub ifs: crate::Reg<ifs::IFS_SPEC>,
    #[doc = "0x11c - Interrupt Flag Clear Register"]
    pub ifc: crate::Reg<ifc::IFC_SPEC>,
    #[doc = "0x120 - I/O Routing Register"]
    pub route: crate::Reg<route::ROUTE_SPEC>,
    #[doc = "0x124 - Input Sense Register"]
    pub insense: crate::Reg<insense::INSENSE_SPEC>,
    #[doc = "0x128 - Configuration Lock Register"]
    pub lock: crate::Reg<lock::LOCK_SPEC>,
    #[doc = "0x12c - GPIO Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x130 - GPIO Command Register"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x134 - EM4 Wake-up Enable Register"]
    pub em4wuen: crate::Reg<em4wuen::EM4WUEN_SPEC>,
    #[doc = "0x138 - EM4 Wake-up Polarity Register"]
    pub em4wupol: crate::Reg<em4wupol::EM4WUPOL_SPEC>,
    #[doc = "0x13c - EM4 Wake-up Cause Register"]
    pub em4wucause: crate::Reg<em4wucause::EM4WUCAUSE_SPEC>,
}
#[doc = "PA_CTRL register accessor: an alias for `Reg<PA_CTRL_SPEC>`"]
pub type PA_CTRL = crate::Reg<pa_ctrl::PA_CTRL_SPEC>;
#[doc = "Port Control Register"]
pub mod pa_ctrl;
#[doc = "PA_MODEL register accessor: an alias for `Reg<PA_MODEL_SPEC>`"]
pub type PA_MODEL = crate::Reg<pa_model::PA_MODEL_SPEC>;
#[doc = "Port Pin Mode Low Register"]
pub mod pa_model;
#[doc = "PA_MODEH register accessor: an alias for `Reg<PA_MODEH_SPEC>`"]
pub type PA_MODEH = crate::Reg<pa_modeh::PA_MODEH_SPEC>;
#[doc = "Port Pin Mode High Register"]
pub mod pa_modeh;
#[doc = "PA_DOUT register accessor: an alias for `Reg<PA_DOUT_SPEC>`"]
pub type PA_DOUT = crate::Reg<pa_dout::PA_DOUT_SPEC>;
#[doc = "Port Data Out Register"]
pub mod pa_dout;
#[doc = "PA_DOUTSET register accessor: an alias for `Reg<PA_DOUTSET_SPEC>`"]
pub type PA_DOUTSET = crate::Reg<pa_doutset::PA_DOUTSET_SPEC>;
#[doc = "Port Data Out Set Register"]
pub mod pa_doutset;
#[doc = "PA_DOUTCLR register accessor: an alias for `Reg<PA_DOUTCLR_SPEC>`"]
pub type PA_DOUTCLR = crate::Reg<pa_doutclr::PA_DOUTCLR_SPEC>;
#[doc = "Port Data Out Clear Register"]
pub mod pa_doutclr;
#[doc = "PA_DOUTTGL register accessor: an alias for `Reg<PA_DOUTTGL_SPEC>`"]
pub type PA_DOUTTGL = crate::Reg<pa_douttgl::PA_DOUTTGL_SPEC>;
#[doc = "Port Data Out Toggle Register"]
pub mod pa_douttgl;
#[doc = "PA_DIN register accessor: an alias for `Reg<PA_DIN_SPEC>`"]
pub type PA_DIN = crate::Reg<pa_din::PA_DIN_SPEC>;
#[doc = "Port Data In Register"]
pub mod pa_din;
#[doc = "PA_PINLOCKN register accessor: an alias for `Reg<PA_PINLOCKN_SPEC>`"]
pub type PA_PINLOCKN = crate::Reg<pa_pinlockn::PA_PINLOCKN_SPEC>;
#[doc = "Port Unlocked Pins Register"]
pub mod pa_pinlockn;
#[doc = "PB_CTRL register accessor: an alias for `Reg<PB_CTRL_SPEC>`"]
pub type PB_CTRL = crate::Reg<pb_ctrl::PB_CTRL_SPEC>;
#[doc = "Port Control Register"]
pub mod pb_ctrl;
#[doc = "PB_MODEL register accessor: an alias for `Reg<PB_MODEL_SPEC>`"]
pub type PB_MODEL = crate::Reg<pb_model::PB_MODEL_SPEC>;
#[doc = "Port Pin Mode Low Register"]
pub mod pb_model;
#[doc = "PB_MODEH register accessor: an alias for `Reg<PB_MODEH_SPEC>`"]
pub type PB_MODEH = crate::Reg<pb_modeh::PB_MODEH_SPEC>;
#[doc = "Port Pin Mode High Register"]
pub mod pb_modeh;
#[doc = "PB_DOUT register accessor: an alias for `Reg<PB_DOUT_SPEC>`"]
pub type PB_DOUT = crate::Reg<pb_dout::PB_DOUT_SPEC>;
#[doc = "Port Data Out Register"]
pub mod pb_dout;
#[doc = "PB_DOUTSET register accessor: an alias for `Reg<PB_DOUTSET_SPEC>`"]
pub type PB_DOUTSET = crate::Reg<pb_doutset::PB_DOUTSET_SPEC>;
#[doc = "Port Data Out Set Register"]
pub mod pb_doutset;
#[doc = "PB_DOUTCLR register accessor: an alias for `Reg<PB_DOUTCLR_SPEC>`"]
pub type PB_DOUTCLR = crate::Reg<pb_doutclr::PB_DOUTCLR_SPEC>;
#[doc = "Port Data Out Clear Register"]
pub mod pb_doutclr;
#[doc = "PB_DOUTTGL register accessor: an alias for `Reg<PB_DOUTTGL_SPEC>`"]
pub type PB_DOUTTGL = crate::Reg<pb_douttgl::PB_DOUTTGL_SPEC>;
#[doc = "Port Data Out Toggle Register"]
pub mod pb_douttgl;
#[doc = "PB_DIN register accessor: an alias for `Reg<PB_DIN_SPEC>`"]
pub type PB_DIN = crate::Reg<pb_din::PB_DIN_SPEC>;
#[doc = "Port Data In Register"]
pub mod pb_din;
#[doc = "PB_PINLOCKN register accessor: an alias for `Reg<PB_PINLOCKN_SPEC>`"]
pub type PB_PINLOCKN = crate::Reg<pb_pinlockn::PB_PINLOCKN_SPEC>;
#[doc = "Port Unlocked Pins Register"]
pub mod pb_pinlockn;
#[doc = "PC_CTRL register accessor: an alias for `Reg<PC_CTRL_SPEC>`"]
pub type PC_CTRL = crate::Reg<pc_ctrl::PC_CTRL_SPEC>;
#[doc = "Port Control Register"]
pub mod pc_ctrl;
#[doc = "PC_MODEL register accessor: an alias for `Reg<PC_MODEL_SPEC>`"]
pub type PC_MODEL = crate::Reg<pc_model::PC_MODEL_SPEC>;
#[doc = "Port Pin Mode Low Register"]
pub mod pc_model;
#[doc = "PC_MODEH register accessor: an alias for `Reg<PC_MODEH_SPEC>`"]
pub type PC_MODEH = crate::Reg<pc_modeh::PC_MODEH_SPEC>;
#[doc = "Port Pin Mode High Register"]
pub mod pc_modeh;
#[doc = "PC_DOUT register accessor: an alias for `Reg<PC_DOUT_SPEC>`"]
pub type PC_DOUT = crate::Reg<pc_dout::PC_DOUT_SPEC>;
#[doc = "Port Data Out Register"]
pub mod pc_dout;
#[doc = "PC_DOUTSET register accessor: an alias for `Reg<PC_DOUTSET_SPEC>`"]
pub type PC_DOUTSET = crate::Reg<pc_doutset::PC_DOUTSET_SPEC>;
#[doc = "Port Data Out Set Register"]
pub mod pc_doutset;
#[doc = "PC_DOUTCLR register accessor: an alias for `Reg<PC_DOUTCLR_SPEC>`"]
pub type PC_DOUTCLR = crate::Reg<pc_doutclr::PC_DOUTCLR_SPEC>;
#[doc = "Port Data Out Clear Register"]
pub mod pc_doutclr;
#[doc = "PC_DOUTTGL register accessor: an alias for `Reg<PC_DOUTTGL_SPEC>`"]
pub type PC_DOUTTGL = crate::Reg<pc_douttgl::PC_DOUTTGL_SPEC>;
#[doc = "Port Data Out Toggle Register"]
pub mod pc_douttgl;
#[doc = "PC_DIN register accessor: an alias for `Reg<PC_DIN_SPEC>`"]
pub type PC_DIN = crate::Reg<pc_din::PC_DIN_SPEC>;
#[doc = "Port Data In Register"]
pub mod pc_din;
#[doc = "PC_PINLOCKN register accessor: an alias for `Reg<PC_PINLOCKN_SPEC>`"]
pub type PC_PINLOCKN = crate::Reg<pc_pinlockn::PC_PINLOCKN_SPEC>;
#[doc = "Port Unlocked Pins Register"]
pub mod pc_pinlockn;
#[doc = "PD_CTRL register accessor: an alias for `Reg<PD_CTRL_SPEC>`"]
pub type PD_CTRL = crate::Reg<pd_ctrl::PD_CTRL_SPEC>;
#[doc = "Port Control Register"]
pub mod pd_ctrl;
#[doc = "PD_MODEL register accessor: an alias for `Reg<PD_MODEL_SPEC>`"]
pub type PD_MODEL = crate::Reg<pd_model::PD_MODEL_SPEC>;
#[doc = "Port Pin Mode Low Register"]
pub mod pd_model;
#[doc = "PD_MODEH register accessor: an alias for `Reg<PD_MODEH_SPEC>`"]
pub type PD_MODEH = crate::Reg<pd_modeh::PD_MODEH_SPEC>;
#[doc = "Port Pin Mode High Register"]
pub mod pd_modeh;
#[doc = "PD_DOUT register accessor: an alias for `Reg<PD_DOUT_SPEC>`"]
pub type PD_DOUT = crate::Reg<pd_dout::PD_DOUT_SPEC>;
#[doc = "Port Data Out Register"]
pub mod pd_dout;
#[doc = "PD_DOUTSET register accessor: an alias for `Reg<PD_DOUTSET_SPEC>`"]
pub type PD_DOUTSET = crate::Reg<pd_doutset::PD_DOUTSET_SPEC>;
#[doc = "Port Data Out Set Register"]
pub mod pd_doutset;
#[doc = "PD_DOUTCLR register accessor: an alias for `Reg<PD_DOUTCLR_SPEC>`"]
pub type PD_DOUTCLR = crate::Reg<pd_doutclr::PD_DOUTCLR_SPEC>;
#[doc = "Port Data Out Clear Register"]
pub mod pd_doutclr;
#[doc = "PD_DOUTTGL register accessor: an alias for `Reg<PD_DOUTTGL_SPEC>`"]
pub type PD_DOUTTGL = crate::Reg<pd_douttgl::PD_DOUTTGL_SPEC>;
#[doc = "Port Data Out Toggle Register"]
pub mod pd_douttgl;
#[doc = "PD_DIN register accessor: an alias for `Reg<PD_DIN_SPEC>`"]
pub type PD_DIN = crate::Reg<pd_din::PD_DIN_SPEC>;
#[doc = "Port Data In Register"]
pub mod pd_din;
#[doc = "PD_PINLOCKN register accessor: an alias for `Reg<PD_PINLOCKN_SPEC>`"]
pub type PD_PINLOCKN = crate::Reg<pd_pinlockn::PD_PINLOCKN_SPEC>;
#[doc = "Port Unlocked Pins Register"]
pub mod pd_pinlockn;
#[doc = "PE_CTRL register accessor: an alias for `Reg<PE_CTRL_SPEC>`"]
pub type PE_CTRL = crate::Reg<pe_ctrl::PE_CTRL_SPEC>;
#[doc = "Port Control Register"]
pub mod pe_ctrl;
#[doc = "PE_MODEL register accessor: an alias for `Reg<PE_MODEL_SPEC>`"]
pub type PE_MODEL = crate::Reg<pe_model::PE_MODEL_SPEC>;
#[doc = "Port Pin Mode Low Register"]
pub mod pe_model;
#[doc = "PE_MODEH register accessor: an alias for `Reg<PE_MODEH_SPEC>`"]
pub type PE_MODEH = crate::Reg<pe_modeh::PE_MODEH_SPEC>;
#[doc = "Port Pin Mode High Register"]
pub mod pe_modeh;
#[doc = "PE_DOUT register accessor: an alias for `Reg<PE_DOUT_SPEC>`"]
pub type PE_DOUT = crate::Reg<pe_dout::PE_DOUT_SPEC>;
#[doc = "Port Data Out Register"]
pub mod pe_dout;
#[doc = "PE_DOUTSET register accessor: an alias for `Reg<PE_DOUTSET_SPEC>`"]
pub type PE_DOUTSET = crate::Reg<pe_doutset::PE_DOUTSET_SPEC>;
#[doc = "Port Data Out Set Register"]
pub mod pe_doutset;
#[doc = "PE_DOUTCLR register accessor: an alias for `Reg<PE_DOUTCLR_SPEC>`"]
pub type PE_DOUTCLR = crate::Reg<pe_doutclr::PE_DOUTCLR_SPEC>;
#[doc = "Port Data Out Clear Register"]
pub mod pe_doutclr;
#[doc = "PE_DOUTTGL register accessor: an alias for `Reg<PE_DOUTTGL_SPEC>`"]
pub type PE_DOUTTGL = crate::Reg<pe_douttgl::PE_DOUTTGL_SPEC>;
#[doc = "Port Data Out Toggle Register"]
pub mod pe_douttgl;
#[doc = "PE_DIN register accessor: an alias for `Reg<PE_DIN_SPEC>`"]
pub type PE_DIN = crate::Reg<pe_din::PE_DIN_SPEC>;
#[doc = "Port Data In Register"]
pub mod pe_din;
#[doc = "PE_PINLOCKN register accessor: an alias for `Reg<PE_PINLOCKN_SPEC>`"]
pub type PE_PINLOCKN = crate::Reg<pe_pinlockn::PE_PINLOCKN_SPEC>;
#[doc = "Port Unlocked Pins Register"]
pub mod pe_pinlockn;
#[doc = "PF_CTRL register accessor: an alias for `Reg<PF_CTRL_SPEC>`"]
pub type PF_CTRL = crate::Reg<pf_ctrl::PF_CTRL_SPEC>;
#[doc = "Port Control Register"]
pub mod pf_ctrl;
#[doc = "PF_MODEL register accessor: an alias for `Reg<PF_MODEL_SPEC>`"]
pub type PF_MODEL = crate::Reg<pf_model::PF_MODEL_SPEC>;
#[doc = "Port Pin Mode Low Register"]
pub mod pf_model;
#[doc = "PF_MODEH register accessor: an alias for `Reg<PF_MODEH_SPEC>`"]
pub type PF_MODEH = crate::Reg<pf_modeh::PF_MODEH_SPEC>;
#[doc = "Port Pin Mode High Register"]
pub mod pf_modeh;
#[doc = "PF_DOUT register accessor: an alias for `Reg<PF_DOUT_SPEC>`"]
pub type PF_DOUT = crate::Reg<pf_dout::PF_DOUT_SPEC>;
#[doc = "Port Data Out Register"]
pub mod pf_dout;
#[doc = "PF_DOUTSET register accessor: an alias for `Reg<PF_DOUTSET_SPEC>`"]
pub type PF_DOUTSET = crate::Reg<pf_doutset::PF_DOUTSET_SPEC>;
#[doc = "Port Data Out Set Register"]
pub mod pf_doutset;
#[doc = "PF_DOUTCLR register accessor: an alias for `Reg<PF_DOUTCLR_SPEC>`"]
pub type PF_DOUTCLR = crate::Reg<pf_doutclr::PF_DOUTCLR_SPEC>;
#[doc = "Port Data Out Clear Register"]
pub mod pf_doutclr;
#[doc = "PF_DOUTTGL register accessor: an alias for `Reg<PF_DOUTTGL_SPEC>`"]
pub type PF_DOUTTGL = crate::Reg<pf_douttgl::PF_DOUTTGL_SPEC>;
#[doc = "Port Data Out Toggle Register"]
pub mod pf_douttgl;
#[doc = "PF_DIN register accessor: an alias for `Reg<PF_DIN_SPEC>`"]
pub type PF_DIN = crate::Reg<pf_din::PF_DIN_SPEC>;
#[doc = "Port Data In Register"]
pub mod pf_din;
#[doc = "PF_PINLOCKN register accessor: an alias for `Reg<PF_PINLOCKN_SPEC>`"]
pub type PF_PINLOCKN = crate::Reg<pf_pinlockn::PF_PINLOCKN_SPEC>;
#[doc = "Port Unlocked Pins Register"]
pub mod pf_pinlockn;
#[doc = "EXTIPSELL register accessor: an alias for `Reg<EXTIPSELL_SPEC>`"]
pub type EXTIPSELL = crate::Reg<extipsell::EXTIPSELL_SPEC>;
#[doc = "External Interrupt Port Select Low Register"]
pub mod extipsell;
#[doc = "EXTIPSELH register accessor: an alias for `Reg<EXTIPSELH_SPEC>`"]
pub type EXTIPSELH = crate::Reg<extipselh::EXTIPSELH_SPEC>;
#[doc = "External Interrupt Port Select High Register"]
pub mod extipselh;
#[doc = "EXTIRISE register accessor: an alias for `Reg<EXTIRISE_SPEC>`"]
pub type EXTIRISE = crate::Reg<extirise::EXTIRISE_SPEC>;
#[doc = "External Interrupt Rising Edge Trigger Register"]
pub mod extirise;
#[doc = "EXTIFALL register accessor: an alias for `Reg<EXTIFALL_SPEC>`"]
pub type EXTIFALL = crate::Reg<extifall::EXTIFALL_SPEC>;
#[doc = "External Interrupt Falling Edge Trigger Register"]
pub mod extifall;
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
#[doc = "ROUTE register accessor: an alias for `Reg<ROUTE_SPEC>`"]
pub type ROUTE = crate::Reg<route::ROUTE_SPEC>;
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "INSENSE register accessor: an alias for `Reg<INSENSE_SPEC>`"]
pub type INSENSE = crate::Reg<insense::INSENSE_SPEC>;
#[doc = "Input Sense Register"]
pub mod insense;
#[doc = "LOCK register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Configuration Lock Register"]
pub mod lock;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "GPIO Control Register"]
pub mod ctrl;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "GPIO Command Register"]
pub mod cmd;
#[doc = "EM4WUEN register accessor: an alias for `Reg<EM4WUEN_SPEC>`"]
pub type EM4WUEN = crate::Reg<em4wuen::EM4WUEN_SPEC>;
#[doc = "EM4 Wake-up Enable Register"]
pub mod em4wuen;
#[doc = "EM4WUPOL register accessor: an alias for `Reg<EM4WUPOL_SPEC>`"]
pub type EM4WUPOL = crate::Reg<em4wupol::EM4WUPOL_SPEC>;
#[doc = "EM4 Wake-up Polarity Register"]
pub mod em4wupol;
#[doc = "EM4WUCAUSE register accessor: an alias for `Reg<EM4WUCAUSE_SPEC>`"]
pub type EM4WUCAUSE = crate::Reg<em4wucause::EM4WUCAUSE_SPEC>;
#[doc = "EM4 Wake-up Cause Register"]
pub mod em4wucause;
