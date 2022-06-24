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
#[doc = "Field `SINGLESTART` writer - Single Conversion Start"]
pub type SINGLESTART_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 0>;
#[doc = "Field `SINGLESTOP` writer - Single Conversion Stop"]
pub type SINGLESTOP_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 1>;
#[doc = "Field `SCANSTART` writer - Scan Sequence Start"]
pub type SCANSTART_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 2>;
#[doc = "Field `SCANSTOP` writer - Scan Sequence Stop"]
pub type SCANSTOP_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 3>;
impl W {
    #[doc = "Bit 0 - Single Conversion Start"]
    #[inline(always)]
    pub fn singlestart(&mut self) -> SINGLESTART_W {
        SINGLESTART_W::new(self)
    }
    #[doc = "Bit 1 - Single Conversion Stop"]
    #[inline(always)]
    pub fn singlestop(&mut self) -> SINGLESTOP_W {
        SINGLESTOP_W::new(self)
    }
    #[doc = "Bit 2 - Scan Sequence Start"]
    #[inline(always)]
    pub fn scanstart(&mut self) -> SCANSTART_W {
        SCANSTART_W::new(self)
    }
    #[doc = "Bit 3 - Scan Sequence Stop"]
    #[inline(always)]
    pub fn scanstop(&mut self) -> SCANSTOP_W {
        SCANSTOP_W::new(self)
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
