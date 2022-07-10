#[doc = "Register `CHSREQSTATUS` reader"]
pub struct R(crate::R<CHSREQSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHSREQSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHSREQSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHSREQSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CH0SREQSTATUS` reader - Channel 0 Single Request Status"]
pub type CH0SREQSTATUS_R = crate::BitReader<bool>;
#[doc = "Field `CH1SREQSTATUS` reader - Channel 1 Single Request Status"]
pub type CH1SREQSTATUS_R = crate::BitReader<bool>;
#[doc = "Field `CH2SREQSTATUS` reader - Channel 2 Single Request Status"]
pub type CH2SREQSTATUS_R = crate::BitReader<bool>;
#[doc = "Field `CH3SREQSTATUS` reader - Channel 3 Single Request Status"]
pub type CH3SREQSTATUS_R = crate::BitReader<bool>;
#[doc = "Field `CH4SREQSTATUS` reader - Channel 4 Single Request Status"]
pub type CH4SREQSTATUS_R = crate::BitReader<bool>;
#[doc = "Field `CH5SREQSTATUS` reader - Channel 5 Single Request Status"]
pub type CH5SREQSTATUS_R = crate::BitReader<bool>;
#[doc = "Field `CH6SREQSTATUS` reader - Channel 6 Single Request Status"]
pub type CH6SREQSTATUS_R = crate::BitReader<bool>;
#[doc = "Field `CH7SREQSTATUS` reader - Channel 7 Single Request Status"]
pub type CH7SREQSTATUS_R = crate::BitReader<bool>;
#[doc = "Field `CH8SREQSTATUS` reader - Channel 8 Single Request Status"]
pub type CH8SREQSTATUS_R = crate::BitReader<bool>;
#[doc = "Field `CH9SREQSTATUS` reader - Channel 9 Single Request Status"]
pub type CH9SREQSTATUS_R = crate::BitReader<bool>;
#[doc = "Field `CH10SREQSTATUS` reader - Channel 10 Single Request Status"]
pub type CH10SREQSTATUS_R = crate::BitReader<bool>;
#[doc = "Field `CH11SREQSTATUS` reader - Channel 11 Single Request Status"]
pub type CH11SREQSTATUS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Channel 0 Single Request Status"]
    #[inline(always)]
    pub fn ch0sreqstatus(&self) -> CH0SREQSTATUS_R {
        CH0SREQSTATUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Single Request Status"]
    #[inline(always)]
    pub fn ch1sreqstatus(&self) -> CH1SREQSTATUS_R {
        CH1SREQSTATUS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Single Request Status"]
    #[inline(always)]
    pub fn ch2sreqstatus(&self) -> CH2SREQSTATUS_R {
        CH2SREQSTATUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Single Request Status"]
    #[inline(always)]
    pub fn ch3sreqstatus(&self) -> CH3SREQSTATUS_R {
        CH3SREQSTATUS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Single Request Status"]
    #[inline(always)]
    pub fn ch4sreqstatus(&self) -> CH4SREQSTATUS_R {
        CH4SREQSTATUS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Single Request Status"]
    #[inline(always)]
    pub fn ch5sreqstatus(&self) -> CH5SREQSTATUS_R {
        CH5SREQSTATUS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Single Request Status"]
    #[inline(always)]
    pub fn ch6sreqstatus(&self) -> CH6SREQSTATUS_R {
        CH6SREQSTATUS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Single Request Status"]
    #[inline(always)]
    pub fn ch7sreqstatus(&self) -> CH7SREQSTATUS_R {
        CH7SREQSTATUS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 8 Single Request Status"]
    #[inline(always)]
    pub fn ch8sreqstatus(&self) -> CH8SREQSTATUS_R {
        CH8SREQSTATUS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 9 Single Request Status"]
    #[inline(always)]
    pub fn ch9sreqstatus(&self) -> CH9SREQSTATUS_R {
        CH9SREQSTATUS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 10 Single Request Status"]
    #[inline(always)]
    pub fn ch10sreqstatus(&self) -> CH10SREQSTATUS_R {
        CH10SREQSTATUS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 11 Single Request Status"]
    #[inline(always)]
    pub fn ch11sreqstatus(&self) -> CH11SREQSTATUS_R {
        CH11SREQSTATUS_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Channel Single Request Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chsreqstatus](index.html) module"]
pub struct CHSREQSTATUS_SPEC;
impl crate::RegisterSpec for CHSREQSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chsreqstatus::R](R) reader structure"]
impl crate::Readable for CHSREQSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CHSREQSTATUS to value 0"]
impl crate::Resettable for CHSREQSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
