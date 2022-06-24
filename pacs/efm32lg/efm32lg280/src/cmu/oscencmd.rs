#[doc = "Register `OSCENCMD` writer"]
pub struct W(crate::W<OSCENCMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSCENCMD_SPEC>;
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
impl From<crate::W<OSCENCMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSCENCMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HFRCOEN` writer - HFRCO Enable"]
pub type HFRCOEN_W<'a> = crate::BitWriter<'a, u32, OSCENCMD_SPEC, bool, 0>;
#[doc = "Field `HFRCODIS` writer - HFRCO Disable"]
pub type HFRCODIS_W<'a> = crate::BitWriter<'a, u32, OSCENCMD_SPEC, bool, 1>;
#[doc = "Field `HFXOEN` writer - HFXO Enable"]
pub type HFXOEN_W<'a> = crate::BitWriter<'a, u32, OSCENCMD_SPEC, bool, 2>;
#[doc = "Field `HFXODIS` writer - HFXO Disable"]
pub type HFXODIS_W<'a> = crate::BitWriter<'a, u32, OSCENCMD_SPEC, bool, 3>;
#[doc = "Field `AUXHFRCOEN` writer - AUXHFRCO Enable"]
pub type AUXHFRCOEN_W<'a> = crate::BitWriter<'a, u32, OSCENCMD_SPEC, bool, 4>;
#[doc = "Field `AUXHFRCODIS` writer - AUXHFRCO Disable"]
pub type AUXHFRCODIS_W<'a> = crate::BitWriter<'a, u32, OSCENCMD_SPEC, bool, 5>;
#[doc = "Field `LFRCOEN` writer - LFRCO Enable"]
pub type LFRCOEN_W<'a> = crate::BitWriter<'a, u32, OSCENCMD_SPEC, bool, 6>;
#[doc = "Field `LFRCODIS` writer - LFRCO Disable"]
pub type LFRCODIS_W<'a> = crate::BitWriter<'a, u32, OSCENCMD_SPEC, bool, 7>;
#[doc = "Field `LFXOEN` writer - LFXO Enable"]
pub type LFXOEN_W<'a> = crate::BitWriter<'a, u32, OSCENCMD_SPEC, bool, 8>;
#[doc = "Field `LFXODIS` writer - LFXO Disable"]
pub type LFXODIS_W<'a> = crate::BitWriter<'a, u32, OSCENCMD_SPEC, bool, 9>;
impl W {
    #[doc = "Bit 0 - HFRCO Enable"]
    #[inline(always)]
    pub fn hfrcoen(&mut self) -> HFRCOEN_W {
        HFRCOEN_W::new(self)
    }
    #[doc = "Bit 1 - HFRCO Disable"]
    #[inline(always)]
    pub fn hfrcodis(&mut self) -> HFRCODIS_W {
        HFRCODIS_W::new(self)
    }
    #[doc = "Bit 2 - HFXO Enable"]
    #[inline(always)]
    pub fn hfxoen(&mut self) -> HFXOEN_W {
        HFXOEN_W::new(self)
    }
    #[doc = "Bit 3 - HFXO Disable"]
    #[inline(always)]
    pub fn hfxodis(&mut self) -> HFXODIS_W {
        HFXODIS_W::new(self)
    }
    #[doc = "Bit 4 - AUXHFRCO Enable"]
    #[inline(always)]
    pub fn auxhfrcoen(&mut self) -> AUXHFRCOEN_W {
        AUXHFRCOEN_W::new(self)
    }
    #[doc = "Bit 5 - AUXHFRCO Disable"]
    #[inline(always)]
    pub fn auxhfrcodis(&mut self) -> AUXHFRCODIS_W {
        AUXHFRCODIS_W::new(self)
    }
    #[doc = "Bit 6 - LFRCO Enable"]
    #[inline(always)]
    pub fn lfrcoen(&mut self) -> LFRCOEN_W {
        LFRCOEN_W::new(self)
    }
    #[doc = "Bit 7 - LFRCO Disable"]
    #[inline(always)]
    pub fn lfrcodis(&mut self) -> LFRCODIS_W {
        LFRCODIS_W::new(self)
    }
    #[doc = "Bit 8 - LFXO Enable"]
    #[inline(always)]
    pub fn lfxoen(&mut self) -> LFXOEN_W {
        LFXOEN_W::new(self)
    }
    #[doc = "Bit 9 - LFXO Disable"]
    #[inline(always)]
    pub fn lfxodis(&mut self) -> LFXODIS_W {
        LFXODIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Oscillator Enable/Disable Command Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oscencmd](index.html) module"]
pub struct OSCENCMD_SPEC;
impl crate::RegisterSpec for OSCENCMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [oscencmd::W](W) writer structure"]
impl crate::Writable for OSCENCMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSCENCMD to value 0"]
impl crate::Resettable for OSCENCMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
