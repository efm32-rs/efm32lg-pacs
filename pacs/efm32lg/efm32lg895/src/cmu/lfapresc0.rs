#[doc = "Register `LFAPRESC0` reader"]
pub struct R(crate::R<LFAPRESC0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LFAPRESC0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LFAPRESC0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LFAPRESC0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LFAPRESC0` writer"]
pub struct W(crate::W<LFAPRESC0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LFAPRESC0_SPEC>;
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
impl From<crate::W<LFAPRESC0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LFAPRESC0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Low Energy Sensor Interface Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LESENSE_A {
    #[doc = "0: LFACLKLESENSE = LFACLK"]
    DIV1 = 0,
    #[doc = "1: LFACLKLESENSE = LFACLK/2"]
    DIV2 = 1,
    #[doc = "2: LFACLKLESENSE = LFACLK/4"]
    DIV4 = 2,
    #[doc = "3: LFACLKLESENSE = LFACLK/8"]
    DIV8 = 3,
}
impl From<LESENSE_A> for u8 {
    #[inline(always)]
    fn from(variant: LESENSE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LESENSE` reader - Low Energy Sensor Interface Prescaler"]
pub type LESENSE_R = crate::FieldReader<u8, LESENSE_A>;
impl LESENSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LESENSE_A {
        match self.bits {
            0 => LESENSE_A::DIV1,
            1 => LESENSE_A::DIV2,
            2 => LESENSE_A::DIV4,
            3 => LESENSE_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == LESENSE_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == LESENSE_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == LESENSE_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == LESENSE_A::DIV8
    }
}
#[doc = "Field `LESENSE` writer - Low Energy Sensor Interface Prescaler"]
pub type LESENSE_W<'a> = crate::FieldWriterSafe<'a, u32, LFAPRESC0_SPEC, u8, LESENSE_A, 2, 0>;
impl<'a> LESENSE_W<'a> {
    #[doc = "LFACLKLESENSE = LFACLK"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(LESENSE_A::DIV1)
    }
    #[doc = "LFACLKLESENSE = LFACLK/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(LESENSE_A::DIV2)
    }
    #[doc = "LFACLKLESENSE = LFACLK/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(LESENSE_A::DIV4)
    }
    #[doc = "LFACLKLESENSE = LFACLK/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(LESENSE_A::DIV8)
    }
}
#[doc = "Real-Time Counter Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTC_A {
    #[doc = "0: LFACLKRTC = LFACLK"]
    DIV1 = 0,
    #[doc = "1: LFACLKRTC = LFACLK/2"]
    DIV2 = 1,
    #[doc = "2: LFACLKRTC = LFACLK/4"]
    DIV4 = 2,
    #[doc = "3: LFACLKRTC = LFACLK/8"]
    DIV8 = 3,
    #[doc = "4: LFACLKRTC = LFACLK/16"]
    DIV16 = 4,
    #[doc = "5: LFACLKRTC = LFACLK/32"]
    DIV32 = 5,
    #[doc = "6: LFACLKRTC = LFACLK/64"]
    DIV64 = 6,
    #[doc = "7: LFACLKRTC = LFACLK/128"]
    DIV128 = 7,
    #[doc = "8: LFACLKRTC = LFACLK/256"]
    DIV256 = 8,
    #[doc = "9: LFACLKRTC = LFACLK/512"]
    DIV512 = 9,
    #[doc = "10: LFACLKRTC = LFACLK/1024"]
    DIV1024 = 10,
    #[doc = "11: LFACLKRTC = LFACLK/2048"]
    DIV2048 = 11,
    #[doc = "12: LFACLKRTC = LFACLK/4096"]
    DIV4096 = 12,
    #[doc = "13: LFACLKRTC = LFACLK/8192"]
    DIV8192 = 13,
    #[doc = "14: LFACLKRTC = LFACLK/16384"]
    DIV16384 = 14,
    #[doc = "15: LFACLKRTC = LFACLK/32768"]
    DIV32768 = 15,
}
impl From<RTC_A> for u8 {
    #[inline(always)]
    fn from(variant: RTC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RTC` reader - Real-Time Counter Prescaler"]
pub type RTC_R = crate::FieldReader<u8, RTC_A>;
impl RTC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_A {
        match self.bits {
            0 => RTC_A::DIV1,
            1 => RTC_A::DIV2,
            2 => RTC_A::DIV4,
            3 => RTC_A::DIV8,
            4 => RTC_A::DIV16,
            5 => RTC_A::DIV32,
            6 => RTC_A::DIV64,
            7 => RTC_A::DIV128,
            8 => RTC_A::DIV256,
            9 => RTC_A::DIV512,
            10 => RTC_A::DIV1024,
            11 => RTC_A::DIV2048,
            12 => RTC_A::DIV4096,
            13 => RTC_A::DIV8192,
            14 => RTC_A::DIV16384,
            15 => RTC_A::DIV32768,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == RTC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == RTC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == RTC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == RTC_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == RTC_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == RTC_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == RTC_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == RTC_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == RTC_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == RTC_A::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == RTC_A::DIV1024
    }
    #[doc = "Checks if the value of the field is `DIV2048`"]
    #[inline(always)]
    pub fn is_div2048(&self) -> bool {
        *self == RTC_A::DIV2048
    }
    #[doc = "Checks if the value of the field is `DIV4096`"]
    #[inline(always)]
    pub fn is_div4096(&self) -> bool {
        *self == RTC_A::DIV4096
    }
    #[doc = "Checks if the value of the field is `DIV8192`"]
    #[inline(always)]
    pub fn is_div8192(&self) -> bool {
        *self == RTC_A::DIV8192
    }
    #[doc = "Checks if the value of the field is `DIV16384`"]
    #[inline(always)]
    pub fn is_div16384(&self) -> bool {
        *self == RTC_A::DIV16384
    }
    #[doc = "Checks if the value of the field is `DIV32768`"]
    #[inline(always)]
    pub fn is_div32768(&self) -> bool {
        *self == RTC_A::DIV32768
    }
}
#[doc = "Field `RTC` writer - Real-Time Counter Prescaler"]
pub type RTC_W<'a> = crate::FieldWriterSafe<'a, u32, LFAPRESC0_SPEC, u8, RTC_A, 4, 4>;
impl<'a> RTC_W<'a> {
    #[doc = "LFACLKRTC = LFACLK"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(RTC_A::DIV1)
    }
    #[doc = "LFACLKRTC = LFACLK/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(RTC_A::DIV2)
    }
    #[doc = "LFACLKRTC = LFACLK/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(RTC_A::DIV4)
    }
    #[doc = "LFACLKRTC = LFACLK/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(RTC_A::DIV8)
    }
    #[doc = "LFACLKRTC = LFACLK/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(RTC_A::DIV16)
    }
    #[doc = "LFACLKRTC = LFACLK/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(RTC_A::DIV32)
    }
    #[doc = "LFACLKRTC = LFACLK/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(RTC_A::DIV64)
    }
    #[doc = "LFACLKRTC = LFACLK/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(RTC_A::DIV128)
    }
    #[doc = "LFACLKRTC = LFACLK/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(RTC_A::DIV256)
    }
    #[doc = "LFACLKRTC = LFACLK/512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(RTC_A::DIV512)
    }
    #[doc = "LFACLKRTC = LFACLK/1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut W {
        self.variant(RTC_A::DIV1024)
    }
    #[doc = "LFACLKRTC = LFACLK/2048"]
    #[inline(always)]
    pub fn div2048(self) -> &'a mut W {
        self.variant(RTC_A::DIV2048)
    }
    #[doc = "LFACLKRTC = LFACLK/4096"]
    #[inline(always)]
    pub fn div4096(self) -> &'a mut W {
        self.variant(RTC_A::DIV4096)
    }
    #[doc = "LFACLKRTC = LFACLK/8192"]
    #[inline(always)]
    pub fn div8192(self) -> &'a mut W {
        self.variant(RTC_A::DIV8192)
    }
    #[doc = "LFACLKRTC = LFACLK/16384"]
    #[inline(always)]
    pub fn div16384(self) -> &'a mut W {
        self.variant(RTC_A::DIV16384)
    }
    #[doc = "LFACLKRTC = LFACLK/32768"]
    #[inline(always)]
    pub fn div32768(self) -> &'a mut W {
        self.variant(RTC_A::DIV32768)
    }
}
#[doc = "Low Energy Timer 0 Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LETIMER0_A {
    #[doc = "0: LFACLKLETIMER0 = LFACLK"]
    DIV1 = 0,
    #[doc = "1: LFACLKLETIMER0 = LFACLK/2"]
    DIV2 = 1,
    #[doc = "2: LFACLKLETIMER0 = LFACLK/4"]
    DIV4 = 2,
    #[doc = "3: LFACLKLETIMER0 = LFACLK/8"]
    DIV8 = 3,
    #[doc = "4: LFACLKLETIMER0 = LFACLK/16"]
    DIV16 = 4,
    #[doc = "5: LFACLKLETIMER0 = LFACLK/32"]
    DIV32 = 5,
    #[doc = "6: LFACLKLETIMER0 = LFACLK/64"]
    DIV64 = 6,
    #[doc = "7: LFACLKLETIMER0 = LFACLK/128"]
    DIV128 = 7,
    #[doc = "8: LFACLKLETIMER0 = LFACLK/256"]
    DIV256 = 8,
    #[doc = "9: LFACLKLETIMER0 = LFACLK/512"]
    DIV512 = 9,
    #[doc = "10: LFACLKLETIMER0 = LFACLK/1024"]
    DIV1024 = 10,
    #[doc = "11: LFACLKLETIMER0 = LFACLK/2048"]
    DIV2048 = 11,
    #[doc = "12: LFACLKLETIMER0 = LFACLK/4096"]
    DIV4096 = 12,
    #[doc = "13: LFACLKLETIMER0 = LFACLK/8192"]
    DIV8192 = 13,
    #[doc = "14: LFACLKLETIMER0 = LFACLK/16384"]
    DIV16384 = 14,
    #[doc = "15: LFACLKLETIMER0 = LFACLK/32768"]
    DIV32768 = 15,
}
impl From<LETIMER0_A> for u8 {
    #[inline(always)]
    fn from(variant: LETIMER0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LETIMER0` reader - Low Energy Timer 0 Prescaler"]
pub type LETIMER0_R = crate::FieldReader<u8, LETIMER0_A>;
impl LETIMER0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LETIMER0_A {
        match self.bits {
            0 => LETIMER0_A::DIV1,
            1 => LETIMER0_A::DIV2,
            2 => LETIMER0_A::DIV4,
            3 => LETIMER0_A::DIV8,
            4 => LETIMER0_A::DIV16,
            5 => LETIMER0_A::DIV32,
            6 => LETIMER0_A::DIV64,
            7 => LETIMER0_A::DIV128,
            8 => LETIMER0_A::DIV256,
            9 => LETIMER0_A::DIV512,
            10 => LETIMER0_A::DIV1024,
            11 => LETIMER0_A::DIV2048,
            12 => LETIMER0_A::DIV4096,
            13 => LETIMER0_A::DIV8192,
            14 => LETIMER0_A::DIV16384,
            15 => LETIMER0_A::DIV32768,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == LETIMER0_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == LETIMER0_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == LETIMER0_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == LETIMER0_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == LETIMER0_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == LETIMER0_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == LETIMER0_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == LETIMER0_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == LETIMER0_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == LETIMER0_A::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == LETIMER0_A::DIV1024
    }
    #[doc = "Checks if the value of the field is `DIV2048`"]
    #[inline(always)]
    pub fn is_div2048(&self) -> bool {
        *self == LETIMER0_A::DIV2048
    }
    #[doc = "Checks if the value of the field is `DIV4096`"]
    #[inline(always)]
    pub fn is_div4096(&self) -> bool {
        *self == LETIMER0_A::DIV4096
    }
    #[doc = "Checks if the value of the field is `DIV8192`"]
    #[inline(always)]
    pub fn is_div8192(&self) -> bool {
        *self == LETIMER0_A::DIV8192
    }
    #[doc = "Checks if the value of the field is `DIV16384`"]
    #[inline(always)]
    pub fn is_div16384(&self) -> bool {
        *self == LETIMER0_A::DIV16384
    }
    #[doc = "Checks if the value of the field is `DIV32768`"]
    #[inline(always)]
    pub fn is_div32768(&self) -> bool {
        *self == LETIMER0_A::DIV32768
    }
}
#[doc = "Field `LETIMER0` writer - Low Energy Timer 0 Prescaler"]
pub type LETIMER0_W<'a> = crate::FieldWriterSafe<'a, u32, LFAPRESC0_SPEC, u8, LETIMER0_A, 4, 8>;
impl<'a> LETIMER0_W<'a> {
    #[doc = "LFACLKLETIMER0 = LFACLK"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV1)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV2)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV4)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV8)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV16)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV32)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV64)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV128)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV256)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV512)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV1024)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/2048"]
    #[inline(always)]
    pub fn div2048(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV2048)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/4096"]
    #[inline(always)]
    pub fn div4096(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV4096)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/8192"]
    #[inline(always)]
    pub fn div8192(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV8192)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/16384"]
    #[inline(always)]
    pub fn div16384(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV16384)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/32768"]
    #[inline(always)]
    pub fn div32768(self) -> &'a mut W {
        self.variant(LETIMER0_A::DIV32768)
    }
}
#[doc = "Liquid Crystal Display Controller Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LCD_A {
    #[doc = "0: LFACLKLCD = LFACLK/16"]
    DIV16 = 0,
    #[doc = "1: LFACLKLCD = LFACLK/32"]
    DIV32 = 1,
    #[doc = "2: LFACLKLCD = LFACLK/64"]
    DIV64 = 2,
    #[doc = "3: LFACLKLCD = LFACLK/128"]
    DIV128 = 3,
}
impl From<LCD_A> for u8 {
    #[inline(always)]
    fn from(variant: LCD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LCD` reader - Liquid Crystal Display Controller Prescaler"]
pub type LCD_R = crate::FieldReader<u8, LCD_A>;
impl LCD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCD_A {
        match self.bits {
            0 => LCD_A::DIV16,
            1 => LCD_A::DIV32,
            2 => LCD_A::DIV64,
            3 => LCD_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == LCD_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == LCD_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == LCD_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == LCD_A::DIV128
    }
}
#[doc = "Field `LCD` writer - Liquid Crystal Display Controller Prescaler"]
pub type LCD_W<'a> = crate::FieldWriterSafe<'a, u32, LFAPRESC0_SPEC, u8, LCD_A, 2, 12>;
impl<'a> LCD_W<'a> {
    #[doc = "LFACLKLCD = LFACLK/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(LCD_A::DIV16)
    }
    #[doc = "LFACLKLCD = LFACLK/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(LCD_A::DIV32)
    }
    #[doc = "LFACLKLCD = LFACLK/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(LCD_A::DIV64)
    }
    #[doc = "LFACLKLCD = LFACLK/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(LCD_A::DIV128)
    }
}
impl R {
    #[doc = "Bits 0:1 - Low Energy Sensor Interface Prescaler"]
    #[inline(always)]
    pub fn lesense(&self) -> LESENSE_R {
        LESENSE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:7 - Real-Time Counter Prescaler"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Low Energy Timer 0 Prescaler"]
    #[inline(always)]
    pub fn letimer0(&self) -> LETIMER0_R {
        LETIMER0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - Liquid Crystal Display Controller Prescaler"]
    #[inline(always)]
    pub fn lcd(&self) -> LCD_R {
        LCD_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Low Energy Sensor Interface Prescaler"]
    #[inline(always)]
    pub fn lesense(&mut self) -> LESENSE_W {
        LESENSE_W::new(self)
    }
    #[doc = "Bits 4:7 - Real-Time Counter Prescaler"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W {
        RTC_W::new(self)
    }
    #[doc = "Bits 8:11 - Low Energy Timer 0 Prescaler"]
    #[inline(always)]
    pub fn letimer0(&mut self) -> LETIMER0_W {
        LETIMER0_W::new(self)
    }
    #[doc = "Bits 12:13 - Liquid Crystal Display Controller Prescaler"]
    #[inline(always)]
    pub fn lcd(&mut self) -> LCD_W {
        LCD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Frequency A Prescaler Register 0 (Async Reg)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfapresc0](index.html) module"]
pub struct LFAPRESC0_SPEC;
impl crate::RegisterSpec for LFAPRESC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lfapresc0::R](R) reader structure"]
impl crate::Readable for LFAPRESC0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lfapresc0::W](W) writer structure"]
impl crate::Writable for LFAPRESC0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LFAPRESC0 to value 0"]
impl crate::Resettable for LFAPRESC0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
