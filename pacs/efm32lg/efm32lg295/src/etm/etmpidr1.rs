#[doc = "Register `ETMPIDR1` reader"]
pub struct R(crate::R<ETMPIDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETMPIDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETMPIDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETMPIDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PARTNUM` reader - Part Number"]
pub type PARTNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IDCODE` reader - JEP106 Identity Code"]
pub type IDCODE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Part Number"]
    #[inline(always)]
    pub fn partnum(&self) -> PARTNUM_R {
        PARTNUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - JEP106 Identity Code"]
    #[inline(always)]
    pub fn idcode(&self) -> IDCODE_R {
        IDCODE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral ID1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmpidr1](index.html) module"]
pub struct ETMPIDR1_SPEC;
impl crate::RegisterSpec for ETMPIDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etmpidr1::R](R) reader structure"]
impl crate::Readable for ETMPIDR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETMPIDR1 to value 0xb9"]
impl crate::Resettable for ETMPIDR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xb9
    }
}
