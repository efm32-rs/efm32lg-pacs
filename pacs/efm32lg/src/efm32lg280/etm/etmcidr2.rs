#[doc = "Register `ETMCIDR2` reader"]
pub struct R(crate::R<ETMCIDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETMCIDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETMCIDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETMCIDR2_SPEC>) -> Self {
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
#[doc = "Component ID2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmcidr2](index.html) module"]
pub struct ETMCIDR2_SPEC;
impl crate::RegisterSpec for ETMCIDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etmcidr2::R](R) reader structure"]
impl crate::Readable for ETMCIDR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETMCIDR2 to value 0x05"]
impl crate::Resettable for ETMCIDR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
