#[doc = "Register `ETMLSR` reader"]
pub struct R(crate::R<ETMLSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETMLSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETMLSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETMLSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LOCKIMP` reader - ETM Locking Implemented"]
pub type LOCKIMP_R = crate::BitReader<bool>;
#[doc = "Field `LOCKED` reader - ETM locked"]
pub type LOCKED_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - ETM Locking Implemented"]
    #[inline(always)]
    pub fn lockimp(&self) -> LOCKIMP_R {
        LOCKIMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ETM locked"]
    #[inline(always)]
    pub fn locked(&self) -> LOCKED_R {
        LOCKED_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Lock Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmlsr](index.html) module"]
pub struct ETMLSR_SPEC;
impl crate::RegisterSpec for ETMLSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etmlsr::R](R) reader structure"]
impl crate::Readable for ETMLSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETMLSR to value 0x03"]
impl crate::Resettable for ETMLSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
