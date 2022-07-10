#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` writer - Start LETIMER"]
pub type START_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 0>;
#[doc = "Field `STOP` writer - Stop LETIMER"]
pub type STOP_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 1>;
#[doc = "Field `CLEAR` writer - Clear LETIMER"]
pub type CLEAR_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 2>;
#[doc = "Field `CTO0` writer - Clear Toggle Output 0"]
pub type CTO0_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 3>;
#[doc = "Field `CTO1` writer - Clear Toggle Output 1"]
pub type CTO1_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 4>;
impl W {
    #[doc = "Bit 0 - Start LETIMER"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W::new(self)
    }
    #[doc = "Bit 1 - Stop LETIMER"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W::new(self)
    }
    #[doc = "Bit 2 - Clear LETIMER"]
    #[inline(always)]
    pub fn clear(&mut self) -> CLEAR_W {
        CLEAR_W::new(self)
    }
    #[doc = "Bit 3 - Clear Toggle Output 0"]
    #[inline(always)]
    pub fn cto0(&mut self) -> CTO0_W {
        CTO0_W::new(self)
    }
    #[doc = "Bit 4 - Clear Toggle Output 1"]
    #[inline(always)]
    pub fn cto1(&mut self) -> CTO1_W {
        CTO1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
