#[doc = "Register `IF` reader"]
pub struct R(crate::R<IF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VREGOSH` reader - VREGO Sense High Interrupt Flag"]
pub type VREGOSH_R = crate::BitReader<bool>;
#[doc = "Field `VREGOSL` reader - VREGO Sense Low Interrupt Flag"]
pub type VREGOSL_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - VREGO Sense High Interrupt Flag"]
    #[inline(always)]
    pub fn vregosh(&self) -> VREGOSH_R {
        VREGOSH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VREGO Sense Low Interrupt Flag"]
    #[inline(always)]
    pub fn vregosl(&self) -> VREGOSL_R {
        VREGOSL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](index.html) module"]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if_::R](R) reader structure"]
impl crate::Readable for IF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IF to value 0x03"]
impl crate::Resettable for IF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
