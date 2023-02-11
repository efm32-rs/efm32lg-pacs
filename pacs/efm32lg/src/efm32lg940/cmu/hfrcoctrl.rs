#[doc = "Register `HFRCOCTRL` reader"]
pub struct R(crate::R<HFRCOCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFRCOCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFRCOCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFRCOCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFRCOCTRL` writer"]
pub struct W(crate::W<HFRCOCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFRCOCTRL_SPEC>;
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
impl From<crate::W<HFRCOCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFRCOCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TUNING` reader - HFRCO Tuning Value"]
pub type TUNING_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TUNING` writer - HFRCO Tuning Value"]
pub type TUNING_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HFRCOCTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `BAND` reader - HFRCO Band Select"]
pub type BAND_R = crate::FieldReader<u8, BAND_A>;
#[doc = "HFRCO Band Select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BAND_A {
    #[doc = "0: 1 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _1MHZ = 0,
    #[doc = "1: 7 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _7MHZ = 1,
    #[doc = "2: 11 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _11MHZ = 2,
    #[doc = "3: 14 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _14MHZ = 3,
    #[doc = "4: 21 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _21MHZ = 4,
    #[doc = "5: 28 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _28MHZ = 5,
}
impl From<BAND_A> for u8 {
    #[inline(always)]
    fn from(variant: BAND_A) -> Self {
        variant as _
    }
}
impl BAND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BAND_A> {
        match self.bits {
            0 => Some(BAND_A::_1MHZ),
            1 => Some(BAND_A::_7MHZ),
            2 => Some(BAND_A::_11MHZ),
            3 => Some(BAND_A::_14MHZ),
            4 => Some(BAND_A::_21MHZ),
            5 => Some(BAND_A::_28MHZ),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1MHZ`"]
    #[inline(always)]
    pub fn is_1mhz(&self) -> bool {
        *self == BAND_A::_1MHZ
    }
    #[doc = "Checks if the value of the field is `_7MHZ`"]
    #[inline(always)]
    pub fn is_7mhz(&self) -> bool {
        *self == BAND_A::_7MHZ
    }
    #[doc = "Checks if the value of the field is `_11MHZ`"]
    #[inline(always)]
    pub fn is_11mhz(&self) -> bool {
        *self == BAND_A::_11MHZ
    }
    #[doc = "Checks if the value of the field is `_14MHZ`"]
    #[inline(always)]
    pub fn is_14mhz(&self) -> bool {
        *self == BAND_A::_14MHZ
    }
    #[doc = "Checks if the value of the field is `_21MHZ`"]
    #[inline(always)]
    pub fn is_21mhz(&self) -> bool {
        *self == BAND_A::_21MHZ
    }
    #[doc = "Checks if the value of the field is `_28MHZ`"]
    #[inline(always)]
    pub fn is_28mhz(&self) -> bool {
        *self == BAND_A::_28MHZ
    }
}
#[doc = "Field `BAND` writer - HFRCO Band Select"]
pub type BAND_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HFRCOCTRL_SPEC, u8, BAND_A, 3, O>;
impl<'a, const O: u8> BAND_W<'a, O> {
    #[doc = "1 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn _1mhz(self) -> &'a mut W {
        self.variant(BAND_A::_1MHZ)
    }
    #[doc = "7 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn _7mhz(self) -> &'a mut W {
        self.variant(BAND_A::_7MHZ)
    }
    #[doc = "11 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn _11mhz(self) -> &'a mut W {
        self.variant(BAND_A::_11MHZ)
    }
    #[doc = "14 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn _14mhz(self) -> &'a mut W {
        self.variant(BAND_A::_14MHZ)
    }
    #[doc = "21 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn _21mhz(self) -> &'a mut W {
        self.variant(BAND_A::_21MHZ)
    }
    #[doc = "28 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn _28mhz(self) -> &'a mut W {
        self.variant(BAND_A::_28MHZ)
    }
}
#[doc = "Field `SUDELAY` reader - HFRCO Start-up Delay"]
pub type SUDELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SUDELAY` writer - HFRCO Start-up Delay"]
pub type SUDELAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HFRCOCTRL_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:7 - HFRCO Tuning Value"]
    #[inline(always)]
    pub fn tuning(&self) -> TUNING_R {
        TUNING_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - HFRCO Band Select"]
    #[inline(always)]
    pub fn band(&self) -> BAND_R {
        BAND_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:16 - HFRCO Start-up Delay"]
    #[inline(always)]
    pub fn sudelay(&self) -> SUDELAY_R {
        SUDELAY_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - HFRCO Tuning Value"]
    #[inline(always)]
    #[must_use]
    pub fn tuning(&mut self) -> TUNING_W<0> {
        TUNING_W::new(self)
    }
    #[doc = "Bits 8:10 - HFRCO Band Select"]
    #[inline(always)]
    #[must_use]
    pub fn band(&mut self) -> BAND_W<8> {
        BAND_W::new(self)
    }
    #[doc = "Bits 12:16 - HFRCO Start-up Delay"]
    #[inline(always)]
    #[must_use]
    pub fn sudelay(&mut self) -> SUDELAY_W<12> {
        SUDELAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HFRCO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfrcoctrl](index.html) module"]
pub struct HFRCOCTRL_SPEC;
impl crate::RegisterSpec for HFRCOCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfrcoctrl::R](R) reader structure"]
impl crate::Readable for HFRCOCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfrcoctrl::W](W) writer structure"]
impl crate::Writable for HFRCOCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HFRCOCTRL to value 0x0380"]
impl crate::Resettable for HFRCOCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0380;
}
