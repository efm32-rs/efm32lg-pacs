#[doc = "Register `CACHEMISSES` reader"]
pub struct R(crate::R<CACHEMISSES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHEMISSES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHEMISSES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHEMISSES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CACHEMISSES` reader - Cache misses since last performance counter start command."]
pub type CACHEMISSES_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:19 - Cache misses since last performance counter start command."]
    #[inline(always)]
    pub fn cachemisses(&self) -> CACHEMISSES_R {
        CACHEMISSES_R::new(self.bits & 0x000f_ffff)
    }
}
#[doc = "Cache Misses Performance Counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cachemisses](index.html) module"]
pub struct CACHEMISSES_SPEC;
impl crate::RegisterSpec for CACHEMISSES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cachemisses::R](R) reader structure"]
impl crate::Readable for CACHEMISSES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CACHEMISSES to value 0"]
impl crate::Resettable for CACHEMISSES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
