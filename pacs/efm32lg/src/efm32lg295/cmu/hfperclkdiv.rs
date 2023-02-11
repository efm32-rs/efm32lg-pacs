#[doc = "Register `HFPERCLKDIV` reader"]
pub struct R(crate::R<HFPERCLKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFPERCLKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFPERCLKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFPERCLKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFPERCLKDIV` writer"]
pub struct W(crate::W<HFPERCLKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFPERCLKDIV_SPEC>;
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
impl From<crate::W<HFPERCLKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFPERCLKDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HFPERCLKDIV` reader - HFPERCLK Divider"]
pub type HFPERCLKDIV_R = crate::FieldReader<u8, HFPERCLKDIV_A>;
#[doc = "HFPERCLK Divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HFPERCLKDIV_A {
    #[doc = "0: HFPERCLK = HFCLK."]
    HFCLK = 0,
    #[doc = "1: HFPERCLK = HFCLK/2."]
    HFCLK2 = 1,
    #[doc = "2: HFPERCLK = HFCLK/4."]
    HFCLK4 = 2,
    #[doc = "3: HFPERCLK = HFCLK/8."]
    HFCLK8 = 3,
    #[doc = "4: HFPERCLK = HFCLK/16."]
    HFCLK16 = 4,
    #[doc = "5: HFPERCLK = HFCLK/32."]
    HFCLK32 = 5,
    #[doc = "6: HFPERCLK = HFCLK/64."]
    HFCLK64 = 6,
    #[doc = "7: HFPERCLK = HFCLK/128."]
    HFCLK128 = 7,
    #[doc = "8: HFPERCLK = HFCLK/256."]
    HFCLK256 = 8,
    #[doc = "9: HFPERCLK = HFCLK/512."]
    HFCLK512 = 9,
}
impl From<HFPERCLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: HFPERCLKDIV_A) -> Self {
        variant as _
    }
}
impl HFPERCLKDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HFPERCLKDIV_A> {
        match self.bits {
            0 => Some(HFPERCLKDIV_A::HFCLK),
            1 => Some(HFPERCLKDIV_A::HFCLK2),
            2 => Some(HFPERCLKDIV_A::HFCLK4),
            3 => Some(HFPERCLKDIV_A::HFCLK8),
            4 => Some(HFPERCLKDIV_A::HFCLK16),
            5 => Some(HFPERCLKDIV_A::HFCLK32),
            6 => Some(HFPERCLKDIV_A::HFCLK64),
            7 => Some(HFPERCLKDIV_A::HFCLK128),
            8 => Some(HFPERCLKDIV_A::HFCLK256),
            9 => Some(HFPERCLKDIV_A::HFCLK512),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HFCLK`"]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        *self == HFPERCLKDIV_A::HFCLK
    }
    #[doc = "Checks if the value of the field is `HFCLK2`"]
    #[inline(always)]
    pub fn is_hfclk2(&self) -> bool {
        *self == HFPERCLKDIV_A::HFCLK2
    }
    #[doc = "Checks if the value of the field is `HFCLK4`"]
    #[inline(always)]
    pub fn is_hfclk4(&self) -> bool {
        *self == HFPERCLKDIV_A::HFCLK4
    }
    #[doc = "Checks if the value of the field is `HFCLK8`"]
    #[inline(always)]
    pub fn is_hfclk8(&self) -> bool {
        *self == HFPERCLKDIV_A::HFCLK8
    }
    #[doc = "Checks if the value of the field is `HFCLK16`"]
    #[inline(always)]
    pub fn is_hfclk16(&self) -> bool {
        *self == HFPERCLKDIV_A::HFCLK16
    }
    #[doc = "Checks if the value of the field is `HFCLK32`"]
    #[inline(always)]
    pub fn is_hfclk32(&self) -> bool {
        *self == HFPERCLKDIV_A::HFCLK32
    }
    #[doc = "Checks if the value of the field is `HFCLK64`"]
    #[inline(always)]
    pub fn is_hfclk64(&self) -> bool {
        *self == HFPERCLKDIV_A::HFCLK64
    }
    #[doc = "Checks if the value of the field is `HFCLK128`"]
    #[inline(always)]
    pub fn is_hfclk128(&self) -> bool {
        *self == HFPERCLKDIV_A::HFCLK128
    }
    #[doc = "Checks if the value of the field is `HFCLK256`"]
    #[inline(always)]
    pub fn is_hfclk256(&self) -> bool {
        *self == HFPERCLKDIV_A::HFCLK256
    }
    #[doc = "Checks if the value of the field is `HFCLK512`"]
    #[inline(always)]
    pub fn is_hfclk512(&self) -> bool {
        *self == HFPERCLKDIV_A::HFCLK512
    }
}
#[doc = "Field `HFPERCLKDIV` writer - HFPERCLK Divider"]
pub type HFPERCLKDIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HFPERCLKDIV_SPEC, u8, HFPERCLKDIV_A, 4, O>;
impl<'a, const O: u8> HFPERCLKDIV_W<'a, O> {
    #[doc = "HFPERCLK = HFCLK."]
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut W {
        self.variant(HFPERCLKDIV_A::HFCLK)
    }
    #[doc = "HFPERCLK = HFCLK/2."]
    #[inline(always)]
    pub fn hfclk2(self) -> &'a mut W {
        self.variant(HFPERCLKDIV_A::HFCLK2)
    }
    #[doc = "HFPERCLK = HFCLK/4."]
    #[inline(always)]
    pub fn hfclk4(self) -> &'a mut W {
        self.variant(HFPERCLKDIV_A::HFCLK4)
    }
    #[doc = "HFPERCLK = HFCLK/8."]
    #[inline(always)]
    pub fn hfclk8(self) -> &'a mut W {
        self.variant(HFPERCLKDIV_A::HFCLK8)
    }
    #[doc = "HFPERCLK = HFCLK/16."]
    #[inline(always)]
    pub fn hfclk16(self) -> &'a mut W {
        self.variant(HFPERCLKDIV_A::HFCLK16)
    }
    #[doc = "HFPERCLK = HFCLK/32."]
    #[inline(always)]
    pub fn hfclk32(self) -> &'a mut W {
        self.variant(HFPERCLKDIV_A::HFCLK32)
    }
    #[doc = "HFPERCLK = HFCLK/64."]
    #[inline(always)]
    pub fn hfclk64(self) -> &'a mut W {
        self.variant(HFPERCLKDIV_A::HFCLK64)
    }
    #[doc = "HFPERCLK = HFCLK/128."]
    #[inline(always)]
    pub fn hfclk128(self) -> &'a mut W {
        self.variant(HFPERCLKDIV_A::HFCLK128)
    }
    #[doc = "HFPERCLK = HFCLK/256."]
    #[inline(always)]
    pub fn hfclk256(self) -> &'a mut W {
        self.variant(HFPERCLKDIV_A::HFCLK256)
    }
    #[doc = "HFPERCLK = HFCLK/512."]
    #[inline(always)]
    pub fn hfclk512(self) -> &'a mut W {
        self.variant(HFPERCLKDIV_A::HFCLK512)
    }
}
#[doc = "Field `HFPERCLKEN` reader - HFPERCLK Enable"]
pub type HFPERCLKEN_R = crate::BitReader<bool>;
#[doc = "Field `HFPERCLKEN` writer - HFPERCLK Enable"]
pub type HFPERCLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HFPERCLKDIV_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - HFPERCLK Divider"]
    #[inline(always)]
    pub fn hfperclkdiv(&self) -> HFPERCLKDIV_R {
        HFPERCLKDIV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - HFPERCLK Enable"]
    #[inline(always)]
    pub fn hfperclken(&self) -> HFPERCLKEN_R {
        HFPERCLKEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - HFPERCLK Divider"]
    #[inline(always)]
    #[must_use]
    pub fn hfperclkdiv(&mut self) -> HFPERCLKDIV_W<0> {
        HFPERCLKDIV_W::new(self)
    }
    #[doc = "Bit 8 - HFPERCLK Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hfperclken(&mut self) -> HFPERCLKEN_W<8> {
        HFPERCLKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "High Frequency Peripheral Clock Division Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfperclkdiv](index.html) module"]
pub struct HFPERCLKDIV_SPEC;
impl crate::RegisterSpec for HFPERCLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfperclkdiv::R](R) reader structure"]
impl crate::Readable for HFPERCLKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfperclkdiv::W](W) writer structure"]
impl crate::Writable for HFPERCLKDIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HFPERCLKDIV to value 0x0100"]
impl crate::Resettable for HFPERCLKDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
