#[doc = "Register `BIASCTRL` reader"]
pub struct R(crate::R<BIASCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BIASCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BIASCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BIASCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BIASCTRL` writer"]
pub struct W(crate::W<BIASCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BIASCTRL_SPEC>;
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
impl From<crate::W<BIASCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BIASCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Select bias mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BIASMODE_A {
    #[doc = "0: Bias module duty cycled between low power and high accuracy mode"]
    DUTYCYCLE = 0,
    #[doc = "1: Bias module always in high accuracy mode"]
    HIGHACC = 1,
    #[doc = "2: Bias module not affected by LESENSE"]
    DONTTOUCH = 2,
}
impl From<BIASMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: BIASMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BIASMODE` reader - Select bias mode"]
pub type BIASMODE_R = crate::FieldReader<u8, BIASMODE_A>;
impl BIASMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BIASMODE_A> {
        match self.bits {
            0 => Some(BIASMODE_A::DUTYCYCLE),
            1 => Some(BIASMODE_A::HIGHACC),
            2 => Some(BIASMODE_A::DONTTOUCH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DUTYCYCLE`"]
    #[inline(always)]
    pub fn is_dutycycle(&self) -> bool {
        *self == BIASMODE_A::DUTYCYCLE
    }
    #[doc = "Checks if the value of the field is `HIGHACC`"]
    #[inline(always)]
    pub fn is_highacc(&self) -> bool {
        *self == BIASMODE_A::HIGHACC
    }
    #[doc = "Checks if the value of the field is `DONTTOUCH`"]
    #[inline(always)]
    pub fn is_donttouch(&self) -> bool {
        *self == BIASMODE_A::DONTTOUCH
    }
}
#[doc = "Field `BIASMODE` writer - Select bias mode"]
pub type BIASMODE_W<'a> = crate::FieldWriter<'a, u32, BIASCTRL_SPEC, u8, BIASMODE_A, 2, 0>;
impl<'a> BIASMODE_W<'a> {
    #[doc = "Bias module duty cycled between low power and high accuracy mode"]
    #[inline(always)]
    pub fn dutycycle(self) -> &'a mut W {
        self.variant(BIASMODE_A::DUTYCYCLE)
    }
    #[doc = "Bias module always in high accuracy mode"]
    #[inline(always)]
    pub fn highacc(self) -> &'a mut W {
        self.variant(BIASMODE_A::HIGHACC)
    }
    #[doc = "Bias module not affected by LESENSE"]
    #[inline(always)]
    pub fn donttouch(self) -> &'a mut W {
        self.variant(BIASMODE_A::DONTTOUCH)
    }
}
impl R {
    #[doc = "Bits 0:1 - Select bias mode"]
    #[inline(always)]
    pub fn biasmode(&self) -> BIASMODE_R {
        BIASMODE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select bias mode"]
    #[inline(always)]
    pub fn biasmode(&mut self) -> BIASMODE_W {
        BIASMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bias Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [biasctrl](index.html) module"]
pub struct BIASCTRL_SPEC;
impl crate::RegisterSpec for BIASCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [biasctrl::R](R) reader structure"]
impl crate::Readable for BIASCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [biasctrl::W](W) writer structure"]
impl crate::Writable for BIASCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BIASCTRL to value 0"]
impl crate::Resettable for BIASCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
