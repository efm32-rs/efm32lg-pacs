#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Address Timing Register"]
    pub addrtiming: crate::Reg<addrtiming::ADDRTIMING_SPEC>,
    #[doc = "0x08 - Read Timing Register"]
    pub rdtiming: crate::Reg<rdtiming::RDTIMING_SPEC>,
    #[doc = "0x0c - Write Timing Register"]
    pub wrtiming: crate::Reg<wrtiming::WRTIMING_SPEC>,
    #[doc = "0x10 - Polarity Register"]
    pub polarity: crate::Reg<polarity::POLARITY_SPEC>,
    #[doc = "0x14 - I/O Routing Register"]
    pub route: crate::Reg<route::ROUTE_SPEC>,
    #[doc = "0x18 - Address Timing Register 1"]
    pub addrtiming1: crate::Reg<addrtiming1::ADDRTIMING1_SPEC>,
    #[doc = "0x1c - Read Timing Register 1"]
    pub rdtiming1: crate::Reg<rdtiming1::RDTIMING1_SPEC>,
    #[doc = "0x20 - Write Timing Register 1"]
    pub wrtiming1: crate::Reg<wrtiming1::WRTIMING1_SPEC>,
    #[doc = "0x24 - Polarity Register 1"]
    pub polarity1: crate::Reg<polarity1::POLARITY1_SPEC>,
    #[doc = "0x28 - Address Timing Register 2"]
    pub addrtiming2: crate::Reg<addrtiming2::ADDRTIMING2_SPEC>,
    #[doc = "0x2c - Read Timing Register 2"]
    pub rdtiming2: crate::Reg<rdtiming2::RDTIMING2_SPEC>,
    #[doc = "0x30 - Write Timing Register 2"]
    pub wrtiming2: crate::Reg<wrtiming2::WRTIMING2_SPEC>,
    #[doc = "0x34 - Polarity Register 2"]
    pub polarity2: crate::Reg<polarity2::POLARITY2_SPEC>,
    #[doc = "0x38 - Address Timing Register 3"]
    pub addrtiming3: crate::Reg<addrtiming3::ADDRTIMING3_SPEC>,
    #[doc = "0x3c - Read Timing Register 3"]
    pub rdtiming3: crate::Reg<rdtiming3::RDTIMING3_SPEC>,
    #[doc = "0x40 - Write Timing Register 3"]
    pub wrtiming3: crate::Reg<wrtiming3::WRTIMING3_SPEC>,
    #[doc = "0x44 - Polarity Register 3"]
    pub polarity3: crate::Reg<polarity3::POLARITY3_SPEC>,
    #[doc = "0x48 - Page Control Register"]
    pub pagectrl: crate::Reg<pagectrl::PAGECTRL_SPEC>,
    #[doc = "0x4c - NAND Control Register"]
    pub nandctrl: crate::Reg<nandctrl::NANDCTRL_SPEC>,
    #[doc = "0x50 - Command Register"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x54 - Status Register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x58 - ECC Parity register"]
    pub eccparity: crate::Reg<eccparity::ECCPARITY_SPEC>,
    #[doc = "0x5c - TFT Control Register"]
    pub tftctrl: crate::Reg<tftctrl::TFTCTRL_SPEC>,
    #[doc = "0x60 - TFT Status Register"]
    pub tftstatus: crate::Reg<tftstatus::TFTSTATUS_SPEC>,
    #[doc = "0x64 - TFT Frame Base Register"]
    pub tftframebase: crate::Reg<tftframebase::TFTFRAMEBASE_SPEC>,
    #[doc = "0x68 - TFT Stride Register"]
    pub tftstride: crate::Reg<tftstride::TFTSTRIDE_SPEC>,
    #[doc = "0x6c - TFT Size Register"]
    pub tftsize: crate::Reg<tftsize::TFTSIZE_SPEC>,
    #[doc = "0x70 - TFT Horizontal Porch Register"]
    pub tfthporch: crate::Reg<tfthporch::TFTHPORCH_SPEC>,
    #[doc = "0x74 - TFT Vertical Porch Register"]
    pub tftvporch: crate::Reg<tftvporch::TFTVPORCH_SPEC>,
    #[doc = "0x78 - TFT Timing Register"]
    pub tfttiming: crate::Reg<tfttiming::TFTTIMING_SPEC>,
    #[doc = "0x7c - TFT Polarity Register"]
    pub tftpolarity: crate::Reg<tftpolarity::TFTPOLARITY_SPEC>,
    #[doc = "0x80 - TFT Direct Drive Data Register"]
    pub tftdd: crate::Reg<tftdd::TFTDD_SPEC>,
    #[doc = "0x84 - TFT Alpha Blending Register"]
    pub tftalpha: crate::Reg<tftalpha::TFTALPHA_SPEC>,
    #[doc = "0x88 - TFT Pixel 0 Register"]
    pub tftpixel0: crate::Reg<tftpixel0::TFTPIXEL0_SPEC>,
    #[doc = "0x8c - TFT Pixel 1 Register"]
    pub tftpixel1: crate::Reg<tftpixel1::TFTPIXEL1_SPEC>,
    #[doc = "0x90 - TFT Alpha Blending Result Pixel Register"]
    pub tftpixel: crate::Reg<tftpixel::TFTPIXEL_SPEC>,
    #[doc = "0x94 - TFT Masking Register"]
    pub tftmask: crate::Reg<tftmask::TFTMASK_SPEC>,
    #[doc = "0x98 - Interrupt Flag Register"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x9c - Interrupt Flag Set Register"]
    pub ifs: crate::Reg<ifs::IFS_SPEC>,
    #[doc = "0xa0 - Interrupt Flag Clear Register"]
    pub ifc: crate::Reg<ifc::IFC_SPEC>,
    #[doc = "0xa4 - Interrupt Enable Register"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "ADDRTIMING register accessor: an alias for `Reg<ADDRTIMING_SPEC>`"]
pub type ADDRTIMING = crate::Reg<addrtiming::ADDRTIMING_SPEC>;
#[doc = "Address Timing Register"]
pub mod addrtiming;
#[doc = "RDTIMING register accessor: an alias for `Reg<RDTIMING_SPEC>`"]
pub type RDTIMING = crate::Reg<rdtiming::RDTIMING_SPEC>;
#[doc = "Read Timing Register"]
pub mod rdtiming;
#[doc = "WRTIMING register accessor: an alias for `Reg<WRTIMING_SPEC>`"]
pub type WRTIMING = crate::Reg<wrtiming::WRTIMING_SPEC>;
#[doc = "Write Timing Register"]
pub mod wrtiming;
#[doc = "POLARITY register accessor: an alias for `Reg<POLARITY_SPEC>`"]
pub type POLARITY = crate::Reg<polarity::POLARITY_SPEC>;
#[doc = "Polarity Register"]
pub mod polarity;
#[doc = "ROUTE register accessor: an alias for `Reg<ROUTE_SPEC>`"]
pub type ROUTE = crate::Reg<route::ROUTE_SPEC>;
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "ADDRTIMING1 register accessor: an alias for `Reg<ADDRTIMING1_SPEC>`"]
pub type ADDRTIMING1 = crate::Reg<addrtiming1::ADDRTIMING1_SPEC>;
#[doc = "Address Timing Register 1"]
pub mod addrtiming1;
#[doc = "RDTIMING1 register accessor: an alias for `Reg<RDTIMING1_SPEC>`"]
pub type RDTIMING1 = crate::Reg<rdtiming1::RDTIMING1_SPEC>;
#[doc = "Read Timing Register 1"]
pub mod rdtiming1;
#[doc = "WRTIMING1 register accessor: an alias for `Reg<WRTIMING1_SPEC>`"]
pub type WRTIMING1 = crate::Reg<wrtiming1::WRTIMING1_SPEC>;
#[doc = "Write Timing Register 1"]
pub mod wrtiming1;
#[doc = "POLARITY1 register accessor: an alias for `Reg<POLARITY1_SPEC>`"]
pub type POLARITY1 = crate::Reg<polarity1::POLARITY1_SPEC>;
#[doc = "Polarity Register 1"]
pub mod polarity1;
#[doc = "ADDRTIMING2 register accessor: an alias for `Reg<ADDRTIMING2_SPEC>`"]
pub type ADDRTIMING2 = crate::Reg<addrtiming2::ADDRTIMING2_SPEC>;
#[doc = "Address Timing Register 2"]
pub mod addrtiming2;
#[doc = "RDTIMING2 register accessor: an alias for `Reg<RDTIMING2_SPEC>`"]
pub type RDTIMING2 = crate::Reg<rdtiming2::RDTIMING2_SPEC>;
#[doc = "Read Timing Register 2"]
pub mod rdtiming2;
#[doc = "WRTIMING2 register accessor: an alias for `Reg<WRTIMING2_SPEC>`"]
pub type WRTIMING2 = crate::Reg<wrtiming2::WRTIMING2_SPEC>;
#[doc = "Write Timing Register 2"]
pub mod wrtiming2;
#[doc = "POLARITY2 register accessor: an alias for `Reg<POLARITY2_SPEC>`"]
pub type POLARITY2 = crate::Reg<polarity2::POLARITY2_SPEC>;
#[doc = "Polarity Register 2"]
pub mod polarity2;
#[doc = "ADDRTIMING3 register accessor: an alias for `Reg<ADDRTIMING3_SPEC>`"]
pub type ADDRTIMING3 = crate::Reg<addrtiming3::ADDRTIMING3_SPEC>;
#[doc = "Address Timing Register 3"]
pub mod addrtiming3;
#[doc = "RDTIMING3 register accessor: an alias for `Reg<RDTIMING3_SPEC>`"]
pub type RDTIMING3 = crate::Reg<rdtiming3::RDTIMING3_SPEC>;
#[doc = "Read Timing Register 3"]
pub mod rdtiming3;
#[doc = "WRTIMING3 register accessor: an alias for `Reg<WRTIMING3_SPEC>`"]
pub type WRTIMING3 = crate::Reg<wrtiming3::WRTIMING3_SPEC>;
#[doc = "Write Timing Register 3"]
pub mod wrtiming3;
#[doc = "POLARITY3 register accessor: an alias for `Reg<POLARITY3_SPEC>`"]
pub type POLARITY3 = crate::Reg<polarity3::POLARITY3_SPEC>;
#[doc = "Polarity Register 3"]
pub mod polarity3;
#[doc = "PAGECTRL register accessor: an alias for `Reg<PAGECTRL_SPEC>`"]
pub type PAGECTRL = crate::Reg<pagectrl::PAGECTRL_SPEC>;
#[doc = "Page Control Register"]
pub mod pagectrl;
#[doc = "NANDCTRL register accessor: an alias for `Reg<NANDCTRL_SPEC>`"]
pub type NANDCTRL = crate::Reg<nandctrl::NANDCTRL_SPEC>;
#[doc = "NAND Control Register"]
pub mod nandctrl;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "ECCPARITY register accessor: an alias for `Reg<ECCPARITY_SPEC>`"]
pub type ECCPARITY = crate::Reg<eccparity::ECCPARITY_SPEC>;
#[doc = "ECC Parity register"]
pub mod eccparity;
#[doc = "TFTCTRL register accessor: an alias for `Reg<TFTCTRL_SPEC>`"]
pub type TFTCTRL = crate::Reg<tftctrl::TFTCTRL_SPEC>;
#[doc = "TFT Control Register"]
pub mod tftctrl;
#[doc = "TFTSTATUS register accessor: an alias for `Reg<TFTSTATUS_SPEC>`"]
pub type TFTSTATUS = crate::Reg<tftstatus::TFTSTATUS_SPEC>;
#[doc = "TFT Status Register"]
pub mod tftstatus;
#[doc = "TFTFRAMEBASE register accessor: an alias for `Reg<TFTFRAMEBASE_SPEC>`"]
pub type TFTFRAMEBASE = crate::Reg<tftframebase::TFTFRAMEBASE_SPEC>;
#[doc = "TFT Frame Base Register"]
pub mod tftframebase;
#[doc = "TFTSTRIDE register accessor: an alias for `Reg<TFTSTRIDE_SPEC>`"]
pub type TFTSTRIDE = crate::Reg<tftstride::TFTSTRIDE_SPEC>;
#[doc = "TFT Stride Register"]
pub mod tftstride;
#[doc = "TFTSIZE register accessor: an alias for `Reg<TFTSIZE_SPEC>`"]
pub type TFTSIZE = crate::Reg<tftsize::TFTSIZE_SPEC>;
#[doc = "TFT Size Register"]
pub mod tftsize;
#[doc = "TFTHPORCH register accessor: an alias for `Reg<TFTHPORCH_SPEC>`"]
pub type TFTHPORCH = crate::Reg<tfthporch::TFTHPORCH_SPEC>;
#[doc = "TFT Horizontal Porch Register"]
pub mod tfthporch;
#[doc = "TFTVPORCH register accessor: an alias for `Reg<TFTVPORCH_SPEC>`"]
pub type TFTVPORCH = crate::Reg<tftvporch::TFTVPORCH_SPEC>;
#[doc = "TFT Vertical Porch Register"]
pub mod tftvporch;
#[doc = "TFTTIMING register accessor: an alias for `Reg<TFTTIMING_SPEC>`"]
pub type TFTTIMING = crate::Reg<tfttiming::TFTTIMING_SPEC>;
#[doc = "TFT Timing Register"]
pub mod tfttiming;
#[doc = "TFTPOLARITY register accessor: an alias for `Reg<TFTPOLARITY_SPEC>`"]
pub type TFTPOLARITY = crate::Reg<tftpolarity::TFTPOLARITY_SPEC>;
#[doc = "TFT Polarity Register"]
pub mod tftpolarity;
#[doc = "TFTDD register accessor: an alias for `Reg<TFTDD_SPEC>`"]
pub type TFTDD = crate::Reg<tftdd::TFTDD_SPEC>;
#[doc = "TFT Direct Drive Data Register"]
pub mod tftdd;
#[doc = "TFTALPHA register accessor: an alias for `Reg<TFTALPHA_SPEC>`"]
pub type TFTALPHA = crate::Reg<tftalpha::TFTALPHA_SPEC>;
#[doc = "TFT Alpha Blending Register"]
pub mod tftalpha;
#[doc = "TFTPIXEL0 register accessor: an alias for `Reg<TFTPIXEL0_SPEC>`"]
pub type TFTPIXEL0 = crate::Reg<tftpixel0::TFTPIXEL0_SPEC>;
#[doc = "TFT Pixel 0 Register"]
pub mod tftpixel0;
#[doc = "TFTPIXEL1 register accessor: an alias for `Reg<TFTPIXEL1_SPEC>`"]
pub type TFTPIXEL1 = crate::Reg<tftpixel1::TFTPIXEL1_SPEC>;
#[doc = "TFT Pixel 1 Register"]
pub mod tftpixel1;
#[doc = "TFTPIXEL register accessor: an alias for `Reg<TFTPIXEL_SPEC>`"]
pub type TFTPIXEL = crate::Reg<tftpixel::TFTPIXEL_SPEC>;
#[doc = "TFT Alpha Blending Result Pixel Register"]
pub mod tftpixel;
#[doc = "TFTMASK register accessor: an alias for `Reg<TFTMASK_SPEC>`"]
pub type TFTMASK = crate::Reg<tftmask::TFTMASK_SPEC>;
#[doc = "TFT Masking Register"]
pub mod tftmask;
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
