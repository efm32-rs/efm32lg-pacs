#[doc = "Register `HFCORECLKDIV` reader"]
pub struct R(crate::R<HFCORECLKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFCORECLKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFCORECLKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFCORECLKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFCORECLKDIV` writer"]
pub struct W(crate::W<HFCORECLKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFCORECLKDIV_SPEC>;
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
impl From<crate::W<HFCORECLKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFCORECLKDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "HFCORECLK Divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HFCORECLKDIV_A {
    #[doc = "0: HFCORECLK = HFCLK."]
    HFCLK = 0,
    #[doc = "1: HFCORECLK = HFCLK/2."]
    HFCLK2 = 1,
    #[doc = "2: HFCORECLK = HFCLK/4."]
    HFCLK4 = 2,
    #[doc = "3: HFCORECLK = HFCLK/8."]
    HFCLK8 = 3,
    #[doc = "4: HFCORECLK = HFCLK/16."]
    HFCLK16 = 4,
    #[doc = "5: HFCORECLK = HFCLK/32."]
    HFCLK32 = 5,
    #[doc = "6: HFCORECLK = HFCLK/64."]
    HFCLK64 = 6,
    #[doc = "7: HFCORECLK = HFCLK/128."]
    HFCLK128 = 7,
    #[doc = "8: HFCORECLK = HFCLK/256."]
    HFCLK256 = 8,
    #[doc = "9: HFCORECLK = HFCLK/512."]
    HFCLK512 = 9,
}
impl From<HFCORECLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: HFCORECLKDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HFCORECLKDIV` reader - HFCORECLK Divider"]
pub type HFCORECLKDIV_R = crate::FieldReader<u8, HFCORECLKDIV_A>;
impl HFCORECLKDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HFCORECLKDIV_A> {
        match self.bits {
            0 => Some(HFCORECLKDIV_A::HFCLK),
            1 => Some(HFCORECLKDIV_A::HFCLK2),
            2 => Some(HFCORECLKDIV_A::HFCLK4),
            3 => Some(HFCORECLKDIV_A::HFCLK8),
            4 => Some(HFCORECLKDIV_A::HFCLK16),
            5 => Some(HFCORECLKDIV_A::HFCLK32),
            6 => Some(HFCORECLKDIV_A::HFCLK64),
            7 => Some(HFCORECLKDIV_A::HFCLK128),
            8 => Some(HFCORECLKDIV_A::HFCLK256),
            9 => Some(HFCORECLKDIV_A::HFCLK512),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HFCLK`"]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        *self == HFCORECLKDIV_A::HFCLK
    }
    #[doc = "Checks if the value of the field is `HFCLK2`"]
    #[inline(always)]
    pub fn is_hfclk2(&self) -> bool {
        *self == HFCORECLKDIV_A::HFCLK2
    }
    #[doc = "Checks if the value of the field is `HFCLK4`"]
    #[inline(always)]
    pub fn is_hfclk4(&self) -> bool {
        *self == HFCORECLKDIV_A::HFCLK4
    }
    #[doc = "Checks if the value of the field is `HFCLK8`"]
    #[inline(always)]
    pub fn is_hfclk8(&self) -> bool {
        *self == HFCORECLKDIV_A::HFCLK8
    }
    #[doc = "Checks if the value of the field is `HFCLK16`"]
    #[inline(always)]
    pub fn is_hfclk16(&self) -> bool {
        *self == HFCORECLKDIV_A::HFCLK16
    }
    #[doc = "Checks if the value of the field is `HFCLK32`"]
    #[inline(always)]
    pub fn is_hfclk32(&self) -> bool {
        *self == HFCORECLKDIV_A::HFCLK32
    }
    #[doc = "Checks if the value of the field is `HFCLK64`"]
    #[inline(always)]
    pub fn is_hfclk64(&self) -> bool {
        *self == HFCORECLKDIV_A::HFCLK64
    }
    #[doc = "Checks if the value of the field is `HFCLK128`"]
    #[inline(always)]
    pub fn is_hfclk128(&self) -> bool {
        *self == HFCORECLKDIV_A::HFCLK128
    }
    #[doc = "Checks if the value of the field is `HFCLK256`"]
    #[inline(always)]
    pub fn is_hfclk256(&self) -> bool {
        *self == HFCORECLKDIV_A::HFCLK256
    }
    #[doc = "Checks if the value of the field is `HFCLK512`"]
    #[inline(always)]
    pub fn is_hfclk512(&self) -> bool {
        *self == HFCORECLKDIV_A::HFCLK512
    }
}
#[doc = "Field `HFCORECLKDIV` writer - HFCORECLK Divider"]
pub type HFCORECLKDIV_W<'a> =
    crate::FieldWriter<'a, u32, HFCORECLKDIV_SPEC, u8, HFCORECLKDIV_A, 4, 0>;
impl<'a> HFCORECLKDIV_W<'a> {
    #[doc = "HFCORECLK = HFCLK."]
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut W {
        self.variant(HFCORECLKDIV_A::HFCLK)
    }
    #[doc = "HFCORECLK = HFCLK/2."]
    #[inline(always)]
    pub fn hfclk2(self) -> &'a mut W {
        self.variant(HFCORECLKDIV_A::HFCLK2)
    }
    #[doc = "HFCORECLK = HFCLK/4."]
    #[inline(always)]
    pub fn hfclk4(self) -> &'a mut W {
        self.variant(HFCORECLKDIV_A::HFCLK4)
    }
    #[doc = "HFCORECLK = HFCLK/8."]
    #[inline(always)]
    pub fn hfclk8(self) -> &'a mut W {
        self.variant(HFCORECLKDIV_A::HFCLK8)
    }
    #[doc = "HFCORECLK = HFCLK/16."]
    #[inline(always)]
    pub fn hfclk16(self) -> &'a mut W {
        self.variant(HFCORECLKDIV_A::HFCLK16)
    }
    #[doc = "HFCORECLK = HFCLK/32."]
    #[inline(always)]
    pub fn hfclk32(self) -> &'a mut W {
        self.variant(HFCORECLKDIV_A::HFCLK32)
    }
    #[doc = "HFCORECLK = HFCLK/64."]
    #[inline(always)]
    pub fn hfclk64(self) -> &'a mut W {
        self.variant(HFCORECLKDIV_A::HFCLK64)
    }
    #[doc = "HFCORECLK = HFCLK/128."]
    #[inline(always)]
    pub fn hfclk128(self) -> &'a mut W {
        self.variant(HFCORECLKDIV_A::HFCLK128)
    }
    #[doc = "HFCORECLK = HFCLK/256."]
    #[inline(always)]
    pub fn hfclk256(self) -> &'a mut W {
        self.variant(HFCORECLKDIV_A::HFCLK256)
    }
    #[doc = "HFCORECLK = HFCLK/512."]
    #[inline(always)]
    pub fn hfclk512(self) -> &'a mut W {
        self.variant(HFCORECLKDIV_A::HFCLK512)
    }
}
#[doc = "Field `HFCORECLKLEDIV` reader - Additional Division Factor For HFCORECLKLE"]
pub type HFCORECLKLEDIV_R = crate::BitReader<bool>;
#[doc = "Field `HFCORECLKLEDIV` writer - Additional Division Factor For HFCORECLKLE"]
pub type HFCORECLKLEDIV_W<'a> = crate::BitWriter<'a, u32, HFCORECLKDIV_SPEC, bool, 8>;
impl R {
    #[doc = "Bits 0:3 - HFCORECLK Divider"]
    #[inline(always)]
    pub fn hfcoreclkdiv(&self) -> HFCORECLKDIV_R {
        HFCORECLKDIV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Additional Division Factor For HFCORECLKLE"]
    #[inline(always)]
    pub fn hfcoreclklediv(&self) -> HFCORECLKLEDIV_R {
        HFCORECLKLEDIV_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - HFCORECLK Divider"]
    #[inline(always)]
    pub fn hfcoreclkdiv(&mut self) -> HFCORECLKDIV_W {
        HFCORECLKDIV_W::new(self)
    }
    #[doc = "Bit 8 - Additional Division Factor For HFCORECLKLE"]
    #[inline(always)]
    pub fn hfcoreclklediv(&mut self) -> HFCORECLKLEDIV_W {
        HFCORECLKLEDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "High Frequency Core Clock Division Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfcoreclkdiv](index.html) module"]
pub struct HFCORECLKDIV_SPEC;
impl crate::RegisterSpec for HFCORECLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfcoreclkdiv::R](R) reader structure"]
impl crate::Readable for HFCORECLKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfcoreclkdiv::W](W) writer structure"]
impl crate::Writable for HFCORECLKDIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HFCORECLKDIV to value 0"]
impl crate::Resettable for HFCORECLKDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
