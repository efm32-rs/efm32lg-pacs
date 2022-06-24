#[doc = "Register `IF` reader"]
pub struct R(crate::R<IF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXC` reader - TX Complete Interrupt Flag"]
pub type TXC_R = crate::BitReader<bool>;
#[doc = "Field `TXBL` reader - TX Buffer Level Interrupt Flag"]
pub type TXBL_R = crate::BitReader<bool>;
#[doc = "Field `RXDATAV` reader - RX Data Valid Interrupt Flag"]
pub type RXDATAV_R = crate::BitReader<bool>;
#[doc = "Field `RXFULL` reader - RX Buffer Full Interrupt Flag"]
pub type RXFULL_R = crate::BitReader<bool>;
#[doc = "Field `RXOF` reader - RX Overflow Interrupt Flag"]
pub type RXOF_R = crate::BitReader<bool>;
#[doc = "Field `RXUF` reader - RX Underflow Interrupt Flag"]
pub type RXUF_R = crate::BitReader<bool>;
#[doc = "Field `TXOF` reader - TX Overflow Interrupt Flag"]
pub type TXOF_R = crate::BitReader<bool>;
#[doc = "Field `TXUF` reader - TX Underflow Interrupt Flag"]
pub type TXUF_R = crate::BitReader<bool>;
#[doc = "Field `PERR` reader - Parity Error Interrupt Flag"]
pub type PERR_R = crate::BitReader<bool>;
#[doc = "Field `FERR` reader - Framing Error Interrupt Flag"]
pub type FERR_R = crate::BitReader<bool>;
#[doc = "Field `MPAF` reader - Multi-Processor Address Frame Interrupt Flag"]
pub type MPAF_R = crate::BitReader<bool>;
#[doc = "Field `SSM` reader - Slave-Select In Master Mode Interrupt Flag"]
pub type SSM_R = crate::BitReader<bool>;
#[doc = "Field `CCF` reader - Collision Check Fail Interrupt Flag"]
pub type CCF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - TX Complete Interrupt Flag"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX Buffer Level Interrupt Flag"]
    #[inline(always)]
    pub fn txbl(&self) -> TXBL_R {
        TXBL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Data Valid Interrupt Flag"]
    #[inline(always)]
    pub fn rxdatav(&self) -> RXDATAV_R {
        RXDATAV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX Buffer Full Interrupt Flag"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RX Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxof(&self) -> RXOF_R {
        RXOF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RX Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxuf(&self) -> RXUF_R {
        RXUF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn txof(&self) -> TXOF_R {
        TXOF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TX Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn txuf(&self) -> TXUF_R {
        TXUF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity Error Interrupt Flag"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Framing Error Interrupt Flag"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Multi-Processor Address Frame Interrupt Flag"]
    #[inline(always)]
    pub fn mpaf(&self) -> MPAF_R {
        MPAF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Slave-Select In Master Mode Interrupt Flag"]
    #[inline(always)]
    pub fn ssm(&self) -> SSM_R {
        SSM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Collision Check Fail Interrupt Flag"]
    #[inline(always)]
    pub fn ccf(&self) -> CCF_R {
        CCF_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](index.html) module"]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if_::R](R) reader structure"]
impl crate::Readable for IF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IF to value 0x02"]
impl crate::Resettable for IF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
