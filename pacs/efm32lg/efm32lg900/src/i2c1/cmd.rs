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
#[doc = "Field `START` writer - Send start condition"]
pub type START_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 0>;
#[doc = "Field `STOP` writer - Send stop condition"]
pub type STOP_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 1>;
#[doc = "Field `ACK` writer - Send ACK"]
pub type ACK_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 2>;
#[doc = "Field `NACK` writer - Send NACK"]
pub type NACK_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 3>;
#[doc = "Field `CONT` writer - Continue transmission"]
pub type CONT_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 4>;
#[doc = "Field `ABORT` writer - Abort transmission"]
pub type ABORT_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 5>;
#[doc = "Field `CLEARTX` writer - Clear TX"]
pub type CLEARTX_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 6>;
#[doc = "Field `CLEARPC` writer - Clear Pending Commands"]
pub type CLEARPC_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 7>;
impl W {
    #[doc = "Bit 0 - Send start condition"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W::new(self)
    }
    #[doc = "Bit 1 - Send stop condition"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W::new(self)
    }
    #[doc = "Bit 2 - Send ACK"]
    #[inline(always)]
    pub fn ack(&mut self) -> ACK_W {
        ACK_W::new(self)
    }
    #[doc = "Bit 3 - Send NACK"]
    #[inline(always)]
    pub fn nack(&mut self) -> NACK_W {
        NACK_W::new(self)
    }
    #[doc = "Bit 4 - Continue transmission"]
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W {
        CONT_W::new(self)
    }
    #[doc = "Bit 5 - Abort transmission"]
    #[inline(always)]
    pub fn abort(&mut self) -> ABORT_W {
        ABORT_W::new(self)
    }
    #[doc = "Bit 6 - Clear TX"]
    #[inline(always)]
    pub fn cleartx(&mut self) -> CLEARTX_W {
        CLEARTX_W::new(self)
    }
    #[doc = "Bit 7 - Clear Pending Commands"]
    #[inline(always)]
    pub fn clearpc(&mut self) -> CLEARPC_W {
        CLEARPC_W::new(self)
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
