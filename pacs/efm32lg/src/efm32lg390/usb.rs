#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - System Status Register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x08 - Interrupt Flag Register"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x0c - Interrupt Flag Set Register"]
    pub ifs: crate::Reg<ifs::IFS_SPEC>,
    #[doc = "0x10 - Interrupt Flag Clear Register"]
    pub ifc: crate::Reg<ifc::IFC_SPEC>,
    #[doc = "0x14 - Interrupt Enable Register"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    #[doc = "0x18 - I/O Routing Register"]
    pub route: crate::Reg<route::ROUTE_SPEC>,
    _reserved7: [u8; 0x0003_bfe4],
    #[doc = "0x3c000 - OTG Control and Status Register"]
    pub gotgctl: crate::Reg<gotgctl::GOTGCTL_SPEC>,
    #[doc = "0x3c004 - OTG Interrupt Register"]
    pub gotgint: crate::Reg<gotgint::GOTGINT_SPEC>,
    #[doc = "0x3c008 - AHB Configuration Register"]
    pub gahbcfg: crate::Reg<gahbcfg::GAHBCFG_SPEC>,
    #[doc = "0x3c00c - USB Configuration Register"]
    pub gusbcfg: crate::Reg<gusbcfg::GUSBCFG_SPEC>,
    #[doc = "0x3c010 - Reset Register"]
    pub grstctl: crate::Reg<grstctl::GRSTCTL_SPEC>,
    #[doc = "0x3c014 - Interrupt Register"]
    pub gintsts: crate::Reg<gintsts::GINTSTS_SPEC>,
    #[doc = "0x3c018 - Interrupt Mask Register"]
    pub gintmsk: crate::Reg<gintmsk::GINTMSK_SPEC>,
    #[doc = "0x3c01c - Receive Status Debug Read Register"]
    pub grxstsr: crate::Reg<grxstsr::GRXSTSR_SPEC>,
    #[doc = "0x3c020 - Receive Status Read and Pop Register"]
    pub grxstsp: crate::Reg<grxstsp::GRXSTSP_SPEC>,
    #[doc = "0x3c024 - Receive FIFO Size Register"]
    pub grxfsiz: crate::Reg<grxfsiz::GRXFSIZ_SPEC>,
    #[doc = "0x3c028 - Non-periodic Transmit FIFO Size Register"]
    pub gnptxfsiz: crate::Reg<gnptxfsiz::GNPTXFSIZ_SPEC>,
    #[doc = "0x3c02c - Non-periodic Transmit FIFO/Queue Status Register"]
    pub gnptxsts: crate::Reg<gnptxsts::GNPTXSTS_SPEC>,
    _reserved19: [u8; 0x2c],
    #[doc = "0x3c05c - Global DFIFO Configuration Register"]
    pub gdfifocfg: crate::Reg<gdfifocfg::GDFIFOCFG_SPEC>,
    _reserved20: [u8; 0xa0],
    #[doc = "0x3c100 - Host Periodic Transmit FIFO Size Register"]
    pub hptxfsiz: crate::Reg<hptxfsiz::HPTXFSIZ_SPEC>,
    #[doc = "0x3c104 - Device IN Endpoint Transmit FIFO 1 Size Register"]
    pub dieptxf1: crate::Reg<dieptxf1::DIEPTXF1_SPEC>,
    #[doc = "0x3c108 - Device IN Endpoint Transmit FIFO 2 Size Register"]
    pub dieptxf2: crate::Reg<dieptxf2::DIEPTXF2_SPEC>,
    #[doc = "0x3c10c - Device IN Endpoint Transmit FIFO 3 Size Register"]
    pub dieptxf3: crate::Reg<dieptxf3::DIEPTXF3_SPEC>,
    #[doc = "0x3c110 - Device IN Endpoint Transmit FIFO 4 Size Register"]
    pub dieptxf4: crate::Reg<dieptxf4::DIEPTXF4_SPEC>,
    #[doc = "0x3c114 - Device IN Endpoint Transmit FIFO 5 Size Register"]
    pub dieptxf5: crate::Reg<dieptxf5::DIEPTXF5_SPEC>,
    #[doc = "0x3c118 - Device IN Endpoint Transmit FIFO 6 Size Register"]
    pub dieptxf6: crate::Reg<dieptxf6::DIEPTXF6_SPEC>,
    _reserved27: [u8; 0x02e4],
    #[doc = "0x3c400 - Host Configuration Register"]
    pub hcfg: crate::Reg<hcfg::HCFG_SPEC>,
    #[doc = "0x3c404 - Host Frame Interval Register"]
    pub hfir: crate::Reg<hfir::HFIR_SPEC>,
    #[doc = "0x3c408 - Host Frame Number/Frame Time Remaining Register"]
    pub hfnum: crate::Reg<hfnum::HFNUM_SPEC>,
    _reserved30: [u8; 0x04],
    #[doc = "0x3c410 - Host Periodic Transmit FIFO/Queue Status Register"]
    pub hptxsts: crate::Reg<hptxsts::HPTXSTS_SPEC>,
    #[doc = "0x3c414 - Host All Channels Interrupt Register"]
    pub haint: crate::Reg<haint::HAINT_SPEC>,
    #[doc = "0x3c418 - Host All Channels Interrupt Mask Register"]
    pub haintmsk: crate::Reg<haintmsk::HAINTMSK_SPEC>,
    _reserved33: [u8; 0x24],
    #[doc = "0x3c440 - Host Port Control and Status Register"]
    pub hprt: crate::Reg<hprt::HPRT_SPEC>,
    _reserved34: [u8; 0xbc],
    #[doc = "0x3c500 - Host Channel x Characteristics Register"]
    pub hc0_char: crate::Reg<hc0_char::HC0_CHAR_SPEC>,
    _reserved35: [u8; 0x04],
    #[doc = "0x3c508 - Host Channel x Interrupt Register"]
    pub hc0_int: crate::Reg<hc0_int::HC0_INT_SPEC>,
    #[doc = "0x3c50c - Host Channel x Interrupt Mask Register"]
    pub hc0_intmsk: crate::Reg<hc0_intmsk::HC0_INTMSK_SPEC>,
    #[doc = "0x3c510 - Host Channel x Transfer Size Register"]
    pub hc0_tsiz: crate::Reg<hc0_tsiz::HC0_TSIZ_SPEC>,
    #[doc = "0x3c514 - Host Channel x DMA Address Register"]
    pub hc0_dmaaddr: crate::Reg<hc0_dmaaddr::HC0_DMAADDR_SPEC>,
    _reserved39: [u8; 0x08],
    #[doc = "0x3c520 - Host Channel x Characteristics Register"]
    pub hc1_char: crate::Reg<hc1_char::HC1_CHAR_SPEC>,
    _reserved40: [u8; 0x04],
    #[doc = "0x3c528 - Host Channel x Interrupt Register"]
    pub hc1_int: crate::Reg<hc1_int::HC1_INT_SPEC>,
    #[doc = "0x3c52c - Host Channel x Interrupt Mask Register"]
    pub hc1_intmsk: crate::Reg<hc1_intmsk::HC1_INTMSK_SPEC>,
    #[doc = "0x3c530 - Host Channel x Transfer Size Register"]
    pub hc1_tsiz: crate::Reg<hc1_tsiz::HC1_TSIZ_SPEC>,
    #[doc = "0x3c534 - Host Channel x DMA Address Register"]
    pub hc1_dmaaddr: crate::Reg<hc1_dmaaddr::HC1_DMAADDR_SPEC>,
    _reserved44: [u8; 0x08],
    #[doc = "0x3c540 - Host Channel x Characteristics Register"]
    pub hc2_char: crate::Reg<hc2_char::HC2_CHAR_SPEC>,
    _reserved45: [u8; 0x04],
    #[doc = "0x3c548 - Host Channel x Interrupt Register"]
    pub hc2_int: crate::Reg<hc2_int::HC2_INT_SPEC>,
    #[doc = "0x3c54c - Host Channel x Interrupt Mask Register"]
    pub hc2_intmsk: crate::Reg<hc2_intmsk::HC2_INTMSK_SPEC>,
    #[doc = "0x3c550 - Host Channel x Transfer Size Register"]
    pub hc2_tsiz: crate::Reg<hc2_tsiz::HC2_TSIZ_SPEC>,
    #[doc = "0x3c554 - Host Channel x DMA Address Register"]
    pub hc2_dmaaddr: crate::Reg<hc2_dmaaddr::HC2_DMAADDR_SPEC>,
    _reserved49: [u8; 0x08],
    #[doc = "0x3c560 - Host Channel x Characteristics Register"]
    pub hc3_char: crate::Reg<hc3_char::HC3_CHAR_SPEC>,
    _reserved50: [u8; 0x04],
    #[doc = "0x3c568 - Host Channel x Interrupt Register"]
    pub hc3_int: crate::Reg<hc3_int::HC3_INT_SPEC>,
    #[doc = "0x3c56c - Host Channel x Interrupt Mask Register"]
    pub hc3_intmsk: crate::Reg<hc3_intmsk::HC3_INTMSK_SPEC>,
    #[doc = "0x3c570 - Host Channel x Transfer Size Register"]
    pub hc3_tsiz: crate::Reg<hc3_tsiz::HC3_TSIZ_SPEC>,
    #[doc = "0x3c574 - Host Channel x DMA Address Register"]
    pub hc3_dmaaddr: crate::Reg<hc3_dmaaddr::HC3_DMAADDR_SPEC>,
    _reserved54: [u8; 0x08],
    #[doc = "0x3c580 - Host Channel x Characteristics Register"]
    pub hc4_char: crate::Reg<hc4_char::HC4_CHAR_SPEC>,
    _reserved55: [u8; 0x04],
    #[doc = "0x3c588 - Host Channel x Interrupt Register"]
    pub hc4_int: crate::Reg<hc4_int::HC4_INT_SPEC>,
    #[doc = "0x3c58c - Host Channel x Interrupt Mask Register"]
    pub hc4_intmsk: crate::Reg<hc4_intmsk::HC4_INTMSK_SPEC>,
    #[doc = "0x3c590 - Host Channel x Transfer Size Register"]
    pub hc4_tsiz: crate::Reg<hc4_tsiz::HC4_TSIZ_SPEC>,
    #[doc = "0x3c594 - Host Channel x DMA Address Register"]
    pub hc4_dmaaddr: crate::Reg<hc4_dmaaddr::HC4_DMAADDR_SPEC>,
    _reserved59: [u8; 0x08],
    #[doc = "0x3c5a0 - Host Channel x Characteristics Register"]
    pub hc5_char: crate::Reg<hc5_char::HC5_CHAR_SPEC>,
    _reserved60: [u8; 0x04],
    #[doc = "0x3c5a8 - Host Channel x Interrupt Register"]
    pub hc5_int: crate::Reg<hc5_int::HC5_INT_SPEC>,
    #[doc = "0x3c5ac - Host Channel x Interrupt Mask Register"]
    pub hc5_intmsk: crate::Reg<hc5_intmsk::HC5_INTMSK_SPEC>,
    #[doc = "0x3c5b0 - Host Channel x Transfer Size Register"]
    pub hc5_tsiz: crate::Reg<hc5_tsiz::HC5_TSIZ_SPEC>,
    #[doc = "0x3c5b4 - Host Channel x DMA Address Register"]
    pub hc5_dmaaddr: crate::Reg<hc5_dmaaddr::HC5_DMAADDR_SPEC>,
    _reserved64: [u8; 0x08],
    #[doc = "0x3c5c0 - Host Channel x Characteristics Register"]
    pub hc6_char: crate::Reg<hc6_char::HC6_CHAR_SPEC>,
    _reserved65: [u8; 0x04],
    #[doc = "0x3c5c8 - Host Channel x Interrupt Register"]
    pub hc6_int: crate::Reg<hc6_int::HC6_INT_SPEC>,
    #[doc = "0x3c5cc - Host Channel x Interrupt Mask Register"]
    pub hc6_intmsk: crate::Reg<hc6_intmsk::HC6_INTMSK_SPEC>,
    #[doc = "0x3c5d0 - Host Channel x Transfer Size Register"]
    pub hc6_tsiz: crate::Reg<hc6_tsiz::HC6_TSIZ_SPEC>,
    #[doc = "0x3c5d4 - Host Channel x DMA Address Register"]
    pub hc6_dmaaddr: crate::Reg<hc6_dmaaddr::HC6_DMAADDR_SPEC>,
    _reserved69: [u8; 0x08],
    #[doc = "0x3c5e0 - Host Channel x Characteristics Register"]
    pub hc7_char: crate::Reg<hc7_char::HC7_CHAR_SPEC>,
    _reserved70: [u8; 0x04],
    #[doc = "0x3c5e8 - Host Channel x Interrupt Register"]
    pub hc7_int: crate::Reg<hc7_int::HC7_INT_SPEC>,
    #[doc = "0x3c5ec - Host Channel x Interrupt Mask Register"]
    pub hc7_intmsk: crate::Reg<hc7_intmsk::HC7_INTMSK_SPEC>,
    #[doc = "0x3c5f0 - Host Channel x Transfer Size Register"]
    pub hc7_tsiz: crate::Reg<hc7_tsiz::HC7_TSIZ_SPEC>,
    #[doc = "0x3c5f4 - Host Channel x DMA Address Register"]
    pub hc7_dmaaddr: crate::Reg<hc7_dmaaddr::HC7_DMAADDR_SPEC>,
    _reserved74: [u8; 0x08],
    #[doc = "0x3c600 - Host Channel x Characteristics Register"]
    pub hc8_char: crate::Reg<hc8_char::HC8_CHAR_SPEC>,
    _reserved75: [u8; 0x04],
    #[doc = "0x3c608 - Host Channel x Interrupt Register"]
    pub hc8_int: crate::Reg<hc8_int::HC8_INT_SPEC>,
    #[doc = "0x3c60c - Host Channel x Interrupt Mask Register"]
    pub hc8_intmsk: crate::Reg<hc8_intmsk::HC8_INTMSK_SPEC>,
    #[doc = "0x3c610 - Host Channel x Transfer Size Register"]
    pub hc8_tsiz: crate::Reg<hc8_tsiz::HC8_TSIZ_SPEC>,
    #[doc = "0x3c614 - Host Channel x DMA Address Register"]
    pub hc8_dmaaddr: crate::Reg<hc8_dmaaddr::HC8_DMAADDR_SPEC>,
    _reserved79: [u8; 0x08],
    #[doc = "0x3c620 - Host Channel x Characteristics Register"]
    pub hc9_char: crate::Reg<hc9_char::HC9_CHAR_SPEC>,
    _reserved80: [u8; 0x04],
    #[doc = "0x3c628 - Host Channel x Interrupt Register"]
    pub hc9_int: crate::Reg<hc9_int::HC9_INT_SPEC>,
    #[doc = "0x3c62c - Host Channel x Interrupt Mask Register"]
    pub hc9_intmsk: crate::Reg<hc9_intmsk::HC9_INTMSK_SPEC>,
    #[doc = "0x3c630 - Host Channel x Transfer Size Register"]
    pub hc9_tsiz: crate::Reg<hc9_tsiz::HC9_TSIZ_SPEC>,
    #[doc = "0x3c634 - Host Channel x DMA Address Register"]
    pub hc9_dmaaddr: crate::Reg<hc9_dmaaddr::HC9_DMAADDR_SPEC>,
    _reserved84: [u8; 0x08],
    #[doc = "0x3c640 - Host Channel x Characteristics Register"]
    pub hc10_char: crate::Reg<hc10_char::HC10_CHAR_SPEC>,
    _reserved85: [u8; 0x04],
    #[doc = "0x3c648 - Host Channel x Interrupt Register"]
    pub hc10_int: crate::Reg<hc10_int::HC10_INT_SPEC>,
    #[doc = "0x3c64c - Host Channel x Interrupt Mask Register"]
    pub hc10_intmsk: crate::Reg<hc10_intmsk::HC10_INTMSK_SPEC>,
    #[doc = "0x3c650 - Host Channel x Transfer Size Register"]
    pub hc10_tsiz: crate::Reg<hc10_tsiz::HC10_TSIZ_SPEC>,
    #[doc = "0x3c654 - Host Channel x DMA Address Register"]
    pub hc10_dmaaddr: crate::Reg<hc10_dmaaddr::HC10_DMAADDR_SPEC>,
    _reserved89: [u8; 0x08],
    #[doc = "0x3c660 - Host Channel x Characteristics Register"]
    pub hc11_char: crate::Reg<hc11_char::HC11_CHAR_SPEC>,
    _reserved90: [u8; 0x04],
    #[doc = "0x3c668 - Host Channel x Interrupt Register"]
    pub hc11_int: crate::Reg<hc11_int::HC11_INT_SPEC>,
    #[doc = "0x3c66c - Host Channel x Interrupt Mask Register"]
    pub hc11_intmsk: crate::Reg<hc11_intmsk::HC11_INTMSK_SPEC>,
    #[doc = "0x3c670 - Host Channel x Transfer Size Register"]
    pub hc11_tsiz: crate::Reg<hc11_tsiz::HC11_TSIZ_SPEC>,
    #[doc = "0x3c674 - Host Channel x DMA Address Register"]
    pub hc11_dmaaddr: crate::Reg<hc11_dmaaddr::HC11_DMAADDR_SPEC>,
    _reserved94: [u8; 0x08],
    #[doc = "0x3c680 - Host Channel x Characteristics Register"]
    pub hc12_char: crate::Reg<hc12_char::HC12_CHAR_SPEC>,
    _reserved95: [u8; 0x04],
    #[doc = "0x3c688 - Host Channel x Interrupt Register"]
    pub hc12_int: crate::Reg<hc12_int::HC12_INT_SPEC>,
    #[doc = "0x3c68c - Host Channel x Interrupt Mask Register"]
    pub hc12_intmsk: crate::Reg<hc12_intmsk::HC12_INTMSK_SPEC>,
    #[doc = "0x3c690 - Host Channel x Transfer Size Register"]
    pub hc12_tsiz: crate::Reg<hc12_tsiz::HC12_TSIZ_SPEC>,
    #[doc = "0x3c694 - Host Channel x DMA Address Register"]
    pub hc12_dmaaddr: crate::Reg<hc12_dmaaddr::HC12_DMAADDR_SPEC>,
    _reserved99: [u8; 0x08],
    #[doc = "0x3c6a0 - Host Channel x Characteristics Register"]
    pub hc13_char: crate::Reg<hc13_char::HC13_CHAR_SPEC>,
    _reserved100: [u8; 0x04],
    #[doc = "0x3c6a8 - Host Channel x Interrupt Register"]
    pub hc13_int: crate::Reg<hc13_int::HC13_INT_SPEC>,
    #[doc = "0x3c6ac - Host Channel x Interrupt Mask Register"]
    pub hc13_intmsk: crate::Reg<hc13_intmsk::HC13_INTMSK_SPEC>,
    #[doc = "0x3c6b0 - Host Channel x Transfer Size Register"]
    pub hc13_tsiz: crate::Reg<hc13_tsiz::HC13_TSIZ_SPEC>,
    #[doc = "0x3c6b4 - Host Channel x DMA Address Register"]
    pub hc13_dmaaddr: crate::Reg<hc13_dmaaddr::HC13_DMAADDR_SPEC>,
    _reserved104: [u8; 0x0148],
    #[doc = "0x3c800 - Device Configuration Register"]
    pub dcfg: crate::Reg<dcfg::DCFG_SPEC>,
    #[doc = "0x3c804 - Device Control Register"]
    pub dctl: crate::Reg<dctl::DCTL_SPEC>,
    #[doc = "0x3c808 - Device Status Register"]
    pub dsts: crate::Reg<dsts::DSTS_SPEC>,
    _reserved107: [u8; 0x04],
    #[doc = "0x3c810 - Device IN Endpoint Common Interrupt Mask Register"]
    pub diepmsk: crate::Reg<diepmsk::DIEPMSK_SPEC>,
    #[doc = "0x3c814 - Device OUT Endpoint Common Interrupt Mask Register"]
    pub doepmsk: crate::Reg<doepmsk::DOEPMSK_SPEC>,
    #[doc = "0x3c818 - Device All Endpoints Interrupt Register"]
    pub daint: crate::Reg<daint::DAINT_SPEC>,
    #[doc = "0x3c81c - Device All Endpoints Interrupt Mask Register"]
    pub daintmsk: crate::Reg<daintmsk::DAINTMSK_SPEC>,
    _reserved111: [u8; 0x08],
    #[doc = "0x3c828 - Device VBUS Discharge Time Register"]
    pub dvbusdis: crate::Reg<dvbusdis::DVBUSDIS_SPEC>,
    #[doc = "0x3c82c - Device VBUS Pulsing Time Register"]
    pub dvbuspulse: crate::Reg<dvbuspulse::DVBUSPULSE_SPEC>,
    _reserved113: [u8; 0x04],
    #[doc = "0x3c834 - Device IN Endpoint FIFO Empty Interrupt Mask Register"]
    pub diepempmsk: crate::Reg<diepempmsk::DIEPEMPMSK_SPEC>,
    _reserved114: [u8; 0xc8],
    #[doc = "0x3c900 - Device IN Endpoint 0 Control Register"]
    pub diep0ctl: crate::Reg<diep0ctl::DIEP0CTL_SPEC>,
    _reserved115: [u8; 0x04],
    #[doc = "0x3c908 - Device IN Endpoint 0 Interrupt Register"]
    pub diep0int: crate::Reg<diep0int::DIEP0INT_SPEC>,
    _reserved116: [u8; 0x04],
    #[doc = "0x3c910 - Device IN Endpoint 0 Transfer Size Register"]
    pub diep0tsiz: crate::Reg<diep0tsiz::DIEP0TSIZ_SPEC>,
    #[doc = "0x3c914 - Device IN Endpoint 0 DMA Address Register"]
    pub diep0dmaaddr: crate::Reg<diep0dmaaddr::DIEP0DMAADDR_SPEC>,
    #[doc = "0x3c918 - Device IN Endpoint 0 Transmit FIFO Status Register"]
    pub diep0txfsts: crate::Reg<diep0txfsts::DIEP0TXFSTS_SPEC>,
    _reserved119: [u8; 0x04],
    #[doc = "0x3c920 - Device IN Endpoint x+1 Control Register"]
    pub diep0_ctl: crate::Reg<diep0_ctl::DIEP0_CTL_SPEC>,
    _reserved120: [u8; 0x04],
    #[doc = "0x3c928 - Device IN Endpoint x+1 Interrupt Register"]
    pub diep0_int: crate::Reg<diep0_int::DIEP0_INT_SPEC>,
    _reserved121: [u8; 0x04],
    #[doc = "0x3c930 - Device IN Endpoint x+1 Transfer Size Register"]
    pub diep0_tsiz: crate::Reg<diep0_tsiz::DIEP0_TSIZ_SPEC>,
    #[doc = "0x3c934 - Device IN Endpoint x+1 DMA Address Register"]
    pub diep0_dmaaddr: crate::Reg<diep0_dmaaddr::DIEP0_DMAADDR_SPEC>,
    #[doc = "0x3c938 - Device IN Endpoint x+1 Transmit FIFO Status Register"]
    pub diep0_txfsts: crate::Reg<diep0_txfsts::DIEP0_TXFSTS_SPEC>,
    _reserved124: [u8; 0x04],
    #[doc = "0x3c940 - Device IN Endpoint x+1 Control Register"]
    pub diep1_ctl: crate::Reg<diep1_ctl::DIEP1_CTL_SPEC>,
    _reserved125: [u8; 0x04],
    #[doc = "0x3c948 - Device IN Endpoint x+1 Interrupt Register"]
    pub diep1_int: crate::Reg<diep1_int::DIEP1_INT_SPEC>,
    _reserved126: [u8; 0x04],
    #[doc = "0x3c950 - Device IN Endpoint x+1 Transfer Size Register"]
    pub diep1_tsiz: crate::Reg<diep1_tsiz::DIEP1_TSIZ_SPEC>,
    #[doc = "0x3c954 - Device IN Endpoint x+1 DMA Address Register"]
    pub diep1_dmaaddr: crate::Reg<diep1_dmaaddr::DIEP1_DMAADDR_SPEC>,
    #[doc = "0x3c958 - Device IN Endpoint x+1 Transmit FIFO Status Register"]
    pub diep1_txfsts: crate::Reg<diep1_txfsts::DIEP1_TXFSTS_SPEC>,
    _reserved129: [u8; 0x04],
    #[doc = "0x3c960 - Device IN Endpoint x+1 Control Register"]
    pub diep2_ctl: crate::Reg<diep2_ctl::DIEP2_CTL_SPEC>,
    _reserved130: [u8; 0x04],
    #[doc = "0x3c968 - Device IN Endpoint x+1 Interrupt Register"]
    pub diep2_int: crate::Reg<diep2_int::DIEP2_INT_SPEC>,
    _reserved131: [u8; 0x04],
    #[doc = "0x3c970 - Device IN Endpoint x+1 Transfer Size Register"]
    pub diep2_tsiz: crate::Reg<diep2_tsiz::DIEP2_TSIZ_SPEC>,
    #[doc = "0x3c974 - Device IN Endpoint x+1 DMA Address Register"]
    pub diep2_dmaaddr: crate::Reg<diep2_dmaaddr::DIEP2_DMAADDR_SPEC>,
    #[doc = "0x3c978 - Device IN Endpoint x+1 Transmit FIFO Status Register"]
    pub diep2_txfsts: crate::Reg<diep2_txfsts::DIEP2_TXFSTS_SPEC>,
    _reserved134: [u8; 0x04],
    #[doc = "0x3c980 - Device IN Endpoint x+1 Control Register"]
    pub diep3_ctl: crate::Reg<diep3_ctl::DIEP3_CTL_SPEC>,
    _reserved135: [u8; 0x04],
    #[doc = "0x3c988 - Device IN Endpoint x+1 Interrupt Register"]
    pub diep3_int: crate::Reg<diep3_int::DIEP3_INT_SPEC>,
    _reserved136: [u8; 0x04],
    #[doc = "0x3c990 - Device IN Endpoint x+1 Transfer Size Register"]
    pub diep3_tsiz: crate::Reg<diep3_tsiz::DIEP3_TSIZ_SPEC>,
    #[doc = "0x3c994 - Device IN Endpoint x+1 DMA Address Register"]
    pub diep3_dmaaddr: crate::Reg<diep3_dmaaddr::DIEP3_DMAADDR_SPEC>,
    #[doc = "0x3c998 - Device IN Endpoint x+1 Transmit FIFO Status Register"]
    pub diep3_txfsts: crate::Reg<diep3_txfsts::DIEP3_TXFSTS_SPEC>,
    _reserved139: [u8; 0x04],
    #[doc = "0x3c9a0 - Device IN Endpoint x+1 Control Register"]
    pub diep4_ctl: crate::Reg<diep4_ctl::DIEP4_CTL_SPEC>,
    _reserved140: [u8; 0x04],
    #[doc = "0x3c9a8 - Device IN Endpoint x+1 Interrupt Register"]
    pub diep4_int: crate::Reg<diep4_int::DIEP4_INT_SPEC>,
    _reserved141: [u8; 0x04],
    #[doc = "0x3c9b0 - Device IN Endpoint x+1 Transfer Size Register"]
    pub diep4_tsiz: crate::Reg<diep4_tsiz::DIEP4_TSIZ_SPEC>,
    #[doc = "0x3c9b4 - Device IN Endpoint x+1 DMA Address Register"]
    pub diep4_dmaaddr: crate::Reg<diep4_dmaaddr::DIEP4_DMAADDR_SPEC>,
    #[doc = "0x3c9b8 - Device IN Endpoint x+1 Transmit FIFO Status Register"]
    pub diep4_txfsts: crate::Reg<diep4_txfsts::DIEP4_TXFSTS_SPEC>,
    _reserved144: [u8; 0x04],
    #[doc = "0x3c9c0 - Device IN Endpoint x+1 Control Register"]
    pub diep5_ctl: crate::Reg<diep5_ctl::DIEP5_CTL_SPEC>,
    _reserved145: [u8; 0x04],
    #[doc = "0x3c9c8 - Device IN Endpoint x+1 Interrupt Register"]
    pub diep5_int: crate::Reg<diep5_int::DIEP5_INT_SPEC>,
    _reserved146: [u8; 0x04],
    #[doc = "0x3c9d0 - Device IN Endpoint x+1 Transfer Size Register"]
    pub diep5_tsiz: crate::Reg<diep5_tsiz::DIEP5_TSIZ_SPEC>,
    #[doc = "0x3c9d4 - Device IN Endpoint x+1 DMA Address Register"]
    pub diep5_dmaaddr: crate::Reg<diep5_dmaaddr::DIEP5_DMAADDR_SPEC>,
    #[doc = "0x3c9d8 - Device IN Endpoint x+1 Transmit FIFO Status Register"]
    pub diep5_txfsts: crate::Reg<diep5_txfsts::DIEP5_TXFSTS_SPEC>,
    _reserved149: [u8; 0x0124],
    #[doc = "0x3cb00 - Device OUT Endpoint 0 Control Register"]
    pub doep0ctl: crate::Reg<doep0ctl::DOEP0CTL_SPEC>,
    _reserved150: [u8; 0x04],
    #[doc = "0x3cb08 - Device OUT Endpoint 0 Interrupt Register"]
    pub doep0int: crate::Reg<doep0int::DOEP0INT_SPEC>,
    _reserved151: [u8; 0x04],
    #[doc = "0x3cb10 - Device OUT Endpoint 0 Transfer Size Register"]
    pub doep0tsiz: crate::Reg<doep0tsiz::DOEP0TSIZ_SPEC>,
    #[doc = "0x3cb14 - Device OUT Endpoint 0 DMA Address Register"]
    pub doep0dmaaddr: crate::Reg<doep0dmaaddr::DOEP0DMAADDR_SPEC>,
    _reserved153: [u8; 0x08],
    #[doc = "0x3cb20 - Device OUT Endpoint x+1 Control Register"]
    pub doep0_ctl: crate::Reg<doep0_ctl::DOEP0_CTL_SPEC>,
    _reserved154: [u8; 0x04],
    #[doc = "0x3cb28 - Device OUT Endpoint x+1 Interrupt Register"]
    pub doep0_int: crate::Reg<doep0_int::DOEP0_INT_SPEC>,
    _reserved155: [u8; 0x04],
    #[doc = "0x3cb30 - Device OUT Endpoint x+1 Transfer Size Register"]
    pub doep0_tsiz: crate::Reg<doep0_tsiz::DOEP0_TSIZ_SPEC>,
    #[doc = "0x3cb34 - Device OUT Endpoint x+1 DMA Address Register"]
    pub doep0_dmaaddr: crate::Reg<doep0_dmaaddr::DOEP0_DMAADDR_SPEC>,
    _reserved157: [u8; 0x08],
    #[doc = "0x3cb40 - Device OUT Endpoint x+1 Control Register"]
    pub doep1_ctl: crate::Reg<doep1_ctl::DOEP1_CTL_SPEC>,
    _reserved158: [u8; 0x04],
    #[doc = "0x3cb48 - Device OUT Endpoint x+1 Interrupt Register"]
    pub doep1_int: crate::Reg<doep1_int::DOEP1_INT_SPEC>,
    _reserved159: [u8; 0x04],
    #[doc = "0x3cb50 - Device OUT Endpoint x+1 Transfer Size Register"]
    pub doep1_tsiz: crate::Reg<doep1_tsiz::DOEP1_TSIZ_SPEC>,
    #[doc = "0x3cb54 - Device OUT Endpoint x+1 DMA Address Register"]
    pub doep1_dmaaddr: crate::Reg<doep1_dmaaddr::DOEP1_DMAADDR_SPEC>,
    _reserved161: [u8; 0x08],
    #[doc = "0x3cb60 - Device OUT Endpoint x+1 Control Register"]
    pub doep2_ctl: crate::Reg<doep2_ctl::DOEP2_CTL_SPEC>,
    _reserved162: [u8; 0x04],
    #[doc = "0x3cb68 - Device OUT Endpoint x+1 Interrupt Register"]
    pub doep2_int: crate::Reg<doep2_int::DOEP2_INT_SPEC>,
    _reserved163: [u8; 0x04],
    #[doc = "0x3cb70 - Device OUT Endpoint x+1 Transfer Size Register"]
    pub doep2_tsiz: crate::Reg<doep2_tsiz::DOEP2_TSIZ_SPEC>,
    #[doc = "0x3cb74 - Device OUT Endpoint x+1 DMA Address Register"]
    pub doep2_dmaaddr: crate::Reg<doep2_dmaaddr::DOEP2_DMAADDR_SPEC>,
    _reserved165: [u8; 0x08],
    #[doc = "0x3cb80 - Device OUT Endpoint x+1 Control Register"]
    pub doep3_ctl: crate::Reg<doep3_ctl::DOEP3_CTL_SPEC>,
    _reserved166: [u8; 0x04],
    #[doc = "0x3cb88 - Device OUT Endpoint x+1 Interrupt Register"]
    pub doep3_int: crate::Reg<doep3_int::DOEP3_INT_SPEC>,
    _reserved167: [u8; 0x04],
    #[doc = "0x3cb90 - Device OUT Endpoint x+1 Transfer Size Register"]
    pub doep3_tsiz: crate::Reg<doep3_tsiz::DOEP3_TSIZ_SPEC>,
    #[doc = "0x3cb94 - Device OUT Endpoint x+1 DMA Address Register"]
    pub doep3_dmaaddr: crate::Reg<doep3_dmaaddr::DOEP3_DMAADDR_SPEC>,
    _reserved169: [u8; 0x08],
    #[doc = "0x3cba0 - Device OUT Endpoint x+1 Control Register"]
    pub doep4_ctl: crate::Reg<doep4_ctl::DOEP4_CTL_SPEC>,
    _reserved170: [u8; 0x04],
    #[doc = "0x3cba8 - Device OUT Endpoint x+1 Interrupt Register"]
    pub doep4_int: crate::Reg<doep4_int::DOEP4_INT_SPEC>,
    _reserved171: [u8; 0x04],
    #[doc = "0x3cbb0 - Device OUT Endpoint x+1 Transfer Size Register"]
    pub doep4_tsiz: crate::Reg<doep4_tsiz::DOEP4_TSIZ_SPEC>,
    #[doc = "0x3cbb4 - Device OUT Endpoint x+1 DMA Address Register"]
    pub doep4_dmaaddr: crate::Reg<doep4_dmaaddr::DOEP4_DMAADDR_SPEC>,
    _reserved173: [u8; 0x08],
    #[doc = "0x3cbc0 - Device OUT Endpoint x+1 Control Register"]
    pub doep5_ctl: crate::Reg<doep5_ctl::DOEP5_CTL_SPEC>,
    _reserved174: [u8; 0x04],
    #[doc = "0x3cbc8 - Device OUT Endpoint x+1 Interrupt Register"]
    pub doep5_int: crate::Reg<doep5_int::DOEP5_INT_SPEC>,
    _reserved175: [u8; 0x04],
    #[doc = "0x3cbd0 - Device OUT Endpoint x+1 Transfer Size Register"]
    pub doep5_tsiz: crate::Reg<doep5_tsiz::DOEP5_TSIZ_SPEC>,
    #[doc = "0x3cbd4 - Device OUT Endpoint x+1 DMA Address Register"]
    pub doep5_dmaaddr: crate::Reg<doep5_dmaaddr::DOEP5_DMAADDR_SPEC>,
    _reserved177: [u8; 0x0228],
    #[doc = "0x3ce00 - Power and Clock Gating Control Register"]
    pub pcgcctl: crate::Reg<pcgcctl::PCGCCTL_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "System Control Register"]
pub mod ctrl;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "System Status Register"]
pub mod status;
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
#[doc = "ROUTE register accessor: an alias for `Reg<ROUTE_SPEC>`"]
pub type ROUTE = crate::Reg<route::ROUTE_SPEC>;
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "GOTGCTL register accessor: an alias for `Reg<GOTGCTL_SPEC>`"]
pub type GOTGCTL = crate::Reg<gotgctl::GOTGCTL_SPEC>;
#[doc = "OTG Control and Status Register"]
pub mod gotgctl;
#[doc = "GOTGINT register accessor: an alias for `Reg<GOTGINT_SPEC>`"]
pub type GOTGINT = crate::Reg<gotgint::GOTGINT_SPEC>;
#[doc = "OTG Interrupt Register"]
pub mod gotgint;
#[doc = "GAHBCFG register accessor: an alias for `Reg<GAHBCFG_SPEC>`"]
pub type GAHBCFG = crate::Reg<gahbcfg::GAHBCFG_SPEC>;
#[doc = "AHB Configuration Register"]
pub mod gahbcfg;
#[doc = "GUSBCFG register accessor: an alias for `Reg<GUSBCFG_SPEC>`"]
pub type GUSBCFG = crate::Reg<gusbcfg::GUSBCFG_SPEC>;
#[doc = "USB Configuration Register"]
pub mod gusbcfg;
#[doc = "GRSTCTL register accessor: an alias for `Reg<GRSTCTL_SPEC>`"]
pub type GRSTCTL = crate::Reg<grstctl::GRSTCTL_SPEC>;
#[doc = "Reset Register"]
pub mod grstctl;
#[doc = "GINTSTS register accessor: an alias for `Reg<GINTSTS_SPEC>`"]
pub type GINTSTS = crate::Reg<gintsts::GINTSTS_SPEC>;
#[doc = "Interrupt Register"]
pub mod gintsts;
#[doc = "GINTMSK register accessor: an alias for `Reg<GINTMSK_SPEC>`"]
pub type GINTMSK = crate::Reg<gintmsk::GINTMSK_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod gintmsk;
#[doc = "GRXSTSR register accessor: an alias for `Reg<GRXSTSR_SPEC>`"]
pub type GRXSTSR = crate::Reg<grxstsr::GRXSTSR_SPEC>;
#[doc = "Receive Status Debug Read Register"]
pub mod grxstsr;
#[doc = "GRXSTSP register accessor: an alias for `Reg<GRXSTSP_SPEC>`"]
pub type GRXSTSP = crate::Reg<grxstsp::GRXSTSP_SPEC>;
#[doc = "Receive Status Read and Pop Register"]
pub mod grxstsp;
#[doc = "GRXFSIZ register accessor: an alias for `Reg<GRXFSIZ_SPEC>`"]
pub type GRXFSIZ = crate::Reg<grxfsiz::GRXFSIZ_SPEC>;
#[doc = "Receive FIFO Size Register"]
pub mod grxfsiz;
#[doc = "GNPTXFSIZ register accessor: an alias for `Reg<GNPTXFSIZ_SPEC>`"]
pub type GNPTXFSIZ = crate::Reg<gnptxfsiz::GNPTXFSIZ_SPEC>;
#[doc = "Non-periodic Transmit FIFO Size Register"]
pub mod gnptxfsiz;
#[doc = "GNPTXSTS register accessor: an alias for `Reg<GNPTXSTS_SPEC>`"]
pub type GNPTXSTS = crate::Reg<gnptxsts::GNPTXSTS_SPEC>;
#[doc = "Non-periodic Transmit FIFO/Queue Status Register"]
pub mod gnptxsts;
#[doc = "GDFIFOCFG register accessor: an alias for `Reg<GDFIFOCFG_SPEC>`"]
pub type GDFIFOCFG = crate::Reg<gdfifocfg::GDFIFOCFG_SPEC>;
#[doc = "Global DFIFO Configuration Register"]
pub mod gdfifocfg;
#[doc = "HPTXFSIZ register accessor: an alias for `Reg<HPTXFSIZ_SPEC>`"]
pub type HPTXFSIZ = crate::Reg<hptxfsiz::HPTXFSIZ_SPEC>;
#[doc = "Host Periodic Transmit FIFO Size Register"]
pub mod hptxfsiz;
#[doc = "DIEPTXF1 register accessor: an alias for `Reg<DIEPTXF1_SPEC>`"]
pub type DIEPTXF1 = crate::Reg<dieptxf1::DIEPTXF1_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO 1 Size Register"]
pub mod dieptxf1;
#[doc = "DIEPTXF2 register accessor: an alias for `Reg<DIEPTXF2_SPEC>`"]
pub type DIEPTXF2 = crate::Reg<dieptxf2::DIEPTXF2_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO 2 Size Register"]
pub mod dieptxf2;
#[doc = "DIEPTXF3 register accessor: an alias for `Reg<DIEPTXF3_SPEC>`"]
pub type DIEPTXF3 = crate::Reg<dieptxf3::DIEPTXF3_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO 3 Size Register"]
pub mod dieptxf3;
#[doc = "DIEPTXF4 register accessor: an alias for `Reg<DIEPTXF4_SPEC>`"]
pub type DIEPTXF4 = crate::Reg<dieptxf4::DIEPTXF4_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO 4 Size Register"]
pub mod dieptxf4;
#[doc = "DIEPTXF5 register accessor: an alias for `Reg<DIEPTXF5_SPEC>`"]
pub type DIEPTXF5 = crate::Reg<dieptxf5::DIEPTXF5_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO 5 Size Register"]
pub mod dieptxf5;
#[doc = "DIEPTXF6 register accessor: an alias for `Reg<DIEPTXF6_SPEC>`"]
pub type DIEPTXF6 = crate::Reg<dieptxf6::DIEPTXF6_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO 6 Size Register"]
pub mod dieptxf6;
#[doc = "HCFG register accessor: an alias for `Reg<HCFG_SPEC>`"]
pub type HCFG = crate::Reg<hcfg::HCFG_SPEC>;
#[doc = "Host Configuration Register"]
pub mod hcfg;
#[doc = "HFIR register accessor: an alias for `Reg<HFIR_SPEC>`"]
pub type HFIR = crate::Reg<hfir::HFIR_SPEC>;
#[doc = "Host Frame Interval Register"]
pub mod hfir;
#[doc = "HFNUM register accessor: an alias for `Reg<HFNUM_SPEC>`"]
pub type HFNUM = crate::Reg<hfnum::HFNUM_SPEC>;
#[doc = "Host Frame Number/Frame Time Remaining Register"]
pub mod hfnum;
#[doc = "HPTXSTS register accessor: an alias for `Reg<HPTXSTS_SPEC>`"]
pub type HPTXSTS = crate::Reg<hptxsts::HPTXSTS_SPEC>;
#[doc = "Host Periodic Transmit FIFO/Queue Status Register"]
pub mod hptxsts;
#[doc = "HAINT register accessor: an alias for `Reg<HAINT_SPEC>`"]
pub type HAINT = crate::Reg<haint::HAINT_SPEC>;
#[doc = "Host All Channels Interrupt Register"]
pub mod haint;
#[doc = "HAINTMSK register accessor: an alias for `Reg<HAINTMSK_SPEC>`"]
pub type HAINTMSK = crate::Reg<haintmsk::HAINTMSK_SPEC>;
#[doc = "Host All Channels Interrupt Mask Register"]
pub mod haintmsk;
#[doc = "HPRT register accessor: an alias for `Reg<HPRT_SPEC>`"]
pub type HPRT = crate::Reg<hprt::HPRT_SPEC>;
#[doc = "Host Port Control and Status Register"]
pub mod hprt;
#[doc = "HC0_CHAR register accessor: an alias for `Reg<HC0_CHAR_SPEC>`"]
pub type HC0_CHAR = crate::Reg<hc0_char::HC0_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc0_char;
#[doc = "HC0_INT register accessor: an alias for `Reg<HC0_INT_SPEC>`"]
pub type HC0_INT = crate::Reg<hc0_int::HC0_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc0_int;
#[doc = "HC0_INTMSK register accessor: an alias for `Reg<HC0_INTMSK_SPEC>`"]
pub type HC0_INTMSK = crate::Reg<hc0_intmsk::HC0_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc0_intmsk;
#[doc = "HC0_TSIZ register accessor: an alias for `Reg<HC0_TSIZ_SPEC>`"]
pub type HC0_TSIZ = crate::Reg<hc0_tsiz::HC0_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc0_tsiz;
#[doc = "HC0_DMAADDR register accessor: an alias for `Reg<HC0_DMAADDR_SPEC>`"]
pub type HC0_DMAADDR = crate::Reg<hc0_dmaaddr::HC0_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc0_dmaaddr;
#[doc = "HC1_CHAR register accessor: an alias for `Reg<HC1_CHAR_SPEC>`"]
pub type HC1_CHAR = crate::Reg<hc1_char::HC1_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc1_char;
#[doc = "HC1_INT register accessor: an alias for `Reg<HC1_INT_SPEC>`"]
pub type HC1_INT = crate::Reg<hc1_int::HC1_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc1_int;
#[doc = "HC1_INTMSK register accessor: an alias for `Reg<HC1_INTMSK_SPEC>`"]
pub type HC1_INTMSK = crate::Reg<hc1_intmsk::HC1_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc1_intmsk;
#[doc = "HC1_TSIZ register accessor: an alias for `Reg<HC1_TSIZ_SPEC>`"]
pub type HC1_TSIZ = crate::Reg<hc1_tsiz::HC1_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc1_tsiz;
#[doc = "HC1_DMAADDR register accessor: an alias for `Reg<HC1_DMAADDR_SPEC>`"]
pub type HC1_DMAADDR = crate::Reg<hc1_dmaaddr::HC1_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc1_dmaaddr;
#[doc = "HC2_CHAR register accessor: an alias for `Reg<HC2_CHAR_SPEC>`"]
pub type HC2_CHAR = crate::Reg<hc2_char::HC2_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc2_char;
#[doc = "HC2_INT register accessor: an alias for `Reg<HC2_INT_SPEC>`"]
pub type HC2_INT = crate::Reg<hc2_int::HC2_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc2_int;
#[doc = "HC2_INTMSK register accessor: an alias for `Reg<HC2_INTMSK_SPEC>`"]
pub type HC2_INTMSK = crate::Reg<hc2_intmsk::HC2_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc2_intmsk;
#[doc = "HC2_TSIZ register accessor: an alias for `Reg<HC2_TSIZ_SPEC>`"]
pub type HC2_TSIZ = crate::Reg<hc2_tsiz::HC2_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc2_tsiz;
#[doc = "HC2_DMAADDR register accessor: an alias for `Reg<HC2_DMAADDR_SPEC>`"]
pub type HC2_DMAADDR = crate::Reg<hc2_dmaaddr::HC2_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc2_dmaaddr;
#[doc = "HC3_CHAR register accessor: an alias for `Reg<HC3_CHAR_SPEC>`"]
pub type HC3_CHAR = crate::Reg<hc3_char::HC3_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc3_char;
#[doc = "HC3_INT register accessor: an alias for `Reg<HC3_INT_SPEC>`"]
pub type HC3_INT = crate::Reg<hc3_int::HC3_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc3_int;
#[doc = "HC3_INTMSK register accessor: an alias for `Reg<HC3_INTMSK_SPEC>`"]
pub type HC3_INTMSK = crate::Reg<hc3_intmsk::HC3_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc3_intmsk;
#[doc = "HC3_TSIZ register accessor: an alias for `Reg<HC3_TSIZ_SPEC>`"]
pub type HC3_TSIZ = crate::Reg<hc3_tsiz::HC3_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc3_tsiz;
#[doc = "HC3_DMAADDR register accessor: an alias for `Reg<HC3_DMAADDR_SPEC>`"]
pub type HC3_DMAADDR = crate::Reg<hc3_dmaaddr::HC3_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc3_dmaaddr;
#[doc = "HC4_CHAR register accessor: an alias for `Reg<HC4_CHAR_SPEC>`"]
pub type HC4_CHAR = crate::Reg<hc4_char::HC4_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc4_char;
#[doc = "HC4_INT register accessor: an alias for `Reg<HC4_INT_SPEC>`"]
pub type HC4_INT = crate::Reg<hc4_int::HC4_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc4_int;
#[doc = "HC4_INTMSK register accessor: an alias for `Reg<HC4_INTMSK_SPEC>`"]
pub type HC4_INTMSK = crate::Reg<hc4_intmsk::HC4_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc4_intmsk;
#[doc = "HC4_TSIZ register accessor: an alias for `Reg<HC4_TSIZ_SPEC>`"]
pub type HC4_TSIZ = crate::Reg<hc4_tsiz::HC4_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc4_tsiz;
#[doc = "HC4_DMAADDR register accessor: an alias for `Reg<HC4_DMAADDR_SPEC>`"]
pub type HC4_DMAADDR = crate::Reg<hc4_dmaaddr::HC4_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc4_dmaaddr;
#[doc = "HC5_CHAR register accessor: an alias for `Reg<HC5_CHAR_SPEC>`"]
pub type HC5_CHAR = crate::Reg<hc5_char::HC5_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc5_char;
#[doc = "HC5_INT register accessor: an alias for `Reg<HC5_INT_SPEC>`"]
pub type HC5_INT = crate::Reg<hc5_int::HC5_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc5_int;
#[doc = "HC5_INTMSK register accessor: an alias for `Reg<HC5_INTMSK_SPEC>`"]
pub type HC5_INTMSK = crate::Reg<hc5_intmsk::HC5_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc5_intmsk;
#[doc = "HC5_TSIZ register accessor: an alias for `Reg<HC5_TSIZ_SPEC>`"]
pub type HC5_TSIZ = crate::Reg<hc5_tsiz::HC5_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc5_tsiz;
#[doc = "HC5_DMAADDR register accessor: an alias for `Reg<HC5_DMAADDR_SPEC>`"]
pub type HC5_DMAADDR = crate::Reg<hc5_dmaaddr::HC5_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc5_dmaaddr;
#[doc = "HC6_CHAR register accessor: an alias for `Reg<HC6_CHAR_SPEC>`"]
pub type HC6_CHAR = crate::Reg<hc6_char::HC6_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc6_char;
#[doc = "HC6_INT register accessor: an alias for `Reg<HC6_INT_SPEC>`"]
pub type HC6_INT = crate::Reg<hc6_int::HC6_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc6_int;
#[doc = "HC6_INTMSK register accessor: an alias for `Reg<HC6_INTMSK_SPEC>`"]
pub type HC6_INTMSK = crate::Reg<hc6_intmsk::HC6_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc6_intmsk;
#[doc = "HC6_TSIZ register accessor: an alias for `Reg<HC6_TSIZ_SPEC>`"]
pub type HC6_TSIZ = crate::Reg<hc6_tsiz::HC6_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc6_tsiz;
#[doc = "HC6_DMAADDR register accessor: an alias for `Reg<HC6_DMAADDR_SPEC>`"]
pub type HC6_DMAADDR = crate::Reg<hc6_dmaaddr::HC6_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc6_dmaaddr;
#[doc = "HC7_CHAR register accessor: an alias for `Reg<HC7_CHAR_SPEC>`"]
pub type HC7_CHAR = crate::Reg<hc7_char::HC7_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc7_char;
#[doc = "HC7_INT register accessor: an alias for `Reg<HC7_INT_SPEC>`"]
pub type HC7_INT = crate::Reg<hc7_int::HC7_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc7_int;
#[doc = "HC7_INTMSK register accessor: an alias for `Reg<HC7_INTMSK_SPEC>`"]
pub type HC7_INTMSK = crate::Reg<hc7_intmsk::HC7_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc7_intmsk;
#[doc = "HC7_TSIZ register accessor: an alias for `Reg<HC7_TSIZ_SPEC>`"]
pub type HC7_TSIZ = crate::Reg<hc7_tsiz::HC7_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc7_tsiz;
#[doc = "HC7_DMAADDR register accessor: an alias for `Reg<HC7_DMAADDR_SPEC>`"]
pub type HC7_DMAADDR = crate::Reg<hc7_dmaaddr::HC7_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc7_dmaaddr;
#[doc = "HC8_CHAR register accessor: an alias for `Reg<HC8_CHAR_SPEC>`"]
pub type HC8_CHAR = crate::Reg<hc8_char::HC8_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc8_char;
#[doc = "HC8_INT register accessor: an alias for `Reg<HC8_INT_SPEC>`"]
pub type HC8_INT = crate::Reg<hc8_int::HC8_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc8_int;
#[doc = "HC8_INTMSK register accessor: an alias for `Reg<HC8_INTMSK_SPEC>`"]
pub type HC8_INTMSK = crate::Reg<hc8_intmsk::HC8_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc8_intmsk;
#[doc = "HC8_TSIZ register accessor: an alias for `Reg<HC8_TSIZ_SPEC>`"]
pub type HC8_TSIZ = crate::Reg<hc8_tsiz::HC8_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc8_tsiz;
#[doc = "HC8_DMAADDR register accessor: an alias for `Reg<HC8_DMAADDR_SPEC>`"]
pub type HC8_DMAADDR = crate::Reg<hc8_dmaaddr::HC8_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc8_dmaaddr;
#[doc = "HC9_CHAR register accessor: an alias for `Reg<HC9_CHAR_SPEC>`"]
pub type HC9_CHAR = crate::Reg<hc9_char::HC9_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc9_char;
#[doc = "HC9_INT register accessor: an alias for `Reg<HC9_INT_SPEC>`"]
pub type HC9_INT = crate::Reg<hc9_int::HC9_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc9_int;
#[doc = "HC9_INTMSK register accessor: an alias for `Reg<HC9_INTMSK_SPEC>`"]
pub type HC9_INTMSK = crate::Reg<hc9_intmsk::HC9_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc9_intmsk;
#[doc = "HC9_TSIZ register accessor: an alias for `Reg<HC9_TSIZ_SPEC>`"]
pub type HC9_TSIZ = crate::Reg<hc9_tsiz::HC9_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc9_tsiz;
#[doc = "HC9_DMAADDR register accessor: an alias for `Reg<HC9_DMAADDR_SPEC>`"]
pub type HC9_DMAADDR = crate::Reg<hc9_dmaaddr::HC9_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc9_dmaaddr;
#[doc = "HC10_CHAR register accessor: an alias for `Reg<HC10_CHAR_SPEC>`"]
pub type HC10_CHAR = crate::Reg<hc10_char::HC10_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc10_char;
#[doc = "HC10_INT register accessor: an alias for `Reg<HC10_INT_SPEC>`"]
pub type HC10_INT = crate::Reg<hc10_int::HC10_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc10_int;
#[doc = "HC10_INTMSK register accessor: an alias for `Reg<HC10_INTMSK_SPEC>`"]
pub type HC10_INTMSK = crate::Reg<hc10_intmsk::HC10_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc10_intmsk;
#[doc = "HC10_TSIZ register accessor: an alias for `Reg<HC10_TSIZ_SPEC>`"]
pub type HC10_TSIZ = crate::Reg<hc10_tsiz::HC10_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc10_tsiz;
#[doc = "HC10_DMAADDR register accessor: an alias for `Reg<HC10_DMAADDR_SPEC>`"]
pub type HC10_DMAADDR = crate::Reg<hc10_dmaaddr::HC10_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc10_dmaaddr;
#[doc = "HC11_CHAR register accessor: an alias for `Reg<HC11_CHAR_SPEC>`"]
pub type HC11_CHAR = crate::Reg<hc11_char::HC11_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc11_char;
#[doc = "HC11_INT register accessor: an alias for `Reg<HC11_INT_SPEC>`"]
pub type HC11_INT = crate::Reg<hc11_int::HC11_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc11_int;
#[doc = "HC11_INTMSK register accessor: an alias for `Reg<HC11_INTMSK_SPEC>`"]
pub type HC11_INTMSK = crate::Reg<hc11_intmsk::HC11_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc11_intmsk;
#[doc = "HC11_TSIZ register accessor: an alias for `Reg<HC11_TSIZ_SPEC>`"]
pub type HC11_TSIZ = crate::Reg<hc11_tsiz::HC11_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc11_tsiz;
#[doc = "HC11_DMAADDR register accessor: an alias for `Reg<HC11_DMAADDR_SPEC>`"]
pub type HC11_DMAADDR = crate::Reg<hc11_dmaaddr::HC11_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc11_dmaaddr;
#[doc = "HC12_CHAR register accessor: an alias for `Reg<HC12_CHAR_SPEC>`"]
pub type HC12_CHAR = crate::Reg<hc12_char::HC12_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc12_char;
#[doc = "HC12_INT register accessor: an alias for `Reg<HC12_INT_SPEC>`"]
pub type HC12_INT = crate::Reg<hc12_int::HC12_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc12_int;
#[doc = "HC12_INTMSK register accessor: an alias for `Reg<HC12_INTMSK_SPEC>`"]
pub type HC12_INTMSK = crate::Reg<hc12_intmsk::HC12_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc12_intmsk;
#[doc = "HC12_TSIZ register accessor: an alias for `Reg<HC12_TSIZ_SPEC>`"]
pub type HC12_TSIZ = crate::Reg<hc12_tsiz::HC12_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc12_tsiz;
#[doc = "HC12_DMAADDR register accessor: an alias for `Reg<HC12_DMAADDR_SPEC>`"]
pub type HC12_DMAADDR = crate::Reg<hc12_dmaaddr::HC12_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc12_dmaaddr;
#[doc = "HC13_CHAR register accessor: an alias for `Reg<HC13_CHAR_SPEC>`"]
pub type HC13_CHAR = crate::Reg<hc13_char::HC13_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc13_char;
#[doc = "HC13_INT register accessor: an alias for `Reg<HC13_INT_SPEC>`"]
pub type HC13_INT = crate::Reg<hc13_int::HC13_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc13_int;
#[doc = "HC13_INTMSK register accessor: an alias for `Reg<HC13_INTMSK_SPEC>`"]
pub type HC13_INTMSK = crate::Reg<hc13_intmsk::HC13_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc13_intmsk;
#[doc = "HC13_TSIZ register accessor: an alias for `Reg<HC13_TSIZ_SPEC>`"]
pub type HC13_TSIZ = crate::Reg<hc13_tsiz::HC13_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc13_tsiz;
#[doc = "HC13_DMAADDR register accessor: an alias for `Reg<HC13_DMAADDR_SPEC>`"]
pub type HC13_DMAADDR = crate::Reg<hc13_dmaaddr::HC13_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc13_dmaaddr;
#[doc = "DCFG register accessor: an alias for `Reg<DCFG_SPEC>`"]
pub type DCFG = crate::Reg<dcfg::DCFG_SPEC>;
#[doc = "Device Configuration Register"]
pub mod dcfg;
#[doc = "DCTL register accessor: an alias for `Reg<DCTL_SPEC>`"]
pub type DCTL = crate::Reg<dctl::DCTL_SPEC>;
#[doc = "Device Control Register"]
pub mod dctl;
#[doc = "DSTS register accessor: an alias for `Reg<DSTS_SPEC>`"]
pub type DSTS = crate::Reg<dsts::DSTS_SPEC>;
#[doc = "Device Status Register"]
pub mod dsts;
#[doc = "DIEPMSK register accessor: an alias for `Reg<DIEPMSK_SPEC>`"]
pub type DIEPMSK = crate::Reg<diepmsk::DIEPMSK_SPEC>;
#[doc = "Device IN Endpoint Common Interrupt Mask Register"]
pub mod diepmsk;
#[doc = "DOEPMSK register accessor: an alias for `Reg<DOEPMSK_SPEC>`"]
pub type DOEPMSK = crate::Reg<doepmsk::DOEPMSK_SPEC>;
#[doc = "Device OUT Endpoint Common Interrupt Mask Register"]
pub mod doepmsk;
#[doc = "DAINT register accessor: an alias for `Reg<DAINT_SPEC>`"]
pub type DAINT = crate::Reg<daint::DAINT_SPEC>;
#[doc = "Device All Endpoints Interrupt Register"]
pub mod daint;
#[doc = "DAINTMSK register accessor: an alias for `Reg<DAINTMSK_SPEC>`"]
pub type DAINTMSK = crate::Reg<daintmsk::DAINTMSK_SPEC>;
#[doc = "Device All Endpoints Interrupt Mask Register"]
pub mod daintmsk;
#[doc = "DVBUSDIS register accessor: an alias for `Reg<DVBUSDIS_SPEC>`"]
pub type DVBUSDIS = crate::Reg<dvbusdis::DVBUSDIS_SPEC>;
#[doc = "Device VBUS Discharge Time Register"]
pub mod dvbusdis;
#[doc = "DVBUSPULSE register accessor: an alias for `Reg<DVBUSPULSE_SPEC>`"]
pub type DVBUSPULSE = crate::Reg<dvbuspulse::DVBUSPULSE_SPEC>;
#[doc = "Device VBUS Pulsing Time Register"]
pub mod dvbuspulse;
#[doc = "DIEPEMPMSK register accessor: an alias for `Reg<DIEPEMPMSK_SPEC>`"]
pub type DIEPEMPMSK = crate::Reg<diepempmsk::DIEPEMPMSK_SPEC>;
#[doc = "Device IN Endpoint FIFO Empty Interrupt Mask Register"]
pub mod diepempmsk;
#[doc = "DIEP0CTL register accessor: an alias for `Reg<DIEP0CTL_SPEC>`"]
pub type DIEP0CTL = crate::Reg<diep0ctl::DIEP0CTL_SPEC>;
#[doc = "Device IN Endpoint 0 Control Register"]
pub mod diep0ctl;
#[doc = "DIEP0INT register accessor: an alias for `Reg<DIEP0INT_SPEC>`"]
pub type DIEP0INT = crate::Reg<diep0int::DIEP0INT_SPEC>;
#[doc = "Device IN Endpoint 0 Interrupt Register"]
pub mod diep0int;
#[doc = "DIEP0TSIZ register accessor: an alias for `Reg<DIEP0TSIZ_SPEC>`"]
pub type DIEP0TSIZ = crate::Reg<diep0tsiz::DIEP0TSIZ_SPEC>;
#[doc = "Device IN Endpoint 0 Transfer Size Register"]
pub mod diep0tsiz;
#[doc = "DIEP0DMAADDR register accessor: an alias for `Reg<DIEP0DMAADDR_SPEC>`"]
pub type DIEP0DMAADDR = crate::Reg<diep0dmaaddr::DIEP0DMAADDR_SPEC>;
#[doc = "Device IN Endpoint 0 DMA Address Register"]
pub mod diep0dmaaddr;
#[doc = "DIEP0TXFSTS register accessor: an alias for `Reg<DIEP0TXFSTS_SPEC>`"]
pub type DIEP0TXFSTS = crate::Reg<diep0txfsts::DIEP0TXFSTS_SPEC>;
#[doc = "Device IN Endpoint 0 Transmit FIFO Status Register"]
pub mod diep0txfsts;
#[doc = "DIEP0_CTL register accessor: an alias for `Reg<DIEP0_CTL_SPEC>`"]
pub type DIEP0_CTL = crate::Reg<diep0_ctl::DIEP0_CTL_SPEC>;
#[doc = "Device IN Endpoint x+1 Control Register"]
pub mod diep0_ctl;
#[doc = "DIEP0_INT register accessor: an alias for `Reg<DIEP0_INT_SPEC>`"]
pub type DIEP0_INT = crate::Reg<diep0_int::DIEP0_INT_SPEC>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep0_int;
#[doc = "DIEP0_TSIZ register accessor: an alias for `Reg<DIEP0_TSIZ_SPEC>`"]
pub type DIEP0_TSIZ = crate::Reg<diep0_tsiz::DIEP0_TSIZ_SPEC>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep0_tsiz;
#[doc = "DIEP0_DMAADDR register accessor: an alias for `Reg<DIEP0_DMAADDR_SPEC>`"]
pub type DIEP0_DMAADDR = crate::Reg<diep0_dmaaddr::DIEP0_DMAADDR_SPEC>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep0_dmaaddr;
#[doc = "DIEP0_TXFSTS register accessor: an alias for `Reg<DIEP0_TXFSTS_SPEC>`"]
pub type DIEP0_TXFSTS = crate::Reg<diep0_txfsts::DIEP0_TXFSTS_SPEC>;
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register"]
pub mod diep0_txfsts;
#[doc = "DIEP1_CTL register accessor: an alias for `Reg<DIEP1_CTL_SPEC>`"]
pub type DIEP1_CTL = crate::Reg<diep1_ctl::DIEP1_CTL_SPEC>;
#[doc = "Device IN Endpoint x+1 Control Register"]
pub mod diep1_ctl;
#[doc = "DIEP1_INT register accessor: an alias for `Reg<DIEP1_INT_SPEC>`"]
pub type DIEP1_INT = crate::Reg<diep1_int::DIEP1_INT_SPEC>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep1_int;
#[doc = "DIEP1_TSIZ register accessor: an alias for `Reg<DIEP1_TSIZ_SPEC>`"]
pub type DIEP1_TSIZ = crate::Reg<diep1_tsiz::DIEP1_TSIZ_SPEC>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep1_tsiz;
#[doc = "DIEP1_DMAADDR register accessor: an alias for `Reg<DIEP1_DMAADDR_SPEC>`"]
pub type DIEP1_DMAADDR = crate::Reg<diep1_dmaaddr::DIEP1_DMAADDR_SPEC>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep1_dmaaddr;
#[doc = "DIEP1_TXFSTS register accessor: an alias for `Reg<DIEP1_TXFSTS_SPEC>`"]
pub type DIEP1_TXFSTS = crate::Reg<diep1_txfsts::DIEP1_TXFSTS_SPEC>;
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register"]
pub mod diep1_txfsts;
#[doc = "DIEP2_CTL register accessor: an alias for `Reg<DIEP2_CTL_SPEC>`"]
pub type DIEP2_CTL = crate::Reg<diep2_ctl::DIEP2_CTL_SPEC>;
#[doc = "Device IN Endpoint x+1 Control Register"]
pub mod diep2_ctl;
#[doc = "DIEP2_INT register accessor: an alias for `Reg<DIEP2_INT_SPEC>`"]
pub type DIEP2_INT = crate::Reg<diep2_int::DIEP2_INT_SPEC>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep2_int;
#[doc = "DIEP2_TSIZ register accessor: an alias for `Reg<DIEP2_TSIZ_SPEC>`"]
pub type DIEP2_TSIZ = crate::Reg<diep2_tsiz::DIEP2_TSIZ_SPEC>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep2_tsiz;
#[doc = "DIEP2_DMAADDR register accessor: an alias for `Reg<DIEP2_DMAADDR_SPEC>`"]
pub type DIEP2_DMAADDR = crate::Reg<diep2_dmaaddr::DIEP2_DMAADDR_SPEC>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep2_dmaaddr;
#[doc = "DIEP2_TXFSTS register accessor: an alias for `Reg<DIEP2_TXFSTS_SPEC>`"]
pub type DIEP2_TXFSTS = crate::Reg<diep2_txfsts::DIEP2_TXFSTS_SPEC>;
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register"]
pub mod diep2_txfsts;
#[doc = "DIEP3_CTL register accessor: an alias for `Reg<DIEP3_CTL_SPEC>`"]
pub type DIEP3_CTL = crate::Reg<diep3_ctl::DIEP3_CTL_SPEC>;
#[doc = "Device IN Endpoint x+1 Control Register"]
pub mod diep3_ctl;
#[doc = "DIEP3_INT register accessor: an alias for `Reg<DIEP3_INT_SPEC>`"]
pub type DIEP3_INT = crate::Reg<diep3_int::DIEP3_INT_SPEC>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep3_int;
#[doc = "DIEP3_TSIZ register accessor: an alias for `Reg<DIEP3_TSIZ_SPEC>`"]
pub type DIEP3_TSIZ = crate::Reg<diep3_tsiz::DIEP3_TSIZ_SPEC>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep3_tsiz;
#[doc = "DIEP3_DMAADDR register accessor: an alias for `Reg<DIEP3_DMAADDR_SPEC>`"]
pub type DIEP3_DMAADDR = crate::Reg<diep3_dmaaddr::DIEP3_DMAADDR_SPEC>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep3_dmaaddr;
#[doc = "DIEP3_TXFSTS register accessor: an alias for `Reg<DIEP3_TXFSTS_SPEC>`"]
pub type DIEP3_TXFSTS = crate::Reg<diep3_txfsts::DIEP3_TXFSTS_SPEC>;
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register"]
pub mod diep3_txfsts;
#[doc = "DIEP4_CTL register accessor: an alias for `Reg<DIEP4_CTL_SPEC>`"]
pub type DIEP4_CTL = crate::Reg<diep4_ctl::DIEP4_CTL_SPEC>;
#[doc = "Device IN Endpoint x+1 Control Register"]
pub mod diep4_ctl;
#[doc = "DIEP4_INT register accessor: an alias for `Reg<DIEP4_INT_SPEC>`"]
pub type DIEP4_INT = crate::Reg<diep4_int::DIEP4_INT_SPEC>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep4_int;
#[doc = "DIEP4_TSIZ register accessor: an alias for `Reg<DIEP4_TSIZ_SPEC>`"]
pub type DIEP4_TSIZ = crate::Reg<diep4_tsiz::DIEP4_TSIZ_SPEC>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep4_tsiz;
#[doc = "DIEP4_DMAADDR register accessor: an alias for `Reg<DIEP4_DMAADDR_SPEC>`"]
pub type DIEP4_DMAADDR = crate::Reg<diep4_dmaaddr::DIEP4_DMAADDR_SPEC>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep4_dmaaddr;
#[doc = "DIEP4_TXFSTS register accessor: an alias for `Reg<DIEP4_TXFSTS_SPEC>`"]
pub type DIEP4_TXFSTS = crate::Reg<diep4_txfsts::DIEP4_TXFSTS_SPEC>;
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register"]
pub mod diep4_txfsts;
#[doc = "DIEP5_CTL register accessor: an alias for `Reg<DIEP5_CTL_SPEC>`"]
pub type DIEP5_CTL = crate::Reg<diep5_ctl::DIEP5_CTL_SPEC>;
#[doc = "Device IN Endpoint x+1 Control Register"]
pub mod diep5_ctl;
#[doc = "DIEP5_INT register accessor: an alias for `Reg<DIEP5_INT_SPEC>`"]
pub type DIEP5_INT = crate::Reg<diep5_int::DIEP5_INT_SPEC>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep5_int;
#[doc = "DIEP5_TSIZ register accessor: an alias for `Reg<DIEP5_TSIZ_SPEC>`"]
pub type DIEP5_TSIZ = crate::Reg<diep5_tsiz::DIEP5_TSIZ_SPEC>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep5_tsiz;
#[doc = "DIEP5_DMAADDR register accessor: an alias for `Reg<DIEP5_DMAADDR_SPEC>`"]
pub type DIEP5_DMAADDR = crate::Reg<diep5_dmaaddr::DIEP5_DMAADDR_SPEC>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep5_dmaaddr;
#[doc = "DIEP5_TXFSTS register accessor: an alias for `Reg<DIEP5_TXFSTS_SPEC>`"]
pub type DIEP5_TXFSTS = crate::Reg<diep5_txfsts::DIEP5_TXFSTS_SPEC>;
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register"]
pub mod diep5_txfsts;
#[doc = "DOEP0CTL register accessor: an alias for `Reg<DOEP0CTL_SPEC>`"]
pub type DOEP0CTL = crate::Reg<doep0ctl::DOEP0CTL_SPEC>;
#[doc = "Device OUT Endpoint 0 Control Register"]
pub mod doep0ctl;
#[doc = "DOEP0INT register accessor: an alias for `Reg<DOEP0INT_SPEC>`"]
pub type DOEP0INT = crate::Reg<doep0int::DOEP0INT_SPEC>;
#[doc = "Device OUT Endpoint 0 Interrupt Register"]
pub mod doep0int;
#[doc = "DOEP0TSIZ register accessor: an alias for `Reg<DOEP0TSIZ_SPEC>`"]
pub type DOEP0TSIZ = crate::Reg<doep0tsiz::DOEP0TSIZ_SPEC>;
#[doc = "Device OUT Endpoint 0 Transfer Size Register"]
pub mod doep0tsiz;
#[doc = "DOEP0DMAADDR register accessor: an alias for `Reg<DOEP0DMAADDR_SPEC>`"]
pub type DOEP0DMAADDR = crate::Reg<doep0dmaaddr::DOEP0DMAADDR_SPEC>;
#[doc = "Device OUT Endpoint 0 DMA Address Register"]
pub mod doep0dmaaddr;
#[doc = "DOEP0_CTL register accessor: an alias for `Reg<DOEP0_CTL_SPEC>`"]
pub type DOEP0_CTL = crate::Reg<doep0_ctl::DOEP0_CTL_SPEC>;
#[doc = "Device OUT Endpoint x+1 Control Register"]
pub mod doep0_ctl;
#[doc = "DOEP0_INT register accessor: an alias for `Reg<DOEP0_INT_SPEC>`"]
pub type DOEP0_INT = crate::Reg<doep0_int::DOEP0_INT_SPEC>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep0_int;
#[doc = "DOEP0_TSIZ register accessor: an alias for `Reg<DOEP0_TSIZ_SPEC>`"]
pub type DOEP0_TSIZ = crate::Reg<doep0_tsiz::DOEP0_TSIZ_SPEC>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep0_tsiz;
#[doc = "DOEP0_DMAADDR register accessor: an alias for `Reg<DOEP0_DMAADDR_SPEC>`"]
pub type DOEP0_DMAADDR = crate::Reg<doep0_dmaaddr::DOEP0_DMAADDR_SPEC>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep0_dmaaddr;
#[doc = "DOEP1_CTL register accessor: an alias for `Reg<DOEP1_CTL_SPEC>`"]
pub type DOEP1_CTL = crate::Reg<doep1_ctl::DOEP1_CTL_SPEC>;
#[doc = "Device OUT Endpoint x+1 Control Register"]
pub mod doep1_ctl;
#[doc = "DOEP1_INT register accessor: an alias for `Reg<DOEP1_INT_SPEC>`"]
pub type DOEP1_INT = crate::Reg<doep1_int::DOEP1_INT_SPEC>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep1_int;
#[doc = "DOEP1_TSIZ register accessor: an alias for `Reg<DOEP1_TSIZ_SPEC>`"]
pub type DOEP1_TSIZ = crate::Reg<doep1_tsiz::DOEP1_TSIZ_SPEC>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep1_tsiz;
#[doc = "DOEP1_DMAADDR register accessor: an alias for `Reg<DOEP1_DMAADDR_SPEC>`"]
pub type DOEP1_DMAADDR = crate::Reg<doep1_dmaaddr::DOEP1_DMAADDR_SPEC>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep1_dmaaddr;
#[doc = "DOEP2_CTL register accessor: an alias for `Reg<DOEP2_CTL_SPEC>`"]
pub type DOEP2_CTL = crate::Reg<doep2_ctl::DOEP2_CTL_SPEC>;
#[doc = "Device OUT Endpoint x+1 Control Register"]
pub mod doep2_ctl;
#[doc = "DOEP2_INT register accessor: an alias for `Reg<DOEP2_INT_SPEC>`"]
pub type DOEP2_INT = crate::Reg<doep2_int::DOEP2_INT_SPEC>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep2_int;
#[doc = "DOEP2_TSIZ register accessor: an alias for `Reg<DOEP2_TSIZ_SPEC>`"]
pub type DOEP2_TSIZ = crate::Reg<doep2_tsiz::DOEP2_TSIZ_SPEC>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep2_tsiz;
#[doc = "DOEP2_DMAADDR register accessor: an alias for `Reg<DOEP2_DMAADDR_SPEC>`"]
pub type DOEP2_DMAADDR = crate::Reg<doep2_dmaaddr::DOEP2_DMAADDR_SPEC>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep2_dmaaddr;
#[doc = "DOEP3_CTL register accessor: an alias for `Reg<DOEP3_CTL_SPEC>`"]
pub type DOEP3_CTL = crate::Reg<doep3_ctl::DOEP3_CTL_SPEC>;
#[doc = "Device OUT Endpoint x+1 Control Register"]
pub mod doep3_ctl;
#[doc = "DOEP3_INT register accessor: an alias for `Reg<DOEP3_INT_SPEC>`"]
pub type DOEP3_INT = crate::Reg<doep3_int::DOEP3_INT_SPEC>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep3_int;
#[doc = "DOEP3_TSIZ register accessor: an alias for `Reg<DOEP3_TSIZ_SPEC>`"]
pub type DOEP3_TSIZ = crate::Reg<doep3_tsiz::DOEP3_TSIZ_SPEC>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep3_tsiz;
#[doc = "DOEP3_DMAADDR register accessor: an alias for `Reg<DOEP3_DMAADDR_SPEC>`"]
pub type DOEP3_DMAADDR = crate::Reg<doep3_dmaaddr::DOEP3_DMAADDR_SPEC>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep3_dmaaddr;
#[doc = "DOEP4_CTL register accessor: an alias for `Reg<DOEP4_CTL_SPEC>`"]
pub type DOEP4_CTL = crate::Reg<doep4_ctl::DOEP4_CTL_SPEC>;
#[doc = "Device OUT Endpoint x+1 Control Register"]
pub mod doep4_ctl;
#[doc = "DOEP4_INT register accessor: an alias for `Reg<DOEP4_INT_SPEC>`"]
pub type DOEP4_INT = crate::Reg<doep4_int::DOEP4_INT_SPEC>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep4_int;
#[doc = "DOEP4_TSIZ register accessor: an alias for `Reg<DOEP4_TSIZ_SPEC>`"]
pub type DOEP4_TSIZ = crate::Reg<doep4_tsiz::DOEP4_TSIZ_SPEC>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep4_tsiz;
#[doc = "DOEP4_DMAADDR register accessor: an alias for `Reg<DOEP4_DMAADDR_SPEC>`"]
pub type DOEP4_DMAADDR = crate::Reg<doep4_dmaaddr::DOEP4_DMAADDR_SPEC>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep4_dmaaddr;
#[doc = "DOEP5_CTL register accessor: an alias for `Reg<DOEP5_CTL_SPEC>`"]
pub type DOEP5_CTL = crate::Reg<doep5_ctl::DOEP5_CTL_SPEC>;
#[doc = "Device OUT Endpoint x+1 Control Register"]
pub mod doep5_ctl;
#[doc = "DOEP5_INT register accessor: an alias for `Reg<DOEP5_INT_SPEC>`"]
pub type DOEP5_INT = crate::Reg<doep5_int::DOEP5_INT_SPEC>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep5_int;
#[doc = "DOEP5_TSIZ register accessor: an alias for `Reg<DOEP5_TSIZ_SPEC>`"]
pub type DOEP5_TSIZ = crate::Reg<doep5_tsiz::DOEP5_TSIZ_SPEC>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep5_tsiz;
#[doc = "DOEP5_DMAADDR register accessor: an alias for `Reg<DOEP5_DMAADDR_SPEC>`"]
pub type DOEP5_DMAADDR = crate::Reg<doep5_dmaaddr::DOEP5_DMAADDR_SPEC>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep5_dmaaddr;
#[doc = "PCGCCTL register accessor: an alias for `Reg<PCGCCTL_SPEC>`"]
pub type PCGCCTL = crate::Reg<pcgcctl::PCGCCTL_SPEC>;
#[doc = "Power and Clock Gating Control Register"]
pub mod pcgcctl;
