#[doc = "Register `DISPCTRL` reader"]
pub struct R(crate::R<DISPCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DISPCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DISPCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DISPCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DISPCTRL` writer"]
pub struct W(crate::W<DISPCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DISPCTRL_SPEC>;
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
impl From<crate::W<DISPCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DISPCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Mux Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MUX_A {
    #[doc = "0: Static"]
    STATIC = 0,
    #[doc = "1: Duplex"]
    DUPLEX = 1,
    #[doc = "2: Triplex"]
    TRIPLEX = 2,
    #[doc = "3: Quadruplex"]
    QUADRUPLEX = 3,
}
impl From<MUX_A> for u8 {
    #[inline(always)]
    fn from(variant: MUX_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MUX` reader - Mux Configuration"]
pub type MUX_R = crate::FieldReader<u8, MUX_A>;
impl MUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUX_A {
        match self.bits {
            0 => MUX_A::STATIC,
            1 => MUX_A::DUPLEX,
            2 => MUX_A::TRIPLEX,
            3 => MUX_A::QUADRUPLEX,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STATIC`"]
    #[inline(always)]
    pub fn is_static(&self) -> bool {
        *self == MUX_A::STATIC
    }
    #[doc = "Checks if the value of the field is `DUPLEX`"]
    #[inline(always)]
    pub fn is_duplex(&self) -> bool {
        *self == MUX_A::DUPLEX
    }
    #[doc = "Checks if the value of the field is `TRIPLEX`"]
    #[inline(always)]
    pub fn is_triplex(&self) -> bool {
        *self == MUX_A::TRIPLEX
    }
    #[doc = "Checks if the value of the field is `QUADRUPLEX`"]
    #[inline(always)]
    pub fn is_quadruplex(&self) -> bool {
        *self == MUX_A::QUADRUPLEX
    }
}
#[doc = "Field `MUX` writer - Mux Configuration"]
pub type MUX_W<'a> = crate::FieldWriterSafe<'a, u32, DISPCTRL_SPEC, u8, MUX_A, 2, 0>;
impl<'a> MUX_W<'a> {
    #[doc = "Static"]
    #[inline(always)]
    pub fn static_(self) -> &'a mut W {
        self.variant(MUX_A::STATIC)
    }
    #[doc = "Duplex"]
    #[inline(always)]
    pub fn duplex(self) -> &'a mut W {
        self.variant(MUX_A::DUPLEX)
    }
    #[doc = "Triplex"]
    #[inline(always)]
    pub fn triplex(self) -> &'a mut W {
        self.variant(MUX_A::TRIPLEX)
    }
    #[doc = "Quadruplex"]
    #[inline(always)]
    pub fn quadruplex(self) -> &'a mut W {
        self.variant(MUX_A::QUADRUPLEX)
    }
}
#[doc = "Bias Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BIAS_A {
    #[doc = "0: Static"]
    STATIC = 0,
    #[doc = "1: 1/2 Bias"]
    ONEHALF = 1,
    #[doc = "2: 1/3 Bias"]
    ONETHIRD = 2,
    #[doc = "3: 1/4 Bias"]
    ONEFOURTH = 3,
}
impl From<BIAS_A> for u8 {
    #[inline(always)]
    fn from(variant: BIAS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BIAS` reader - Bias Configuration"]
pub type BIAS_R = crate::FieldReader<u8, BIAS_A>;
impl BIAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIAS_A {
        match self.bits {
            0 => BIAS_A::STATIC,
            1 => BIAS_A::ONEHALF,
            2 => BIAS_A::ONETHIRD,
            3 => BIAS_A::ONEFOURTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STATIC`"]
    #[inline(always)]
    pub fn is_static(&self) -> bool {
        *self == BIAS_A::STATIC
    }
    #[doc = "Checks if the value of the field is `ONEHALF`"]
    #[inline(always)]
    pub fn is_onehalf(&self) -> bool {
        *self == BIAS_A::ONEHALF
    }
    #[doc = "Checks if the value of the field is `ONETHIRD`"]
    #[inline(always)]
    pub fn is_onethird(&self) -> bool {
        *self == BIAS_A::ONETHIRD
    }
    #[doc = "Checks if the value of the field is `ONEFOURTH`"]
    #[inline(always)]
    pub fn is_onefourth(&self) -> bool {
        *self == BIAS_A::ONEFOURTH
    }
}
#[doc = "Field `BIAS` writer - Bias Configuration"]
pub type BIAS_W<'a> = crate::FieldWriterSafe<'a, u32, DISPCTRL_SPEC, u8, BIAS_A, 2, 2>;
impl<'a> BIAS_W<'a> {
    #[doc = "Static"]
    #[inline(always)]
    pub fn static_(self) -> &'a mut W {
        self.variant(BIAS_A::STATIC)
    }
    #[doc = "1/2 Bias"]
    #[inline(always)]
    pub fn onehalf(self) -> &'a mut W {
        self.variant(BIAS_A::ONEHALF)
    }
    #[doc = "1/3 Bias"]
    #[inline(always)]
    pub fn onethird(self) -> &'a mut W {
        self.variant(BIAS_A::ONETHIRD)
    }
    #[doc = "1/4 Bias"]
    #[inline(always)]
    pub fn onefourth(self) -> &'a mut W {
        self.variant(BIAS_A::ONEFOURTH)
    }
}
#[doc = "Field `WAVE` reader - Waveform Selection"]
pub type WAVE_R = crate::BitReader<bool>;
#[doc = "Field `WAVE` writer - Waveform Selection"]
pub type WAVE_W<'a> = crate::BitWriter<'a, u32, DISPCTRL_SPEC, bool, 4>;
#[doc = "Contrast Level\n\nValue on reset: 31"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CONLEV_A {
    #[doc = "0: Minimum contrast"]
    MIN = 0,
    #[doc = "31: Maximum contrast"]
    MAX = 31,
}
impl From<CONLEV_A> for u8 {
    #[inline(always)]
    fn from(variant: CONLEV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CONLEV` reader - Contrast Level"]
pub type CONLEV_R = crate::FieldReader<u8, CONLEV_A>;
impl CONLEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CONLEV_A> {
        match self.bits {
            0 => Some(CONLEV_A::MIN),
            31 => Some(CONLEV_A::MAX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MIN`"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == CONLEV_A::MIN
    }
    #[doc = "Checks if the value of the field is `MAX`"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == CONLEV_A::MAX
    }
}
#[doc = "Field `CONLEV` writer - Contrast Level"]
pub type CONLEV_W<'a> = crate::FieldWriter<'a, u32, DISPCTRL_SPEC, u8, CONLEV_A, 5, 8>;
impl<'a> CONLEV_W<'a> {
    #[doc = "Minimum contrast"]
    #[inline(always)]
    pub fn min(self) -> &'a mut W {
        self.variant(CONLEV_A::MIN)
    }
    #[doc = "Maximum contrast"]
    #[inline(always)]
    pub fn max(self) -> &'a mut W {
        self.variant(CONLEV_A::MAX)
    }
}
#[doc = "Field `CONCONF` reader - Contrast Configuration"]
pub type CONCONF_R = crate::BitReader<bool>;
#[doc = "Field `CONCONF` writer - Contrast Configuration"]
pub type CONCONF_W<'a> = crate::BitWriter<'a, u32, DISPCTRL_SPEC, bool, 15>;
#[doc = "Field `VLCDSEL` reader - VLCD Selection"]
pub type VLCDSEL_R = crate::BitReader<bool>;
#[doc = "Field `VLCDSEL` writer - VLCD Selection"]
pub type VLCDSEL_W<'a> = crate::BitWriter<'a, u32, DISPCTRL_SPEC, bool, 16>;
#[doc = "Voltage Boost Level\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VBLEV_A {
    #[doc = "0: Minimum boost level"]
    LEVEL0 = 0,
    #[doc = "1: `1`"]
    LEVEL1 = 1,
    #[doc = "2: `10`"]
    LEVEL2 = 2,
    #[doc = "3: `11`"]
    LEVEL3 = 3,
    #[doc = "4: `100`"]
    LEVEL4 = 4,
    #[doc = "5: `101`"]
    LEVEL5 = 5,
    #[doc = "6: `110`"]
    LEVEL6 = 6,
    #[doc = "7: Maximum boost level"]
    LEVEL7 = 7,
}
impl From<VBLEV_A> for u8 {
    #[inline(always)]
    fn from(variant: VBLEV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `VBLEV` reader - Voltage Boost Level"]
pub type VBLEV_R = crate::FieldReader<u8, VBLEV_A>;
impl VBLEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBLEV_A {
        match self.bits {
            0 => VBLEV_A::LEVEL0,
            1 => VBLEV_A::LEVEL1,
            2 => VBLEV_A::LEVEL2,
            3 => VBLEV_A::LEVEL3,
            4 => VBLEV_A::LEVEL4,
            5 => VBLEV_A::LEVEL5,
            6 => VBLEV_A::LEVEL6,
            7 => VBLEV_A::LEVEL7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL0`"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == VBLEV_A::LEVEL0
    }
    #[doc = "Checks if the value of the field is `LEVEL1`"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == VBLEV_A::LEVEL1
    }
    #[doc = "Checks if the value of the field is `LEVEL2`"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == VBLEV_A::LEVEL2
    }
    #[doc = "Checks if the value of the field is `LEVEL3`"]
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == VBLEV_A::LEVEL3
    }
    #[doc = "Checks if the value of the field is `LEVEL4`"]
    #[inline(always)]
    pub fn is_level4(&self) -> bool {
        *self == VBLEV_A::LEVEL4
    }
    #[doc = "Checks if the value of the field is `LEVEL5`"]
    #[inline(always)]
    pub fn is_level5(&self) -> bool {
        *self == VBLEV_A::LEVEL5
    }
    #[doc = "Checks if the value of the field is `LEVEL6`"]
    #[inline(always)]
    pub fn is_level6(&self) -> bool {
        *self == VBLEV_A::LEVEL6
    }
    #[doc = "Checks if the value of the field is `LEVEL7`"]
    #[inline(always)]
    pub fn is_level7(&self) -> bool {
        *self == VBLEV_A::LEVEL7
    }
}
#[doc = "Field `VBLEV` writer - Voltage Boost Level"]
pub type VBLEV_W<'a> = crate::FieldWriterSafe<'a, u32, DISPCTRL_SPEC, u8, VBLEV_A, 3, 18>;
impl<'a> VBLEV_W<'a> {
    #[doc = "Minimum boost level"]
    #[inline(always)]
    pub fn level0(self) -> &'a mut W {
        self.variant(VBLEV_A::LEVEL0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn level1(self) -> &'a mut W {
        self.variant(VBLEV_A::LEVEL1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn level2(self) -> &'a mut W {
        self.variant(VBLEV_A::LEVEL2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn level3(self) -> &'a mut W {
        self.variant(VBLEV_A::LEVEL3)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn level4(self) -> &'a mut W {
        self.variant(VBLEV_A::LEVEL4)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn level5(self) -> &'a mut W {
        self.variant(VBLEV_A::LEVEL5)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn level6(self) -> &'a mut W {
        self.variant(VBLEV_A::LEVEL6)
    }
    #[doc = "Maximum boost level"]
    #[inline(always)]
    pub fn level7(self) -> &'a mut W {
        self.variant(VBLEV_A::LEVEL7)
    }
}
#[doc = "Field `MUXE` reader - Extended Mux Configuration"]
pub type MUXE_R = crate::BitReader<bool>;
#[doc = "Field `MUXE` writer - Extended Mux Configuration"]
pub type MUXE_W<'a> = crate::BitWriter<'a, u32, DISPCTRL_SPEC, bool, 22>;
impl R {
    #[doc = "Bits 0:1 - Mux Configuration"]
    #[inline(always)]
    pub fn mux(&self) -> MUX_R {
        MUX_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Bias Configuration"]
    #[inline(always)]
    pub fn bias(&self) -> BIAS_R {
        BIAS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Waveform Selection"]
    #[inline(always)]
    pub fn wave(&self) -> WAVE_R {
        WAVE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Contrast Level"]
    #[inline(always)]
    pub fn conlev(&self) -> CONLEV_R {
        CONLEV_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Contrast Configuration"]
    #[inline(always)]
    pub fn conconf(&self) -> CONCONF_R {
        CONCONF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - VLCD Selection"]
    #[inline(always)]
    pub fn vlcdsel(&self) -> VLCDSEL_R {
        VLCDSEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 18:20 - Voltage Boost Level"]
    #[inline(always)]
    pub fn vblev(&self) -> VBLEV_R {
        VBLEV_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 22 - Extended Mux Configuration"]
    #[inline(always)]
    pub fn muxe(&self) -> MUXE_R {
        MUXE_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Mux Configuration"]
    #[inline(always)]
    pub fn mux(&mut self) -> MUX_W {
        MUX_W::new(self)
    }
    #[doc = "Bits 2:3 - Bias Configuration"]
    #[inline(always)]
    pub fn bias(&mut self) -> BIAS_W {
        BIAS_W::new(self)
    }
    #[doc = "Bit 4 - Waveform Selection"]
    #[inline(always)]
    pub fn wave(&mut self) -> WAVE_W {
        WAVE_W::new(self)
    }
    #[doc = "Bits 8:12 - Contrast Level"]
    #[inline(always)]
    pub fn conlev(&mut self) -> CONLEV_W {
        CONLEV_W::new(self)
    }
    #[doc = "Bit 15 - Contrast Configuration"]
    #[inline(always)]
    pub fn conconf(&mut self) -> CONCONF_W {
        CONCONF_W::new(self)
    }
    #[doc = "Bit 16 - VLCD Selection"]
    #[inline(always)]
    pub fn vlcdsel(&mut self) -> VLCDSEL_W {
        VLCDSEL_W::new(self)
    }
    #[doc = "Bits 18:20 - Voltage Boost Level"]
    #[inline(always)]
    pub fn vblev(&mut self) -> VBLEV_W {
        VBLEV_W::new(self)
    }
    #[doc = "Bit 22 - Extended Mux Configuration"]
    #[inline(always)]
    pub fn muxe(&mut self) -> MUXE_W {
        MUXE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Display Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dispctrl](index.html) module"]
pub struct DISPCTRL_SPEC;
impl crate::RegisterSpec for DISPCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dispctrl::R](R) reader structure"]
impl crate::Readable for DISPCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dispctrl::W](W) writer structure"]
impl crate::Writable for DISPCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DISPCTRL to value 0x000c_1f00"]
impl crate::Resettable for DISPCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x000c_1f00
    }
}
