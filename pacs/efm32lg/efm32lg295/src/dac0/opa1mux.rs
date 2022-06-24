#[doc = "Register `OPA1MUX` reader"]
pub struct R(crate::R<OPA1MUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPA1MUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPA1MUX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPA1MUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPA1MUX` writer"]
pub struct W(crate::W<OPA1MUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPA1MUX_SPEC>;
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
impl From<crate::W<OPA1MUX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPA1MUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "OPA1 non-inverting Input Mux\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum POSSEL_A {
    #[doc = "0: Input disabled"]
    DISABLE = 0,
    #[doc = "1: DAC as input"]
    DAC = 1,
    #[doc = "2: POS PAD as input"]
    POSPAD = 2,
    #[doc = "3: OPA0 as input"]
    OPA0INP = 3,
    #[doc = "4: OPA 1 Resistor ladder as input"]
    OPATAP = 4,
}
impl From<POSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: POSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `POSSEL` reader - OPA1 non-inverting Input Mux"]
pub type POSSEL_R = crate::FieldReader<u8, POSSEL_A>;
impl POSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<POSSEL_A> {
        match self.bits {
            0 => Some(POSSEL_A::DISABLE),
            1 => Some(POSSEL_A::DAC),
            2 => Some(POSSEL_A::POSPAD),
            3 => Some(POSSEL_A::OPA0INP),
            4 => Some(POSSEL_A::OPATAP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == POSSEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == POSSEL_A::DAC
    }
    #[doc = "Checks if the value of the field is `POSPAD`"]
    #[inline(always)]
    pub fn is_pospad(&self) -> bool {
        *self == POSSEL_A::POSPAD
    }
    #[doc = "Checks if the value of the field is `OPA0INP`"]
    #[inline(always)]
    pub fn is_opa0inp(&self) -> bool {
        *self == POSSEL_A::OPA0INP
    }
    #[doc = "Checks if the value of the field is `OPATAP`"]
    #[inline(always)]
    pub fn is_opatap(&self) -> bool {
        *self == POSSEL_A::OPATAP
    }
}
#[doc = "Field `POSSEL` writer - OPA1 non-inverting Input Mux"]
pub type POSSEL_W<'a> = crate::FieldWriter<'a, u32, OPA1MUX_SPEC, u8, POSSEL_A, 3, 0>;
impl<'a> POSSEL_W<'a> {
    #[doc = "Input disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(POSSEL_A::DISABLE)
    }
    #[doc = "DAC as input"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(POSSEL_A::DAC)
    }
    #[doc = "POS PAD as input"]
    #[inline(always)]
    pub fn pospad(self) -> &'a mut W {
        self.variant(POSSEL_A::POSPAD)
    }
    #[doc = "OPA0 as input"]
    #[inline(always)]
    pub fn opa0inp(self) -> &'a mut W {
        self.variant(POSSEL_A::OPA0INP)
    }
    #[doc = "OPA 1 Resistor ladder as input"]
    #[inline(always)]
    pub fn opatap(self) -> &'a mut W {
        self.variant(POSSEL_A::OPATAP)
    }
}
#[doc = "OPA1 inverting Input Mux\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NEGSEL_A {
    #[doc = "0: Input disabled"]
    DISABLE = 0,
    #[doc = "1: Unity Gain feedback path"]
    UG = 1,
    #[doc = "2: OPA1 Resistor ladder as input"]
    OPATAP = 2,
    #[doc = "3: Input from NEG PAD"]
    NEGPAD = 3,
}
impl From<NEGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: NEGSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NEGSEL` reader - OPA1 inverting Input Mux"]
pub type NEGSEL_R = crate::FieldReader<u8, NEGSEL_A>;
impl NEGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NEGSEL_A {
        match self.bits {
            0 => NEGSEL_A::DISABLE,
            1 => NEGSEL_A::UG,
            2 => NEGSEL_A::OPATAP,
            3 => NEGSEL_A::NEGPAD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == NEGSEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `UG`"]
    #[inline(always)]
    pub fn is_ug(&self) -> bool {
        *self == NEGSEL_A::UG
    }
    #[doc = "Checks if the value of the field is `OPATAP`"]
    #[inline(always)]
    pub fn is_opatap(&self) -> bool {
        *self == NEGSEL_A::OPATAP
    }
    #[doc = "Checks if the value of the field is `NEGPAD`"]
    #[inline(always)]
    pub fn is_negpad(&self) -> bool {
        *self == NEGSEL_A::NEGPAD
    }
}
#[doc = "Field `NEGSEL` writer - OPA1 inverting Input Mux"]
pub type NEGSEL_W<'a> = crate::FieldWriterSafe<'a, u32, OPA1MUX_SPEC, u8, NEGSEL_A, 2, 4>;
impl<'a> NEGSEL_W<'a> {
    #[doc = "Input disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(NEGSEL_A::DISABLE)
    }
    #[doc = "Unity Gain feedback path"]
    #[inline(always)]
    pub fn ug(self) -> &'a mut W {
        self.variant(NEGSEL_A::UG)
    }
    #[doc = "OPA1 Resistor ladder as input"]
    #[inline(always)]
    pub fn opatap(self) -> &'a mut W {
        self.variant(NEGSEL_A::OPATAP)
    }
    #[doc = "Input from NEG PAD"]
    #[inline(always)]
    pub fn negpad(self) -> &'a mut W {
        self.variant(NEGSEL_A::NEGPAD)
    }
}
#[doc = "OPA1 Resistor Ladder Input Mux\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RESINMUX_A {
    #[doc = "0: Set for Unity Gain"]
    DISABLE = 0,
    #[doc = "1: Set for OPA0 input"]
    OPA0INP = 1,
    #[doc = "2: NEG PAD connected"]
    NEGPAD = 2,
    #[doc = "3: POS PAD connected"]
    POSPAD = 3,
    #[doc = "4: VSS connected"]
    VSS = 4,
}
impl From<RESINMUX_A> for u8 {
    #[inline(always)]
    fn from(variant: RESINMUX_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RESINMUX` reader - OPA1 Resistor Ladder Input Mux"]
pub type RESINMUX_R = crate::FieldReader<u8, RESINMUX_A>;
impl RESINMUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RESINMUX_A> {
        match self.bits {
            0 => Some(RESINMUX_A::DISABLE),
            1 => Some(RESINMUX_A::OPA0INP),
            2 => Some(RESINMUX_A::NEGPAD),
            3 => Some(RESINMUX_A::POSPAD),
            4 => Some(RESINMUX_A::VSS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RESINMUX_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `OPA0INP`"]
    #[inline(always)]
    pub fn is_opa0inp(&self) -> bool {
        *self == RESINMUX_A::OPA0INP
    }
    #[doc = "Checks if the value of the field is `NEGPAD`"]
    #[inline(always)]
    pub fn is_negpad(&self) -> bool {
        *self == RESINMUX_A::NEGPAD
    }
    #[doc = "Checks if the value of the field is `POSPAD`"]
    #[inline(always)]
    pub fn is_pospad(&self) -> bool {
        *self == RESINMUX_A::POSPAD
    }
    #[doc = "Checks if the value of the field is `VSS`"]
    #[inline(always)]
    pub fn is_vss(&self) -> bool {
        *self == RESINMUX_A::VSS
    }
}
#[doc = "Field `RESINMUX` writer - OPA1 Resistor Ladder Input Mux"]
pub type RESINMUX_W<'a> = crate::FieldWriter<'a, u32, OPA1MUX_SPEC, u8, RESINMUX_A, 3, 8>;
impl<'a> RESINMUX_W<'a> {
    #[doc = "Set for Unity Gain"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RESINMUX_A::DISABLE)
    }
    #[doc = "Set for OPA0 input"]
    #[inline(always)]
    pub fn opa0inp(self) -> &'a mut W {
        self.variant(RESINMUX_A::OPA0INP)
    }
    #[doc = "NEG PAD connected"]
    #[inline(always)]
    pub fn negpad(self) -> &'a mut W {
        self.variant(RESINMUX_A::NEGPAD)
    }
    #[doc = "POS PAD connected"]
    #[inline(always)]
    pub fn pospad(self) -> &'a mut W {
        self.variant(RESINMUX_A::POSPAD)
    }
    #[doc = "VSS connected"]
    #[inline(always)]
    pub fn vss(self) -> &'a mut W {
        self.variant(RESINMUX_A::VSS)
    }
}
#[doc = "Field `PPEN` reader - OPA1 Positive Pad Input Enable"]
pub type PPEN_R = crate::BitReader<bool>;
#[doc = "Field `PPEN` writer - OPA1 Positive Pad Input Enable"]
pub type PPEN_W<'a> = crate::BitWriter<'a, u32, OPA1MUX_SPEC, bool, 12>;
#[doc = "Field `NPEN` reader - OPA1 Negative Pad Input Enable"]
pub type NPEN_R = crate::BitReader<bool>;
#[doc = "Field `NPEN` writer - OPA1 Negative Pad Input Enable"]
pub type NPEN_W<'a> = crate::BitWriter<'a, u32, OPA1MUX_SPEC, bool, 13>;
#[doc = "OPA1 Output Enable Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OUTPEN_A {
    #[doc = "1: Alternate Output 0"]
    OUT0 = 1,
    #[doc = "2: Alternate Output 1"]
    OUT1 = 2,
    #[doc = "4: Alternate Output 2"]
    OUT2 = 4,
    #[doc = "8: Alternate Output 3"]
    OUT3 = 8,
    #[doc = "16: Alternate Output 4"]
    OUT4 = 16,
}
impl From<OUTPEN_A> for u8 {
    #[inline(always)]
    fn from(variant: OUTPEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OUTPEN` reader - OPA1 Output Enable Value"]
pub type OUTPEN_R = crate::FieldReader<u8, OUTPEN_A>;
impl OUTPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OUTPEN_A> {
        match self.bits {
            1 => Some(OUTPEN_A::OUT0),
            2 => Some(OUTPEN_A::OUT1),
            4 => Some(OUTPEN_A::OUT2),
            8 => Some(OUTPEN_A::OUT3),
            16 => Some(OUTPEN_A::OUT4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == OUTPEN_A::OUT0
    }
    #[doc = "Checks if the value of the field is `OUT1`"]
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == OUTPEN_A::OUT1
    }
    #[doc = "Checks if the value of the field is `OUT2`"]
    #[inline(always)]
    pub fn is_out2(&self) -> bool {
        *self == OUTPEN_A::OUT2
    }
    #[doc = "Checks if the value of the field is `OUT3`"]
    #[inline(always)]
    pub fn is_out3(&self) -> bool {
        *self == OUTPEN_A::OUT3
    }
    #[doc = "Checks if the value of the field is `OUT4`"]
    #[inline(always)]
    pub fn is_out4(&self) -> bool {
        *self == OUTPEN_A::OUT4
    }
}
#[doc = "Field `OUTPEN` writer - OPA1 Output Enable Value"]
pub type OUTPEN_W<'a> = crate::FieldWriter<'a, u32, OPA1MUX_SPEC, u8, OUTPEN_A, 5, 14>;
impl<'a> OUTPEN_W<'a> {
    #[doc = "Alternate Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTPEN_A::OUT0)
    }
    #[doc = "Alternate Output 1"]
    #[inline(always)]
    pub fn out1(self) -> &'a mut W {
        self.variant(OUTPEN_A::OUT1)
    }
    #[doc = "Alternate Output 2"]
    #[inline(always)]
    pub fn out2(self) -> &'a mut W {
        self.variant(OUTPEN_A::OUT2)
    }
    #[doc = "Alternate Output 3"]
    #[inline(always)]
    pub fn out3(self) -> &'a mut W {
        self.variant(OUTPEN_A::OUT3)
    }
    #[doc = "Alternate Output 4"]
    #[inline(always)]
    pub fn out4(self) -> &'a mut W {
        self.variant(OUTPEN_A::OUT4)
    }
}
#[doc = "Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OUTMODE_A {
    #[doc = "0: OPA0 output is disabled"]
    DISABLE = 0,
    #[doc = "1: Main OPA1 output to pin enabled"]
    MAIN = 1,
    #[doc = "2: OPA1 alternative output enabled."]
    ALT = 2,
    #[doc = "3: Main OPA1 output drives both main and alternative outputs."]
    ALL = 3,
}
impl From<OUTMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: OUTMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OUTMODE` reader - Output Select"]
pub type OUTMODE_R = crate::FieldReader<u8, OUTMODE_A>;
impl OUTMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTMODE_A {
        match self.bits {
            0 => OUTMODE_A::DISABLE,
            1 => OUTMODE_A::MAIN,
            2 => OUTMODE_A::ALT,
            3 => OUTMODE_A::ALL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == OUTMODE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `MAIN`"]
    #[inline(always)]
    pub fn is_main(&self) -> bool {
        *self == OUTMODE_A::MAIN
    }
    #[doc = "Checks if the value of the field is `ALT`"]
    #[inline(always)]
    pub fn is_alt(&self) -> bool {
        *self == OUTMODE_A::ALT
    }
    #[doc = "Checks if the value of the field is `ALL`"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == OUTMODE_A::ALL
    }
}
#[doc = "Field `OUTMODE` writer - Output Select"]
pub type OUTMODE_W<'a> = crate::FieldWriterSafe<'a, u32, OPA1MUX_SPEC, u8, OUTMODE_A, 2, 22>;
impl<'a> OUTMODE_W<'a> {
    #[doc = "OPA0 output is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(OUTMODE_A::DISABLE)
    }
    #[doc = "Main OPA1 output to pin enabled"]
    #[inline(always)]
    pub fn main(self) -> &'a mut W {
        self.variant(OUTMODE_A::MAIN)
    }
    #[doc = "OPA1 alternative output enabled."]
    #[inline(always)]
    pub fn alt(self) -> &'a mut W {
        self.variant(OUTMODE_A::ALT)
    }
    #[doc = "Main OPA1 output drives both main and alternative outputs."]
    #[inline(always)]
    pub fn all(self) -> &'a mut W {
        self.variant(OUTMODE_A::ALL)
    }
}
#[doc = "Field `NEXTOUT` reader - OPA1 Next Enable"]
pub type NEXTOUT_R = crate::BitReader<bool>;
#[doc = "Field `NEXTOUT` writer - OPA1 Next Enable"]
pub type NEXTOUT_W<'a> = crate::BitWriter<'a, u32, OPA1MUX_SPEC, bool, 26>;
#[doc = "OPA1 Resistor Ladder Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RESSEL_A {
    #[doc = "0: Gain of 1/3"]
    RES0 = 0,
    #[doc = "1: Gain of 1"]
    RES1 = 1,
    #[doc = "2: Gain of 1 2/3"]
    RES2 = 2,
    #[doc = "3: Gain of 2"]
    RES3 = 3,
    #[doc = "4: Gain of 3"]
    RES4 = 4,
    #[doc = "5: Gain of 4 1/3"]
    RES5 = 5,
    #[doc = "6: Gain of 7"]
    RES6 = 6,
    #[doc = "7: Gain of 15"]
    RES7 = 7,
}
impl From<RESSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RESSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RESSEL` reader - OPA1 Resistor Ladder Select"]
pub type RESSEL_R = crate::FieldReader<u8, RESSEL_A>;
impl RESSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESSEL_A {
        match self.bits {
            0 => RESSEL_A::RES0,
            1 => RESSEL_A::RES1,
            2 => RESSEL_A::RES2,
            3 => RESSEL_A::RES3,
            4 => RESSEL_A::RES4,
            5 => RESSEL_A::RES5,
            6 => RESSEL_A::RES6,
            7 => RESSEL_A::RES7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RES0`"]
    #[inline(always)]
    pub fn is_res0(&self) -> bool {
        *self == RESSEL_A::RES0
    }
    #[doc = "Checks if the value of the field is `RES1`"]
    #[inline(always)]
    pub fn is_res1(&self) -> bool {
        *self == RESSEL_A::RES1
    }
    #[doc = "Checks if the value of the field is `RES2`"]
    #[inline(always)]
    pub fn is_res2(&self) -> bool {
        *self == RESSEL_A::RES2
    }
    #[doc = "Checks if the value of the field is `RES3`"]
    #[inline(always)]
    pub fn is_res3(&self) -> bool {
        *self == RESSEL_A::RES3
    }
    #[doc = "Checks if the value of the field is `RES4`"]
    #[inline(always)]
    pub fn is_res4(&self) -> bool {
        *self == RESSEL_A::RES4
    }
    #[doc = "Checks if the value of the field is `RES5`"]
    #[inline(always)]
    pub fn is_res5(&self) -> bool {
        *self == RESSEL_A::RES5
    }
    #[doc = "Checks if the value of the field is `RES6`"]
    #[inline(always)]
    pub fn is_res6(&self) -> bool {
        *self == RESSEL_A::RES6
    }
    #[doc = "Checks if the value of the field is `RES7`"]
    #[inline(always)]
    pub fn is_res7(&self) -> bool {
        *self == RESSEL_A::RES7
    }
}
#[doc = "Field `RESSEL` writer - OPA1 Resistor Ladder Select"]
pub type RESSEL_W<'a> = crate::FieldWriterSafe<'a, u32, OPA1MUX_SPEC, u8, RESSEL_A, 3, 28>;
impl<'a> RESSEL_W<'a> {
    #[doc = "Gain of 1/3"]
    #[inline(always)]
    pub fn res0(self) -> &'a mut W {
        self.variant(RESSEL_A::RES0)
    }
    #[doc = "Gain of 1"]
    #[inline(always)]
    pub fn res1(self) -> &'a mut W {
        self.variant(RESSEL_A::RES1)
    }
    #[doc = "Gain of 1 2/3"]
    #[inline(always)]
    pub fn res2(self) -> &'a mut W {
        self.variant(RESSEL_A::RES2)
    }
    #[doc = "Gain of 2"]
    #[inline(always)]
    pub fn res3(self) -> &'a mut W {
        self.variant(RESSEL_A::RES3)
    }
    #[doc = "Gain of 3"]
    #[inline(always)]
    pub fn res4(self) -> &'a mut W {
        self.variant(RESSEL_A::RES4)
    }
    #[doc = "Gain of 4 1/3"]
    #[inline(always)]
    pub fn res5(self) -> &'a mut W {
        self.variant(RESSEL_A::RES5)
    }
    #[doc = "Gain of 7"]
    #[inline(always)]
    pub fn res6(self) -> &'a mut W {
        self.variant(RESSEL_A::RES6)
    }
    #[doc = "Gain of 15"]
    #[inline(always)]
    pub fn res7(self) -> &'a mut W {
        self.variant(RESSEL_A::RES7)
    }
}
impl R {
    #[doc = "Bits 0:2 - OPA1 non-inverting Input Mux"]
    #[inline(always)]
    pub fn possel(&self) -> POSSEL_R {
        POSSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5 - OPA1 inverting Input Mux"]
    #[inline(always)]
    pub fn negsel(&self) -> NEGSEL_R {
        NEGSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:10 - OPA1 Resistor Ladder Input Mux"]
    #[inline(always)]
    pub fn resinmux(&self) -> RESINMUX_R {
        RESINMUX_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - OPA1 Positive Pad Input Enable"]
    #[inline(always)]
    pub fn ppen(&self) -> PPEN_R {
        PPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - OPA1 Negative Pad Input Enable"]
    #[inline(always)]
    pub fn npen(&self) -> NPEN_R {
        NPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:18 - OPA1 Output Enable Value"]
    #[inline(always)]
    pub fn outpen(&self) -> OUTPEN_R {
        OUTPEN_R::new(((self.bits >> 14) & 0x1f) as u8)
    }
    #[doc = "Bits 22:23 - Output Select"]
    #[inline(always)]
    pub fn outmode(&self) -> OUTMODE_R {
        OUTMODE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 26 - OPA1 Next Enable"]
    #[inline(always)]
    pub fn nextout(&self) -> NEXTOUT_R {
        NEXTOUT_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 28:30 - OPA1 Resistor Ladder Select"]
    #[inline(always)]
    pub fn ressel(&self) -> RESSEL_R {
        RESSEL_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - OPA1 non-inverting Input Mux"]
    #[inline(always)]
    pub fn possel(&mut self) -> POSSEL_W {
        POSSEL_W::new(self)
    }
    #[doc = "Bits 4:5 - OPA1 inverting Input Mux"]
    #[inline(always)]
    pub fn negsel(&mut self) -> NEGSEL_W {
        NEGSEL_W::new(self)
    }
    #[doc = "Bits 8:10 - OPA1 Resistor Ladder Input Mux"]
    #[inline(always)]
    pub fn resinmux(&mut self) -> RESINMUX_W {
        RESINMUX_W::new(self)
    }
    #[doc = "Bit 12 - OPA1 Positive Pad Input Enable"]
    #[inline(always)]
    pub fn ppen(&mut self) -> PPEN_W {
        PPEN_W::new(self)
    }
    #[doc = "Bit 13 - OPA1 Negative Pad Input Enable"]
    #[inline(always)]
    pub fn npen(&mut self) -> NPEN_W {
        NPEN_W::new(self)
    }
    #[doc = "Bits 14:18 - OPA1 Output Enable Value"]
    #[inline(always)]
    pub fn outpen(&mut self) -> OUTPEN_W {
        OUTPEN_W::new(self)
    }
    #[doc = "Bits 22:23 - Output Select"]
    #[inline(always)]
    pub fn outmode(&mut self) -> OUTMODE_W {
        OUTMODE_W::new(self)
    }
    #[doc = "Bit 26 - OPA1 Next Enable"]
    #[inline(always)]
    pub fn nextout(&mut self) -> NEXTOUT_W {
        NEXTOUT_W::new(self)
    }
    #[doc = "Bits 28:30 - OPA1 Resistor Ladder Select"]
    #[inline(always)]
    pub fn ressel(&mut self) -> RESSEL_W {
        RESSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Operational Amplifier Mux Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa1mux](index.html) module"]
pub struct OPA1MUX_SPEC;
impl crate::RegisterSpec for OPA1MUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opa1mux::R](R) reader structure"]
impl crate::Readable for OPA1MUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opa1mux::W](W) writer structure"]
impl crate::Writable for OPA1MUX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPA1MUX to value 0"]
impl crate::Resettable for OPA1MUX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
