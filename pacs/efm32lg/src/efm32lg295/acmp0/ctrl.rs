#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Analog Comparator Enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Analog Comparator Enable"]
pub type EN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 0>;
#[doc = "Field `MUXEN` reader - Input Mux Enable"]
pub type MUXEN_R = crate::BitReader<bool>;
#[doc = "Field `MUXEN` writer - Input Mux Enable"]
pub type MUXEN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 1>;
#[doc = "Field `INACTVAL` reader - Inactive Value"]
pub type INACTVAL_R = crate::BitReader<bool>;
#[doc = "Field `INACTVAL` writer - Inactive Value"]
pub type INACTVAL_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 2>;
#[doc = "Field `GPIOINV` reader - Comparator GPIO Output Invert"]
pub type GPIOINV_R = crate::BitReader<bool>;
#[doc = "Field `GPIOINV` writer - Comparator GPIO Output Invert"]
pub type GPIOINV_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 3>;
#[doc = "Hysteresis Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HYSTSEL_A {
    #[doc = "0: No hysteresis."]
    HYST0 = 0,
    #[doc = "1: ~15 mV hysteresis."]
    HYST1 = 1,
    #[doc = "2: ~22 mV hysteresis."]
    HYST2 = 2,
    #[doc = "3: ~29 mV hysteresis."]
    HYST3 = 3,
    #[doc = "4: ~36 mV hysteresis."]
    HYST4 = 4,
    #[doc = "5: ~43 mV hysteresis."]
    HYST5 = 5,
    #[doc = "6: ~50 mV hysteresis."]
    HYST6 = 6,
    #[doc = "7: ~57 mV hysteresis."]
    HYST7 = 7,
}
impl From<HYSTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: HYSTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HYSTSEL` reader - Hysteresis Select"]
pub type HYSTSEL_R = crate::FieldReader<u8, HYSTSEL_A>;
impl HYSTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HYSTSEL_A {
        match self.bits {
            0 => HYSTSEL_A::HYST0,
            1 => HYSTSEL_A::HYST1,
            2 => HYSTSEL_A::HYST2,
            3 => HYSTSEL_A::HYST3,
            4 => HYSTSEL_A::HYST4,
            5 => HYSTSEL_A::HYST5,
            6 => HYSTSEL_A::HYST6,
            7 => HYSTSEL_A::HYST7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HYST0`"]
    #[inline(always)]
    pub fn is_hyst0(&self) -> bool {
        *self == HYSTSEL_A::HYST0
    }
    #[doc = "Checks if the value of the field is `HYST1`"]
    #[inline(always)]
    pub fn is_hyst1(&self) -> bool {
        *self == HYSTSEL_A::HYST1
    }
    #[doc = "Checks if the value of the field is `HYST2`"]
    #[inline(always)]
    pub fn is_hyst2(&self) -> bool {
        *self == HYSTSEL_A::HYST2
    }
    #[doc = "Checks if the value of the field is `HYST3`"]
    #[inline(always)]
    pub fn is_hyst3(&self) -> bool {
        *self == HYSTSEL_A::HYST3
    }
    #[doc = "Checks if the value of the field is `HYST4`"]
    #[inline(always)]
    pub fn is_hyst4(&self) -> bool {
        *self == HYSTSEL_A::HYST4
    }
    #[doc = "Checks if the value of the field is `HYST5`"]
    #[inline(always)]
    pub fn is_hyst5(&self) -> bool {
        *self == HYSTSEL_A::HYST5
    }
    #[doc = "Checks if the value of the field is `HYST6`"]
    #[inline(always)]
    pub fn is_hyst6(&self) -> bool {
        *self == HYSTSEL_A::HYST6
    }
    #[doc = "Checks if the value of the field is `HYST7`"]
    #[inline(always)]
    pub fn is_hyst7(&self) -> bool {
        *self == HYSTSEL_A::HYST7
    }
}
#[doc = "Field `HYSTSEL` writer - Hysteresis Select"]
pub type HYSTSEL_W<'a> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, HYSTSEL_A, 3, 4>;
impl<'a> HYSTSEL_W<'a> {
    #[doc = "No hysteresis."]
    #[inline(always)]
    pub fn hyst0(self) -> &'a mut W {
        self.variant(HYSTSEL_A::HYST0)
    }
    #[doc = "~15 mV hysteresis."]
    #[inline(always)]
    pub fn hyst1(self) -> &'a mut W {
        self.variant(HYSTSEL_A::HYST1)
    }
    #[doc = "~22 mV hysteresis."]
    #[inline(always)]
    pub fn hyst2(self) -> &'a mut W {
        self.variant(HYSTSEL_A::HYST2)
    }
    #[doc = "~29 mV hysteresis."]
    #[inline(always)]
    pub fn hyst3(self) -> &'a mut W {
        self.variant(HYSTSEL_A::HYST3)
    }
    #[doc = "~36 mV hysteresis."]
    #[inline(always)]
    pub fn hyst4(self) -> &'a mut W {
        self.variant(HYSTSEL_A::HYST4)
    }
    #[doc = "~43 mV hysteresis."]
    #[inline(always)]
    pub fn hyst5(self) -> &'a mut W {
        self.variant(HYSTSEL_A::HYST5)
    }
    #[doc = "~50 mV hysteresis."]
    #[inline(always)]
    pub fn hyst6(self) -> &'a mut W {
        self.variant(HYSTSEL_A::HYST6)
    }
    #[doc = "~57 mV hysteresis."]
    #[inline(always)]
    pub fn hyst7(self) -> &'a mut W {
        self.variant(HYSTSEL_A::HYST7)
    }
}
#[doc = "Warm-up Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WARMTIME_A {
    #[doc = "0: 4 HFPERCLK cycles."]
    _4CYCLES = 0,
    #[doc = "1: 8 HFPERCLK cycles."]
    _8CYCLES = 1,
    #[doc = "2: 16 HFPERCLK cycles."]
    _16CYCLES = 2,
    #[doc = "3: 32 HFPERCLK cycles."]
    _32CYCLES = 3,
    #[doc = "4: 64 HFPERCLK cycles."]
    _64CYCLES = 4,
    #[doc = "5: 128 HFPERCLK cycles."]
    _128CYCLES = 5,
    #[doc = "6: 256 HFPERCLK cycles."]
    _256CYCLES = 6,
    #[doc = "7: 512 HFPERCLK cycles."]
    _512CYCLES = 7,
}
impl From<WARMTIME_A> for u8 {
    #[inline(always)]
    fn from(variant: WARMTIME_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WARMTIME` reader - Warm-up Time"]
pub type WARMTIME_R = crate::FieldReader<u8, WARMTIME_A>;
impl WARMTIME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WARMTIME_A {
        match self.bits {
            0 => WARMTIME_A::_4CYCLES,
            1 => WARMTIME_A::_8CYCLES,
            2 => WARMTIME_A::_16CYCLES,
            3 => WARMTIME_A::_32CYCLES,
            4 => WARMTIME_A::_64CYCLES,
            5 => WARMTIME_A::_128CYCLES,
            6 => WARMTIME_A::_256CYCLES,
            7 => WARMTIME_A::_512CYCLES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_4CYCLES`"]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == WARMTIME_A::_4CYCLES
    }
    #[doc = "Checks if the value of the field is `_8CYCLES`"]
    #[inline(always)]
    pub fn is_8cycles(&self) -> bool {
        *self == WARMTIME_A::_8CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == WARMTIME_A::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == WARMTIME_A::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_64CYCLES`"]
    #[inline(always)]
    pub fn is_64cycles(&self) -> bool {
        *self == WARMTIME_A::_64CYCLES
    }
    #[doc = "Checks if the value of the field is `_128CYCLES`"]
    #[inline(always)]
    pub fn is_128cycles(&self) -> bool {
        *self == WARMTIME_A::_128CYCLES
    }
    #[doc = "Checks if the value of the field is `_256CYCLES`"]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == WARMTIME_A::_256CYCLES
    }
    #[doc = "Checks if the value of the field is `_512CYCLES`"]
    #[inline(always)]
    pub fn is_512cycles(&self) -> bool {
        *self == WARMTIME_A::_512CYCLES
    }
}
#[doc = "Field `WARMTIME` writer - Warm-up Time"]
pub type WARMTIME_W<'a> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, WARMTIME_A, 3, 8>;
impl<'a> WARMTIME_W<'a> {
    #[doc = "4 HFPERCLK cycles."]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut W {
        self.variant(WARMTIME_A::_4CYCLES)
    }
    #[doc = "8 HFPERCLK cycles."]
    #[inline(always)]
    pub fn _8cycles(self) -> &'a mut W {
        self.variant(WARMTIME_A::_8CYCLES)
    }
    #[doc = "16 HFPERCLK cycles."]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(WARMTIME_A::_16CYCLES)
    }
    #[doc = "32 HFPERCLK cycles."]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(WARMTIME_A::_32CYCLES)
    }
    #[doc = "64 HFPERCLK cycles."]
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut W {
        self.variant(WARMTIME_A::_64CYCLES)
    }
    #[doc = "128 HFPERCLK cycles."]
    #[inline(always)]
    pub fn _128cycles(self) -> &'a mut W {
        self.variant(WARMTIME_A::_128CYCLES)
    }
    #[doc = "256 HFPERCLK cycles."]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut W {
        self.variant(WARMTIME_A::_256CYCLES)
    }
    #[doc = "512 HFPERCLK cycles."]
    #[inline(always)]
    pub fn _512cycles(self) -> &'a mut W {
        self.variant(WARMTIME_A::_512CYCLES)
    }
}
#[doc = "Field `IRISE` reader - Rising Edge Interrupt Sense"]
pub type IRISE_R = crate::BitReader<bool>;
#[doc = "Field `IRISE` writer - Rising Edge Interrupt Sense"]
pub type IRISE_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 16>;
#[doc = "Field `IFALL` reader - Falling Edge Interrupt Sense"]
pub type IFALL_R = crate::BitReader<bool>;
#[doc = "Field `IFALL` writer - Falling Edge Interrupt Sense"]
pub type IFALL_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 17>;
#[doc = "Field `BIASPROG` reader - Bias Configuration"]
pub type BIASPROG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BIASPROG` writer - Bias Configuration"]
pub type BIASPROG_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, 24>;
#[doc = "Field `HALFBIAS` reader - Half Bias Current"]
pub type HALFBIAS_R = crate::BitReader<bool>;
#[doc = "Field `HALFBIAS` writer - Half Bias Current"]
pub type HALFBIAS_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 30>;
#[doc = "Field `FULLBIAS` reader - Full Bias Current"]
pub type FULLBIAS_R = crate::BitReader<bool>;
#[doc = "Field `FULLBIAS` writer - Full Bias Current"]
pub type FULLBIAS_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - Analog Comparator Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input Mux Enable"]
    #[inline(always)]
    pub fn muxen(&self) -> MUXEN_R {
        MUXEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Inactive Value"]
    #[inline(always)]
    pub fn inactval(&self) -> INACTVAL_R {
        INACTVAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Comparator GPIO Output Invert"]
    #[inline(always)]
    pub fn gpioinv(&self) -> GPIOINV_R {
        GPIOINV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Hysteresis Select"]
    #[inline(always)]
    pub fn hystsel(&self) -> HYSTSEL_R {
        HYSTSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Warm-up Time"]
    #[inline(always)]
    pub fn warmtime(&self) -> WARMTIME_R {
        WARMTIME_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - Rising Edge Interrupt Sense"]
    #[inline(always)]
    pub fn irise(&self) -> IRISE_R {
        IRISE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Falling Edge Interrupt Sense"]
    #[inline(always)]
    pub fn ifall(&self) -> IFALL_R {
        IFALL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Bias Configuration"]
    #[inline(always)]
    pub fn biasprog(&self) -> BIASPROG_R {
        BIASPROG_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Half Bias Current"]
    #[inline(always)]
    pub fn halfbias(&self) -> HALFBIAS_R {
        HALFBIAS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Full Bias Current"]
    #[inline(always)]
    pub fn fullbias(&self) -> FULLBIAS_R {
        FULLBIAS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog Comparator Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Input Mux Enable"]
    #[inline(always)]
    pub fn muxen(&mut self) -> MUXEN_W {
        MUXEN_W::new(self)
    }
    #[doc = "Bit 2 - Inactive Value"]
    #[inline(always)]
    pub fn inactval(&mut self) -> INACTVAL_W {
        INACTVAL_W::new(self)
    }
    #[doc = "Bit 3 - Comparator GPIO Output Invert"]
    #[inline(always)]
    pub fn gpioinv(&mut self) -> GPIOINV_W {
        GPIOINV_W::new(self)
    }
    #[doc = "Bits 4:6 - Hysteresis Select"]
    #[inline(always)]
    pub fn hystsel(&mut self) -> HYSTSEL_W {
        HYSTSEL_W::new(self)
    }
    #[doc = "Bits 8:10 - Warm-up Time"]
    #[inline(always)]
    pub fn warmtime(&mut self) -> WARMTIME_W {
        WARMTIME_W::new(self)
    }
    #[doc = "Bit 16 - Rising Edge Interrupt Sense"]
    #[inline(always)]
    pub fn irise(&mut self) -> IRISE_W {
        IRISE_W::new(self)
    }
    #[doc = "Bit 17 - Falling Edge Interrupt Sense"]
    #[inline(always)]
    pub fn ifall(&mut self) -> IFALL_W {
        IFALL_W::new(self)
    }
    #[doc = "Bits 24:27 - Bias Configuration"]
    #[inline(always)]
    pub fn biasprog(&mut self) -> BIASPROG_W {
        BIASPROG_W::new(self)
    }
    #[doc = "Bit 30 - Half Bias Current"]
    #[inline(always)]
    pub fn halfbias(&mut self) -> HALFBIAS_W {
        HALFBIAS_W::new(self)
    }
    #[doc = "Bit 31 - Full Bias Current"]
    #[inline(always)]
    pub fn fullbias(&mut self) -> FULLBIAS_W {
        FULLBIAS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0x4700_0000"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4700_0000
    }
}
