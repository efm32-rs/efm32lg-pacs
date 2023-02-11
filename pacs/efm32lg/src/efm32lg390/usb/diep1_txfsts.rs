#[doc = "Register `DIEP1_TXFSTS` reader"]
pub struct R(crate::R<DIEP1_TXFSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEP1_TXFSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEP1_TXFSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEP1_TXFSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SPCAVAIL` reader - TxFIFO Space Available"]
pub type SPCAVAIL_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - TxFIFO Space Available"]
    #[inline(always)]
    pub fn spcavail(&self) -> SPCAVAIL_R {
        SPCAVAIL_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep1_txfsts](index.html) module"]
pub struct DIEP1_TXFSTS_SPEC;
impl crate::RegisterSpec for DIEP1_TXFSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diep1_txfsts::R](R) reader structure"]
impl crate::Readable for DIEP1_TXFSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DIEP1_TXFSTS to value 0x0200"]
impl crate::Resettable for DIEP1_TXFSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
