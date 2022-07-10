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
#[doc = "Field `INVCACHE` writer - Invalidate Instruction Cache"]
pub type INVCACHE_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 0>;
#[doc = "Field `STARTPC` writer - Start Performance Counters"]
pub type STARTPC_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 1>;
#[doc = "Field `STOPPC` writer - Stop Performance Counters"]
pub type STOPPC_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 2>;
impl W {
    #[doc = "Bit 0 - Invalidate Instruction Cache"]
    #[inline(always)]
    pub fn invcache(&mut self) -> INVCACHE_W {
        INVCACHE_W::new(self)
    }
    #[doc = "Bit 1 - Start Performance Counters"]
    #[inline(always)]
    pub fn startpc(&mut self) -> STARTPC_W {
        STARTPC_W::new(self)
    }
    #[doc = "Bit 2 - Stop Performance Counters"]
    #[inline(always)]
    pub fn stoppc(&mut self) -> STOPPC_W {
        STOPPC_W::new(self)
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
