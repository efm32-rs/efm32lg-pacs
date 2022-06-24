#[doc = "Register `CHUSEBURSTC` writer"]
pub struct W(crate::W<CHUSEBURSTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHUSEBURSTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CHUSEBURSTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHUSEBURSTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0USEBURSTC` writer - Channel 0 Useburst Clear"]
pub type CH0USEBURSTC_W<'a> = crate::BitWriter<'a, u32, CHUSEBURSTC_SPEC, bool, 0>;
#[doc = "Field `CH1USEBURSTC` writer - Channel 1 Useburst Clear"]
pub type CH1USEBURSTC_W<'a> = crate::BitWriter<'a, u32, CHUSEBURSTC_SPEC, bool, 1>;
#[doc = "Field `CH2USEBURSTC` writer - Channel 2 Useburst Clear"]
pub type CH2USEBURSTC_W<'a> = crate::BitWriter<'a, u32, CHUSEBURSTC_SPEC, bool, 2>;
#[doc = "Field `CH3USEBURSTC` writer - Channel 3 Useburst Clear"]
pub type CH3USEBURSTC_W<'a> = crate::BitWriter<'a, u32, CHUSEBURSTC_SPEC, bool, 3>;
#[doc = "Field `CH4USEBURSTC` writer - Channel 4 Useburst Clear"]
pub type CH4USEBURSTC_W<'a> = crate::BitWriter<'a, u32, CHUSEBURSTC_SPEC, bool, 4>;
#[doc = "Field `CH5USEBURSTC` writer - Channel 5 Useburst Clear"]
pub type CH5USEBURSTC_W<'a> = crate::BitWriter<'a, u32, CHUSEBURSTC_SPEC, bool, 5>;
#[doc = "Field `CH6USEBURSTC` writer - Channel 6 Useburst Clear"]
pub type CH6USEBURSTC_W<'a> = crate::BitWriter<'a, u32, CHUSEBURSTC_SPEC, bool, 6>;
#[doc = "Field `CH7USEBURSTC` writer - Channel 7 Useburst Clear"]
pub type CH7USEBURSTC_W<'a> = crate::BitWriter<'a, u32, CHUSEBURSTC_SPEC, bool, 7>;
#[doc = "Field `CH08USEBURSTC` writer - Channel 8 Useburst Clear"]
pub type CH08USEBURSTC_W<'a> = crate::BitWriter<'a, u32, CHUSEBURSTC_SPEC, bool, 8>;
#[doc = "Field `CH9USEBURSTC` writer - Channel 9 Useburst Clear"]
pub type CH9USEBURSTC_W<'a> = crate::BitWriter<'a, u32, CHUSEBURSTC_SPEC, bool, 9>;
#[doc = "Field `CH10USEBURSTC` writer - Channel 10 Useburst Clear"]
pub type CH10USEBURSTC_W<'a> = crate::BitWriter<'a, u32, CHUSEBURSTC_SPEC, bool, 10>;
#[doc = "Field `CH11USEBURSTC` writer - Channel 11 Useburst Clear"]
pub type CH11USEBURSTC_W<'a> = crate::BitWriter<'a, u32, CHUSEBURSTC_SPEC, bool, 11>;
impl W {
    #[doc = "Bit 0 - Channel 0 Useburst Clear"]
    #[inline(always)]
    pub fn ch0useburstc(&mut self) -> CH0USEBURSTC_W {
        CH0USEBURSTC_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Useburst Clear"]
    #[inline(always)]
    pub fn ch1useburstc(&mut self) -> CH1USEBURSTC_W {
        CH1USEBURSTC_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Useburst Clear"]
    #[inline(always)]
    pub fn ch2useburstc(&mut self) -> CH2USEBURSTC_W {
        CH2USEBURSTC_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Useburst Clear"]
    #[inline(always)]
    pub fn ch3useburstc(&mut self) -> CH3USEBURSTC_W {
        CH3USEBURSTC_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 Useburst Clear"]
    #[inline(always)]
    pub fn ch4useburstc(&mut self) -> CH4USEBURSTC_W {
        CH4USEBURSTC_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 Useburst Clear"]
    #[inline(always)]
    pub fn ch5useburstc(&mut self) -> CH5USEBURSTC_W {
        CH5USEBURSTC_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 Useburst Clear"]
    #[inline(always)]
    pub fn ch6useburstc(&mut self) -> CH6USEBURSTC_W {
        CH6USEBURSTC_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 Useburst Clear"]
    #[inline(always)]
    pub fn ch7useburstc(&mut self) -> CH7USEBURSTC_W {
        CH7USEBURSTC_W::new(self)
    }
    #[doc = "Bit 8 - Channel 8 Useburst Clear"]
    #[inline(always)]
    pub fn ch08useburstc(&mut self) -> CH08USEBURSTC_W {
        CH08USEBURSTC_W::new(self)
    }
    #[doc = "Bit 9 - Channel 9 Useburst Clear"]
    #[inline(always)]
    pub fn ch9useburstc(&mut self) -> CH9USEBURSTC_W {
        CH9USEBURSTC_W::new(self)
    }
    #[doc = "Bit 10 - Channel 10 Useburst Clear"]
    #[inline(always)]
    pub fn ch10useburstc(&mut self) -> CH10USEBURSTC_W {
        CH10USEBURSTC_W::new(self)
    }
    #[doc = "Bit 11 - Channel 11 Useburst Clear"]
    #[inline(always)]
    pub fn ch11useburstc(&mut self) -> CH11USEBURSTC_W {
        CH11USEBURSTC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Useburst Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chuseburstc](index.html) module"]
pub struct CHUSEBURSTC_SPEC;
impl crate::RegisterSpec for CHUSEBURSTC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [chuseburstc::W](W) writer structure"]
impl crate::Writable for CHUSEBURSTC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHUSEBURSTC to value 0"]
impl crate::Resettable for CHUSEBURSTC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
