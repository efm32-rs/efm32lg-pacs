#[doc = "Register `PERCTRL` reader"]
pub struct R(crate::R<PERCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERCTRL` writer"]
pub struct W(crate::W<PERCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERCTRL_SPEC>;
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
impl From<crate::W<PERCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DACCH0DATA` reader - DAC CH0 data selection."]
pub type DACCH0DATA_R = crate::BitReader<bool>;
#[doc = "Field `DACCH0DATA` writer - DAC CH0 data selection."]
pub type DACCH0DATA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERCTRL_SPEC, bool, O>;
#[doc = "Field `DACCH1DATA` reader - DAC CH1 data selection."]
pub type DACCH1DATA_R = crate::BitReader<bool>;
#[doc = "Field `DACCH1DATA` writer - DAC CH1 data selection."]
pub type DACCH1DATA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERCTRL_SPEC, bool, O>;
#[doc = "Field `DACCH0CONV` reader - DAC channel 0 conversion mode"]
pub type DACCH0CONV_R = crate::FieldReader<u8, DACCH0CONV_A>;
#[doc = "DAC channel 0 conversion mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DACCH0CONV_A {
    #[doc = "0: LESENSE does not control DAC CH0."]
    DISABLE = 0,
    #[doc = "1: DAC channel 0 is driven in continuous mode."]
    CONTINUOUS = 1,
    #[doc = "2: DAC channel 0 is driven in sample hold mode."]
    SAMPLEHOLD = 2,
    #[doc = "3: DAC channel 0 is driven in sample off mode."]
    SAMPLEOFF = 3,
}
impl From<DACCH0CONV_A> for u8 {
    #[inline(always)]
    fn from(variant: DACCH0CONV_A) -> Self {
        variant as _
    }
}
impl DACCH0CONV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACCH0CONV_A {
        match self.bits {
            0 => DACCH0CONV_A::DISABLE,
            1 => DACCH0CONV_A::CONTINUOUS,
            2 => DACCH0CONV_A::SAMPLEHOLD,
            3 => DACCH0CONV_A::SAMPLEOFF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DACCH0CONV_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == DACCH0CONV_A::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `SAMPLEHOLD`"]
    #[inline(always)]
    pub fn is_samplehold(&self) -> bool {
        *self == DACCH0CONV_A::SAMPLEHOLD
    }
    #[doc = "Checks if the value of the field is `SAMPLEOFF`"]
    #[inline(always)]
    pub fn is_sampleoff(&self) -> bool {
        *self == DACCH0CONV_A::SAMPLEOFF
    }
}
#[doc = "Field `DACCH0CONV` writer - DAC channel 0 conversion mode"]
pub type DACCH0CONV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PERCTRL_SPEC, u8, DACCH0CONV_A, 2, O>;
impl<'a, const O: u8> DACCH0CONV_W<'a, O> {
    #[doc = "LESENSE does not control DAC CH0."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DACCH0CONV_A::DISABLE)
    }
    #[doc = "DAC channel 0 is driven in continuous mode."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(DACCH0CONV_A::CONTINUOUS)
    }
    #[doc = "DAC channel 0 is driven in sample hold mode."]
    #[inline(always)]
    pub fn samplehold(self) -> &'a mut W {
        self.variant(DACCH0CONV_A::SAMPLEHOLD)
    }
    #[doc = "DAC channel 0 is driven in sample off mode."]
    #[inline(always)]
    pub fn sampleoff(self) -> &'a mut W {
        self.variant(DACCH0CONV_A::SAMPLEOFF)
    }
}
#[doc = "Field `DACCH1CONV` reader - DAC channel 1 conversion mode"]
pub type DACCH1CONV_R = crate::FieldReader<u8, DACCH1CONV_A>;
#[doc = "DAC channel 1 conversion mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DACCH1CONV_A {
    #[doc = "0: LESENSE does not control DAC CH1."]
    DISABLE = 0,
    #[doc = "1: DAC channel 1 is driven in continuous mode."]
    CONTINUOUS = 1,
    #[doc = "2: DAC channel 1 is driven in sample hold mode."]
    SAMPLEHOLD = 2,
    #[doc = "3: DAC channel 1 is driven in sample off mode."]
    SAMPLEOFF = 3,
}
impl From<DACCH1CONV_A> for u8 {
    #[inline(always)]
    fn from(variant: DACCH1CONV_A) -> Self {
        variant as _
    }
}
impl DACCH1CONV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACCH1CONV_A {
        match self.bits {
            0 => DACCH1CONV_A::DISABLE,
            1 => DACCH1CONV_A::CONTINUOUS,
            2 => DACCH1CONV_A::SAMPLEHOLD,
            3 => DACCH1CONV_A::SAMPLEOFF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DACCH1CONV_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == DACCH1CONV_A::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `SAMPLEHOLD`"]
    #[inline(always)]
    pub fn is_samplehold(&self) -> bool {
        *self == DACCH1CONV_A::SAMPLEHOLD
    }
    #[doc = "Checks if the value of the field is `SAMPLEOFF`"]
    #[inline(always)]
    pub fn is_sampleoff(&self) -> bool {
        *self == DACCH1CONV_A::SAMPLEOFF
    }
}
#[doc = "Field `DACCH1CONV` writer - DAC channel 1 conversion mode"]
pub type DACCH1CONV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PERCTRL_SPEC, u8, DACCH1CONV_A, 2, O>;
impl<'a, const O: u8> DACCH1CONV_W<'a, O> {
    #[doc = "LESENSE does not control DAC CH1."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DACCH1CONV_A::DISABLE)
    }
    #[doc = "DAC channel 1 is driven in continuous mode."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(DACCH1CONV_A::CONTINUOUS)
    }
    #[doc = "DAC channel 1 is driven in sample hold mode."]
    #[inline(always)]
    pub fn samplehold(self) -> &'a mut W {
        self.variant(DACCH1CONV_A::SAMPLEHOLD)
    }
    #[doc = "DAC channel 1 is driven in sample off mode."]
    #[inline(always)]
    pub fn sampleoff(self) -> &'a mut W {
        self.variant(DACCH1CONV_A::SAMPLEOFF)
    }
}
#[doc = "Field `DACCH0OUT` reader - DAC channel 0 output mode"]
pub type DACCH0OUT_R = crate::FieldReader<u8, DACCH0OUT_A>;
#[doc = "DAC channel 0 output mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DACCH0OUT_A {
    #[doc = "0: DAC CH0 output to pin and ACMP/ADC disabled"]
    DISABLE = 0,
    #[doc = "1: DAC CH0 output to pin enabled, output to ADC and ACMP disabled"]
    PIN = 1,
    #[doc = "2: DAC CH0 output to pin disabled, output to ADC and ACMP enabled"]
    ADCACMP = 2,
    #[doc = "3: DAC CH0 output to pin, ADC, and ACMP enabled."]
    PINADCACMP = 3,
}
impl From<DACCH0OUT_A> for u8 {
    #[inline(always)]
    fn from(variant: DACCH0OUT_A) -> Self {
        variant as _
    }
}
impl DACCH0OUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACCH0OUT_A {
        match self.bits {
            0 => DACCH0OUT_A::DISABLE,
            1 => DACCH0OUT_A::PIN,
            2 => DACCH0OUT_A::ADCACMP,
            3 => DACCH0OUT_A::PINADCACMP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DACCH0OUT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `PIN`"]
    #[inline(always)]
    pub fn is_pin(&self) -> bool {
        *self == DACCH0OUT_A::PIN
    }
    #[doc = "Checks if the value of the field is `ADCACMP`"]
    #[inline(always)]
    pub fn is_adcacmp(&self) -> bool {
        *self == DACCH0OUT_A::ADCACMP
    }
    #[doc = "Checks if the value of the field is `PINADCACMP`"]
    #[inline(always)]
    pub fn is_pinadcacmp(&self) -> bool {
        *self == DACCH0OUT_A::PINADCACMP
    }
}
#[doc = "Field `DACCH0OUT` writer - DAC channel 0 output mode"]
pub type DACCH0OUT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PERCTRL_SPEC, u8, DACCH0OUT_A, 2, O>;
impl<'a, const O: u8> DACCH0OUT_W<'a, O> {
    #[doc = "DAC CH0 output to pin and ACMP/ADC disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DACCH0OUT_A::DISABLE)
    }
    #[doc = "DAC CH0 output to pin enabled, output to ADC and ACMP disabled"]
    #[inline(always)]
    pub fn pin(self) -> &'a mut W {
        self.variant(DACCH0OUT_A::PIN)
    }
    #[doc = "DAC CH0 output to pin disabled, output to ADC and ACMP enabled"]
    #[inline(always)]
    pub fn adcacmp(self) -> &'a mut W {
        self.variant(DACCH0OUT_A::ADCACMP)
    }
    #[doc = "DAC CH0 output to pin, ADC, and ACMP enabled."]
    #[inline(always)]
    pub fn pinadcacmp(self) -> &'a mut W {
        self.variant(DACCH0OUT_A::PINADCACMP)
    }
}
#[doc = "Field `DACCH1OUT` reader - DAC channel 1 output mode"]
pub type DACCH1OUT_R = crate::FieldReader<u8, DACCH1OUT_A>;
#[doc = "DAC channel 1 output mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DACCH1OUT_A {
    #[doc = "0: DAC CH1 output to pin and ACMP/ADC disabled"]
    DISABLE = 0,
    #[doc = "1: DAC CH1 output to pin enabled, output to ADC and ACMP disabled"]
    PIN = 1,
    #[doc = "2: DAC CH1 output to pin disabled, output to ADC and ACMP enabled"]
    ADCACMP = 2,
    #[doc = "3: DAC CH1 output to pin, ADC, and ACMP enabled."]
    PINADCACMP = 3,
}
impl From<DACCH1OUT_A> for u8 {
    #[inline(always)]
    fn from(variant: DACCH1OUT_A) -> Self {
        variant as _
    }
}
impl DACCH1OUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACCH1OUT_A {
        match self.bits {
            0 => DACCH1OUT_A::DISABLE,
            1 => DACCH1OUT_A::PIN,
            2 => DACCH1OUT_A::ADCACMP,
            3 => DACCH1OUT_A::PINADCACMP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DACCH1OUT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `PIN`"]
    #[inline(always)]
    pub fn is_pin(&self) -> bool {
        *self == DACCH1OUT_A::PIN
    }
    #[doc = "Checks if the value of the field is `ADCACMP`"]
    #[inline(always)]
    pub fn is_adcacmp(&self) -> bool {
        *self == DACCH1OUT_A::ADCACMP
    }
    #[doc = "Checks if the value of the field is `PINADCACMP`"]
    #[inline(always)]
    pub fn is_pinadcacmp(&self) -> bool {
        *self == DACCH1OUT_A::PINADCACMP
    }
}
#[doc = "Field `DACCH1OUT` writer - DAC channel 1 output mode"]
pub type DACCH1OUT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PERCTRL_SPEC, u8, DACCH1OUT_A, 2, O>;
impl<'a, const O: u8> DACCH1OUT_W<'a, O> {
    #[doc = "DAC CH1 output to pin and ACMP/ADC disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DACCH1OUT_A::DISABLE)
    }
    #[doc = "DAC CH1 output to pin enabled, output to ADC and ACMP disabled"]
    #[inline(always)]
    pub fn pin(self) -> &'a mut W {
        self.variant(DACCH1OUT_A::PIN)
    }
    #[doc = "DAC CH1 output to pin disabled, output to ADC and ACMP enabled"]
    #[inline(always)]
    pub fn adcacmp(self) -> &'a mut W {
        self.variant(DACCH1OUT_A::ADCACMP)
    }
    #[doc = "DAC CH1 output to pin, ADC, and ACMP enabled."]
    #[inline(always)]
    pub fn pinadcacmp(self) -> &'a mut W {
        self.variant(DACCH1OUT_A::PINADCACMP)
    }
}
#[doc = "Field `DACPRESC` reader - DAC prescaler configuration."]
pub type DACPRESC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DACPRESC` writer - DAC prescaler configuration."]
pub type DACPRESC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PERCTRL_SPEC, u8, u8, 5, O>;
#[doc = "Field `DACREF` reader - DAC bandgap reference used"]
pub type DACREF_R = crate::BitReader<bool>;
#[doc = "Field `DACREF` writer - DAC bandgap reference used"]
pub type DACREF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERCTRL_SPEC, bool, O>;
#[doc = "Field `ACMP0MODE` reader - ACMP0 mode"]
pub type ACMP0MODE_R = crate::FieldReader<u8, ACMP0MODE_A>;
#[doc = "ACMP0 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACMP0MODE_A {
    #[doc = "0: LESENSE does not control ACMP0"]
    DISABLE = 0,
    #[doc = "1: LESENSE controls the input mux (POSSEL) of ACMP0"]
    MUX = 1,
    #[doc = "2: LESENSE controls the input mux (POSSEL) and the threshold value (VDDLEVEL) of ACMP0"]
    MUXTHRES = 2,
}
impl From<ACMP0MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: ACMP0MODE_A) -> Self {
        variant as _
    }
}
impl ACMP0MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ACMP0MODE_A> {
        match self.bits {
            0 => Some(ACMP0MODE_A::DISABLE),
            1 => Some(ACMP0MODE_A::MUX),
            2 => Some(ACMP0MODE_A::MUXTHRES),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ACMP0MODE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `MUX`"]
    #[inline(always)]
    pub fn is_mux(&self) -> bool {
        *self == ACMP0MODE_A::MUX
    }
    #[doc = "Checks if the value of the field is `MUXTHRES`"]
    #[inline(always)]
    pub fn is_muxthres(&self) -> bool {
        *self == ACMP0MODE_A::MUXTHRES
    }
}
#[doc = "Field `ACMP0MODE` writer - ACMP0 mode"]
pub type ACMP0MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PERCTRL_SPEC, u8, ACMP0MODE_A, 2, O>;
impl<'a, const O: u8> ACMP0MODE_W<'a, O> {
    #[doc = "LESENSE does not control ACMP0"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ACMP0MODE_A::DISABLE)
    }
    #[doc = "LESENSE controls the input mux (POSSEL) of ACMP0"]
    #[inline(always)]
    pub fn mux(self) -> &'a mut W {
        self.variant(ACMP0MODE_A::MUX)
    }
    #[doc = "LESENSE controls the input mux (POSSEL) and the threshold value (VDDLEVEL) of ACMP0"]
    #[inline(always)]
    pub fn muxthres(self) -> &'a mut W {
        self.variant(ACMP0MODE_A::MUXTHRES)
    }
}
#[doc = "Field `ACMP1MODE` reader - ACMP1 mode"]
pub type ACMP1MODE_R = crate::FieldReader<u8, ACMP1MODE_A>;
#[doc = "ACMP1 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACMP1MODE_A {
    #[doc = "0: LESENSE does not control ACMP1"]
    DISABLE = 0,
    #[doc = "1: LESENSE controls the input mux (POSSEL) of ACMP1"]
    MUX = 1,
    #[doc = "2: LESENSE controls the input mux and the threshold value (VDDLEVEL) of ACMP1"]
    MUXTHRES = 2,
}
impl From<ACMP1MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: ACMP1MODE_A) -> Self {
        variant as _
    }
}
impl ACMP1MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ACMP1MODE_A> {
        match self.bits {
            0 => Some(ACMP1MODE_A::DISABLE),
            1 => Some(ACMP1MODE_A::MUX),
            2 => Some(ACMP1MODE_A::MUXTHRES),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ACMP1MODE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `MUX`"]
    #[inline(always)]
    pub fn is_mux(&self) -> bool {
        *self == ACMP1MODE_A::MUX
    }
    #[doc = "Checks if the value of the field is `MUXTHRES`"]
    #[inline(always)]
    pub fn is_muxthres(&self) -> bool {
        *self == ACMP1MODE_A::MUXTHRES
    }
}
#[doc = "Field `ACMP1MODE` writer - ACMP1 mode"]
pub type ACMP1MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PERCTRL_SPEC, u8, ACMP1MODE_A, 2, O>;
impl<'a, const O: u8> ACMP1MODE_W<'a, O> {
    #[doc = "LESENSE does not control ACMP1"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ACMP1MODE_A::DISABLE)
    }
    #[doc = "LESENSE controls the input mux (POSSEL) of ACMP1"]
    #[inline(always)]
    pub fn mux(self) -> &'a mut W {
        self.variant(ACMP1MODE_A::MUX)
    }
    #[doc = "LESENSE controls the input mux and the threshold value (VDDLEVEL) of ACMP1"]
    #[inline(always)]
    pub fn muxthres(self) -> &'a mut W {
        self.variant(ACMP1MODE_A::MUXTHRES)
    }
}
#[doc = "Field `WARMUPMODE` reader - ACMP and DAC duty cycle mode"]
pub type WARMUPMODE_R = crate::FieldReader<u8, WARMUPMODE_A>;
#[doc = "ACMP and DAC duty cycle mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WARMUPMODE_A {
    #[doc = "0: The analog comparators and DAC are shut down when LESENSE is idle"]
    NORMAL = 0,
    #[doc = "1: The analog comparators are kept powered up when LESENSE is idle"]
    KEEPACMPWARM = 1,
    #[doc = "2: The DAC is kept powered up when LESENSE is idle"]
    KEEPDACWARM = 2,
    #[doc = "3: The analog comparators and DAC are kept powered up when LESENSE is idle"]
    KEEPACMPDACWARM = 3,
}
impl From<WARMUPMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: WARMUPMODE_A) -> Self {
        variant as _
    }
}
impl WARMUPMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WARMUPMODE_A {
        match self.bits {
            0 => WARMUPMODE_A::NORMAL,
            1 => WARMUPMODE_A::KEEPACMPWARM,
            2 => WARMUPMODE_A::KEEPDACWARM,
            3 => WARMUPMODE_A::KEEPACMPDACWARM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == WARMUPMODE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `KEEPACMPWARM`"]
    #[inline(always)]
    pub fn is_keepacmpwarm(&self) -> bool {
        *self == WARMUPMODE_A::KEEPACMPWARM
    }
    #[doc = "Checks if the value of the field is `KEEPDACWARM`"]
    #[inline(always)]
    pub fn is_keepdacwarm(&self) -> bool {
        *self == WARMUPMODE_A::KEEPDACWARM
    }
    #[doc = "Checks if the value of the field is `KEEPACMPDACWARM`"]
    #[inline(always)]
    pub fn is_keepacmpdacwarm(&self) -> bool {
        *self == WARMUPMODE_A::KEEPACMPDACWARM
    }
}
#[doc = "Field `WARMUPMODE` writer - ACMP and DAC duty cycle mode"]
pub type WARMUPMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PERCTRL_SPEC, u8, WARMUPMODE_A, 2, O>;
impl<'a, const O: u8> WARMUPMODE_W<'a, O> {
    #[doc = "The analog comparators and DAC are shut down when LESENSE is idle"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(WARMUPMODE_A::NORMAL)
    }
    #[doc = "The analog comparators are kept powered up when LESENSE is idle"]
    #[inline(always)]
    pub fn keepacmpwarm(self) -> &'a mut W {
        self.variant(WARMUPMODE_A::KEEPACMPWARM)
    }
    #[doc = "The DAC is kept powered up when LESENSE is idle"]
    #[inline(always)]
    pub fn keepdacwarm(self) -> &'a mut W {
        self.variant(WARMUPMODE_A::KEEPDACWARM)
    }
    #[doc = "The analog comparators and DAC are kept powered up when LESENSE is idle"]
    #[inline(always)]
    pub fn keepacmpdacwarm(self) -> &'a mut W {
        self.variant(WARMUPMODE_A::KEEPACMPDACWARM)
    }
}
impl R {
    #[doc = "Bit 0 - DAC CH0 data selection."]
    #[inline(always)]
    pub fn dacch0data(&self) -> DACCH0DATA_R {
        DACCH0DATA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC CH1 data selection."]
    #[inline(always)]
    pub fn dacch1data(&self) -> DACCH1DATA_R {
        DACCH1DATA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - DAC channel 0 conversion mode"]
    #[inline(always)]
    pub fn dacch0conv(&self) -> DACCH0CONV_R {
        DACCH0CONV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - DAC channel 1 conversion mode"]
    #[inline(always)]
    pub fn dacch1conv(&self) -> DACCH1CONV_R {
        DACCH1CONV_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - DAC channel 0 output mode"]
    #[inline(always)]
    pub fn dacch0out(&self) -> DACCH0OUT_R {
        DACCH0OUT_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - DAC channel 1 output mode"]
    #[inline(always)]
    pub fn dacch1out(&self) -> DACCH1OUT_R {
        DACCH1OUT_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:14 - DAC prescaler configuration."]
    #[inline(always)]
    pub fn dacpresc(&self) -> DACPRESC_R {
        DACPRESC_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 18 - DAC bandgap reference used"]
    #[inline(always)]
    pub fn dacref(&self) -> DACREF_R {
        DACREF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:21 - ACMP0 mode"]
    #[inline(always)]
    pub fn acmp0mode(&self) -> ACMP0MODE_R {
        ACMP0MODE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - ACMP1 mode"]
    #[inline(always)]
    pub fn acmp1mode(&self) -> ACMP1MODE_R {
        ACMP1MODE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 26:27 - ACMP and DAC duty cycle mode"]
    #[inline(always)]
    pub fn warmupmode(&self) -> WARMUPMODE_R {
        WARMUPMODE_R::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DAC CH0 data selection."]
    #[inline(always)]
    #[must_use]
    pub fn dacch0data(&mut self) -> DACCH0DATA_W<0> {
        DACCH0DATA_W::new(self)
    }
    #[doc = "Bit 1 - DAC CH1 data selection."]
    #[inline(always)]
    #[must_use]
    pub fn dacch1data(&mut self) -> DACCH1DATA_W<1> {
        DACCH1DATA_W::new(self)
    }
    #[doc = "Bits 2:3 - DAC channel 0 conversion mode"]
    #[inline(always)]
    #[must_use]
    pub fn dacch0conv(&mut self) -> DACCH0CONV_W<2> {
        DACCH0CONV_W::new(self)
    }
    #[doc = "Bits 4:5 - DAC channel 1 conversion mode"]
    #[inline(always)]
    #[must_use]
    pub fn dacch1conv(&mut self) -> DACCH1CONV_W<4> {
        DACCH1CONV_W::new(self)
    }
    #[doc = "Bits 6:7 - DAC channel 0 output mode"]
    #[inline(always)]
    #[must_use]
    pub fn dacch0out(&mut self) -> DACCH0OUT_W<6> {
        DACCH0OUT_W::new(self)
    }
    #[doc = "Bits 8:9 - DAC channel 1 output mode"]
    #[inline(always)]
    #[must_use]
    pub fn dacch1out(&mut self) -> DACCH1OUT_W<8> {
        DACCH1OUT_W::new(self)
    }
    #[doc = "Bits 10:14 - DAC prescaler configuration."]
    #[inline(always)]
    #[must_use]
    pub fn dacpresc(&mut self) -> DACPRESC_W<10> {
        DACPRESC_W::new(self)
    }
    #[doc = "Bit 18 - DAC bandgap reference used"]
    #[inline(always)]
    #[must_use]
    pub fn dacref(&mut self) -> DACREF_W<18> {
        DACREF_W::new(self)
    }
    #[doc = "Bits 20:21 - ACMP0 mode"]
    #[inline(always)]
    #[must_use]
    pub fn acmp0mode(&mut self) -> ACMP0MODE_W<20> {
        ACMP0MODE_W::new(self)
    }
    #[doc = "Bits 22:23 - ACMP1 mode"]
    #[inline(always)]
    #[must_use]
    pub fn acmp1mode(&mut self) -> ACMP1MODE_W<22> {
        ACMP1MODE_W::new(self)
    }
    #[doc = "Bits 26:27 - ACMP and DAC duty cycle mode"]
    #[inline(always)]
    #[must_use]
    pub fn warmupmode(&mut self) -> WARMUPMODE_W<26> {
        WARMUPMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perctrl](index.html) module"]
pub struct PERCTRL_SPEC;
impl crate::RegisterSpec for PERCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perctrl::R](R) reader structure"]
impl crate::Readable for PERCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perctrl::W](W) writer structure"]
impl crate::Writable for PERCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERCTRL to value 0"]
impl crate::Resettable for PERCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
