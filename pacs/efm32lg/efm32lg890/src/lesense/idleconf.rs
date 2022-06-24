#[doc = "Register `IDLECONF` reader"]
pub struct R(crate::R<IDLECONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDLECONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDLECONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDLECONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IDLECONF` writer"]
pub struct W(crate::W<IDLECONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDLECONF_SPEC>;
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
impl From<crate::W<IDLECONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDLECONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel 0 idle phase configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH0_A {
    #[doc = "0: CH0 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: CH0 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: CH0 output is low in idle phase"]
    LOW = 2,
    #[doc = "3: CH0 output is connected to DAC CH0 output in idle phase"]
    DACCH0 = 3,
}
impl From<CH0_A> for u8 {
    #[inline(always)]
    fn from(variant: CH0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH0` reader - Channel 0 idle phase configuration"]
pub type CH0_R = crate::FieldReader<u8, CH0_A>;
impl CH0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0_A {
        match self.bits {
            0 => CH0_A::DISABLE,
            1 => CH0_A::HIGH,
            2 => CH0_A::LOW,
            3 => CH0_A::DACCH0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH0_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH0_A::LOW
    }
    #[doc = "Checks if the value of the field is `DACCH0`"]
    #[inline(always)]
    pub fn is_dacch0(&self) -> bool {
        *self == CH0_A::DACCH0
    }
}
#[doc = "Field `CH0` writer - Channel 0 idle phase configuration"]
pub type CH0_W<'a> = crate::FieldWriterSafe<'a, u32, IDLECONF_SPEC, u8, CH0_A, 2, 0>;
impl<'a> CH0_W<'a> {
    #[doc = "CH0 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH0_A::DISABLE)
    }
    #[doc = "CH0 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CH0_A::HIGH)
    }
    #[doc = "CH0 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CH0_A::LOW)
    }
    #[doc = "CH0 output is connected to DAC CH0 output in idle phase"]
    #[inline(always)]
    pub fn dacch0(self) -> &'a mut W {
        self.variant(CH0_A::DACCH0)
    }
}
#[doc = "Channel 1 idle phase configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH1_A {
    #[doc = "0: CH1 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: CH1 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: CH1 output is low in idle phase"]
    LOW = 2,
    #[doc = "3: CH1 output is connected to DAC CH0 output in idle phase"]
    DACCH0 = 3,
}
impl From<CH1_A> for u8 {
    #[inline(always)]
    fn from(variant: CH1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH1` reader - Channel 1 idle phase configuration"]
pub type CH1_R = crate::FieldReader<u8, CH1_A>;
impl CH1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1_A {
        match self.bits {
            0 => CH1_A::DISABLE,
            1 => CH1_A::HIGH,
            2 => CH1_A::LOW,
            3 => CH1_A::DACCH0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH1_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH1_A::LOW
    }
    #[doc = "Checks if the value of the field is `DACCH0`"]
    #[inline(always)]
    pub fn is_dacch0(&self) -> bool {
        *self == CH1_A::DACCH0
    }
}
#[doc = "Field `CH1` writer - Channel 1 idle phase configuration"]
pub type CH1_W<'a> = crate::FieldWriterSafe<'a, u32, IDLECONF_SPEC, u8, CH1_A, 2, 2>;
impl<'a> CH1_W<'a> {
    #[doc = "CH1 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH1_A::DISABLE)
    }
    #[doc = "CH1 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CH1_A::HIGH)
    }
    #[doc = "CH1 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CH1_A::LOW)
    }
    #[doc = "CH1 output is connected to DAC CH0 output in idle phase"]
    #[inline(always)]
    pub fn dacch0(self) -> &'a mut W {
        self.variant(CH1_A::DACCH0)
    }
}
#[doc = "Channel 2 idle phase configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH2_A {
    #[doc = "0: CH2 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: CH2 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: CH2 output is low in idle phase"]
    LOW = 2,
    #[doc = "3: CH2 output is connected to DAC CH0 output in idle phase"]
    DACCH0 = 3,
}
impl From<CH2_A> for u8 {
    #[inline(always)]
    fn from(variant: CH2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH2` reader - Channel 2 idle phase configuration"]
pub type CH2_R = crate::FieldReader<u8, CH2_A>;
impl CH2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2_A {
        match self.bits {
            0 => CH2_A::DISABLE,
            1 => CH2_A::HIGH,
            2 => CH2_A::LOW,
            3 => CH2_A::DACCH0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH2_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH2_A::LOW
    }
    #[doc = "Checks if the value of the field is `DACCH0`"]
    #[inline(always)]
    pub fn is_dacch0(&self) -> bool {
        *self == CH2_A::DACCH0
    }
}
#[doc = "Field `CH2` writer - Channel 2 idle phase configuration"]
pub type CH2_W<'a> = crate::FieldWriterSafe<'a, u32, IDLECONF_SPEC, u8, CH2_A, 2, 4>;
impl<'a> CH2_W<'a> {
    #[doc = "CH2 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH2_A::DISABLE)
    }
    #[doc = "CH2 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CH2_A::HIGH)
    }
    #[doc = "CH2 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CH2_A::LOW)
    }
    #[doc = "CH2 output is connected to DAC CH0 output in idle phase"]
    #[inline(always)]
    pub fn dacch0(self) -> &'a mut W {
        self.variant(CH2_A::DACCH0)
    }
}
#[doc = "Channel 3 idle phase configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH3_A {
    #[doc = "0: CH3 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: CH3 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: CH3 output is low in idle phase"]
    LOW = 2,
    #[doc = "3: CH3 output is connected to DAC CH0 output in idle phase"]
    DACCH0 = 3,
}
impl From<CH3_A> for u8 {
    #[inline(always)]
    fn from(variant: CH3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH3` reader - Channel 3 idle phase configuration"]
pub type CH3_R = crate::FieldReader<u8, CH3_A>;
impl CH3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3_A {
        match self.bits {
            0 => CH3_A::DISABLE,
            1 => CH3_A::HIGH,
            2 => CH3_A::LOW,
            3 => CH3_A::DACCH0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH3_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH3_A::LOW
    }
    #[doc = "Checks if the value of the field is `DACCH0`"]
    #[inline(always)]
    pub fn is_dacch0(&self) -> bool {
        *self == CH3_A::DACCH0
    }
}
#[doc = "Field `CH3` writer - Channel 3 idle phase configuration"]
pub type CH3_W<'a> = crate::FieldWriterSafe<'a, u32, IDLECONF_SPEC, u8, CH3_A, 2, 6>;
impl<'a> CH3_W<'a> {
    #[doc = "CH3 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH3_A::DISABLE)
    }
    #[doc = "CH3 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CH3_A::HIGH)
    }
    #[doc = "CH3 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CH3_A::LOW)
    }
    #[doc = "CH3 output is connected to DAC CH0 output in idle phase"]
    #[inline(always)]
    pub fn dacch0(self) -> &'a mut W {
        self.variant(CH3_A::DACCH0)
    }
}
#[doc = "Channel 4 idle phase configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH4_A {
    #[doc = "0: CH4 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: CH4 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: CH4 output is low in idle phase"]
    LOW = 2,
}
impl From<CH4_A> for u8 {
    #[inline(always)]
    fn from(variant: CH4_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH4` reader - Channel 4 idle phase configuration"]
pub type CH4_R = crate::FieldReader<u8, CH4_A>;
impl CH4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH4_A> {
        match self.bits {
            0 => Some(CH4_A::DISABLE),
            1 => Some(CH4_A::HIGH),
            2 => Some(CH4_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH4_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH4_A::LOW
    }
}
#[doc = "Field `CH4` writer - Channel 4 idle phase configuration"]
pub type CH4_W<'a> = crate::FieldWriter<'a, u32, IDLECONF_SPEC, u8, CH4_A, 2, 8>;
impl<'a> CH4_W<'a> {
    #[doc = "CH4 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH4_A::DISABLE)
    }
    #[doc = "CH4 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CH4_A::HIGH)
    }
    #[doc = "CH4 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CH4_A::LOW)
    }
}
#[doc = "Channel 5 idle phase configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH5_A {
    #[doc = "0: CH5 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: CH5 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: CH5 output is low in idle phase"]
    LOW = 2,
}
impl From<CH5_A> for u8 {
    #[inline(always)]
    fn from(variant: CH5_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH5` reader - Channel 5 idle phase configuration"]
pub type CH5_R = crate::FieldReader<u8, CH5_A>;
impl CH5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH5_A> {
        match self.bits {
            0 => Some(CH5_A::DISABLE),
            1 => Some(CH5_A::HIGH),
            2 => Some(CH5_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH5_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH5_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH5_A::LOW
    }
}
#[doc = "Field `CH5` writer - Channel 5 idle phase configuration"]
pub type CH5_W<'a> = crate::FieldWriter<'a, u32, IDLECONF_SPEC, u8, CH5_A, 2, 10>;
impl<'a> CH5_W<'a> {
    #[doc = "CH5 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH5_A::DISABLE)
    }
    #[doc = "CH5 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CH5_A::HIGH)
    }
    #[doc = "CH5 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CH5_A::LOW)
    }
}
#[doc = "Channel 6 idle phase configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH6_A {
    #[doc = "0: CH6 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: CH6 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: CH6 output is low in idle phase"]
    LOW = 2,
}
impl From<CH6_A> for u8 {
    #[inline(always)]
    fn from(variant: CH6_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH6` reader - Channel 6 idle phase configuration"]
pub type CH6_R = crate::FieldReader<u8, CH6_A>;
impl CH6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH6_A> {
        match self.bits {
            0 => Some(CH6_A::DISABLE),
            1 => Some(CH6_A::HIGH),
            2 => Some(CH6_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH6_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH6_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH6_A::LOW
    }
}
#[doc = "Field `CH6` writer - Channel 6 idle phase configuration"]
pub type CH6_W<'a> = crate::FieldWriter<'a, u32, IDLECONF_SPEC, u8, CH6_A, 2, 12>;
impl<'a> CH6_W<'a> {
    #[doc = "CH6 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH6_A::DISABLE)
    }
    #[doc = "CH6 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CH6_A::HIGH)
    }
    #[doc = "CH6 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CH6_A::LOW)
    }
}
#[doc = "Channel 7 idle phase configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH7_A {
    #[doc = "0: CH7 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: CH7 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: CH7 output is low in idle phase"]
    LOW = 2,
}
impl From<CH7_A> for u8 {
    #[inline(always)]
    fn from(variant: CH7_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH7` reader - Channel 7 idle phase configuration"]
pub type CH7_R = crate::FieldReader<u8, CH7_A>;
impl CH7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH7_A> {
        match self.bits {
            0 => Some(CH7_A::DISABLE),
            1 => Some(CH7_A::HIGH),
            2 => Some(CH7_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH7_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH7_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH7_A::LOW
    }
}
#[doc = "Field `CH7` writer - Channel 7 idle phase configuration"]
pub type CH7_W<'a> = crate::FieldWriter<'a, u32, IDLECONF_SPEC, u8, CH7_A, 2, 14>;
impl<'a> CH7_W<'a> {
    #[doc = "CH7 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH7_A::DISABLE)
    }
    #[doc = "CH7 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CH7_A::HIGH)
    }
    #[doc = "CH7 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CH7_A::LOW)
    }
}
#[doc = "Channel 8 idle phase configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH8_A {
    #[doc = "0: CH8 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: CH8 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: CH8 output is low in idle phase"]
    LOW = 2,
}
impl From<CH8_A> for u8 {
    #[inline(always)]
    fn from(variant: CH8_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH8` reader - Channel 8 idle phase configuration"]
pub type CH8_R = crate::FieldReader<u8, CH8_A>;
impl CH8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH8_A> {
        match self.bits {
            0 => Some(CH8_A::DISABLE),
            1 => Some(CH8_A::HIGH),
            2 => Some(CH8_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH8_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH8_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH8_A::LOW
    }
}
#[doc = "Field `CH8` writer - Channel 8 idle phase configuration"]
pub type CH8_W<'a> = crate::FieldWriter<'a, u32, IDLECONF_SPEC, u8, CH8_A, 2, 16>;
impl<'a> CH8_W<'a> {
    #[doc = "CH8 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH8_A::DISABLE)
    }
    #[doc = "CH8 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CH8_A::HIGH)
    }
    #[doc = "CH8 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CH8_A::LOW)
    }
}
#[doc = "Channel 9 idle phase configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH9_A {
    #[doc = "0: CH9 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: CH9 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: CH9 output is low in idle phase"]
    LOW = 2,
}
impl From<CH9_A> for u8 {
    #[inline(always)]
    fn from(variant: CH9_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH9` reader - Channel 9 idle phase configuration"]
pub type CH9_R = crate::FieldReader<u8, CH9_A>;
impl CH9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH9_A> {
        match self.bits {
            0 => Some(CH9_A::DISABLE),
            1 => Some(CH9_A::HIGH),
            2 => Some(CH9_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH9_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH9_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH9_A::LOW
    }
}
#[doc = "Field `CH9` writer - Channel 9 idle phase configuration"]
pub type CH9_W<'a> = crate::FieldWriter<'a, u32, IDLECONF_SPEC, u8, CH9_A, 2, 18>;
impl<'a> CH9_W<'a> {
    #[doc = "CH9 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH9_A::DISABLE)
    }
    #[doc = "CH9 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CH9_A::HIGH)
    }
    #[doc = "CH9 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CH9_A::LOW)
    }
}
#[doc = "Channel 10 idle phase configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH10_A {
    #[doc = "0: CH10 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: CH10 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: CH10 output is low in idle phase"]
    LOW = 2,
}
impl From<CH10_A> for u8 {
    #[inline(always)]
    fn from(variant: CH10_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH10` reader - Channel 10 idle phase configuration"]
pub type CH10_R = crate::FieldReader<u8, CH10_A>;
impl CH10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH10_A> {
        match self.bits {
            0 => Some(CH10_A::DISABLE),
            1 => Some(CH10_A::HIGH),
            2 => Some(CH10_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH10_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH10_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH10_A::LOW
    }
}
#[doc = "Field `CH10` writer - Channel 10 idle phase configuration"]
pub type CH10_W<'a> = crate::FieldWriter<'a, u32, IDLECONF_SPEC, u8, CH10_A, 2, 20>;
impl<'a> CH10_W<'a> {
    #[doc = "CH10 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH10_A::DISABLE)
    }
    #[doc = "CH10 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CH10_A::HIGH)
    }
    #[doc = "CH10 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CH10_A::LOW)
    }
}
#[doc = "Channel 11 idle phase configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH11_A {
    #[doc = "0: CH11 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: CH11 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: CH11 output is low in idle phase"]
    LOW = 2,
}
impl From<CH11_A> for u8 {
    #[inline(always)]
    fn from(variant: CH11_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH11` reader - Channel 11 idle phase configuration"]
pub type CH11_R = crate::FieldReader<u8, CH11_A>;
impl CH11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH11_A> {
        match self.bits {
            0 => Some(CH11_A::DISABLE),
            1 => Some(CH11_A::HIGH),
            2 => Some(CH11_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH11_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH11_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH11_A::LOW
    }
}
#[doc = "Field `CH11` writer - Channel 11 idle phase configuration"]
pub type CH11_W<'a> = crate::FieldWriter<'a, u32, IDLECONF_SPEC, u8, CH11_A, 2, 22>;
impl<'a> CH11_W<'a> {
    #[doc = "CH11 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH11_A::DISABLE)
    }
    #[doc = "CH11 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CH11_A::HIGH)
    }
    #[doc = "CH11 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CH11_A::LOW)
    }
}
#[doc = "Channel 12 idle phase configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH12_A {
    #[doc = "0: CH12 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: CH12 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: CH12 output is low in idle phase"]
    LOW = 2,
    #[doc = "3: CH12 output is connected to DAC CH1 output in idle phase"]
    DACCH1 = 3,
}
impl From<CH12_A> for u8 {
    #[inline(always)]
    fn from(variant: CH12_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH12` reader - Channel 12 idle phase configuration"]
pub type CH12_R = crate::FieldReader<u8, CH12_A>;
impl CH12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH12_A {
        match self.bits {
            0 => CH12_A::DISABLE,
            1 => CH12_A::HIGH,
            2 => CH12_A::LOW,
            3 => CH12_A::DACCH1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH12_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH12_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH12_A::LOW
    }
    #[doc = "Checks if the value of the field is `DACCH1`"]
    #[inline(always)]
    pub fn is_dacch1(&self) -> bool {
        *self == CH12_A::DACCH1
    }
}
#[doc = "Field `CH12` writer - Channel 12 idle phase configuration"]
pub type CH12_W<'a> = crate::FieldWriterSafe<'a, u32, IDLECONF_SPEC, u8, CH12_A, 2, 24>;
impl<'a> CH12_W<'a> {
    #[doc = "CH12 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH12_A::DISABLE)
    }
    #[doc = "CH12 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CH12_A::HIGH)
    }
    #[doc = "CH12 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CH12_A::LOW)
    }
    #[doc = "CH12 output is connected to DAC CH1 output in idle phase"]
    #[inline(always)]
    pub fn dacch1(self) -> &'a mut W {
        self.variant(CH12_A::DACCH1)
    }
}
#[doc = "Channel 13 idle phase configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH13_A {
    #[doc = "0: CH13 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: CH13 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: CH13 output is low in idle phase"]
    LOW = 2,
    #[doc = "3: CH13 output is connected to DAC CH1 output in idle phase"]
    DACCH1 = 3,
}
impl From<CH13_A> for u8 {
    #[inline(always)]
    fn from(variant: CH13_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH13` reader - Channel 13 idle phase configuration"]
pub type CH13_R = crate::FieldReader<u8, CH13_A>;
impl CH13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH13_A {
        match self.bits {
            0 => CH13_A::DISABLE,
            1 => CH13_A::HIGH,
            2 => CH13_A::LOW,
            3 => CH13_A::DACCH1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH13_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH13_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH13_A::LOW
    }
    #[doc = "Checks if the value of the field is `DACCH1`"]
    #[inline(always)]
    pub fn is_dacch1(&self) -> bool {
        *self == CH13_A::DACCH1
    }
}
#[doc = "Field `CH13` writer - Channel 13 idle phase configuration"]
pub type CH13_W<'a> = crate::FieldWriterSafe<'a, u32, IDLECONF_SPEC, u8, CH13_A, 2, 26>;
impl<'a> CH13_W<'a> {
    #[doc = "CH13 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH13_A::DISABLE)
    }
    #[doc = "CH13 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CH13_A::HIGH)
    }
    #[doc = "CH13 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CH13_A::LOW)
    }
    #[doc = "CH13 output is connected to DAC CH1 output in idle phase"]
    #[inline(always)]
    pub fn dacch1(self) -> &'a mut W {
        self.variant(CH13_A::DACCH1)
    }
}
#[doc = "Channel 14 idle phase configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH14_A {
    #[doc = "0: CH14 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: CH14 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: CH14 output is low in idle phase"]
    LOW = 2,
    #[doc = "3: CH14 output is connected to DAC CH1 output in idle phase"]
    DACCH1 = 3,
}
impl From<CH14_A> for u8 {
    #[inline(always)]
    fn from(variant: CH14_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH14` reader - Channel 14 idle phase configuration"]
pub type CH14_R = crate::FieldReader<u8, CH14_A>;
impl CH14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH14_A {
        match self.bits {
            0 => CH14_A::DISABLE,
            1 => CH14_A::HIGH,
            2 => CH14_A::LOW,
            3 => CH14_A::DACCH1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH14_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH14_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH14_A::LOW
    }
    #[doc = "Checks if the value of the field is `DACCH1`"]
    #[inline(always)]
    pub fn is_dacch1(&self) -> bool {
        *self == CH14_A::DACCH1
    }
}
#[doc = "Field `CH14` writer - Channel 14 idle phase configuration"]
pub type CH14_W<'a> = crate::FieldWriterSafe<'a, u32, IDLECONF_SPEC, u8, CH14_A, 2, 28>;
impl<'a> CH14_W<'a> {
    #[doc = "CH14 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH14_A::DISABLE)
    }
    #[doc = "CH14 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CH14_A::HIGH)
    }
    #[doc = "CH14 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CH14_A::LOW)
    }
    #[doc = "CH14 output is connected to DAC CH1 output in idle phase"]
    #[inline(always)]
    pub fn dacch1(self) -> &'a mut W {
        self.variant(CH14_A::DACCH1)
    }
}
#[doc = "Channel 15 idle phase configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH15_A {
    #[doc = "0: CH15 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: CH15 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: CH15 output is low in idle phase"]
    LOW = 2,
    #[doc = "3: CH15 output is connected to DAC CH1 output in idle phase"]
    DACCH1 = 3,
}
impl From<CH15_A> for u8 {
    #[inline(always)]
    fn from(variant: CH15_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH15` reader - Channel 15 idle phase configuration"]
pub type CH15_R = crate::FieldReader<u8, CH15_A>;
impl CH15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH15_A {
        match self.bits {
            0 => CH15_A::DISABLE,
            1 => CH15_A::HIGH,
            2 => CH15_A::LOW,
            3 => CH15_A::DACCH1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH15_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH15_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH15_A::LOW
    }
    #[doc = "Checks if the value of the field is `DACCH1`"]
    #[inline(always)]
    pub fn is_dacch1(&self) -> bool {
        *self == CH15_A::DACCH1
    }
}
#[doc = "Field `CH15` writer - Channel 15 idle phase configuration"]
pub type CH15_W<'a> = crate::FieldWriterSafe<'a, u32, IDLECONF_SPEC, u8, CH15_A, 2, 30>;
impl<'a> CH15_W<'a> {
    #[doc = "CH15 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH15_A::DISABLE)
    }
    #[doc = "CH15 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CH15_A::HIGH)
    }
    #[doc = "CH15 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CH15_A::LOW)
    }
    #[doc = "CH15 output is connected to DAC CH1 output in idle phase"]
    #[inline(always)]
    pub fn dacch1(self) -> &'a mut W {
        self.variant(CH15_A::DACCH1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Channel 0 idle phase configuration"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Channel 1 idle phase configuration"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Channel 2 idle phase configuration"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Channel 3 idle phase configuration"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Channel 4 idle phase configuration"]
    #[inline(always)]
    pub fn ch4(&self) -> CH4_R {
        CH4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Channel 5 idle phase configuration"]
    #[inline(always)]
    pub fn ch5(&self) -> CH5_R {
        CH5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Channel 6 idle phase configuration"]
    #[inline(always)]
    pub fn ch6(&self) -> CH6_R {
        CH6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Channel 7 idle phase configuration"]
    #[inline(always)]
    pub fn ch7(&self) -> CH7_R {
        CH7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Channel 8 idle phase configuration"]
    #[inline(always)]
    pub fn ch8(&self) -> CH8_R {
        CH8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Channel 9 idle phase configuration"]
    #[inline(always)]
    pub fn ch9(&self) -> CH9_R {
        CH9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Channel 10 idle phase configuration"]
    #[inline(always)]
    pub fn ch10(&self) -> CH10_R {
        CH10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Channel 11 idle phase configuration"]
    #[inline(always)]
    pub fn ch11(&self) -> CH11_R {
        CH11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Channel 12 idle phase configuration"]
    #[inline(always)]
    pub fn ch12(&self) -> CH12_R {
        CH12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Channel 13 idle phase configuration"]
    #[inline(always)]
    pub fn ch13(&self) -> CH13_R {
        CH13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Channel 14 idle phase configuration"]
    #[inline(always)]
    pub fn ch14(&self) -> CH14_R {
        CH14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Channel 15 idle phase configuration"]
    #[inline(always)]
    pub fn ch15(&self) -> CH15_R {
        CH15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 0 idle phase configuration"]
    #[inline(always)]
    pub fn ch0(&mut self) -> CH0_W {
        CH0_W::new(self)
    }
    #[doc = "Bits 2:3 - Channel 1 idle phase configuration"]
    #[inline(always)]
    pub fn ch1(&mut self) -> CH1_W {
        CH1_W::new(self)
    }
    #[doc = "Bits 4:5 - Channel 2 idle phase configuration"]
    #[inline(always)]
    pub fn ch2(&mut self) -> CH2_W {
        CH2_W::new(self)
    }
    #[doc = "Bits 6:7 - Channel 3 idle phase configuration"]
    #[inline(always)]
    pub fn ch3(&mut self) -> CH3_W {
        CH3_W::new(self)
    }
    #[doc = "Bits 8:9 - Channel 4 idle phase configuration"]
    #[inline(always)]
    pub fn ch4(&mut self) -> CH4_W {
        CH4_W::new(self)
    }
    #[doc = "Bits 10:11 - Channel 5 idle phase configuration"]
    #[inline(always)]
    pub fn ch5(&mut self) -> CH5_W {
        CH5_W::new(self)
    }
    #[doc = "Bits 12:13 - Channel 6 idle phase configuration"]
    #[inline(always)]
    pub fn ch6(&mut self) -> CH6_W {
        CH6_W::new(self)
    }
    #[doc = "Bits 14:15 - Channel 7 idle phase configuration"]
    #[inline(always)]
    pub fn ch7(&mut self) -> CH7_W {
        CH7_W::new(self)
    }
    #[doc = "Bits 16:17 - Channel 8 idle phase configuration"]
    #[inline(always)]
    pub fn ch8(&mut self) -> CH8_W {
        CH8_W::new(self)
    }
    #[doc = "Bits 18:19 - Channel 9 idle phase configuration"]
    #[inline(always)]
    pub fn ch9(&mut self) -> CH9_W {
        CH9_W::new(self)
    }
    #[doc = "Bits 20:21 - Channel 10 idle phase configuration"]
    #[inline(always)]
    pub fn ch10(&mut self) -> CH10_W {
        CH10_W::new(self)
    }
    #[doc = "Bits 22:23 - Channel 11 idle phase configuration"]
    #[inline(always)]
    pub fn ch11(&mut self) -> CH11_W {
        CH11_W::new(self)
    }
    #[doc = "Bits 24:25 - Channel 12 idle phase configuration"]
    #[inline(always)]
    pub fn ch12(&mut self) -> CH12_W {
        CH12_W::new(self)
    }
    #[doc = "Bits 26:27 - Channel 13 idle phase configuration"]
    #[inline(always)]
    pub fn ch13(&mut self) -> CH13_W {
        CH13_W::new(self)
    }
    #[doc = "Bits 28:29 - Channel 14 idle phase configuration"]
    #[inline(always)]
    pub fn ch14(&mut self) -> CH14_W {
        CH14_W::new(self)
    }
    #[doc = "Bits 30:31 - Channel 15 idle phase configuration"]
    #[inline(always)]
    pub fn ch15(&mut self) -> CH15_W {
        CH15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Idle phase configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idleconf](index.html) module"]
pub struct IDLECONF_SPEC;
impl crate::RegisterSpec for IDLECONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idleconf::R](R) reader structure"]
impl crate::Readable for IDLECONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [idleconf::W](W) writer structure"]
impl crate::Writable for IDLECONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IDLECONF to value 0"]
impl crate::Resettable for IDLECONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
