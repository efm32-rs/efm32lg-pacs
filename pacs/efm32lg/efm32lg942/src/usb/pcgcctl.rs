#[doc = "Register `PCGCCTL` reader"]
pub struct R(crate::R<PCGCCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCGCCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCGCCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCGCCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCGCCTL` writer"]
pub struct W(crate::W<PCGCCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCGCCTL_SPEC>;
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
impl From<crate::W<PCGCCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCGCCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STOPPCLK` reader - Stop PHY clock"]
pub type STOPPCLK_R = crate::BitReader<bool>;
#[doc = "Field `STOPPCLK` writer - Stop PHY clock"]
pub type STOPPCLK_W<'a> = crate::BitWriter<'a, u32, PCGCCTL_SPEC, bool, 0>;
#[doc = "Field `GATEHCLK` reader - Gate HCLK"]
pub type GATEHCLK_R = crate::BitReader<bool>;
#[doc = "Field `GATEHCLK` writer - Gate HCLK"]
pub type GATEHCLK_W<'a> = crate::BitWriter<'a, u32, PCGCCTL_SPEC, bool, 1>;
#[doc = "Field `PWRCLMP` reader - Power Clamp"]
pub type PWRCLMP_R = crate::BitReader<bool>;
#[doc = "Field `PWRCLMP` writer - Power Clamp"]
pub type PWRCLMP_W<'a> = crate::BitWriter<'a, u32, PCGCCTL_SPEC, bool, 2>;
#[doc = "Field `RSTPDWNMODULE` reader - Reset Power-Down Modules"]
pub type RSTPDWNMODULE_R = crate::BitReader<bool>;
#[doc = "Field `RSTPDWNMODULE` writer - Reset Power-Down Modules"]
pub type RSTPDWNMODULE_W<'a> = crate::BitWriter<'a, u32, PCGCCTL_SPEC, bool, 3>;
#[doc = "Field `PHYSLEEP` reader - PHY In Sleep"]
pub type PHYSLEEP_R = crate::BitReader<bool>;
#[doc = "Field `RESETAFTERSUSP` reader - Reset after suspend"]
pub type RESETAFTERSUSP_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Stop PHY clock"]
    #[inline(always)]
    pub fn stoppclk(&self) -> STOPPCLK_R {
        STOPPCLK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Gate HCLK"]
    #[inline(always)]
    pub fn gatehclk(&self) -> GATEHCLK_R {
        GATEHCLK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power Clamp"]
    #[inline(always)]
    pub fn pwrclmp(&self) -> PWRCLMP_R {
        PWRCLMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset Power-Down Modules"]
    #[inline(always)]
    pub fn rstpdwnmodule(&self) -> RSTPDWNMODULE_R {
        RSTPDWNMODULE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - PHY In Sleep"]
    #[inline(always)]
    pub fn physleep(&self) -> PHYSLEEP_R {
        PHYSLEEP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Reset after suspend"]
    #[inline(always)]
    pub fn resetaftersusp(&self) -> RESETAFTERSUSP_R {
        RESETAFTERSUSP_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop PHY clock"]
    #[inline(always)]
    pub fn stoppclk(&mut self) -> STOPPCLK_W {
        STOPPCLK_W::new(self)
    }
    #[doc = "Bit 1 - Gate HCLK"]
    #[inline(always)]
    pub fn gatehclk(&mut self) -> GATEHCLK_W {
        GATEHCLK_W::new(self)
    }
    #[doc = "Bit 2 - Power Clamp"]
    #[inline(always)]
    pub fn pwrclmp(&mut self) -> PWRCLMP_W {
        PWRCLMP_W::new(self)
    }
    #[doc = "Bit 3 - Reset Power-Down Modules"]
    #[inline(always)]
    pub fn rstpdwnmodule(&mut self) -> RSTPDWNMODULE_W {
        RSTPDWNMODULE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power and Clock Gating Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcgcctl](index.html) module"]
pub struct PCGCCTL_SPEC;
impl crate::RegisterSpec for PCGCCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcgcctl::R](R) reader structure"]
impl crate::Readable for PCGCCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcgcctl::W](W) writer structure"]
impl crate::Writable for PCGCCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCGCCTL to value 0"]
impl crate::Resettable for PCGCCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
