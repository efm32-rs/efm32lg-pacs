#[doc = "Register `CHWAITSTATUS` reader"]
pub struct R(crate::R<CHWAITSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHWAITSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHWAITSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHWAITSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CH0WAITSTATUS` reader - Channel 0 Wait on Request Status"]
pub type CH0WAITSTATUS_R = crate::BitReader<bool>;
#[doc = "Field `CH1WAITSTATUS` reader - Channel 1 Wait on Request Status"]
pub type CH1WAITSTATUS_R = crate::BitReader<bool>;
#[doc = "Field `CH2WAITSTATUS` reader - Channel 2 Wait on Request Status"]
pub type CH2WAITSTATUS_R = crate::BitReader<bool>;
#[doc = "Field `CH3WAITSTATUS` reader - Channel 3 Wait on Request Status"]
pub type CH3WAITSTATUS_R = crate::BitReader<bool>;
#[doc = "Field `CH4WAITSTATUS` reader - Channel 4 Wait on Request Status"]
pub type CH4WAITSTATUS_R = crate::BitReader<bool>;
#[doc = "Field `CH5WAITSTATUS` reader - Channel 5 Wait on Request Status"]
pub type CH5WAITSTATUS_R = crate::BitReader<bool>;
#[doc = "Field `CH6WAITSTATUS` reader - Channel 6 Wait on Request Status"]
pub type CH6WAITSTATUS_R = crate::BitReader<bool>;
#[doc = "Field `CH7WAITSTATUS` reader - Channel 7 Wait on Request Status"]
pub type CH7WAITSTATUS_R = crate::BitReader<bool>;
#[doc = "Field `CH8WAITSTATUS` reader - Channel 8 Wait on Request Status"]
pub type CH8WAITSTATUS_R = crate::BitReader<bool>;
#[doc = "Field `CH9WAITSTATUS` reader - Channel 9 Wait on Request Status"]
pub type CH9WAITSTATUS_R = crate::BitReader<bool>;
#[doc = "Field `CH10WAITSTATUS` reader - Channel 10 Wait on Request Status"]
pub type CH10WAITSTATUS_R = crate::BitReader<bool>;
#[doc = "Field `CH11WAITSTATUS` reader - Channel 11 Wait on Request Status"]
pub type CH11WAITSTATUS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Channel 0 Wait on Request Status"]
    #[inline(always)]
    pub fn ch0waitstatus(&self) -> CH0WAITSTATUS_R {
        CH0WAITSTATUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Wait on Request Status"]
    #[inline(always)]
    pub fn ch1waitstatus(&self) -> CH1WAITSTATUS_R {
        CH1WAITSTATUS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Wait on Request Status"]
    #[inline(always)]
    pub fn ch2waitstatus(&self) -> CH2WAITSTATUS_R {
        CH2WAITSTATUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Wait on Request Status"]
    #[inline(always)]
    pub fn ch3waitstatus(&self) -> CH3WAITSTATUS_R {
        CH3WAITSTATUS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Wait on Request Status"]
    #[inline(always)]
    pub fn ch4waitstatus(&self) -> CH4WAITSTATUS_R {
        CH4WAITSTATUS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Wait on Request Status"]
    #[inline(always)]
    pub fn ch5waitstatus(&self) -> CH5WAITSTATUS_R {
        CH5WAITSTATUS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Wait on Request Status"]
    #[inline(always)]
    pub fn ch6waitstatus(&self) -> CH6WAITSTATUS_R {
        CH6WAITSTATUS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Wait on Request Status"]
    #[inline(always)]
    pub fn ch7waitstatus(&self) -> CH7WAITSTATUS_R {
        CH7WAITSTATUS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 8 Wait on Request Status"]
    #[inline(always)]
    pub fn ch8waitstatus(&self) -> CH8WAITSTATUS_R {
        CH8WAITSTATUS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 9 Wait on Request Status"]
    #[inline(always)]
    pub fn ch9waitstatus(&self) -> CH9WAITSTATUS_R {
        CH9WAITSTATUS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 10 Wait on Request Status"]
    #[inline(always)]
    pub fn ch10waitstatus(&self) -> CH10WAITSTATUS_R {
        CH10WAITSTATUS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 11 Wait on Request Status"]
    #[inline(always)]
    pub fn ch11waitstatus(&self) -> CH11WAITSTATUS_R {
        CH11WAITSTATUS_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Channel Wait on Request Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chwaitstatus](index.html) module"]
pub struct CHWAITSTATUS_SPEC;
impl crate::RegisterSpec for CHWAITSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chwaitstatus::R](R) reader structure"]
impl crate::Readable for CHWAITSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CHWAITSTATUS to value 0x0fff"]
impl crate::Resettable for CHWAITSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff;
}
