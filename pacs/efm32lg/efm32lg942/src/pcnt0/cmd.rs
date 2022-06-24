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
#[doc = "Field `LCNTIM` writer - Load CNT Immediately"]
pub type LCNTIM_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 0>;
#[doc = "Field `LTOPBIM` writer - Load TOPB Immediately"]
pub type LTOPBIM_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 1>;
impl W {
    #[doc = "Bit 0 - Load CNT Immediately"]
    #[inline(always)]
    pub fn lcntim(&mut self) -> LCNTIM_W {
        LCNTIM_W::new(self)
    }
    #[doc = "Bit 1 - Load TOPB Immediately"]
    #[inline(always)]
    pub fn ltopbim(&mut self) -> LTOPBIM_W {
        LTOPBIM_W::new(self)
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
