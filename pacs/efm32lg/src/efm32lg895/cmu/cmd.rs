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
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub type HFCLKSEL_W<'a> = crate::FieldWriter<'a, u32, CMD_SPEC, u8, HFCLKSEL_AW, 3, 0>;
impl<'a> HFCLKSEL_W<'a> {
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
pub type CALSTART_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 3>;
#[doc = "Field `CALSTOP` writer - Calibration Stop"]
pub type CALSTOP_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 4>;
impl W {
    #[doc = "Bits 0:2 - HFCLK Select"]
    #[inline(always)]
    pub fn hfclksel(&mut self) -> HFCLKSEL_W {
        HFCLKSEL_W::new(self)
    }
    #[doc = "Bit 3 - Calibration Start"]
    #[inline(always)]
    pub fn calstart(&mut self) -> CALSTART_W {
        CALSTART_W::new(self)
    }
    #[doc = "Bit 4 - Calibration Stop"]
    #[inline(always)]
    pub fn calstop(&mut self) -> CALSTOP_W {
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
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
