#[doc = "Register `EM4WUCAUSE` reader"]
pub struct R(crate::R<EM4WUCAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EM4WUCAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EM4WUCAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EM4WUCAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "EM4 wake-up cause\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EM4WUCAUSE_A {
    #[doc = "1: This bit indicates an em4 wake-up request occurred on pin A0"]
    A0 = 1,
    #[doc = "2: This bit indicates an em4 wake-up request occurred on pin A6"]
    A6 = 2,
    #[doc = "4: This bit indicates an em4 wake-up request occurred on pin C9"]
    C9 = 4,
    #[doc = "8: This bit indicates an em4 wake-up request occurred on pin F1"]
    F1 = 8,
    #[doc = "16: This bit indicates an em4 wake-up request occurred on pin F2"]
    F2 = 16,
    #[doc = "32: This bit indicates an em4 wake-up request occurred on pin E13"]
    E13 = 32,
}
impl From<EM4WUCAUSE_A> for u8 {
    #[inline(always)]
    fn from(variant: EM4WUCAUSE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EM4WUCAUSE` reader - EM4 wake-up cause"]
pub type EM4WUCAUSE_R = crate::FieldReader<u8, EM4WUCAUSE_A>;
impl EM4WUCAUSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EM4WUCAUSE_A> {
        match self.bits {
            1 => Some(EM4WUCAUSE_A::A0),
            2 => Some(EM4WUCAUSE_A::A6),
            4 => Some(EM4WUCAUSE_A::C9),
            8 => Some(EM4WUCAUSE_A::F1),
            16 => Some(EM4WUCAUSE_A::F2),
            32 => Some(EM4WUCAUSE_A::E13),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `A0`"]
    #[inline(always)]
    pub fn is_a0(&self) -> bool {
        *self == EM4WUCAUSE_A::A0
    }
    #[doc = "Checks if the value of the field is `A6`"]
    #[inline(always)]
    pub fn is_a6(&self) -> bool {
        *self == EM4WUCAUSE_A::A6
    }
    #[doc = "Checks if the value of the field is `C9`"]
    #[inline(always)]
    pub fn is_c9(&self) -> bool {
        *self == EM4WUCAUSE_A::C9
    }
    #[doc = "Checks if the value of the field is `F1`"]
    #[inline(always)]
    pub fn is_f1(&self) -> bool {
        *self == EM4WUCAUSE_A::F1
    }
    #[doc = "Checks if the value of the field is `F2`"]
    #[inline(always)]
    pub fn is_f2(&self) -> bool {
        *self == EM4WUCAUSE_A::F2
    }
    #[doc = "Checks if the value of the field is `E13`"]
    #[inline(always)]
    pub fn is_e13(&self) -> bool {
        *self == EM4WUCAUSE_A::E13
    }
}
impl R {
    #[doc = "Bits 0:5 - EM4 wake-up cause"]
    #[inline(always)]
    pub fn em4wucause(&self) -> EM4WUCAUSE_R {
        EM4WUCAUSE_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "EM4 Wake-up Cause Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [em4wucause](index.html) module"]
pub struct EM4WUCAUSE_SPEC;
impl crate::RegisterSpec for EM4WUCAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [em4wucause::R](R) reader structure"]
impl crate::Readable for EM4WUCAUSE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EM4WUCAUSE to value 0"]
impl crate::Resettable for EM4WUCAUSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
