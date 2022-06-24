#[doc = "Register `AUXHFRCOCTRL` reader"]
pub struct R(crate::R<AUXHFRCOCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUXHFRCOCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUXHFRCOCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUXHFRCOCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUXHFRCOCTRL` writer"]
pub struct W(crate::W<AUXHFRCOCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUXHFRCOCTRL_SPEC>;
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
impl From<crate::W<AUXHFRCOCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUXHFRCOCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TUNING` reader - AUXHFRCO Tuning Value"]
pub type TUNING_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TUNING` writer - AUXHFRCO Tuning Value"]
pub type TUNING_W<'a> = crate::FieldWriter<'a, u32, AUXHFRCOCTRL_SPEC, u8, u8, 8, 0>;
#[doc = "AUXHFRCO Band Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BAND_A {
    #[doc = "0: 14 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _14MHZ = 0,
    #[doc = "1: 11 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _11MHZ = 1,
    #[doc = "2: 7 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _7MHZ = 2,
    #[doc = "3: 1 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _1MHZ = 3,
    #[doc = "6: 28 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _28MHZ = 6,
    #[doc = "7: 21 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _21MHZ = 7,
}
impl From<BAND_A> for u8 {
    #[inline(always)]
    fn from(variant: BAND_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BAND` reader - AUXHFRCO Band Select"]
pub type BAND_R = crate::FieldReader<u8, BAND_A>;
impl BAND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BAND_A> {
        match self.bits {
            0 => Some(BAND_A::_14MHZ),
            1 => Some(BAND_A::_11MHZ),
            2 => Some(BAND_A::_7MHZ),
            3 => Some(BAND_A::_1MHZ),
            6 => Some(BAND_A::_28MHZ),
            7 => Some(BAND_A::_21MHZ),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_14MHZ`"]
    #[inline(always)]
    pub fn is_14mhz(&self) -> bool {
        *self == BAND_A::_14MHZ
    }
    #[doc = "Checks if the value of the field is `_11MHZ`"]
    #[inline(always)]
    pub fn is_11mhz(&self) -> bool {
        *self == BAND_A::_11MHZ
    }
    #[doc = "Checks if the value of the field is `_7MHZ`"]
    #[inline(always)]
    pub fn is_7mhz(&self) -> bool {
        *self == BAND_A::_7MHZ
    }
    #[doc = "Checks if the value of the field is `_1MHZ`"]
    #[inline(always)]
    pub fn is_1mhz(&self) -> bool {
        *self == BAND_A::_1MHZ
    }
    #[doc = "Checks if the value of the field is `_28MHZ`"]
    #[inline(always)]
    pub fn is_28mhz(&self) -> bool {
        *self == BAND_A::_28MHZ
    }
    #[doc = "Checks if the value of the field is `_21MHZ`"]
    #[inline(always)]
    pub fn is_21mhz(&self) -> bool {
        *self == BAND_A::_21MHZ
    }
}
#[doc = "Field `BAND` writer - AUXHFRCO Band Select"]
pub type BAND_W<'a> = crate::FieldWriter<'a, u32, AUXHFRCOCTRL_SPEC, u8, BAND_A, 3, 8>;
impl<'a> BAND_W<'a> {
    #[doc = "14 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn _14mhz(self) -> &'a mut W {
        self.variant(BAND_A::_14MHZ)
    }
    #[doc = "11 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn _11mhz(self) -> &'a mut W {
        self.variant(BAND_A::_11MHZ)
    }
    #[doc = "7 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn _7mhz(self) -> &'a mut W {
        self.variant(BAND_A::_7MHZ)
    }
    #[doc = "1 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn _1mhz(self) -> &'a mut W {
        self.variant(BAND_A::_1MHZ)
    }
    #[doc = "28 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn _28mhz(self) -> &'a mut W {
        self.variant(BAND_A::_28MHZ)
    }
    #[doc = "21 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn _21mhz(self) -> &'a mut W {
        self.variant(BAND_A::_21MHZ)
    }
}
impl R {
    #[doc = "Bits 0:7 - AUXHFRCO Tuning Value"]
    #[inline(always)]
    pub fn tuning(&self) -> TUNING_R {
        TUNING_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - AUXHFRCO Band Select"]
    #[inline(always)]
    pub fn band(&self) -> BAND_R {
        BAND_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - AUXHFRCO Tuning Value"]
    #[inline(always)]
    pub fn tuning(&mut self) -> TUNING_W {
        TUNING_W::new(self)
    }
    #[doc = "Bits 8:10 - AUXHFRCO Band Select"]
    #[inline(always)]
    pub fn band(&mut self) -> BAND_W {
        BAND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AUXHFRCO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auxhfrcoctrl](index.html) module"]
pub struct AUXHFRCOCTRL_SPEC;
impl crate::RegisterSpec for AUXHFRCOCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [auxhfrcoctrl::R](R) reader structure"]
impl crate::Readable for AUXHFRCOCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [auxhfrcoctrl::W](W) writer structure"]
impl crate::Writable for AUXHFRCOCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUXHFRCOCTRL to value 0x80"]
impl crate::Resettable for AUXHFRCOCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
