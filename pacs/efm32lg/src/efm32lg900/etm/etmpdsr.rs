#[doc = "Register `ETMPDSR` reader"]
pub struct R(crate::R<ETMPDSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETMPDSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETMPDSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETMPDSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ETMUP` reader - ETM Powered Up"]
pub type ETMUP_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - ETM Powered Up"]
    #[inline(always)]
    pub fn etmup(&self) -> ETMUP_R {
        ETMUP_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Device Power-down Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmpdsr](index.html) module"]
pub struct ETMPDSR_SPEC;
impl crate::RegisterSpec for ETMPDSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etmpdsr::R](R) reader structure"]
impl crate::Readable for ETMPDSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETMPDSR to value 0x01"]
impl crate::Resettable for ETMPDSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
