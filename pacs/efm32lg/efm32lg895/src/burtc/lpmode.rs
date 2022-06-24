#[doc = "Register `LPMODE` reader"]
pub struct R(crate::R<LPMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPMODE` writer"]
pub struct W(crate::W<LPMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPMODE_SPEC>;
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
impl From<crate::W<LPMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Low power mode configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPMODE_A {
    #[doc = "0: Low power mode is disabled."]
    DISABLE = 0,
    #[doc = "1: Low power mode always enabled."]
    ENABLE = 1,
    #[doc = "2: Low power mode enabled in backup mode."]
    BUEN = 2,
}
impl From<LPMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: LPMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LPMODE` reader - Low power mode configuration."]
pub type LPMODE_R = crate::FieldReader<u8, LPMODE_A>;
impl LPMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LPMODE_A> {
        match self.bits {
            0 => Some(LPMODE_A::DISABLE),
            1 => Some(LPMODE_A::ENABLE),
            2 => Some(LPMODE_A::BUEN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LPMODE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LPMODE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `BUEN`"]
    #[inline(always)]
    pub fn is_buen(&self) -> bool {
        *self == LPMODE_A::BUEN
    }
}
#[doc = "Field `LPMODE` writer - Low power mode configuration."]
pub type LPMODE_W<'a> = crate::FieldWriter<'a, u32, LPMODE_SPEC, u8, LPMODE_A, 2, 0>;
impl<'a> LPMODE_W<'a> {
    #[doc = "Low power mode is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LPMODE_A::DISABLE)
    }
    #[doc = "Low power mode always enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LPMODE_A::ENABLE)
    }
    #[doc = "Low power mode enabled in backup mode."]
    #[inline(always)]
    pub fn buen(self) -> &'a mut W {
        self.variant(LPMODE_A::BUEN)
    }
}
impl R {
    #[doc = "Bits 0:1 - Low power mode configuration."]
    #[inline(always)]
    pub fn lpmode(&self) -> LPMODE_R {
        LPMODE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Low power mode configuration."]
    #[inline(always)]
    pub fn lpmode(&mut self) -> LPMODE_W {
        LPMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low power mode configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmode](index.html) module"]
pub struct LPMODE_SPEC;
impl crate::RegisterSpec for LPMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpmode::R](R) reader structure"]
impl crate::Readable for LPMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpmode::W](W) writer structure"]
impl crate::Writable for LPMODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPMODE to value 0"]
impl crate::Resettable for LPMODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
