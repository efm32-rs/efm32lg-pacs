#[doc = "Register `ETMCIDR1` reader"]
pub struct R(crate::R<ETMCIDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETMCIDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETMCIDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETMCIDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PREAMB` reader - CoreSight Preamble"]
pub type PREAMB_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - CoreSight Preamble"]
    #[inline(always)]
    pub fn preamb(&self) -> PREAMB_R {
        PREAMB_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Component ID1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmcidr1](index.html) module"]
pub struct ETMCIDR1_SPEC;
impl crate::RegisterSpec for ETMCIDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etmcidr1::R](R) reader structure"]
impl crate::Readable for ETMCIDR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETMCIDR1 to value 0x90"]
impl crate::Resettable for ETMCIDR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x90
    }
}
