#[doc = "Register `ETMDEVTYPE` reader"]
pub struct R(crate::R<ETMDEVTYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETMDEVTYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETMDEVTYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETMDEVTYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TRACESRC` reader - Trace Source"]
pub type TRACESRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PROCTRACE` reader - Processor Trace"]
pub type PROCTRACE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Trace Source"]
    #[inline(always)]
    pub fn tracesrc(&self) -> TRACESRC_R {
        TRACESRC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Processor Trace"]
    #[inline(always)]
    pub fn proctrace(&self) -> PROCTRACE_R {
        PROCTRACE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "CoreSight Device Type Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmdevtype](index.html) module"]
pub struct ETMDEVTYPE_SPEC;
impl crate::RegisterSpec for ETMDEVTYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etmdevtype::R](R) reader structure"]
impl crate::Readable for ETMDEVTYPE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETMDEVTYPE to value 0x13"]
impl crate::Resettable for ETMDEVTYPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x13
    }
}
