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
pub type HFRCOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCENCMD_SPEC, bool, O>;
#[doc = "Field `HFRCODIS` writer - HFRCO Disable"]
pub type HFRCODIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCENCMD_SPEC, bool, O>;
#[doc = "Field `HFXOEN` writer - HFXO Enable"]
pub type HFXOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCENCMD_SPEC, bool, O>;
#[doc = "Field `HFXODIS` writer - HFXO Disable"]
pub type HFXODIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCENCMD_SPEC, bool, O>;
#[doc = "Field `AUXHFRCOEN` writer - AUXHFRCO Enable"]
pub type AUXHFRCOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCENCMD_SPEC, bool, O>;
#[doc = "Field `AUXHFRCODIS` writer - AUXHFRCO Disable"]
pub type AUXHFRCODIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCENCMD_SPEC, bool, O>;
#[doc = "Field `LFRCOEN` writer - LFRCO Enable"]
pub type LFRCOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCENCMD_SPEC, bool, O>;
#[doc = "Field `LFRCODIS` writer - LFRCO Disable"]
pub type LFRCODIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCENCMD_SPEC, bool, O>;
#[doc = "Field `LFXOEN` writer - LFXO Enable"]
pub type LFXOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCENCMD_SPEC, bool, O>;
#[doc = "Field `LFXODIS` writer - LFXO Disable"]
pub type LFXODIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCENCMD_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - HFRCO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hfrcoen(&mut self) -> HFRCOEN_W<0> {
        HFRCOEN_W::new(self)
    }
    #[doc = "Bit 1 - HFRCO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn hfrcodis(&mut self) -> HFRCODIS_W<1> {
        HFRCODIS_W::new(self)
    }
    #[doc = "Bit 2 - HFXO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hfxoen(&mut self) -> HFXOEN_W<2> {
        HFXOEN_W::new(self)
    }
    #[doc = "Bit 3 - HFXO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn hfxodis(&mut self) -> HFXODIS_W<3> {
        HFXODIS_W::new(self)
    }
    #[doc = "Bit 4 - AUXHFRCO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn auxhfrcoen(&mut self) -> AUXHFRCOEN_W<4> {
        AUXHFRCOEN_W::new(self)
    }
    #[doc = "Bit 5 - AUXHFRCO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn auxhfrcodis(&mut self) -> AUXHFRCODIS_W<5> {
        AUXHFRCODIS_W::new(self)
    }
    #[doc = "Bit 6 - LFRCO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lfrcoen(&mut self) -> LFRCOEN_W<6> {
        LFRCOEN_W::new(self)
    }
    #[doc = "Bit 7 - LFRCO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn lfrcodis(&mut self) -> LFRCODIS_W<7> {
        LFRCODIS_W::new(self)
    }
    #[doc = "Bit 8 - LFXO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lfxoen(&mut self) -> LFXOEN_W<8> {
        LFXOEN_W::new(self)
    }
    #[doc = "Bit 9 - LFXO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn lfxodis(&mut self) -> LFXODIS_W<9> {
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSCENCMD to value 0"]
impl crate::Resettable for OSCENCMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
