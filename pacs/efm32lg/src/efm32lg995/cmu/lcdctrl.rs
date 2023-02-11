#[doc = "Register `LCDCTRL` reader"]
pub struct R(crate::R<LCDCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCDCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCDCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCDCTRL` writer"]
pub struct W(crate::W<LCDCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDCTRL_SPEC>;
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
impl From<crate::W<LCDCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCDCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FDIV` reader - Frame Rate Control"]
pub type FDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FDIV` writer - Frame Rate Control"]
pub type FDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LCDCTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `VBOOSTEN` reader - Voltage Boost Enable"]
pub type VBOOSTEN_R = crate::BitReader<bool>;
#[doc = "Field `VBOOSTEN` writer - Voltage Boost Enable"]
pub type VBOOSTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCTRL_SPEC, bool, O>;
#[doc = "Field `VBFDIV` reader - Voltage Boost Frequency Division"]
pub type VBFDIV_R = crate::FieldReader<u8, VBFDIV_A>;
#[doc = "Voltage Boost Frequency Division\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VBFDIV_A {
    #[doc = "0: Voltage Boost update Frequency = LFACLK."]
    DIV1 = 0,
    #[doc = "1: Voltage Boost update Frequency = LFACLK/2."]
    DIV2 = 1,
    #[doc = "2: Voltage Boost update Frequency = LFACLK/4."]
    DIV4 = 2,
    #[doc = "3: Voltage Boost update Frequency = LFACLK/8."]
    DIV8 = 3,
    #[doc = "4: Voltage Boost update Frequency = LFACLK/16."]
    DIV16 = 4,
    #[doc = "5: Voltage Boost update Frequency = LFACLK/32."]
    DIV32 = 5,
    #[doc = "6: Voltage Boost update Frequency = LFACLK/64."]
    DIV64 = 6,
    #[doc = "7: Voltage Boost update Frequency = LFACLK/128."]
    DIV128 = 7,
}
impl From<VBFDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: VBFDIV_A) -> Self {
        variant as _
    }
}
impl VBFDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBFDIV_A {
        match self.bits {
            0 => VBFDIV_A::DIV1,
            1 => VBFDIV_A::DIV2,
            2 => VBFDIV_A::DIV4,
            3 => VBFDIV_A::DIV8,
            4 => VBFDIV_A::DIV16,
            5 => VBFDIV_A::DIV32,
            6 => VBFDIV_A::DIV64,
            7 => VBFDIV_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == VBFDIV_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == VBFDIV_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == VBFDIV_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == VBFDIV_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == VBFDIV_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == VBFDIV_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == VBFDIV_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == VBFDIV_A::DIV128
    }
}
#[doc = "Field `VBFDIV` writer - Voltage Boost Frequency Division"]
pub type VBFDIV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LCDCTRL_SPEC, u8, VBFDIV_A, 3, O>;
impl<'a, const O: u8> VBFDIV_W<'a, O> {
    #[doc = "Voltage Boost update Frequency = LFACLK."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(VBFDIV_A::DIV1)
    }
    #[doc = "Voltage Boost update Frequency = LFACLK/2."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(VBFDIV_A::DIV2)
    }
    #[doc = "Voltage Boost update Frequency = LFACLK/4."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(VBFDIV_A::DIV4)
    }
    #[doc = "Voltage Boost update Frequency = LFACLK/8."]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(VBFDIV_A::DIV8)
    }
    #[doc = "Voltage Boost update Frequency = LFACLK/16."]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(VBFDIV_A::DIV16)
    }
    #[doc = "Voltage Boost update Frequency = LFACLK/32."]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(VBFDIV_A::DIV32)
    }
    #[doc = "Voltage Boost update Frequency = LFACLK/64."]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(VBFDIV_A::DIV64)
    }
    #[doc = "Voltage Boost update Frequency = LFACLK/128."]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(VBFDIV_A::DIV128)
    }
}
impl R {
    #[doc = "Bits 0:2 - Frame Rate Control"]
    #[inline(always)]
    pub fn fdiv(&self) -> FDIV_R {
        FDIV_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Voltage Boost Enable"]
    #[inline(always)]
    pub fn vboosten(&self) -> VBOOSTEN_R {
        VBOOSTEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Voltage Boost Frequency Division"]
    #[inline(always)]
    pub fn vbfdiv(&self) -> VBFDIV_R {
        VBFDIV_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Frame Rate Control"]
    #[inline(always)]
    #[must_use]
    pub fn fdiv(&mut self) -> FDIV_W<0> {
        FDIV_W::new(self)
    }
    #[doc = "Bit 3 - Voltage Boost Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vboosten(&mut self) -> VBOOSTEN_W<3> {
        VBOOSTEN_W::new(self)
    }
    #[doc = "Bits 4:6 - Voltage Boost Frequency Division"]
    #[inline(always)]
    #[must_use]
    pub fn vbfdiv(&mut self) -> VBFDIV_W<4> {
        VBFDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdctrl](index.html) module"]
pub struct LCDCTRL_SPEC;
impl crate::RegisterSpec for LCDCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcdctrl::R](R) reader structure"]
impl crate::Readable for LCDCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcdctrl::W](W) writer structure"]
impl crate::Writable for LCDCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCDCTRL to value 0x20"]
impl crate::Resettable for LCDCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
