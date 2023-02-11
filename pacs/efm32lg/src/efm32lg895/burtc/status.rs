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
#[doc = "Field `LPMODEACT` reader - Low power mode active"]
pub type LPMODEACT_R = crate::BitReader<bool>;
#[doc = "Field `BUMODETS` reader - Timestamp for backup mode entry stored."]
pub type BUMODETS_R = crate::BitReader<bool>;
#[doc = "Field `RAMWERR` reader - RAM write error."]
pub type RAMWERR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Low power mode active"]
    #[inline(always)]
    pub fn lpmodeact(&self) -> LPMODEACT_R {
        LPMODEACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timestamp for backup mode entry stored."]
    #[inline(always)]
    pub fn bumodets(&self) -> BUMODETS_R {
        BUMODETS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RAM write error."]
    #[inline(always)]
    pub fn ramwerr(&self) -> RAMWERR_R {
        RAMWERR_R::new(((self.bits >> 2) & 1) != 0)
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
    const RESET_VALUE: Self::Ux = 0;
}
