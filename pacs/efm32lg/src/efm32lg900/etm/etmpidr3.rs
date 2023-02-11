#[doc = "Register `ETMPIDR3` reader"]
pub struct R(crate::R<ETMPIDR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETMPIDR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETMPIDR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETMPIDR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CUSTMOD` reader - Customer Modified"]
pub type CUSTMOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REVAND` reader - RevAnd"]
pub type REVAND_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Customer Modified"]
    #[inline(always)]
    pub fn custmod(&self) -> CUSTMOD_R {
        CUSTMOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - RevAnd"]
    #[inline(always)]
    pub fn revand(&self) -> REVAND_R {
        REVAND_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral ID3 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmpidr3](index.html) module"]
pub struct ETMPIDR3_SPEC;
impl crate::RegisterSpec for ETMPIDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etmpidr3::R](R) reader structure"]
impl crate::Readable for ETMPIDR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETMPIDR3 to value 0"]
impl crate::Resettable for ETMPIDR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
