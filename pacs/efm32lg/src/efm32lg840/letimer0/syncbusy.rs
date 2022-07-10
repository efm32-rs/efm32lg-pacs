#[doc = "Register `SYNCBUSY` reader"]
pub struct R(crate::R<SYNCBUSY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCBUSY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCBUSY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCBUSY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CTRL` reader - CTRL Register Busy"]
pub type CTRL_R = crate::BitReader<bool>;
#[doc = "Field `CMD` reader - CMD Register Busy"]
pub type CMD_R = crate::BitReader<bool>;
#[doc = "Field `COMP0` reader - COMP0 Register Busy"]
pub type COMP0_R = crate::BitReader<bool>;
#[doc = "Field `COMP1` reader - COMP1 Register Busy"]
pub type COMP1_R = crate::BitReader<bool>;
#[doc = "Field `REP0` reader - REP0 Register Busy"]
pub type REP0_R = crate::BitReader<bool>;
#[doc = "Field `REP1` reader - REP1 Register Busy"]
pub type REP1_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - CTRL Register Busy"]
    #[inline(always)]
    pub fn ctrl(&self) -> CTRL_R {
        CTRL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMD Register Busy"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - COMP0 Register Busy"]
    #[inline(always)]
    pub fn comp0(&self) -> COMP0_R {
        COMP0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - COMP1 Register Busy"]
    #[inline(always)]
    pub fn comp1(&self) -> COMP1_R {
        COMP1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - REP0 Register Busy"]
    #[inline(always)]
    pub fn rep0(&self) -> REP0_R {
        REP0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - REP1 Register Busy"]
    #[inline(always)]
    pub fn rep1(&self) -> REP1_R {
        REP1_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Synchronization Busy Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncbusy](index.html) module"]
pub struct SYNCBUSY_SPEC;
impl crate::RegisterSpec for SYNCBUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syncbusy::R](R) reader structure"]
impl crate::Readable for SYNCBUSY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SYNCBUSY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
