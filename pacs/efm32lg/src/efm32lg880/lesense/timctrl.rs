#[doc = "Register `TIMCTRL` reader"]
pub struct R(crate::R<TIMCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMCTRL` writer"]
pub struct W(crate::W<TIMCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMCTRL_SPEC>;
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
impl From<crate::W<TIMCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Prescaling factor for high frequency timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AUXPRESC_A {
    #[doc = "0: High frequency timer is clocked with AUXHFRCO/1"]
    DIV1 = 0,
    #[doc = "1: High frequency timer is clocked with AUXHFRCO/2"]
    DIV2 = 1,
    #[doc = "2: High frequency timer is clocked with AUXHFRCO/4"]
    DIV4 = 2,
    #[doc = "3: High frequency timer is clocked with AUXHFRCO/8"]
    DIV8 = 3,
}
impl From<AUXPRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: AUXPRESC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AUXPRESC` reader - Prescaling factor for high frequency timer"]
pub type AUXPRESC_R = crate::FieldReader<u8, AUXPRESC_A>;
impl AUXPRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUXPRESC_A {
        match self.bits {
            0 => AUXPRESC_A::DIV1,
            1 => AUXPRESC_A::DIV2,
            2 => AUXPRESC_A::DIV4,
            3 => AUXPRESC_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == AUXPRESC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == AUXPRESC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == AUXPRESC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == AUXPRESC_A::DIV8
    }
}
#[doc = "Field `AUXPRESC` writer - Prescaling factor for high frequency timer"]
pub type AUXPRESC_W<'a> = crate::FieldWriterSafe<'a, u32, TIMCTRL_SPEC, u8, AUXPRESC_A, 2, 0>;
impl<'a> AUXPRESC_W<'a> {
    #[doc = "High frequency timer is clocked with AUXHFRCO/1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(AUXPRESC_A::DIV1)
    }
    #[doc = "High frequency timer is clocked with AUXHFRCO/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(AUXPRESC_A::DIV2)
    }
    #[doc = "High frequency timer is clocked with AUXHFRCO/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(AUXPRESC_A::DIV4)
    }
    #[doc = "High frequency timer is clocked with AUXHFRCO/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(AUXPRESC_A::DIV8)
    }
}
#[doc = "Prescaling factor for low frequency timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LFPRESC_A {
    #[doc = "0: Low frequency timer is clocked with LFACLKLESENSE/1"]
    DIV1 = 0,
    #[doc = "1: Low frequency timer is clocked with LFACLKLESENSE/2"]
    DIV2 = 1,
    #[doc = "2: Low frequency timer is clocked with LFACLKLESENSE/4"]
    DIV4 = 2,
    #[doc = "3: Low frequency timer is clocked with LFACLKLESENSE/8"]
    DIV8 = 3,
    #[doc = "4: Low frequency timer is clocked with LFACLKLESENSE/16"]
    DIV16 = 4,
    #[doc = "5: Low frequency timer is clocked with LFACLKLESENSE/32"]
    DIV32 = 5,
    #[doc = "6: Low frequency timer is clocked with LFACLKLESENSE/64"]
    DIV64 = 6,
    #[doc = "7: Low frequency timer is clocked with LFACLKLESENSE/128"]
    DIV128 = 7,
}
impl From<LFPRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: LFPRESC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LFPRESC` reader - Prescaling factor for low frequency timer"]
pub type LFPRESC_R = crate::FieldReader<u8, LFPRESC_A>;
impl LFPRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFPRESC_A {
        match self.bits {
            0 => LFPRESC_A::DIV1,
            1 => LFPRESC_A::DIV2,
            2 => LFPRESC_A::DIV4,
            3 => LFPRESC_A::DIV8,
            4 => LFPRESC_A::DIV16,
            5 => LFPRESC_A::DIV32,
            6 => LFPRESC_A::DIV64,
            7 => LFPRESC_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == LFPRESC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == LFPRESC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == LFPRESC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == LFPRESC_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == LFPRESC_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == LFPRESC_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == LFPRESC_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == LFPRESC_A::DIV128
    }
}
#[doc = "Field `LFPRESC` writer - Prescaling factor for low frequency timer"]
pub type LFPRESC_W<'a> = crate::FieldWriterSafe<'a, u32, TIMCTRL_SPEC, u8, LFPRESC_A, 3, 4>;
impl<'a> LFPRESC_W<'a> {
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(LFPRESC_A::DIV1)
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(LFPRESC_A::DIV2)
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(LFPRESC_A::DIV4)
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(LFPRESC_A::DIV8)
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(LFPRESC_A::DIV16)
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(LFPRESC_A::DIV32)
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(LFPRESC_A::DIV64)
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(LFPRESC_A::DIV128)
    }
}
#[doc = "Period counter prescaling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCPRESC_A {
    #[doc = "0: The period counter clock frequency is LFACLKLESENSE/1"]
    DIV1 = 0,
    #[doc = "1: The period counter clock frequency is LFACLKLESENSE/2"]
    DIV2 = 1,
    #[doc = "2: The period counter clock frequency is LFACLKLESENSE/4"]
    DIV4 = 2,
    #[doc = "3: The period counter clock frequency is LFACLKLESENSE/8"]
    DIV8 = 3,
    #[doc = "4: The period counter clock frequency is LFACLKLESENSE/16"]
    DIV16 = 4,
    #[doc = "5: The period counter clock frequency is LFACLKLESENSE/32"]
    DIV32 = 5,
    #[doc = "6: The period counter clock frequency is LFACLKLESENSE/64"]
    DIV64 = 6,
    #[doc = "7: The period counter clock frequency is LFACLKLESENSE/128"]
    DIV128 = 7,
}
impl From<PCPRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PCPRESC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PCPRESC` reader - Period counter prescaling"]
pub type PCPRESC_R = crate::FieldReader<u8, PCPRESC_A>;
impl PCPRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCPRESC_A {
        match self.bits {
            0 => PCPRESC_A::DIV1,
            1 => PCPRESC_A::DIV2,
            2 => PCPRESC_A::DIV4,
            3 => PCPRESC_A::DIV8,
            4 => PCPRESC_A::DIV16,
            5 => PCPRESC_A::DIV32,
            6 => PCPRESC_A::DIV64,
            7 => PCPRESC_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PCPRESC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PCPRESC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PCPRESC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PCPRESC_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PCPRESC_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PCPRESC_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PCPRESC_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PCPRESC_A::DIV128
    }
}
#[doc = "Field `PCPRESC` writer - Period counter prescaling"]
pub type PCPRESC_W<'a> = crate::FieldWriterSafe<'a, u32, TIMCTRL_SPEC, u8, PCPRESC_A, 3, 8>;
impl<'a> PCPRESC_W<'a> {
    #[doc = "The period counter clock frequency is LFACLKLESENSE/1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PCPRESC_A::DIV1)
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PCPRESC_A::DIV2)
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PCPRESC_A::DIV4)
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PCPRESC_A::DIV8)
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PCPRESC_A::DIV16)
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PCPRESC_A::DIV32)
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PCPRESC_A::DIV64)
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(PCPRESC_A::DIV128)
    }
}
#[doc = "Field `PCTOP` reader - Period counter top value"]
pub type PCTOP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCTOP` writer - Period counter top value"]
pub type PCTOP_W<'a> = crate::FieldWriter<'a, u32, TIMCTRL_SPEC, u8, u8, 8, 12>;
#[doc = "Field `STARTDLY` reader - Start delay configuration"]
pub type STARTDLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STARTDLY` writer - Start delay configuration"]
pub type STARTDLY_W<'a> = crate::FieldWriter<'a, u32, TIMCTRL_SPEC, u8, u8, 2, 22>;
impl R {
    #[doc = "Bits 0:1 - Prescaling factor for high frequency timer"]
    #[inline(always)]
    pub fn auxpresc(&self) -> AUXPRESC_R {
        AUXPRESC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6 - Prescaling factor for low frequency timer"]
    #[inline(always)]
    pub fn lfpresc(&self) -> LFPRESC_R {
        LFPRESC_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Period counter prescaling"]
    #[inline(always)]
    pub fn pcpresc(&self) -> PCPRESC_R {
        PCPRESC_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:19 - Period counter top value"]
    #[inline(always)]
    pub fn pctop(&self) -> PCTOP_R {
        PCTOP_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 22:23 - Start delay configuration"]
    #[inline(always)]
    pub fn startdly(&self) -> STARTDLY_R {
        STARTDLY_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Prescaling factor for high frequency timer"]
    #[inline(always)]
    pub fn auxpresc(&mut self) -> AUXPRESC_W {
        AUXPRESC_W::new(self)
    }
    #[doc = "Bits 4:6 - Prescaling factor for low frequency timer"]
    #[inline(always)]
    pub fn lfpresc(&mut self) -> LFPRESC_W {
        LFPRESC_W::new(self)
    }
    #[doc = "Bits 8:10 - Period counter prescaling"]
    #[inline(always)]
    pub fn pcpresc(&mut self) -> PCPRESC_W {
        PCPRESC_W::new(self)
    }
    #[doc = "Bits 12:19 - Period counter top value"]
    #[inline(always)]
    pub fn pctop(&mut self) -> PCTOP_W {
        PCTOP_W::new(self)
    }
    #[doc = "Bits 22:23 - Start delay configuration"]
    #[inline(always)]
    pub fn startdly(&mut self) -> STARTDLY_W {
        STARTDLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timing Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timctrl](index.html) module"]
pub struct TIMCTRL_SPEC;
impl crate::RegisterSpec for TIMCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timctrl::R](R) reader structure"]
impl crate::Readable for TIMCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timctrl::W](W) writer structure"]
impl crate::Writable for TIMCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMCTRL to value 0"]
impl crate::Resettable for TIMCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
