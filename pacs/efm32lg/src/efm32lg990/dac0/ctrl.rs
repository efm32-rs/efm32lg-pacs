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
#[doc = "Field `DIFF` reader - Differential Mode"]
pub type DIFF_R = crate::BitReader<bool>;
#[doc = "Field `DIFF` writer - Differential Mode"]
pub type DIFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SINEMODE` reader - Sine Mode"]
pub type SINEMODE_R = crate::BitReader<bool>;
#[doc = "Field `SINEMODE` writer - Sine Mode"]
pub type SINEMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CONVMODE` reader - Conversion Mode"]
pub type CONVMODE_R = crate::FieldReader<u8, CONVMODE_A>;
#[doc = "Conversion Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CONVMODE_A {
    #[doc = "0: DAC is set in continuous mode"]
    CONTINUOUS = 0,
    #[doc = "1: DAC is set in sample/hold mode"]
    SAMPLEHOLD = 1,
    #[doc = "2: DAC is set in sample/shut off mode"]
    SAMPLEOFF = 2,
}
impl From<CONVMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CONVMODE_A) -> Self {
        variant as _
    }
}
impl CONVMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CONVMODE_A> {
        match self.bits {
            0 => Some(CONVMODE_A::CONTINUOUS),
            1 => Some(CONVMODE_A::SAMPLEHOLD),
            2 => Some(CONVMODE_A::SAMPLEOFF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CONVMODE_A::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `SAMPLEHOLD`"]
    #[inline(always)]
    pub fn is_samplehold(&self) -> bool {
        *self == CONVMODE_A::SAMPLEHOLD
    }
    #[doc = "Checks if the value of the field is `SAMPLEOFF`"]
    #[inline(always)]
    pub fn is_sampleoff(&self) -> bool {
        *self == CONVMODE_A::SAMPLEOFF
    }
}
#[doc = "Field `CONVMODE` writer - Conversion Mode"]
pub type CONVMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, CONVMODE_A, 2, O>;
impl<'a, const O: u8> CONVMODE_W<'a, O> {
    #[doc = "DAC is set in continuous mode"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CONVMODE_A::CONTINUOUS)
    }
    #[doc = "DAC is set in sample/hold mode"]
    #[inline(always)]
    pub fn samplehold(self) -> &'a mut W {
        self.variant(CONVMODE_A::SAMPLEHOLD)
    }
    #[doc = "DAC is set in sample/shut off mode"]
    #[inline(always)]
    pub fn sampleoff(self) -> &'a mut W {
        self.variant(CONVMODE_A::SAMPLEOFF)
    }
}
#[doc = "Field `OUTMODE` reader - Output Mode"]
pub type OUTMODE_R = crate::FieldReader<u8, OUTMODE_A>;
#[doc = "Output Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OUTMODE_A {
    #[doc = "0: DAC output to pin and ADC disabled"]
    DISABLE = 0,
    #[doc = "1: DAC output to pin enabled. DAC output to ADC and ACMP disabled"]
    PIN = 1,
    #[doc = "2: DAC output to pin disabled. DAC output to ADC and ACMP enabled"]
    ADC = 2,
    #[doc = "3: DAC output to pin, ADC, and ACMP enabled"]
    PINADC = 3,
}
impl From<OUTMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: OUTMODE_A) -> Self {
        variant as _
    }
}
impl OUTMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTMODE_A {
        match self.bits {
            0 => OUTMODE_A::DISABLE,
            1 => OUTMODE_A::PIN,
            2 => OUTMODE_A::ADC,
            3 => OUTMODE_A::PINADC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == OUTMODE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `PIN`"]
    #[inline(always)]
    pub fn is_pin(&self) -> bool {
        *self == OUTMODE_A::PIN
    }
    #[doc = "Checks if the value of the field is `ADC`"]
    #[inline(always)]
    pub fn is_adc(&self) -> bool {
        *self == OUTMODE_A::ADC
    }
    #[doc = "Checks if the value of the field is `PINADC`"]
    #[inline(always)]
    pub fn is_pinadc(&self) -> bool {
        *self == OUTMODE_A::PINADC
    }
}
#[doc = "Field `OUTMODE` writer - Output Mode"]
pub type OUTMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, OUTMODE_A, 2, O>;
impl<'a, const O: u8> OUTMODE_W<'a, O> {
    #[doc = "DAC output to pin and ADC disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(OUTMODE_A::DISABLE)
    }
    #[doc = "DAC output to pin enabled. DAC output to ADC and ACMP disabled"]
    #[inline(always)]
    pub fn pin(self) -> &'a mut W {
        self.variant(OUTMODE_A::PIN)
    }
    #[doc = "DAC output to pin disabled. DAC output to ADC and ACMP enabled"]
    #[inline(always)]
    pub fn adc(self) -> &'a mut W {
        self.variant(OUTMODE_A::ADC)
    }
    #[doc = "DAC output to pin, ADC, and ACMP enabled"]
    #[inline(always)]
    pub fn pinadc(self) -> &'a mut W {
        self.variant(OUTMODE_A::PINADC)
    }
}
#[doc = "Field `OUTENPRS` reader - PRS Controlled Output Enable"]
pub type OUTENPRS_R = crate::BitReader<bool>;
#[doc = "Field `OUTENPRS` writer - PRS Controlled Output Enable"]
pub type OUTENPRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CH0PRESCRST` reader - Channel 0 Start Reset Prescaler"]
pub type CH0PRESCRST_R = crate::BitReader<bool>;
#[doc = "Field `CH0PRESCRST` writer - Channel 0 Start Reset Prescaler"]
pub type CH0PRESCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `REFSEL` reader - Reference Selection"]
pub type REFSEL_R = crate::FieldReader<u8, REFSEL_A>;
#[doc = "Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REFSEL_A {
    #[doc = "0: Internal 1.25 V bandgap reference"]
    _1V25 = 0,
    #[doc = "1: Internal 2.5 V bandgap reference"]
    _2V5 = 1,
    #[doc = "2: VDD reference"]
    VDD = 2,
}
impl From<REFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFSEL_A) -> Self {
        variant as _
    }
}
impl REFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REFSEL_A> {
        match self.bits {
            0 => Some(REFSEL_A::_1V25),
            1 => Some(REFSEL_A::_2V5),
            2 => Some(REFSEL_A::VDD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1V25`"]
    #[inline(always)]
    pub fn is_1v25(&self) -> bool {
        *self == REFSEL_A::_1V25
    }
    #[doc = "Checks if the value of the field is `_2V5`"]
    #[inline(always)]
    pub fn is_2v5(&self) -> bool {
        *self == REFSEL_A::_2V5
    }
    #[doc = "Checks if the value of the field is `VDD`"]
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == REFSEL_A::VDD
    }
}
#[doc = "Field `REFSEL` writer - Reference Selection"]
pub type REFSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, REFSEL_A, 2, O>;
impl<'a, const O: u8> REFSEL_W<'a, O> {
    #[doc = "Internal 1.25 V bandgap reference"]
    #[inline(always)]
    pub fn _1v25(self) -> &'a mut W {
        self.variant(REFSEL_A::_1V25)
    }
    #[doc = "Internal 2.5 V bandgap reference"]
    #[inline(always)]
    pub fn _2v5(self) -> &'a mut W {
        self.variant(REFSEL_A::_2V5)
    }
    #[doc = "VDD reference"]
    #[inline(always)]
    pub fn vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::VDD)
    }
}
#[doc = "Field `PRESC` reader - Prescaler Setting"]
pub type PRESC_R = crate::FieldReader<u8, PRESC_A>;
#[doc = "Prescaler Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESC_A {
    #[doc = "0: `0`"]
    NODIVISION = 0,
}
impl From<PRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        variant as _
    }
}
impl PRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRESC_A> {
        match self.bits {
            0 => Some(PRESC_A::NODIVISION),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NODIVISION`"]
    #[inline(always)]
    pub fn is_nodivision(&self) -> bool {
        *self == PRESC_A::NODIVISION
    }
}
#[doc = "Field `PRESC` writer - Prescaler Setting"]
pub type PRESC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, PRESC_A, 3, O>;
impl<'a, const O: u8> PRESC_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nodivision(self) -> &'a mut W {
        self.variant(PRESC_A::NODIVISION)
    }
}
#[doc = "Field `REFRSEL` reader - Refresh Interval Select"]
pub type REFRSEL_R = crate::FieldReader<u8, REFRSEL_A>;
#[doc = "Refresh Interval Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REFRSEL_A {
    #[doc = "0: All channels with enabled refresh are refreshed every 8 prescaled cycles"]
    _8CYCLES = 0,
    #[doc = "1: All channels with enabled refresh are refreshed every 16 prescaled cycles"]
    _16CYCLES = 1,
    #[doc = "2: All channels with enabled refresh are refreshed every 32 prescaled cycles"]
    _32CYCLES = 2,
    #[doc = "3: All channels with enabled refresh are refreshed every 64 prescaled cycles"]
    _64CYCLES = 3,
}
impl From<REFRSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFRSEL_A) -> Self {
        variant as _
    }
}
impl REFRSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFRSEL_A {
        match self.bits {
            0 => REFRSEL_A::_8CYCLES,
            1 => REFRSEL_A::_16CYCLES,
            2 => REFRSEL_A::_32CYCLES,
            3 => REFRSEL_A::_64CYCLES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8CYCLES`"]
    #[inline(always)]
    pub fn is_8cycles(&self) -> bool {
        *self == REFRSEL_A::_8CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == REFRSEL_A::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == REFRSEL_A::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_64CYCLES`"]
    #[inline(always)]
    pub fn is_64cycles(&self) -> bool {
        *self == REFRSEL_A::_64CYCLES
    }
}
#[doc = "Field `REFRSEL` writer - Refresh Interval Select"]
pub type REFRSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, REFRSEL_A, 2, O>;
impl<'a, const O: u8> REFRSEL_W<'a, O> {
    #[doc = "All channels with enabled refresh are refreshed every 8 prescaled cycles"]
    #[inline(always)]
    pub fn _8cycles(self) -> &'a mut W {
        self.variant(REFRSEL_A::_8CYCLES)
    }
    #[doc = "All channels with enabled refresh are refreshed every 16 prescaled cycles"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(REFRSEL_A::_16CYCLES)
    }
    #[doc = "All channels with enabled refresh are refreshed every 32 prescaled cycles"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(REFRSEL_A::_32CYCLES)
    }
    #[doc = "All channels with enabled refresh are refreshed every 64 prescaled cycles"]
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut W {
        self.variant(REFRSEL_A::_64CYCLES)
    }
}
impl R {
    #[doc = "Bit 0 - Differential Mode"]
    #[inline(always)]
    pub fn diff(&self) -> DIFF_R {
        DIFF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sine Mode"]
    #[inline(always)]
    pub fn sinemode(&self) -> SINEMODE_R {
        SINEMODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Conversion Mode"]
    #[inline(always)]
    pub fn convmode(&self) -> CONVMODE_R {
        CONVMODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Output Mode"]
    #[inline(always)]
    pub fn outmode(&self) -> OUTMODE_R {
        OUTMODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - PRS Controlled Output Enable"]
    #[inline(always)]
    pub fn outenprs(&self) -> OUTENPRS_R {
        OUTENPRS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 0 Start Reset Prescaler"]
    #[inline(always)]
    pub fn ch0prescrst(&self) -> CH0PRESCRST_R {
        CH0PRESCRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Reference Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:18 - Prescaler Setting"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:21 - Refresh Interval Select"]
    #[inline(always)]
    pub fn refrsel(&self) -> REFRSEL_R {
        REFRSEL_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Differential Mode"]
    #[inline(always)]
    #[must_use]
    pub fn diff(&mut self) -> DIFF_W<0> {
        DIFF_W::new(self)
    }
    #[doc = "Bit 1 - Sine Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sinemode(&mut self) -> SINEMODE_W<1> {
        SINEMODE_W::new(self)
    }
    #[doc = "Bits 2:3 - Conversion Mode"]
    #[inline(always)]
    #[must_use]
    pub fn convmode(&mut self) -> CONVMODE_W<2> {
        CONVMODE_W::new(self)
    }
    #[doc = "Bits 4:5 - Output Mode"]
    #[inline(always)]
    #[must_use]
    pub fn outmode(&mut self) -> OUTMODE_W<4> {
        OUTMODE_W::new(self)
    }
    #[doc = "Bit 6 - PRS Controlled Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn outenprs(&mut self) -> OUTENPRS_W<6> {
        OUTENPRS_W::new(self)
    }
    #[doc = "Bit 7 - Channel 0 Start Reset Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn ch0prescrst(&mut self) -> CH0PRESCRST_W<7> {
        CH0PRESCRST_W::new(self)
    }
    #[doc = "Bits 8:9 - Reference Selection"]
    #[inline(always)]
    #[must_use]
    pub fn refsel(&mut self) -> REFSEL_W<8> {
        REFSEL_W::new(self)
    }
    #[doc = "Bits 16:18 - Prescaler Setting"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<16> {
        PRESC_W::new(self)
    }
    #[doc = "Bits 20:21 - Refresh Interval Select"]
    #[inline(always)]
    #[must_use]
    pub fn refrsel(&mut self) -> REFRSEL_W<20> {
        REFRSEL_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x10"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
