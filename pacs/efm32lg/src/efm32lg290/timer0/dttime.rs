#[doc = "Register `DTTIME` reader"]
pub struct R(crate::R<DTTIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTTIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTTIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTTIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTTIME` writer"]
pub struct W(crate::W<DTTIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTTIME_SPEC>;
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
impl From<crate::W<DTTIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTTIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DTI Prescaler Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DTPRESC_A {
    #[doc = "0: The HFPERCLK is undivided"]
    DIV1 = 0,
    #[doc = "1: The HFPERCLK is divided by 2"]
    DIV2 = 1,
    #[doc = "2: The HFPERCLK is divided by 4"]
    DIV4 = 2,
    #[doc = "3: The HFPERCLK is divided by 8"]
    DIV8 = 3,
    #[doc = "4: The HFPERCLK is divided by 16"]
    DIV16 = 4,
    #[doc = "5: The HFPERCLK is divided by 32"]
    DIV32 = 5,
    #[doc = "6: The HFPERCLK is divided by 64"]
    DIV64 = 6,
    #[doc = "7: The HFPERCLK is divided by 128"]
    DIV128 = 7,
    #[doc = "8: The HFPERCLK is divided by 256"]
    DIV256 = 8,
    #[doc = "9: The HFPERCLK is divided by 512"]
    DIV512 = 9,
    #[doc = "10: The HFPERCLK is divided by 1024"]
    DIV1024 = 10,
}
impl From<DTPRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: DTPRESC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DTPRESC` reader - DTI Prescaler Setting"]
pub type DTPRESC_R = crate::FieldReader<u8, DTPRESC_A>;
impl DTPRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DTPRESC_A> {
        match self.bits {
            0 => Some(DTPRESC_A::DIV1),
            1 => Some(DTPRESC_A::DIV2),
            2 => Some(DTPRESC_A::DIV4),
            3 => Some(DTPRESC_A::DIV8),
            4 => Some(DTPRESC_A::DIV16),
            5 => Some(DTPRESC_A::DIV32),
            6 => Some(DTPRESC_A::DIV64),
            7 => Some(DTPRESC_A::DIV128),
            8 => Some(DTPRESC_A::DIV256),
            9 => Some(DTPRESC_A::DIV512),
            10 => Some(DTPRESC_A::DIV1024),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == DTPRESC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == DTPRESC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == DTPRESC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == DTPRESC_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == DTPRESC_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == DTPRESC_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == DTPRESC_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == DTPRESC_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == DTPRESC_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == DTPRESC_A::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == DTPRESC_A::DIV1024
    }
}
#[doc = "Field `DTPRESC` writer - DTI Prescaler Setting"]
pub type DTPRESC_W<'a> = crate::FieldWriter<'a, u32, DTTIME_SPEC, u8, DTPRESC_A, 4, 0>;
impl<'a> DTPRESC_W<'a> {
    #[doc = "The HFPERCLK is undivided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(DTPRESC_A::DIV1)
    }
    #[doc = "The HFPERCLK is divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(DTPRESC_A::DIV2)
    }
    #[doc = "The HFPERCLK is divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(DTPRESC_A::DIV4)
    }
    #[doc = "The HFPERCLK is divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(DTPRESC_A::DIV8)
    }
    #[doc = "The HFPERCLK is divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(DTPRESC_A::DIV16)
    }
    #[doc = "The HFPERCLK is divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(DTPRESC_A::DIV32)
    }
    #[doc = "The HFPERCLK is divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(DTPRESC_A::DIV64)
    }
    #[doc = "The HFPERCLK is divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(DTPRESC_A::DIV128)
    }
    #[doc = "The HFPERCLK is divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(DTPRESC_A::DIV256)
    }
    #[doc = "The HFPERCLK is divided by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(DTPRESC_A::DIV512)
    }
    #[doc = "The HFPERCLK is divided by 1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut W {
        self.variant(DTPRESC_A::DIV1024)
    }
}
#[doc = "Field `DTRISET` reader - DTI Rise-time"]
pub type DTRISET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTRISET` writer - DTI Rise-time"]
pub type DTRISET_W<'a> = crate::FieldWriter<'a, u32, DTTIME_SPEC, u8, u8, 6, 8>;
#[doc = "Field `DTFALLT` reader - DTI Fall-time"]
pub type DTFALLT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTFALLT` writer - DTI Fall-time"]
pub type DTFALLT_W<'a> = crate::FieldWriter<'a, u32, DTTIME_SPEC, u8, u8, 6, 16>;
impl R {
    #[doc = "Bits 0:3 - DTI Prescaler Setting"]
    #[inline(always)]
    pub fn dtpresc(&self) -> DTPRESC_R {
        DTPRESC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - DTI Rise-time"]
    #[inline(always)]
    pub fn dtriset(&self) -> DTRISET_R {
        DTRISET_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - DTI Fall-time"]
    #[inline(always)]
    pub fn dtfallt(&self) -> DTFALLT_R {
        DTFALLT_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DTI Prescaler Setting"]
    #[inline(always)]
    pub fn dtpresc(&mut self) -> DTPRESC_W {
        DTPRESC_W::new(self)
    }
    #[doc = "Bits 8:13 - DTI Rise-time"]
    #[inline(always)]
    pub fn dtriset(&mut self) -> DTRISET_W {
        DTRISET_W::new(self)
    }
    #[doc = "Bits 16:21 - DTI Fall-time"]
    #[inline(always)]
    pub fn dtfallt(&mut self) -> DTFALLT_W {
        DTFALLT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DTI Time Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dttime](index.html) module"]
pub struct DTTIME_SPEC;
impl crate::RegisterSpec for DTTIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dttime::R](R) reader structure"]
impl crate::Readable for DTTIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dttime::W](W) writer structure"]
impl crate::Writable for DTTIME_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DTTIME to value 0"]
impl crate::Resettable for DTTIME_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
