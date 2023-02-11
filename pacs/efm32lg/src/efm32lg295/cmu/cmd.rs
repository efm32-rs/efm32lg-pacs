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
#[doc = "HFCLK Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HFCLKSEL_AW {
    #[doc = "1: Select HFRCO as HFCLK."]
    HFRCO = 1,
    #[doc = "2: Select HFXO as HFCLK."]
    HFXO = 2,
    #[doc = "3: Select LFRCO as HFCLK."]
    LFRCO = 3,
    #[doc = "4: Select LFXO as HFCLK."]
    LFXO = 4,
}
impl From<HFCLKSEL_AW> for u8 {
    #[inline(always)]
    fn from(variant: HFCLKSEL_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `HFCLKSEL` writer - HFCLK Select"]
pub type HFCLKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMD_SPEC, u8, HFCLKSEL_AW, 3, O>;
impl<'a, const O: u8> HFCLKSEL_W<'a, O> {
    #[doc = "Select HFRCO as HFCLK."]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut W {
        self.variant(HFCLKSEL_AW::HFRCO)
    }
    #[doc = "Select HFXO as HFCLK."]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(HFCLKSEL_AW::HFXO)
    }
    #[doc = "Select LFRCO as HFCLK."]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(HFCLKSEL_AW::LFRCO)
    }
    #[doc = "Select LFXO as HFCLK."]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(HFCLKSEL_AW::LFXO)
    }
}
#[doc = "Field `CALSTART` writer - Calibration Start"]
pub type CALSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `CALSTOP` writer - Calibration Stop"]
pub type CALSTOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
impl W {
    #[doc = "Bits 0:2 - HFCLK Select"]
    #[inline(always)]
    #[must_use]
    pub fn hfclksel(&mut self) -> HFCLKSEL_W<0> {
        HFCLKSEL_W::new(self)
    }
    #[doc = "Bit 3 - Calibration Start"]
    #[inline(always)]
    #[must_use]
    pub fn calstart(&mut self) -> CALSTART_W<3> {
        CALSTART_W::new(self)
    }
    #[doc = "Bit 4 - Calibration Stop"]
    #[inline(always)]
    #[must_use]
    pub fn calstop(&mut self) -> CALSTOP_W<4> {
        CALSTOP_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
