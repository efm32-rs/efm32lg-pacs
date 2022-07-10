#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VCMPACT` reader - Voltage Supply Comparator Active"]
pub type VCMPACT_R = crate::BitReader<bool>;
#[doc = "Field `VCMPOUT` reader - Voltage Supply Comparator Output"]
pub type VCMPOUT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Voltage Supply Comparator Active"]
    #[inline(always)]
    pub fn vcmpact(&self) -> VCMPACT_R {
        VCMPACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Voltage Supply Comparator Output"]
    #[inline(always)]
    pub fn vcmpout(&self) -> VCMPOUT_R {
        VCMPOUT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
