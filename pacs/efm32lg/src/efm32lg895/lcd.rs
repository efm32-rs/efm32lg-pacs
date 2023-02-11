#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Display Control Register"]
    pub dispctrl: DISPCTRL,
    #[doc = "0x08 - Segment Enable Register"]
    pub segen: SEGEN,
    #[doc = "0x0c - Blink and Animation Control Register"]
    pub bactrl: BACTRL,
    #[doc = "0x10 - Status Register"]
    pub status: STATUS,
    #[doc = "0x14 - Animation Register A"]
    pub arega: AREGA,
    #[doc = "0x18 - Animation Register B"]
    pub aregb: AREGB,
    #[doc = "0x1c - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x20 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x24 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x28 - Interrupt Enable Register"]
    pub ien: IEN,
    _reserved11: [u8; 0x14],
    #[doc = "0x40 - Segment Data Low Register 0"]
    pub segd0l: SEGD0L,
    #[doc = "0x44 - Segment Data Low Register 1"]
    pub segd1l: SEGD1L,
    #[doc = "0x48 - Segment Data Low Register 2"]
    pub segd2l: SEGD2L,
    #[doc = "0x4c - Segment Data Low Register 3"]
    pub segd3l: SEGD3L,
    #[doc = "0x50 - Segment Data High Register 0"]
    pub segd0h: SEGD0H,
    #[doc = "0x54 - Segment Data High Register 1"]
    pub segd1h: SEGD1H,
    #[doc = "0x58 - Segment Data High Register 2"]
    pub segd2h: SEGD2H,
    #[doc = "0x5c - Segment Data High Register 3"]
    pub segd3h: SEGD3H,
    #[doc = "0x60 - Freeze Register"]
    pub freeze: FREEZE,
    #[doc = "0x64 - Synchronization Busy Register"]
    pub syncbusy: SYNCBUSY,
    _reserved21: [u8; 0x4c],
    #[doc = "0xb4 - Segment Data High Register 4"]
    pub segd4h: SEGD4H,
    #[doc = "0xb8 - Segment Data High Register 5"]
    pub segd5h: SEGD5H,
    #[doc = "0xbc - Segment Data High Register 6"]
    pub segd6h: SEGD6H,
    #[doc = "0xc0 - Segment Data High Register 7"]
    pub segd7h: SEGD7H,
    _reserved25: [u8; 0x08],
    #[doc = "0xcc - Segment Data Low Register 4"]
    pub segd4l: SEGD4L,
    #[doc = "0xd0 - Segment Data Low Register 5"]
    pub segd5l: SEGD5L,
    #[doc = "0xd4 - Segment Data Low Register 6"]
    pub segd6l: SEGD6L,
    #[doc = "0xd8 - Segment Data Low Register 7"]
    pub segd7l: SEGD7L,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "DISPCTRL (rw) register accessor: an alias for `Reg<DISPCTRL_SPEC>`"]
pub type DISPCTRL = crate::Reg<dispctrl::DISPCTRL_SPEC>;
#[doc = "Display Control Register"]
pub mod dispctrl;
#[doc = "SEGEN (rw) register accessor: an alias for `Reg<SEGEN_SPEC>`"]
pub type SEGEN = crate::Reg<segen::SEGEN_SPEC>;
#[doc = "Segment Enable Register"]
pub mod segen;
#[doc = "BACTRL (rw) register accessor: an alias for `Reg<BACTRL_SPEC>`"]
pub type BACTRL = crate::Reg<bactrl::BACTRL_SPEC>;
#[doc = "Blink and Animation Control Register"]
pub mod bactrl;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "AREGA (rw) register accessor: an alias for `Reg<AREGA_SPEC>`"]
pub type AREGA = crate::Reg<arega::AREGA_SPEC>;
#[doc = "Animation Register A"]
pub mod arega;
#[doc = "AREGB (rw) register accessor: an alias for `Reg<AREGB_SPEC>`"]
pub type AREGB = crate::Reg<aregb::AREGB_SPEC>;
#[doc = "Animation Register B"]
pub mod aregb;
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
#[doc = "SEGD0L (rw) register accessor: an alias for `Reg<SEGD0L_SPEC>`"]
pub type SEGD0L = crate::Reg<segd0l::SEGD0L_SPEC>;
#[doc = "Segment Data Low Register 0"]
pub mod segd0l;
#[doc = "SEGD1L (rw) register accessor: an alias for `Reg<SEGD1L_SPEC>`"]
pub type SEGD1L = crate::Reg<segd1l::SEGD1L_SPEC>;
#[doc = "Segment Data Low Register 1"]
pub mod segd1l;
#[doc = "SEGD2L (rw) register accessor: an alias for `Reg<SEGD2L_SPEC>`"]
pub type SEGD2L = crate::Reg<segd2l::SEGD2L_SPEC>;
#[doc = "Segment Data Low Register 2"]
pub mod segd2l;
#[doc = "SEGD3L (rw) register accessor: an alias for `Reg<SEGD3L_SPEC>`"]
pub type SEGD3L = crate::Reg<segd3l::SEGD3L_SPEC>;
#[doc = "Segment Data Low Register 3"]
pub mod segd3l;
#[doc = "SEGD0H (rw) register accessor: an alias for `Reg<SEGD0H_SPEC>`"]
pub type SEGD0H = crate::Reg<segd0h::SEGD0H_SPEC>;
#[doc = "Segment Data High Register 0"]
pub mod segd0h;
#[doc = "SEGD1H (rw) register accessor: an alias for `Reg<SEGD1H_SPEC>`"]
pub type SEGD1H = crate::Reg<segd1h::SEGD1H_SPEC>;
#[doc = "Segment Data High Register 1"]
pub mod segd1h;
#[doc = "SEGD2H (rw) register accessor: an alias for `Reg<SEGD2H_SPEC>`"]
pub type SEGD2H = crate::Reg<segd2h::SEGD2H_SPEC>;
#[doc = "Segment Data High Register 2"]
pub mod segd2h;
#[doc = "SEGD3H (rw) register accessor: an alias for `Reg<SEGD3H_SPEC>`"]
pub type SEGD3H = crate::Reg<segd3h::SEGD3H_SPEC>;
#[doc = "Segment Data High Register 3"]
pub mod segd3h;
#[doc = "FREEZE (rw) register accessor: an alias for `Reg<FREEZE_SPEC>`"]
pub type FREEZE = crate::Reg<freeze::FREEZE_SPEC>;
#[doc = "Freeze Register"]
pub mod freeze;
#[doc = "SYNCBUSY (r) register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "SEGD4H (rw) register accessor: an alias for `Reg<SEGD4H_SPEC>`"]
pub type SEGD4H = crate::Reg<segd4h::SEGD4H_SPEC>;
#[doc = "Segment Data High Register 4"]
pub mod segd4h;
#[doc = "SEGD5H (rw) register accessor: an alias for `Reg<SEGD5H_SPEC>`"]
pub type SEGD5H = crate::Reg<segd5h::SEGD5H_SPEC>;
#[doc = "Segment Data High Register 5"]
pub mod segd5h;
#[doc = "SEGD6H (rw) register accessor: an alias for `Reg<SEGD6H_SPEC>`"]
pub type SEGD6H = crate::Reg<segd6h::SEGD6H_SPEC>;
#[doc = "Segment Data High Register 6"]
pub mod segd6h;
#[doc = "SEGD7H (rw) register accessor: an alias for `Reg<SEGD7H_SPEC>`"]
pub type SEGD7H = crate::Reg<segd7h::SEGD7H_SPEC>;
#[doc = "Segment Data High Register 7"]
pub mod segd7h;
#[doc = "SEGD4L (rw) register accessor: an alias for `Reg<SEGD4L_SPEC>`"]
pub type SEGD4L = crate::Reg<segd4l::SEGD4L_SPEC>;
#[doc = "Segment Data Low Register 4"]
pub mod segd4l;
#[doc = "SEGD5L (rw) register accessor: an alias for `Reg<SEGD5L_SPEC>`"]
pub type SEGD5L = crate::Reg<segd5l::SEGD5L_SPEC>;
#[doc = "Segment Data Low Register 5"]
pub mod segd5l;
#[doc = "SEGD6L (rw) register accessor: an alias for `Reg<SEGD6L_SPEC>`"]
pub type SEGD6L = crate::Reg<segd6l::SEGD6L_SPEC>;
#[doc = "Segment Data Low Register 6"]
pub mod segd6l;
#[doc = "SEGD7L (rw) register accessor: an alias for `Reg<SEGD7L_SPEC>`"]
pub type SEGD7L = crate::Reg<segd7l::SEGD7L_SPEC>;
#[doc = "Segment Data Low Register 7"]
pub mod segd7l;
