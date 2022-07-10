#[doc = "Register `LFXOFDET` reader"]
pub struct R(crate::R<LFXOFDET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LFXOFDET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LFXOFDET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LFXOFDET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LFXOFDET` writer"]
pub struct W(crate::W<LFXOFDET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LFXOFDET_SPEC>;
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
impl From<crate::W<LFXOFDET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LFXOFDET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "LFXO failure detection configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OSC_A {
    #[doc = "0: LFXO failure detection disabled."]
    DISABLE = 0,
    #[doc = "1: LFRCO used for LFXO failure detection."]
    LFRCO = 1,
    #[doc = "2: ULFRCO used for LFXO failure detection."]
    ULFRCO = 2,
}
impl From<OSC_A> for u8 {
    #[inline(always)]
    fn from(variant: OSC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OSC` reader - LFXO failure detection configuration."]
pub type OSC_R = crate::FieldReader<u8, OSC_A>;
impl OSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OSC_A> {
        match self.bits {
            0 => Some(OSC_A::DISABLE),
            1 => Some(OSC_A::LFRCO),
            2 => Some(OSC_A::ULFRCO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == OSC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == OSC_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == OSC_A::ULFRCO
    }
}
#[doc = "Field `OSC` writer - LFXO failure detection configuration."]
pub type OSC_W<'a> = crate::FieldWriter<'a, u32, LFXOFDET_SPEC, u8, OSC_A, 2, 0>;
impl<'a> OSC_W<'a> {
    #[doc = "LFXO failure detection disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(OSC_A::DISABLE)
    }
    #[doc = "LFRCO used for LFXO failure detection."]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(OSC_A::LFRCO)
    }
    #[doc = "ULFRCO used for LFXO failure detection."]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(OSC_A::ULFRCO)
    }
}
#[doc = "Field `TOP` reader - LFXO failure counter top value."]
pub type TOP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TOP` writer - LFXO failure counter top value."]
pub type TOP_W<'a> = crate::FieldWriter<'a, u32, LFXOFDET_SPEC, u8, u8, 5, 4>;
impl R {
    #[doc = "Bits 0:1 - LFXO failure detection configuration."]
    #[inline(always)]
    pub fn osc(&self) -> OSC_R {
        OSC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:8 - LFXO failure counter top value."]
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - LFXO failure detection configuration."]
    #[inline(always)]
    pub fn osc(&mut self) -> OSC_W {
        OSC_W::new(self)
    }
    #[doc = "Bits 4:8 - LFXO failure counter top value."]
    #[inline(always)]
    pub fn top(&mut self) -> TOP_W {
        TOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LFXO\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfxofdet](index.html) module"]
pub struct LFXOFDET_SPEC;
impl crate::RegisterSpec for LFXOFDET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lfxofdet::R](R) reader structure"]
impl crate::Readable for LFXOFDET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lfxofdet::W](W) writer structure"]
impl crate::Writable for LFXOFDET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LFXOFDET to value 0"]
impl crate::Resettable for LFXOFDET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
