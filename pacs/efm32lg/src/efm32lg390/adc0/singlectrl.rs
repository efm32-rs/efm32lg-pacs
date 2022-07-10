#[doc = "Register `SINGLECTRL` reader"]
pub struct R(crate::R<SINGLECTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SINGLECTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SINGLECTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SINGLECTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SINGLECTRL` writer"]
pub struct W(crate::W<SINGLECTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SINGLECTRL_SPEC>;
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
impl From<crate::W<SINGLECTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SINGLECTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REP` reader - Single Sample Repetitive Mode"]
pub type REP_R = crate::BitReader<bool>;
#[doc = "Field `REP` writer - Single Sample Repetitive Mode"]
pub type REP_W<'a> = crate::BitWriter<'a, u32, SINGLECTRL_SPEC, bool, 0>;
#[doc = "Field `DIFF` reader - Single Sample Differential Mode"]
pub type DIFF_R = crate::BitReader<bool>;
#[doc = "Field `DIFF` writer - Single Sample Differential Mode"]
pub type DIFF_W<'a> = crate::BitWriter<'a, u32, SINGLECTRL_SPEC, bool, 1>;
#[doc = "Field `ADJ` reader - Single Sample Result Adjustment"]
pub type ADJ_R = crate::BitReader<bool>;
#[doc = "Field `ADJ` writer - Single Sample Result Adjustment"]
pub type ADJ_W<'a> = crate::BitWriter<'a, u32, SINGLECTRL_SPEC, bool, 2>;
#[doc = "Single Sample Resolution Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RES_A {
    #[doc = "0: 12-bit resolution"]
    _12BIT = 0,
    #[doc = "1: 8-bit resolution"]
    _8BIT = 1,
    #[doc = "2: 6-bit resolution"]
    _6BIT = 2,
    #[doc = "3: Oversampling enabled. Oversampling rate is set in OVSRSEL"]
    OVS = 3,
}
impl From<RES_A> for u8 {
    #[inline(always)]
    fn from(variant: RES_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RES` reader - Single Sample Resolution Select"]
pub type RES_R = crate::FieldReader<u8, RES_A>;
impl RES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RES_A {
        match self.bits {
            0 => RES_A::_12BIT,
            1 => RES_A::_8BIT,
            2 => RES_A::_6BIT,
            3 => RES_A::OVS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_12BIT`"]
    #[inline(always)]
    pub fn is_12bit(&self) -> bool {
        *self == RES_A::_12BIT
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == RES_A::_8BIT
    }
    #[doc = "Checks if the value of the field is `_6BIT`"]
    #[inline(always)]
    pub fn is_6bit(&self) -> bool {
        *self == RES_A::_6BIT
    }
    #[doc = "Checks if the value of the field is `OVS`"]
    #[inline(always)]
    pub fn is_ovs(&self) -> bool {
        *self == RES_A::OVS
    }
}
#[doc = "Field `RES` writer - Single Sample Resolution Select"]
pub type RES_W<'a> = crate::FieldWriterSafe<'a, u32, SINGLECTRL_SPEC, u8, RES_A, 2, 4>;
impl<'a> RES_W<'a> {
    #[doc = "12-bit resolution"]
    #[inline(always)]
    pub fn _12bit(self) -> &'a mut W {
        self.variant(RES_A::_12BIT)
    }
    #[doc = "8-bit resolution"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(RES_A::_8BIT)
    }
    #[doc = "6-bit resolution"]
    #[inline(always)]
    pub fn _6bit(self) -> &'a mut W {
        self.variant(RES_A::_6BIT)
    }
    #[doc = "Oversampling enabled. Oversampling rate is set in OVSRSEL"]
    #[inline(always)]
    pub fn ovs(self) -> &'a mut W {
        self.variant(RES_A::OVS)
    }
}
#[doc = "Field `INPUTSEL` reader - Single Sample Input Selection"]
pub type INPUTSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INPUTSEL` writer - Single Sample Input Selection"]
pub type INPUTSEL_W<'a> = crate::FieldWriter<'a, u32, SINGLECTRL_SPEC, u8, u8, 4, 8>;
#[doc = "Single Sample Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REF_A {
    #[doc = "0: Internal 1.25 V reference"]
    _1V25 = 0,
    #[doc = "1: Internal 2.5 V reference"]
    _2V5 = 1,
    #[doc = "2: Buffered VDD"]
    VDD = 2,
    #[doc = "3: Internal differential 5 V reference"]
    _5VDIFF = 3,
    #[doc = "4: Single ended external reference from ADCn_CH6"]
    EXTSINGLE = 4,
    #[doc = "5: Differential external reference, 2x(ADCn_CH6 - ADCn_CH7)"]
    _2XEXTDIFF = 5,
    #[doc = "6: Unbuffered 2xVDD"]
    _2XVDD = 6,
}
impl From<REF_A> for u8 {
    #[inline(always)]
    fn from(variant: REF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `REF` reader - Single Sample Reference Selection"]
pub type REF_R = crate::FieldReader<u8, REF_A>;
impl REF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REF_A> {
        match self.bits {
            0 => Some(REF_A::_1V25),
            1 => Some(REF_A::_2V5),
            2 => Some(REF_A::VDD),
            3 => Some(REF_A::_5VDIFF),
            4 => Some(REF_A::EXTSINGLE),
            5 => Some(REF_A::_2XEXTDIFF),
            6 => Some(REF_A::_2XVDD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1V25`"]
    #[inline(always)]
    pub fn is_1v25(&self) -> bool {
        *self == REF_A::_1V25
    }
    #[doc = "Checks if the value of the field is `_2V5`"]
    #[inline(always)]
    pub fn is_2v5(&self) -> bool {
        *self == REF_A::_2V5
    }
    #[doc = "Checks if the value of the field is `VDD`"]
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == REF_A::VDD
    }
    #[doc = "Checks if the value of the field is `_5VDIFF`"]
    #[inline(always)]
    pub fn is_5vdiff(&self) -> bool {
        *self == REF_A::_5VDIFF
    }
    #[doc = "Checks if the value of the field is `EXTSINGLE`"]
    #[inline(always)]
    pub fn is_extsingle(&self) -> bool {
        *self == REF_A::EXTSINGLE
    }
    #[doc = "Checks if the value of the field is `_2XEXTDIFF`"]
    #[inline(always)]
    pub fn is_2xextdiff(&self) -> bool {
        *self == REF_A::_2XEXTDIFF
    }
    #[doc = "Checks if the value of the field is `_2XVDD`"]
    #[inline(always)]
    pub fn is_2xvdd(&self) -> bool {
        *self == REF_A::_2XVDD
    }
}
#[doc = "Field `REF` writer - Single Sample Reference Selection"]
pub type REF_W<'a> = crate::FieldWriter<'a, u32, SINGLECTRL_SPEC, u8, REF_A, 3, 16>;
impl<'a> REF_W<'a> {
    #[doc = "Internal 1.25 V reference"]
    #[inline(always)]
    pub fn _1v25(self) -> &'a mut W {
        self.variant(REF_A::_1V25)
    }
    #[doc = "Internal 2.5 V reference"]
    #[inline(always)]
    pub fn _2v5(self) -> &'a mut W {
        self.variant(REF_A::_2V5)
    }
    #[doc = "Buffered VDD"]
    #[inline(always)]
    pub fn vdd(self) -> &'a mut W {
        self.variant(REF_A::VDD)
    }
    #[doc = "Internal differential 5 V reference"]
    #[inline(always)]
    pub fn _5vdiff(self) -> &'a mut W {
        self.variant(REF_A::_5VDIFF)
    }
    #[doc = "Single ended external reference from ADCn_CH6"]
    #[inline(always)]
    pub fn extsingle(self) -> &'a mut W {
        self.variant(REF_A::EXTSINGLE)
    }
    #[doc = "Differential external reference, 2x(ADCn_CH6 - ADCn_CH7)"]
    #[inline(always)]
    pub fn _2xextdiff(self) -> &'a mut W {
        self.variant(REF_A::_2XEXTDIFF)
    }
    #[doc = "Unbuffered 2xVDD"]
    #[inline(always)]
    pub fn _2xvdd(self) -> &'a mut W {
        self.variant(REF_A::_2XVDD)
    }
}
#[doc = "Single Sample Acquisition Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AT_A {
    #[doc = "0: 1 ADC_CLK cycle acquisition time for single sample"]
    _1CYCLE = 0,
    #[doc = "1: 2 ADC_CLK cycles acquisition time for single sample"]
    _2CYCLES = 1,
    #[doc = "2: 4 ADC_CLK cycles acquisition time for single sample"]
    _4CYCLES = 2,
    #[doc = "3: 8 ADC_CLK cycles acquisition time for single sample"]
    _8CYCLES = 3,
    #[doc = "4: 16 ADC_CLK cycles acquisition time for single sample"]
    _16CYCLES = 4,
    #[doc = "5: 32 ADC_CLK cycles acquisition time for single sample"]
    _32CYCLES = 5,
    #[doc = "6: 64 ADC_CLK cycles acquisition time for single sample"]
    _64CYCLES = 6,
    #[doc = "7: 128 ADC_CLK cycles acquisition time for single sample"]
    _128CYCLES = 7,
    #[doc = "8: 256 ADC_CLK cycles acquisition time for single sample"]
    _256CYCLES = 8,
}
impl From<AT_A> for u8 {
    #[inline(always)]
    fn from(variant: AT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AT` reader - Single Sample Acquisition Time"]
pub type AT_R = crate::FieldReader<u8, AT_A>;
impl AT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AT_A> {
        match self.bits {
            0 => Some(AT_A::_1CYCLE),
            1 => Some(AT_A::_2CYCLES),
            2 => Some(AT_A::_4CYCLES),
            3 => Some(AT_A::_8CYCLES),
            4 => Some(AT_A::_16CYCLES),
            5 => Some(AT_A::_32CYCLES),
            6 => Some(AT_A::_64CYCLES),
            7 => Some(AT_A::_128CYCLES),
            8 => Some(AT_A::_256CYCLES),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1CYCLE`"]
    #[inline(always)]
    pub fn is_1cycle(&self) -> bool {
        *self == AT_A::_1CYCLE
    }
    #[doc = "Checks if the value of the field is `_2CYCLES`"]
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == AT_A::_2CYCLES
    }
    #[doc = "Checks if the value of the field is `_4CYCLES`"]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == AT_A::_4CYCLES
    }
    #[doc = "Checks if the value of the field is `_8CYCLES`"]
    #[inline(always)]
    pub fn is_8cycles(&self) -> bool {
        *self == AT_A::_8CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == AT_A::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == AT_A::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_64CYCLES`"]
    #[inline(always)]
    pub fn is_64cycles(&self) -> bool {
        *self == AT_A::_64CYCLES
    }
    #[doc = "Checks if the value of the field is `_128CYCLES`"]
    #[inline(always)]
    pub fn is_128cycles(&self) -> bool {
        *self == AT_A::_128CYCLES
    }
    #[doc = "Checks if the value of the field is `_256CYCLES`"]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == AT_A::_256CYCLES
    }
}
#[doc = "Field `AT` writer - Single Sample Acquisition Time"]
pub type AT_W<'a> = crate::FieldWriter<'a, u32, SINGLECTRL_SPEC, u8, AT_A, 4, 20>;
impl<'a> AT_W<'a> {
    #[doc = "1 ADC_CLK cycle acquisition time for single sample"]
    #[inline(always)]
    pub fn _1cycle(self) -> &'a mut W {
        self.variant(AT_A::_1CYCLE)
    }
    #[doc = "2 ADC_CLK cycles acquisition time for single sample"]
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut W {
        self.variant(AT_A::_2CYCLES)
    }
    #[doc = "4 ADC_CLK cycles acquisition time for single sample"]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut W {
        self.variant(AT_A::_4CYCLES)
    }
    #[doc = "8 ADC_CLK cycles acquisition time for single sample"]
    #[inline(always)]
    pub fn _8cycles(self) -> &'a mut W {
        self.variant(AT_A::_8CYCLES)
    }
    #[doc = "16 ADC_CLK cycles acquisition time for single sample"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(AT_A::_16CYCLES)
    }
    #[doc = "32 ADC_CLK cycles acquisition time for single sample"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(AT_A::_32CYCLES)
    }
    #[doc = "64 ADC_CLK cycles acquisition time for single sample"]
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut W {
        self.variant(AT_A::_64CYCLES)
    }
    #[doc = "128 ADC_CLK cycles acquisition time for single sample"]
    #[inline(always)]
    pub fn _128cycles(self) -> &'a mut W {
        self.variant(AT_A::_128CYCLES)
    }
    #[doc = "256 ADC_CLK cycles acquisition time for single sample"]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut W {
        self.variant(AT_A::_256CYCLES)
    }
}
#[doc = "Field `PRSEN` reader - Single Sample PRS Trigger Enable"]
pub type PRSEN_R = crate::BitReader<bool>;
#[doc = "Field `PRSEN` writer - Single Sample PRS Trigger Enable"]
pub type PRSEN_W<'a> = crate::BitWriter<'a, u32, SINGLECTRL_SPEC, bool, 24>;
#[doc = "Single Sample PRS Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRSSEL_A {
    #[doc = "0: PRS ch 0 triggers single sample"]
    PRSCH0 = 0,
    #[doc = "1: PRS ch 1 triggers single sample"]
    PRSCH1 = 1,
    #[doc = "2: PRS ch 2 triggers single sample"]
    PRSCH2 = 2,
    #[doc = "3: PRS ch 3 triggers single sample"]
    PRSCH3 = 3,
    #[doc = "4: PRS ch 4 triggers single sample"]
    PRSCH4 = 4,
    #[doc = "5: PRS ch 5 triggers single sample"]
    PRSCH5 = 5,
    #[doc = "6: PRS ch 6 triggers single sample"]
    PRSCH6 = 6,
    #[doc = "7: PRS ch 7 triggers single sample"]
    PRSCH7 = 7,
    #[doc = "8: PRS ch 8 triggers single sample"]
    PRSCH8 = 8,
    #[doc = "9: PRS ch 9 triggers single sample"]
    PRSCH9 = 9,
    #[doc = "10: PRS ch 10 triggers single sample"]
    PRSCH10 = 10,
    #[doc = "11: PRS ch 11 triggers single sample"]
    PRSCH11 = 11,
}
impl From<PRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRSSEL` reader - Single Sample PRS Trigger Select"]
pub type PRSSEL_R = crate::FieldReader<u8, PRSSEL_A>;
impl PRSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRSSEL_A> {
        match self.bits {
            0 => Some(PRSSEL_A::PRSCH0),
            1 => Some(PRSSEL_A::PRSCH1),
            2 => Some(PRSSEL_A::PRSCH2),
            3 => Some(PRSSEL_A::PRSCH3),
            4 => Some(PRSSEL_A::PRSCH4),
            5 => Some(PRSSEL_A::PRSCH5),
            6 => Some(PRSSEL_A::PRSCH6),
            7 => Some(PRSSEL_A::PRSCH7),
            8 => Some(PRSSEL_A::PRSCH8),
            9 => Some(PRSSEL_A::PRSCH9),
            10 => Some(PRSSEL_A::PRSCH10),
            11 => Some(PRSSEL_A::PRSCH11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL_A::PRSCH11
    }
}
#[doc = "Field `PRSSEL` writer - Single Sample PRS Trigger Select"]
pub type PRSSEL_W<'a> = crate::FieldWriter<'a, u32, SINGLECTRL_SPEC, u8, PRSSEL_A, 4, 28>;
impl<'a> PRSSEL_W<'a> {
    #[doc = "PRS ch 0 triggers single sample"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH0)
    }
    #[doc = "PRS ch 1 triggers single sample"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH1)
    }
    #[doc = "PRS ch 2 triggers single sample"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH2)
    }
    #[doc = "PRS ch 3 triggers single sample"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH3)
    }
    #[doc = "PRS ch 4 triggers single sample"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH4)
    }
    #[doc = "PRS ch 5 triggers single sample"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH5)
    }
    #[doc = "PRS ch 6 triggers single sample"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH6)
    }
    #[doc = "PRS ch 7 triggers single sample"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH7)
    }
    #[doc = "PRS ch 8 triggers single sample"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH8)
    }
    #[doc = "PRS ch 9 triggers single sample"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH9)
    }
    #[doc = "PRS ch 10 triggers single sample"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH10)
    }
    #[doc = "PRS ch 11 triggers single sample"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH11)
    }
}
impl R {
    #[doc = "Bit 0 - Single Sample Repetitive Mode"]
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Single Sample Differential Mode"]
    #[inline(always)]
    pub fn diff(&self) -> DIFF_R {
        DIFF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Single Sample Result Adjustment"]
    #[inline(always)]
    pub fn adj(&self) -> ADJ_R {
        ADJ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Single Sample Resolution Select"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Single Sample Input Selection"]
    #[inline(always)]
    pub fn inputsel(&self) -> INPUTSEL_R {
        INPUTSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - Single Sample Reference Selection"]
    #[inline(always)]
    pub fn ref_(&self) -> REF_R {
        REF_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:23 - Single Sample Acquisition Time"]
    #[inline(always)]
    pub fn at(&self) -> AT_R {
        AT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Single Sample PRS Trigger Enable"]
    #[inline(always)]
    pub fn prsen(&self) -> PRSEN_R {
        PRSEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Single Sample PRS Trigger Select"]
    #[inline(always)]
    pub fn prssel(&self) -> PRSSEL_R {
        PRSSEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Single Sample Repetitive Mode"]
    #[inline(always)]
    pub fn rep(&mut self) -> REP_W {
        REP_W::new(self)
    }
    #[doc = "Bit 1 - Single Sample Differential Mode"]
    #[inline(always)]
    pub fn diff(&mut self) -> DIFF_W {
        DIFF_W::new(self)
    }
    #[doc = "Bit 2 - Single Sample Result Adjustment"]
    #[inline(always)]
    pub fn adj(&mut self) -> ADJ_W {
        ADJ_W::new(self)
    }
    #[doc = "Bits 4:5 - Single Sample Resolution Select"]
    #[inline(always)]
    pub fn res(&mut self) -> RES_W {
        RES_W::new(self)
    }
    #[doc = "Bits 8:11 - Single Sample Input Selection"]
    #[inline(always)]
    pub fn inputsel(&mut self) -> INPUTSEL_W {
        INPUTSEL_W::new(self)
    }
    #[doc = "Bits 16:18 - Single Sample Reference Selection"]
    #[inline(always)]
    pub fn ref_(&mut self) -> REF_W {
        REF_W::new(self)
    }
    #[doc = "Bits 20:23 - Single Sample Acquisition Time"]
    #[inline(always)]
    pub fn at(&mut self) -> AT_W {
        AT_W::new(self)
    }
    #[doc = "Bit 24 - Single Sample PRS Trigger Enable"]
    #[inline(always)]
    pub fn prsen(&mut self) -> PRSEN_W {
        PRSEN_W::new(self)
    }
    #[doc = "Bits 28:31 - Single Sample PRS Trigger Select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PRSSEL_W {
        PRSSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Single Sample Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [singlectrl](index.html) module"]
pub struct SINGLECTRL_SPEC;
impl crate::RegisterSpec for SINGLECTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [singlectrl::R](R) reader structure"]
impl crate::Readable for SINGLECTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [singlectrl::W](W) writer structure"]
impl crate::Writable for SINGLECTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SINGLECTRL to value 0"]
impl crate::Resettable for SINGLECTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
