#[doc = "Register `ETMPIDR2` reader"]
pub struct R(crate::R<ETMPIDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETMPIDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETMPIDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETMPIDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IDCODE` reader - JEP106 Identity Code"]
pub type IDCODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALWAYS1` reader - Always 1"]
pub type ALWAYS1_R = crate::BitReader<bool>;
#[doc = "Field `REV` reader - Revision"]
pub type REV_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - JEP106 Identity Code"]
    #[inline(always)]
    pub fn idcode(&self) -> IDCODE_R {
        IDCODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Always 1"]
    #[inline(always)]
    pub fn always1(&self) -> ALWAYS1_R {
        ALWAYS1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Revision"]
    #[inline(always)]
    pub fn rev(&self) -> REV_R {
        REV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral ID2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmpidr2](index.html) module"]
pub struct ETMPIDR2_SPEC;
impl crate::RegisterSpec for ETMPIDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etmpidr2::R](R) reader structure"]
impl crate::Readable for ETMPIDR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETMPIDR2 to value 0x3b"]
impl crate::Resettable for ETMPIDR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3b
    }
}
