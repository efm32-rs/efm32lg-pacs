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
#[doc = "Field `BUFDATAV` reader - Result data valid"]
pub type BUFDATAV_R = crate::BitReader<bool>;
#[doc = "Field `BUFHALFFULL` reader - Result buffer half full"]
pub type BUFHALFFULL_R = crate::BitReader<bool>;
#[doc = "Field `BUFFULL` reader - Result buffer full"]
pub type BUFFULL_R = crate::BitReader<bool>;
#[doc = "Field `RUNNING` reader - LESENSE is active"]
pub type RUNNING_R = crate::BitReader<bool>;
#[doc = "Field `SCANACTIVE` reader - LESENSE is currently interfacing sensors."]
pub type SCANACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `DACACTIVE` reader - LESENSE DAC interface is active"]
pub type DACACTIVE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Result data valid"]
    #[inline(always)]
    pub fn bufdatav(&self) -> BUFDATAV_R {
        BUFDATAV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Result buffer half full"]
    #[inline(always)]
    pub fn bufhalffull(&self) -> BUFHALFFULL_R {
        BUFHALFFULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Result buffer full"]
    #[inline(always)]
    pub fn buffull(&self) -> BUFFULL_R {
        BUFFULL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LESENSE is active"]
    #[inline(always)]
    pub fn running(&self) -> RUNNING_R {
        RUNNING_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LESENSE is currently interfacing sensors."]
    #[inline(always)]
    pub fn scanactive(&self) -> SCANACTIVE_R {
        SCANACTIVE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LESENSE DAC interface is active"]
    #[inline(always)]
    pub fn dacactive(&self) -> DACACTIVE_R {
        DACACTIVE_R::new(((self.bits >> 5) & 1) != 0)
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
