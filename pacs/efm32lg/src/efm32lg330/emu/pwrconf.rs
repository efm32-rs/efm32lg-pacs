#[doc = "Register `PWRCONF` reader"]
pub struct R(crate::R<PWRCONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRCONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRCONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRCONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRCONF` writer"]
pub struct W(crate::W<PWRCONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRCONF_SPEC>;
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
impl From<crate::W<PWRCONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRCONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VOUTWEAK` reader - BU_VOUT weak enable"]
pub type VOUTWEAK_R = crate::BitReader<bool>;
#[doc = "Field `VOUTWEAK` writer - BU_VOUT weak enable"]
pub type VOUTWEAK_W<'a> = crate::BitWriter<'a, u32, PWRCONF_SPEC, bool, 0>;
#[doc = "Field `VOUTMED` reader - BU_VOUT medium enable"]
pub type VOUTMED_R = crate::BitReader<bool>;
#[doc = "Field `VOUTMED` writer - BU_VOUT medium enable"]
pub type VOUTMED_W<'a> = crate::BitWriter<'a, u32, PWRCONF_SPEC, bool, 1>;
#[doc = "Field `VOUTSTRONG` reader - BU_VOUT strong enable"]
pub type VOUTSTRONG_R = crate::BitReader<bool>;
#[doc = "Field `VOUTSTRONG` writer - BU_VOUT strong enable"]
pub type VOUTSTRONG_W<'a> = crate::BitWriter<'a, u32, PWRCONF_SPEC, bool, 2>;
#[doc = "Power domain resistor select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWRRES_A {
    #[doc = "0: Main power and backup power connected with RES0 series resistance."]
    RES0 = 0,
    #[doc = "1: Main power and backup power connected with RES1 series resistance."]
    RES1 = 1,
    #[doc = "2: Main power and backup power connected with RES2 series resistance."]
    RES2 = 2,
    #[doc = "3: Main power and backup power connected with RES3 series resistance."]
    RES3 = 3,
}
impl From<PWRRES_A> for u8 {
    #[inline(always)]
    fn from(variant: PWRRES_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWRRES` reader - Power domain resistor select"]
pub type PWRRES_R = crate::FieldReader<u8, PWRRES_A>;
impl PWRRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRRES_A {
        match self.bits {
            0 => PWRRES_A::RES0,
            1 => PWRRES_A::RES1,
            2 => PWRRES_A::RES2,
            3 => PWRRES_A::RES3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RES0`"]
    #[inline(always)]
    pub fn is_res0(&self) -> bool {
        *self == PWRRES_A::RES0
    }
    #[doc = "Checks if the value of the field is `RES1`"]
    #[inline(always)]
    pub fn is_res1(&self) -> bool {
        *self == PWRRES_A::RES1
    }
    #[doc = "Checks if the value of the field is `RES2`"]
    #[inline(always)]
    pub fn is_res2(&self) -> bool {
        *self == PWRRES_A::RES2
    }
    #[doc = "Checks if the value of the field is `RES3`"]
    #[inline(always)]
    pub fn is_res3(&self) -> bool {
        *self == PWRRES_A::RES3
    }
}
#[doc = "Field `PWRRES` writer - Power domain resistor select"]
pub type PWRRES_W<'a> = crate::FieldWriterSafe<'a, u32, PWRCONF_SPEC, u8, PWRRES_A, 2, 3>;
impl<'a> PWRRES_W<'a> {
    #[doc = "Main power and backup power connected with RES0 series resistance."]
    #[inline(always)]
    pub fn res0(self) -> &'a mut W {
        self.variant(PWRRES_A::RES0)
    }
    #[doc = "Main power and backup power connected with RES1 series resistance."]
    #[inline(always)]
    pub fn res1(self) -> &'a mut W {
        self.variant(PWRRES_A::RES1)
    }
    #[doc = "Main power and backup power connected with RES2 series resistance."]
    #[inline(always)]
    pub fn res2(self) -> &'a mut W {
        self.variant(PWRRES_A::RES2)
    }
    #[doc = "Main power and backup power connected with RES3 series resistance."]
    #[inline(always)]
    pub fn res3(self) -> &'a mut W {
        self.variant(PWRRES_A::RES3)
    }
}
impl R {
    #[doc = "Bit 0 - BU_VOUT weak enable"]
    #[inline(always)]
    pub fn voutweak(&self) -> VOUTWEAK_R {
        VOUTWEAK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BU_VOUT medium enable"]
    #[inline(always)]
    pub fn voutmed(&self) -> VOUTMED_R {
        VOUTMED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BU_VOUT strong enable"]
    #[inline(always)]
    pub fn voutstrong(&self) -> VOUTSTRONG_R {
        VOUTSTRONG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Power domain resistor select"]
    #[inline(always)]
    pub fn pwrres(&self) -> PWRRES_R {
        PWRRES_R::new(((self.bits >> 3) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - BU_VOUT weak enable"]
    #[inline(always)]
    pub fn voutweak(&mut self) -> VOUTWEAK_W {
        VOUTWEAK_W::new(self)
    }
    #[doc = "Bit 1 - BU_VOUT medium enable"]
    #[inline(always)]
    pub fn voutmed(&mut self) -> VOUTMED_W {
        VOUTMED_W::new(self)
    }
    #[doc = "Bit 2 - BU_VOUT strong enable"]
    #[inline(always)]
    pub fn voutstrong(&mut self) -> VOUTSTRONG_W {
        VOUTSTRONG_W::new(self)
    }
    #[doc = "Bits 3:4 - Power domain resistor select"]
    #[inline(always)]
    pub fn pwrres(&mut self) -> PWRRES_W {
        PWRRES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power connection configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrconf](index.html) module"]
pub struct PWRCONF_SPEC;
impl crate::RegisterSpec for PWRCONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrconf::R](R) reader structure"]
impl crate::Readable for PWRCONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrconf::W](W) writer structure"]
impl crate::Writable for PWRCONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWRCONF to value 0"]
impl crate::Resettable for PWRCONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
