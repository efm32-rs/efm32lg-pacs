#[doc = "Register `OPACTRL` reader"]
pub struct R(crate::R<OPACTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPACTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPACTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPACTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPACTRL` writer"]
pub struct W(crate::W<OPACTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPACTRL_SPEC>;
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
impl From<crate::W<OPACTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPACTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPA0EN` reader - OPA0 Enable"]
pub type OPA0EN_R = crate::BitReader<bool>;
#[doc = "Field `OPA0EN` writer - OPA0 Enable"]
pub type OPA0EN_W<'a> = crate::BitWriter<'a, u32, OPACTRL_SPEC, bool, 0>;
#[doc = "Field `OPA1EN` reader - OPA1 Enable"]
pub type OPA1EN_R = crate::BitReader<bool>;
#[doc = "Field `OPA1EN` writer - OPA1 Enable"]
pub type OPA1EN_W<'a> = crate::BitWriter<'a, u32, OPACTRL_SPEC, bool, 1>;
#[doc = "Field `OPA2EN` reader - OPA2 Enable"]
pub type OPA2EN_R = crate::BitReader<bool>;
#[doc = "Field `OPA2EN` writer - OPA2 Enable"]
pub type OPA2EN_W<'a> = crate::BitWriter<'a, u32, OPACTRL_SPEC, bool, 2>;
#[doc = "Field `OPA0HCMDIS` reader - High Common Mode Disable."]
pub type OPA0HCMDIS_R = crate::BitReader<bool>;
#[doc = "Field `OPA0HCMDIS` writer - High Common Mode Disable."]
pub type OPA0HCMDIS_W<'a> = crate::BitWriter<'a, u32, OPACTRL_SPEC, bool, 6>;
#[doc = "Field `OPA1HCMDIS` reader - High Common Mode Disable."]
pub type OPA1HCMDIS_R = crate::BitReader<bool>;
#[doc = "Field `OPA1HCMDIS` writer - High Common Mode Disable."]
pub type OPA1HCMDIS_W<'a> = crate::BitWriter<'a, u32, OPACTRL_SPEC, bool, 7>;
#[doc = "Field `OPA2HCMDIS` reader - High Common Mode Disable."]
pub type OPA2HCMDIS_R = crate::BitReader<bool>;
#[doc = "Field `OPA2HCMDIS` writer - High Common Mode Disable."]
pub type OPA2HCMDIS_W<'a> = crate::BitWriter<'a, u32, OPACTRL_SPEC, bool, 8>;
#[doc = "Disables Low Pass Filter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPA0LPFDIS_A {
    #[doc = "1: Disables the LPF between positive pad and positive input."]
    PLPFDIS = 1,
    #[doc = "2: Disables the LPF between negative pad and negative input."]
    NLPFDIS = 2,
}
impl From<OPA0LPFDIS_A> for u8 {
    #[inline(always)]
    fn from(variant: OPA0LPFDIS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OPA0LPFDIS` reader - Disables Low Pass Filter."]
pub type OPA0LPFDIS_R = crate::FieldReader<u8, OPA0LPFDIS_A>;
impl OPA0LPFDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPA0LPFDIS_A> {
        match self.bits {
            1 => Some(OPA0LPFDIS_A::PLPFDIS),
            2 => Some(OPA0LPFDIS_A::NLPFDIS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PLPFDIS`"]
    #[inline(always)]
    pub fn is_plpfdis(&self) -> bool {
        *self == OPA0LPFDIS_A::PLPFDIS
    }
    #[doc = "Checks if the value of the field is `NLPFDIS`"]
    #[inline(always)]
    pub fn is_nlpfdis(&self) -> bool {
        *self == OPA0LPFDIS_A::NLPFDIS
    }
}
#[doc = "Field `OPA0LPFDIS` writer - Disables Low Pass Filter."]
pub type OPA0LPFDIS_W<'a> = crate::FieldWriter<'a, u32, OPACTRL_SPEC, u8, OPA0LPFDIS_A, 2, 12>;
impl<'a> OPA0LPFDIS_W<'a> {
    #[doc = "Disables the LPF between positive pad and positive input."]
    #[inline(always)]
    pub fn plpfdis(self) -> &'a mut W {
        self.variant(OPA0LPFDIS_A::PLPFDIS)
    }
    #[doc = "Disables the LPF between negative pad and negative input."]
    #[inline(always)]
    pub fn nlpfdis(self) -> &'a mut W {
        self.variant(OPA0LPFDIS_A::NLPFDIS)
    }
}
#[doc = "Disables Low Pass Filter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPA1LPFDIS_A {
    #[doc = "1: Disables the LPF between positive pad and positive input."]
    PLPFDIS = 1,
    #[doc = "2: Disables the LPF between negative pad and negative input."]
    NLPFDIS = 2,
}
impl From<OPA1LPFDIS_A> for u8 {
    #[inline(always)]
    fn from(variant: OPA1LPFDIS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OPA1LPFDIS` reader - Disables Low Pass Filter."]
pub type OPA1LPFDIS_R = crate::FieldReader<u8, OPA1LPFDIS_A>;
impl OPA1LPFDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPA1LPFDIS_A> {
        match self.bits {
            1 => Some(OPA1LPFDIS_A::PLPFDIS),
            2 => Some(OPA1LPFDIS_A::NLPFDIS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PLPFDIS`"]
    #[inline(always)]
    pub fn is_plpfdis(&self) -> bool {
        *self == OPA1LPFDIS_A::PLPFDIS
    }
    #[doc = "Checks if the value of the field is `NLPFDIS`"]
    #[inline(always)]
    pub fn is_nlpfdis(&self) -> bool {
        *self == OPA1LPFDIS_A::NLPFDIS
    }
}
#[doc = "Field `OPA1LPFDIS` writer - Disables Low Pass Filter."]
pub type OPA1LPFDIS_W<'a> = crate::FieldWriter<'a, u32, OPACTRL_SPEC, u8, OPA1LPFDIS_A, 2, 14>;
impl<'a> OPA1LPFDIS_W<'a> {
    #[doc = "Disables the LPF between positive pad and positive input."]
    #[inline(always)]
    pub fn plpfdis(self) -> &'a mut W {
        self.variant(OPA1LPFDIS_A::PLPFDIS)
    }
    #[doc = "Disables the LPF between negative pad and negative input."]
    #[inline(always)]
    pub fn nlpfdis(self) -> &'a mut W {
        self.variant(OPA1LPFDIS_A::NLPFDIS)
    }
}
#[doc = "Disables Low Pass Filter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPA2LPFDIS_A {
    #[doc = "1: Disables the LPF between positive pad and positive input."]
    PLPFDIS = 1,
    #[doc = "2: Disables the LPF between negative pad and negative input."]
    NLPFDIS = 2,
}
impl From<OPA2LPFDIS_A> for u8 {
    #[inline(always)]
    fn from(variant: OPA2LPFDIS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OPA2LPFDIS` reader - Disables Low Pass Filter."]
pub type OPA2LPFDIS_R = crate::FieldReader<u8, OPA2LPFDIS_A>;
impl OPA2LPFDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPA2LPFDIS_A> {
        match self.bits {
            1 => Some(OPA2LPFDIS_A::PLPFDIS),
            2 => Some(OPA2LPFDIS_A::NLPFDIS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PLPFDIS`"]
    #[inline(always)]
    pub fn is_plpfdis(&self) -> bool {
        *self == OPA2LPFDIS_A::PLPFDIS
    }
    #[doc = "Checks if the value of the field is `NLPFDIS`"]
    #[inline(always)]
    pub fn is_nlpfdis(&self) -> bool {
        *self == OPA2LPFDIS_A::NLPFDIS
    }
}
#[doc = "Field `OPA2LPFDIS` writer - Disables Low Pass Filter."]
pub type OPA2LPFDIS_W<'a> = crate::FieldWriter<'a, u32, OPACTRL_SPEC, u8, OPA2LPFDIS_A, 2, 16>;
impl<'a> OPA2LPFDIS_W<'a> {
    #[doc = "Disables the LPF between positive pad and positive input."]
    #[inline(always)]
    pub fn plpfdis(self) -> &'a mut W {
        self.variant(OPA2LPFDIS_A::PLPFDIS)
    }
    #[doc = "Disables the LPF between negative pad and negative input."]
    #[inline(always)]
    pub fn nlpfdis(self) -> &'a mut W {
        self.variant(OPA2LPFDIS_A::NLPFDIS)
    }
}
#[doc = "Field `OPA0SHORT` reader - Short the non-inverting and inverting input."]
pub type OPA0SHORT_R = crate::BitReader<bool>;
#[doc = "Field `OPA0SHORT` writer - Short the non-inverting and inverting input."]
pub type OPA0SHORT_W<'a> = crate::BitWriter<'a, u32, OPACTRL_SPEC, bool, 22>;
#[doc = "Field `OPA1SHORT` reader - Short the non-inverting and inverting input."]
pub type OPA1SHORT_R = crate::BitReader<bool>;
#[doc = "Field `OPA1SHORT` writer - Short the non-inverting and inverting input."]
pub type OPA1SHORT_W<'a> = crate::BitWriter<'a, u32, OPACTRL_SPEC, bool, 23>;
#[doc = "Field `OPA2SHORT` reader - Short the non-inverting and inverting input."]
pub type OPA2SHORT_R = crate::BitReader<bool>;
#[doc = "Field `OPA2SHORT` writer - Short the non-inverting and inverting input."]
pub type OPA2SHORT_W<'a> = crate::BitWriter<'a, u32, OPACTRL_SPEC, bool, 24>;
impl R {
    #[doc = "Bit 0 - OPA0 Enable"]
    #[inline(always)]
    pub fn opa0en(&self) -> OPA0EN_R {
        OPA0EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OPA1 Enable"]
    #[inline(always)]
    pub fn opa1en(&self) -> OPA1EN_R {
        OPA1EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OPA2 Enable"]
    #[inline(always)]
    pub fn opa2en(&self) -> OPA2EN_R {
        OPA2EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - High Common Mode Disable."]
    #[inline(always)]
    pub fn opa0hcmdis(&self) -> OPA0HCMDIS_R {
        OPA0HCMDIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - High Common Mode Disable."]
    #[inline(always)]
    pub fn opa1hcmdis(&self) -> OPA1HCMDIS_R {
        OPA1HCMDIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - High Common Mode Disable."]
    #[inline(always)]
    pub fn opa2hcmdis(&self) -> OPA2HCMDIS_R {
        OPA2HCMDIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Disables Low Pass Filter."]
    #[inline(always)]
    pub fn opa0lpfdis(&self) -> OPA0LPFDIS_R {
        OPA0LPFDIS_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Disables Low Pass Filter."]
    #[inline(always)]
    pub fn opa1lpfdis(&self) -> OPA1LPFDIS_R {
        OPA1LPFDIS_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Disables Low Pass Filter."]
    #[inline(always)]
    pub fn opa2lpfdis(&self) -> OPA2LPFDIS_R {
        OPA2LPFDIS_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 22 - Short the non-inverting and inverting input."]
    #[inline(always)]
    pub fn opa0short(&self) -> OPA0SHORT_R {
        OPA0SHORT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Short the non-inverting and inverting input."]
    #[inline(always)]
    pub fn opa1short(&self) -> OPA1SHORT_R {
        OPA1SHORT_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Short the non-inverting and inverting input."]
    #[inline(always)]
    pub fn opa2short(&self) -> OPA2SHORT_R {
        OPA2SHORT_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OPA0 Enable"]
    #[inline(always)]
    pub fn opa0en(&mut self) -> OPA0EN_W {
        OPA0EN_W::new(self)
    }
    #[doc = "Bit 1 - OPA1 Enable"]
    #[inline(always)]
    pub fn opa1en(&mut self) -> OPA1EN_W {
        OPA1EN_W::new(self)
    }
    #[doc = "Bit 2 - OPA2 Enable"]
    #[inline(always)]
    pub fn opa2en(&mut self) -> OPA2EN_W {
        OPA2EN_W::new(self)
    }
    #[doc = "Bit 6 - High Common Mode Disable."]
    #[inline(always)]
    pub fn opa0hcmdis(&mut self) -> OPA0HCMDIS_W {
        OPA0HCMDIS_W::new(self)
    }
    #[doc = "Bit 7 - High Common Mode Disable."]
    #[inline(always)]
    pub fn opa1hcmdis(&mut self) -> OPA1HCMDIS_W {
        OPA1HCMDIS_W::new(self)
    }
    #[doc = "Bit 8 - High Common Mode Disable."]
    #[inline(always)]
    pub fn opa2hcmdis(&mut self) -> OPA2HCMDIS_W {
        OPA2HCMDIS_W::new(self)
    }
    #[doc = "Bits 12:13 - Disables Low Pass Filter."]
    #[inline(always)]
    pub fn opa0lpfdis(&mut self) -> OPA0LPFDIS_W {
        OPA0LPFDIS_W::new(self)
    }
    #[doc = "Bits 14:15 - Disables Low Pass Filter."]
    #[inline(always)]
    pub fn opa1lpfdis(&mut self) -> OPA1LPFDIS_W {
        OPA1LPFDIS_W::new(self)
    }
    #[doc = "Bits 16:17 - Disables Low Pass Filter."]
    #[inline(always)]
    pub fn opa2lpfdis(&mut self) -> OPA2LPFDIS_W {
        OPA2LPFDIS_W::new(self)
    }
    #[doc = "Bit 22 - Short the non-inverting and inverting input."]
    #[inline(always)]
    pub fn opa0short(&mut self) -> OPA0SHORT_W {
        OPA0SHORT_W::new(self)
    }
    #[doc = "Bit 23 - Short the non-inverting and inverting input."]
    #[inline(always)]
    pub fn opa1short(&mut self) -> OPA1SHORT_W {
        OPA1SHORT_W::new(self)
    }
    #[doc = "Bit 24 - Short the non-inverting and inverting input."]
    #[inline(always)]
    pub fn opa2short(&mut self) -> OPA2SHORT_W {
        OPA2SHORT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Operational Amplifier Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opactrl](index.html) module"]
pub struct OPACTRL_SPEC;
impl crate::RegisterSpec for OPACTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opactrl::R](R) reader structure"]
impl crate::Readable for OPACTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opactrl::W](W) writer structure"]
impl crate::Writable for OPACTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPACTRL to value 0"]
impl crate::Resettable for OPACTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
