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
#[doc = "Field `CTRL` reader - LESENSE_CTRL Register Busy"]
pub type CTRL_R = crate::BitReader<bool>;
#[doc = "Field `TIMCTRL` reader - LESENSE_TIMCTRL Register Busy"]
pub type TIMCTRL_R = crate::BitReader<bool>;
#[doc = "Field `PERCTRL` reader - LESENSE_PERCTRL Register Busy"]
pub type PERCTRL_R = crate::BitReader<bool>;
#[doc = "Field `DECCTRL` reader - LESENSE_DECCTRL Register Busy"]
pub type DECCTRL_R = crate::BitReader<bool>;
#[doc = "Field `BIASCTRL` reader - LESENSE_BIASCTRL Register Busy"]
pub type BIASCTRL_R = crate::BitReader<bool>;
#[doc = "Field `CMD` reader - LESENSE_CMD Register Busy"]
pub type CMD_R = crate::BitReader<bool>;
#[doc = "Field `CHEN` reader - LESENSE_CHEN Register Busy"]
pub type CHEN_R = crate::BitReader<bool>;
#[doc = "Field `SCANRES` reader - LESENSE_SCANRES Register Busy"]
pub type SCANRES_R = crate::BitReader<bool>;
#[doc = "Field `STATUS` reader - LESENSE_STATUS Register Busy"]
pub type STATUS_R = crate::BitReader<bool>;
#[doc = "Field `PTR` reader - LESENSE_PTR Register Busy"]
pub type PTR_R = crate::BitReader<bool>;
#[doc = "Field `BUFDATA` reader - LESENSE_BUFDATA Register Busy"]
pub type BUFDATA_R = crate::BitReader<bool>;
#[doc = "Field `CURCH` reader - LESENSE_CURCH Register Busy"]
pub type CURCH_R = crate::BitReader<bool>;
#[doc = "Field `DECSTATE` reader - LESENSE_DECSTATE Register Busy"]
pub type DECSTATE_R = crate::BitReader<bool>;
#[doc = "Field `SENSORSTATE` reader - LESENSE_SENSORSTATE Register Busy"]
pub type SENSORSTATE_R = crate::BitReader<bool>;
#[doc = "Field `IDLECONF` reader - LESENSE_IDLECONF Register Busy"]
pub type IDLECONF_R = crate::BitReader<bool>;
#[doc = "Field `ALTEXCONF` reader - LESENSE_ALTEXCONF Register Busy"]
pub type ALTEXCONF_R = crate::BitReader<bool>;
#[doc = "Field `ROUTE` reader - LESENSE_ROUTE Register Busy"]
pub type ROUTE_R = crate::BitReader<bool>;
#[doc = "Field `POWERDOWN` reader - LESENSE_POWERDOWN Register Busy"]
pub type POWERDOWN_R = crate::BitReader<bool>;
#[doc = "Field `TCONFA` reader - LESENSE_STx_TCONFA Register Busy"]
pub type TCONFA_R = crate::BitReader<bool>;
#[doc = "Field `TCONFB` reader - LESENSE_STx_TCONFB Register Busy"]
pub type TCONFB_R = crate::BitReader<bool>;
#[doc = "Field `DATA` reader - LESENSE_BUFx_DATA Register Busy"]
pub type DATA_R = crate::BitReader<bool>;
#[doc = "Field `TIMING` reader - LESENSE_CHx_TIMING Register Busy"]
pub type TIMING_R = crate::BitReader<bool>;
#[doc = "Field `INTERACT` reader - LESENSE_CHx_INTERACT Register Busy"]
pub type INTERACT_R = crate::BitReader<bool>;
#[doc = "Field `EVAL` reader - LESENSE_CHx_EVAL Register Busy"]
pub type EVAL_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - LESENSE_CTRL Register Busy"]
    #[inline(always)]
    pub fn ctrl(&self) -> CTRL_R {
        CTRL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LESENSE_TIMCTRL Register Busy"]
    #[inline(always)]
    pub fn timctrl(&self) -> TIMCTRL_R {
        TIMCTRL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LESENSE_PERCTRL Register Busy"]
    #[inline(always)]
    pub fn perctrl(&self) -> PERCTRL_R {
        PERCTRL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LESENSE_DECCTRL Register Busy"]
    #[inline(always)]
    pub fn decctrl(&self) -> DECCTRL_R {
        DECCTRL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LESENSE_BIASCTRL Register Busy"]
    #[inline(always)]
    pub fn biasctrl(&self) -> BIASCTRL_R {
        BIASCTRL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LESENSE_CMD Register Busy"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LESENSE_CHEN Register Busy"]
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LESENSE_SCANRES Register Busy"]
    #[inline(always)]
    pub fn scanres(&self) -> SCANRES_R {
        SCANRES_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LESENSE_STATUS Register Busy"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LESENSE_PTR Register Busy"]
    #[inline(always)]
    pub fn ptr(&self) -> PTR_R {
        PTR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LESENSE_BUFDATA Register Busy"]
    #[inline(always)]
    pub fn bufdata(&self) -> BUFDATA_R {
        BUFDATA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LESENSE_CURCH Register Busy"]
    #[inline(always)]
    pub fn curch(&self) -> CURCH_R {
        CURCH_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LESENSE_DECSTATE Register Busy"]
    #[inline(always)]
    pub fn decstate(&self) -> DECSTATE_R {
        DECSTATE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LESENSE_SENSORSTATE Register Busy"]
    #[inline(always)]
    pub fn sensorstate(&self) -> SENSORSTATE_R {
        SENSORSTATE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LESENSE_IDLECONF Register Busy"]
    #[inline(always)]
    pub fn idleconf(&self) -> IDLECONF_R {
        IDLECONF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LESENSE_ALTEXCONF Register Busy"]
    #[inline(always)]
    pub fn altexconf(&self) -> ALTEXCONF_R {
        ALTEXCONF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - LESENSE_ROUTE Register Busy"]
    #[inline(always)]
    pub fn route(&self) -> ROUTE_R {
        ROUTE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - LESENSE_POWERDOWN Register Busy"]
    #[inline(always)]
    pub fn powerdown(&self) -> POWERDOWN_R {
        POWERDOWN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - LESENSE_STx_TCONFA Register Busy"]
    #[inline(always)]
    pub fn tconfa(&self) -> TCONFA_R {
        TCONFA_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - LESENSE_STx_TCONFB Register Busy"]
    #[inline(always)]
    pub fn tconfb(&self) -> TCONFB_R {
        TCONFB_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - LESENSE_BUFx_DATA Register Busy"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - LESENSE_CHx_TIMING Register Busy"]
    #[inline(always)]
    pub fn timing(&self) -> TIMING_R {
        TIMING_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - LESENSE_CHx_INTERACT Register Busy"]
    #[inline(always)]
    pub fn interact(&self) -> INTERACT_R {
        INTERACT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - LESENSE_CHx_EVAL Register Busy"]
    #[inline(always)]
    pub fn eval(&self) -> EVAL_R {
        EVAL_R::new(((self.bits >> 26) & 1) != 0)
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
