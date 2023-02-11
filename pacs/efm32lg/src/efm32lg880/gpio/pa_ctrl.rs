#[doc = "Register `PA_CTRL` reader"]
pub struct R(crate::R<PA_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PA_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PA_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PA_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PA_CTRL` writer"]
pub struct W(crate::W<PA_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PA_CTRL_SPEC>;
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
impl From<crate::W<PA_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PA_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DRIVEMODE` reader - Drive Mode Select"]
pub type DRIVEMODE_R = crate::FieldReader<u8, DRIVEMODE_A>;
#[doc = "Drive Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DRIVEMODE_A {
    #[doc = "0: 6 mA drive current"]
    STANDARD = 0,
    #[doc = "1: 0.1 mA drive current"]
    LOWEST = 1,
    #[doc = "2: 20 mA drive current"]
    HIGH = 2,
    #[doc = "3: 1 mA drive current"]
    LOW = 3,
}
impl From<DRIVEMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: DRIVEMODE_A) -> Self {
        variant as _
    }
}
impl DRIVEMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRIVEMODE_A {
        match self.bits {
            0 => DRIVEMODE_A::STANDARD,
            1 => DRIVEMODE_A::LOWEST,
            2 => DRIVEMODE_A::HIGH,
            3 => DRIVEMODE_A::LOW,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == DRIVEMODE_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `LOWEST`"]
    #[inline(always)]
    pub fn is_lowest(&self) -> bool {
        *self == DRIVEMODE_A::LOWEST
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == DRIVEMODE_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == DRIVEMODE_A::LOW
    }
}
#[doc = "Field `DRIVEMODE` writer - Drive Mode Select"]
pub type DRIVEMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PA_CTRL_SPEC, u8, DRIVEMODE_A, 2, O>;
impl<'a, const O: u8> DRIVEMODE_W<'a, O> {
    #[doc = "6 mA drive current"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(DRIVEMODE_A::STANDARD)
    }
    #[doc = "0.1 mA drive current"]
    #[inline(always)]
    pub fn lowest(self) -> &'a mut W {
        self.variant(DRIVEMODE_A::LOWEST)
    }
    #[doc = "20 mA drive current"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(DRIVEMODE_A::HIGH)
    }
    #[doc = "1 mA drive current"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(DRIVEMODE_A::LOW)
    }
}
impl R {
    #[doc = "Bits 0:1 - Drive Mode Select"]
    #[inline(always)]
    pub fn drivemode(&self) -> DRIVEMODE_R {
        DRIVEMODE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Drive Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn drivemode(&mut self) -> DRIVEMODE_W<0> {
        DRIVEMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pa_ctrl](index.html) module"]
pub struct PA_CTRL_SPEC;
impl crate::RegisterSpec for PA_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pa_ctrl::R](R) reader structure"]
impl crate::Readable for PA_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pa_ctrl::W](W) writer structure"]
impl crate::Writable for PA_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PA_CTRL to value 0"]
impl crate::Resettable for PA_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
