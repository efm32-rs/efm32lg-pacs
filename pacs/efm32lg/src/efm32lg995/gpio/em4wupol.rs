#[doc = "Register `EM4WUPOL` reader"]
pub struct R(crate::R<EM4WUPOL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EM4WUPOL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EM4WUPOL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EM4WUPOL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EM4WUPOL` writer"]
pub struct W(crate::W<EM4WUPOL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EM4WUPOL_SPEC>;
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
impl From<crate::W<EM4WUPOL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EM4WUPOL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EM4WUPOL` reader - EM4 Wake-up Polarity"]
pub type EM4WUPOL_R = crate::FieldReader<u8, EM4WUPOL_A>;
#[doc = "EM4 Wake-up Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EM4WUPOL_A {
    #[doc = "1: Determines polarity on pin A0"]
    A0 = 1,
    #[doc = "2: Determines polarity on pin A6"]
    A6 = 2,
    #[doc = "4: Determines polarity on pin C9"]
    C9 = 4,
    #[doc = "8: Determines polarity on pin F1"]
    F1 = 8,
    #[doc = "16: Determines polarity on pin F2"]
    F2 = 16,
    #[doc = "32: Determines polarity on pin E13"]
    E13 = 32,
}
impl From<EM4WUPOL_A> for u8 {
    #[inline(always)]
    fn from(variant: EM4WUPOL_A) -> Self {
        variant as _
    }
}
impl EM4WUPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EM4WUPOL_A> {
        match self.bits {
            1 => Some(EM4WUPOL_A::A0),
            2 => Some(EM4WUPOL_A::A6),
            4 => Some(EM4WUPOL_A::C9),
            8 => Some(EM4WUPOL_A::F1),
            16 => Some(EM4WUPOL_A::F2),
            32 => Some(EM4WUPOL_A::E13),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `A0`"]
    #[inline(always)]
    pub fn is_a0(&self) -> bool {
        *self == EM4WUPOL_A::A0
    }
    #[doc = "Checks if the value of the field is `A6`"]
    #[inline(always)]
    pub fn is_a6(&self) -> bool {
        *self == EM4WUPOL_A::A6
    }
    #[doc = "Checks if the value of the field is `C9`"]
    #[inline(always)]
    pub fn is_c9(&self) -> bool {
        *self == EM4WUPOL_A::C9
    }
    #[doc = "Checks if the value of the field is `F1`"]
    #[inline(always)]
    pub fn is_f1(&self) -> bool {
        *self == EM4WUPOL_A::F1
    }
    #[doc = "Checks if the value of the field is `F2`"]
    #[inline(always)]
    pub fn is_f2(&self) -> bool {
        *self == EM4WUPOL_A::F2
    }
    #[doc = "Checks if the value of the field is `E13`"]
    #[inline(always)]
    pub fn is_e13(&self) -> bool {
        *self == EM4WUPOL_A::E13
    }
}
#[doc = "Field `EM4WUPOL` writer - EM4 Wake-up Polarity"]
pub type EM4WUPOL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EM4WUPOL_SPEC, u8, EM4WUPOL_A, 6, O>;
impl<'a, const O: u8> EM4WUPOL_W<'a, O> {
    #[doc = "Determines polarity on pin A0"]
    #[inline(always)]
    pub fn a0(self) -> &'a mut W {
        self.variant(EM4WUPOL_A::A0)
    }
    #[doc = "Determines polarity on pin A6"]
    #[inline(always)]
    pub fn a6(self) -> &'a mut W {
        self.variant(EM4WUPOL_A::A6)
    }
    #[doc = "Determines polarity on pin C9"]
    #[inline(always)]
    pub fn c9(self) -> &'a mut W {
        self.variant(EM4WUPOL_A::C9)
    }
    #[doc = "Determines polarity on pin F1"]
    #[inline(always)]
    pub fn f1(self) -> &'a mut W {
        self.variant(EM4WUPOL_A::F1)
    }
    #[doc = "Determines polarity on pin F2"]
    #[inline(always)]
    pub fn f2(self) -> &'a mut W {
        self.variant(EM4WUPOL_A::F2)
    }
    #[doc = "Determines polarity on pin E13"]
    #[inline(always)]
    pub fn e13(self) -> &'a mut W {
        self.variant(EM4WUPOL_A::E13)
    }
}
impl R {
    #[doc = "Bits 0:5 - EM4 Wake-up Polarity"]
    #[inline(always)]
    pub fn em4wupol(&self) -> EM4WUPOL_R {
        EM4WUPOL_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - EM4 Wake-up Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn em4wupol(&mut self) -> EM4WUPOL_W<0> {
        EM4WUPOL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EM4 Wake-up Polarity Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [em4wupol](index.html) module"]
pub struct EM4WUPOL_SPEC;
impl crate::RegisterSpec for EM4WUPOL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [em4wupol::R](R) reader structure"]
impl crate::Readable for EM4WUPOL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [em4wupol::W](W) writer structure"]
impl crate::Writable for EM4WUPOL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EM4WUPOL to value 0"]
impl crate::Resettable for EM4WUPOL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
