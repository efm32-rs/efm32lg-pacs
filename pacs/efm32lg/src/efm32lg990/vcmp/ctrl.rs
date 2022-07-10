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
#[doc = "Field `EN` reader - Voltage Supply Comparator Enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Voltage Supply Comparator Enable"]
pub type EN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 0>;
#[doc = "Field `INACTVAL` reader - Inactive Value"]
pub type INACTVAL_R = crate::BitReader<bool>;
#[doc = "Field `INACTVAL` writer - Inactive Value"]
pub type INACTVAL_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 2>;
#[doc = "Field `HYSTEN` reader - Hysteresis Enable"]
pub type HYSTEN_R = crate::BitReader<bool>;
#[doc = "Field `HYSTEN` writer - Hysteresis Enable"]
pub type HYSTEN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 4>;
#[doc = "Warm-Up Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WARMTIME_A {
    #[doc = "0: 4 HFPERCLK cycles"]
    _4CYCLES = 0,
    #[doc = "1: 8 HFPERCLK cycles"]
    _8CYCLES = 1,
    #[doc = "2: 16 HFPERCLK cycles"]
    _16CYCLES = 2,
    #[doc = "3: 32 HFPERCLK cycles"]
    _32CYCLES = 3,
    #[doc = "4: 64 HFPERCLK cycles"]
    _64CYCLES = 4,
    #[doc = "5: 128 HFPERCLK cycles"]
    _128CYCLES = 5,
    #[doc = "6: 256 HFPERCLK cycles"]
    _256CYCLES = 6,
    #[doc = "7: 512 HFPERCLK cycles"]
    _512CYCLES = 7,
}
impl From<WARMTIME_A> for u8 {
    #[inline(always)]
    fn from(variant: WARMTIME_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WARMTIME` reader - Warm-Up Time"]
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
#[doc = "Field `WARMTIME` writer - Warm-Up Time"]
pub type WARMTIME_W<'a> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, WARMTIME_A, 3, 8>;
impl<'a> WARMTIME_W<'a> {
    #[doc = "4 HFPERCLK cycles"]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut W {
        self.variant(WARMTIME_A::_4CYCLES)
    }
    #[doc = "8 HFPERCLK cycles"]
    #[inline(always)]
    pub fn _8cycles(self) -> &'a mut W {
        self.variant(WARMTIME_A::_8CYCLES)
    }
    #[doc = "16 HFPERCLK cycles"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(WARMTIME_A::_16CYCLES)
    }
    #[doc = "32 HFPERCLK cycles"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(WARMTIME_A::_32CYCLES)
    }
    #[doc = "64 HFPERCLK cycles"]
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut W {
        self.variant(WARMTIME_A::_64CYCLES)
    }
    #[doc = "128 HFPERCLK cycles"]
    #[inline(always)]
    pub fn _128cycles(self) -> &'a mut W {
        self.variant(WARMTIME_A::_128CYCLES)
    }
    #[doc = "256 HFPERCLK cycles"]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut W {
        self.variant(WARMTIME_A::_256CYCLES)
    }
    #[doc = "512 HFPERCLK cycles"]
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
#[doc = "Field `BIASPROG` reader - VCMP Bias Programming Value"]
pub type BIASPROG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BIASPROG` writer - VCMP Bias Programming Value"]
pub type BIASPROG_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, 24>;
#[doc = "Field `HALFBIAS` reader - Half Bias Current"]
pub type HALFBIAS_R = crate::BitReader<bool>;
#[doc = "Field `HALFBIAS` writer - Half Bias Current"]
pub type HALFBIAS_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 30>;
impl R {
    #[doc = "Bit 0 - Voltage Supply Comparator Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Inactive Value"]
    #[inline(always)]
    pub fn inactval(&self) -> INACTVAL_R {
        INACTVAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Hysteresis Enable"]
    #[inline(always)]
    pub fn hysten(&self) -> HYSTEN_R {
        HYSTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Warm-Up Time"]
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
    #[doc = "Bits 24:27 - VCMP Bias Programming Value"]
    #[inline(always)]
    pub fn biasprog(&self) -> BIASPROG_R {
        BIASPROG_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Half Bias Current"]
    #[inline(always)]
    pub fn halfbias(&self) -> HALFBIAS_R {
        HALFBIAS_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage Supply Comparator Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W::new(self)
    }
    #[doc = "Bit 2 - Inactive Value"]
    #[inline(always)]
    pub fn inactval(&mut self) -> INACTVAL_W {
        INACTVAL_W::new(self)
    }
    #[doc = "Bit 4 - Hysteresis Enable"]
    #[inline(always)]
    pub fn hysten(&mut self) -> HYSTEN_W {
        HYSTEN_W::new(self)
    }
    #[doc = "Bits 8:10 - Warm-Up Time"]
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
    #[doc = "Bits 24:27 - VCMP Bias Programming Value"]
    #[inline(always)]
    pub fn biasprog(&mut self) -> BIASPROG_W {
        BIASPROG_W::new(self)
    }
    #[doc = "Bit 30 - Half Bias Current"]
    #[inline(always)]
    pub fn halfbias(&mut self) -> HALFBIAS_W {
        HALFBIAS_W::new(self)
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
