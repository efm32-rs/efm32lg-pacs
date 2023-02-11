#[doc = "Register `ETMPIDR4` reader"]
pub struct R(crate::R<ETMPIDR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETMPIDR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETMPIDR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETMPIDR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CONTCODE` reader - JEP106 Continuation Code"]
pub type CONTCODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COUNT` reader - 4KB Count"]
pub type COUNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - JEP106 Continuation Code"]
    #[inline(always)]
    pub fn contcode(&self) -> CONTCODE_R {
        CONTCODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 4KB Count"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral ID4 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmpidr4](index.html) module"]
pub struct ETMPIDR4_SPEC;
impl crate::RegisterSpec for ETMPIDR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etmpidr4::R](R) reader structure"]
impl crate::Readable for ETMPIDR4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETMPIDR4 to value 0x04"]
impl crate::Resettable for ETMPIDR4_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
