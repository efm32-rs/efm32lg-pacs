#[doc = "Register `ALTEXCONF` reader"]
pub struct R(crate::R<ALTEXCONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALTEXCONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALTEXCONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALTEXCONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALTEXCONF` writer"]
pub struct W(crate::W<ALTEXCONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALTEXCONF_SPEC>;
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
impl From<crate::W<ALTEXCONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALTEXCONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ALTEX0 idle phase configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IDLECONF0_A {
    #[doc = "0: ALTEX0 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: ALTEX0 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: ALTEX0 output is low in idle phase"]
    LOW = 2,
}
impl From<IDLECONF0_A> for u8 {
    #[inline(always)]
    fn from(variant: IDLECONF0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IDLECONF0` reader - ALTEX0 idle phase configuration"]
pub type IDLECONF0_R = crate::FieldReader<u8, IDLECONF0_A>;
impl IDLECONF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDLECONF0_A> {
        match self.bits {
            0 => Some(IDLECONF0_A::DISABLE),
            1 => Some(IDLECONF0_A::HIGH),
            2 => Some(IDLECONF0_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IDLECONF0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IDLECONF0_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == IDLECONF0_A::LOW
    }
}
#[doc = "Field `IDLECONF0` writer - ALTEX0 idle phase configuration"]
pub type IDLECONF0_W<'a> = crate::FieldWriter<'a, u32, ALTEXCONF_SPEC, u8, IDLECONF0_A, 2, 0>;
impl<'a> IDLECONF0_W<'a> {
    #[doc = "ALTEX0 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IDLECONF0_A::DISABLE)
    }
    #[doc = "ALTEX0 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(IDLECONF0_A::HIGH)
    }
    #[doc = "ALTEX0 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(IDLECONF0_A::LOW)
    }
}
#[doc = "ALTEX1 idle phase configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IDLECONF1_A {
    #[doc = "0: ALTEX1 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: ALTEX1 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: ALTEX1 output is low in idle phase"]
    LOW = 2,
}
impl From<IDLECONF1_A> for u8 {
    #[inline(always)]
    fn from(variant: IDLECONF1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IDLECONF1` reader - ALTEX1 idle phase configuration"]
pub type IDLECONF1_R = crate::FieldReader<u8, IDLECONF1_A>;
impl IDLECONF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDLECONF1_A> {
        match self.bits {
            0 => Some(IDLECONF1_A::DISABLE),
            1 => Some(IDLECONF1_A::HIGH),
            2 => Some(IDLECONF1_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IDLECONF1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IDLECONF1_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == IDLECONF1_A::LOW
    }
}
#[doc = "Field `IDLECONF1` writer - ALTEX1 idle phase configuration"]
pub type IDLECONF1_W<'a> = crate::FieldWriter<'a, u32, ALTEXCONF_SPEC, u8, IDLECONF1_A, 2, 2>;
impl<'a> IDLECONF1_W<'a> {
    #[doc = "ALTEX1 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IDLECONF1_A::DISABLE)
    }
    #[doc = "ALTEX1 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(IDLECONF1_A::HIGH)
    }
    #[doc = "ALTEX1 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(IDLECONF1_A::LOW)
    }
}
#[doc = "ALTEX2 idle phase configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IDLECONF2_A {
    #[doc = "0: ALTEX2 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: ALTEX2 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: ALTEX2 output is low in idle phase"]
    LOW = 2,
}
impl From<IDLECONF2_A> for u8 {
    #[inline(always)]
    fn from(variant: IDLECONF2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IDLECONF2` reader - ALTEX2 idle phase configuration"]
pub type IDLECONF2_R = crate::FieldReader<u8, IDLECONF2_A>;
impl IDLECONF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDLECONF2_A> {
        match self.bits {
            0 => Some(IDLECONF2_A::DISABLE),
            1 => Some(IDLECONF2_A::HIGH),
            2 => Some(IDLECONF2_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IDLECONF2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IDLECONF2_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == IDLECONF2_A::LOW
    }
}
#[doc = "Field `IDLECONF2` writer - ALTEX2 idle phase configuration"]
pub type IDLECONF2_W<'a> = crate::FieldWriter<'a, u32, ALTEXCONF_SPEC, u8, IDLECONF2_A, 2, 4>;
impl<'a> IDLECONF2_W<'a> {
    #[doc = "ALTEX2 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IDLECONF2_A::DISABLE)
    }
    #[doc = "ALTEX2 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(IDLECONF2_A::HIGH)
    }
    #[doc = "ALTEX2 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(IDLECONF2_A::LOW)
    }
}
#[doc = "ALTEX3 idle phase configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IDLECONF3_A {
    #[doc = "0: ALTEX3 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: ALTEX3 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: ALTEX3 output is low in idle phase"]
    LOW = 2,
}
impl From<IDLECONF3_A> for u8 {
    #[inline(always)]
    fn from(variant: IDLECONF3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IDLECONF3` reader - ALTEX3 idle phase configuration"]
pub type IDLECONF3_R = crate::FieldReader<u8, IDLECONF3_A>;
impl IDLECONF3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDLECONF3_A> {
        match self.bits {
            0 => Some(IDLECONF3_A::DISABLE),
            1 => Some(IDLECONF3_A::HIGH),
            2 => Some(IDLECONF3_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IDLECONF3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IDLECONF3_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == IDLECONF3_A::LOW
    }
}
#[doc = "Field `IDLECONF3` writer - ALTEX3 idle phase configuration"]
pub type IDLECONF3_W<'a> = crate::FieldWriter<'a, u32, ALTEXCONF_SPEC, u8, IDLECONF3_A, 2, 6>;
impl<'a> IDLECONF3_W<'a> {
    #[doc = "ALTEX3 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IDLECONF3_A::DISABLE)
    }
    #[doc = "ALTEX3 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(IDLECONF3_A::HIGH)
    }
    #[doc = "ALTEX3 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(IDLECONF3_A::LOW)
    }
}
#[doc = "ALTEX4 idle phase configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IDLECONF4_A {
    #[doc = "0: ALTEX4 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: ALTEX4 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: ALTEX4 output is low in idle phase"]
    LOW = 2,
}
impl From<IDLECONF4_A> for u8 {
    #[inline(always)]
    fn from(variant: IDLECONF4_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IDLECONF4` reader - ALTEX4 idle phase configuration"]
pub type IDLECONF4_R = crate::FieldReader<u8, IDLECONF4_A>;
impl IDLECONF4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDLECONF4_A> {
        match self.bits {
            0 => Some(IDLECONF4_A::DISABLE),
            1 => Some(IDLECONF4_A::HIGH),
            2 => Some(IDLECONF4_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IDLECONF4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IDLECONF4_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == IDLECONF4_A::LOW
    }
}
#[doc = "Field `IDLECONF4` writer - ALTEX4 idle phase configuration"]
pub type IDLECONF4_W<'a> = crate::FieldWriter<'a, u32, ALTEXCONF_SPEC, u8, IDLECONF4_A, 2, 8>;
impl<'a> IDLECONF4_W<'a> {
    #[doc = "ALTEX4 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IDLECONF4_A::DISABLE)
    }
    #[doc = "ALTEX4 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(IDLECONF4_A::HIGH)
    }
    #[doc = "ALTEX4 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(IDLECONF4_A::LOW)
    }
}
#[doc = "ALTEX5 idle phase configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IDLECONF5_A {
    #[doc = "0: ALTEX5 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: ALTEX5 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: ALTEX5 output is low in idle phase"]
    LOW = 2,
}
impl From<IDLECONF5_A> for u8 {
    #[inline(always)]
    fn from(variant: IDLECONF5_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IDLECONF5` reader - ALTEX5 idle phase configuration"]
pub type IDLECONF5_R = crate::FieldReader<u8, IDLECONF5_A>;
impl IDLECONF5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDLECONF5_A> {
        match self.bits {
            0 => Some(IDLECONF5_A::DISABLE),
            1 => Some(IDLECONF5_A::HIGH),
            2 => Some(IDLECONF5_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IDLECONF5_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IDLECONF5_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == IDLECONF5_A::LOW
    }
}
#[doc = "Field `IDLECONF5` writer - ALTEX5 idle phase configuration"]
pub type IDLECONF5_W<'a> = crate::FieldWriter<'a, u32, ALTEXCONF_SPEC, u8, IDLECONF5_A, 2, 10>;
impl<'a> IDLECONF5_W<'a> {
    #[doc = "ALTEX5 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IDLECONF5_A::DISABLE)
    }
    #[doc = "ALTEX5 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(IDLECONF5_A::HIGH)
    }
    #[doc = "ALTEX5 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(IDLECONF5_A::LOW)
    }
}
#[doc = "ALTEX6 idle phase configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IDLECONF6_A {
    #[doc = "0: ALTEX6 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: ALTEX6 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: ALTEX6 output is low in idle phase"]
    LOW = 2,
}
impl From<IDLECONF6_A> for u8 {
    #[inline(always)]
    fn from(variant: IDLECONF6_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IDLECONF6` reader - ALTEX6 idle phase configuration"]
pub type IDLECONF6_R = crate::FieldReader<u8, IDLECONF6_A>;
impl IDLECONF6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDLECONF6_A> {
        match self.bits {
            0 => Some(IDLECONF6_A::DISABLE),
            1 => Some(IDLECONF6_A::HIGH),
            2 => Some(IDLECONF6_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IDLECONF6_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IDLECONF6_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == IDLECONF6_A::LOW
    }
}
#[doc = "Field `IDLECONF6` writer - ALTEX6 idle phase configuration"]
pub type IDLECONF6_W<'a> = crate::FieldWriter<'a, u32, ALTEXCONF_SPEC, u8, IDLECONF6_A, 2, 12>;
impl<'a> IDLECONF6_W<'a> {
    #[doc = "ALTEX6 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IDLECONF6_A::DISABLE)
    }
    #[doc = "ALTEX6 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(IDLECONF6_A::HIGH)
    }
    #[doc = "ALTEX6 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(IDLECONF6_A::LOW)
    }
}
#[doc = "ALTEX7 idle phase configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IDLECONF7_A {
    #[doc = "0: ALTEX7 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: ALTEX7 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: ALTEX7 output is low in idle phase"]
    LOW = 2,
}
impl From<IDLECONF7_A> for u8 {
    #[inline(always)]
    fn from(variant: IDLECONF7_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IDLECONF7` reader - ALTEX7 idle phase configuration"]
pub type IDLECONF7_R = crate::FieldReader<u8, IDLECONF7_A>;
impl IDLECONF7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDLECONF7_A> {
        match self.bits {
            0 => Some(IDLECONF7_A::DISABLE),
            1 => Some(IDLECONF7_A::HIGH),
            2 => Some(IDLECONF7_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IDLECONF7_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IDLECONF7_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == IDLECONF7_A::LOW
    }
}
#[doc = "Field `IDLECONF7` writer - ALTEX7 idle phase configuration"]
pub type IDLECONF7_W<'a> = crate::FieldWriter<'a, u32, ALTEXCONF_SPEC, u8, IDLECONF7_A, 2, 14>;
impl<'a> IDLECONF7_W<'a> {
    #[doc = "ALTEX7 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IDLECONF7_A::DISABLE)
    }
    #[doc = "ALTEX7 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(IDLECONF7_A::HIGH)
    }
    #[doc = "ALTEX7 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(IDLECONF7_A::LOW)
    }
}
#[doc = "Field `AEX0` reader - ALTEX0 always excite enable"]
pub type AEX0_R = crate::BitReader<bool>;
#[doc = "Field `AEX0` writer - ALTEX0 always excite enable"]
pub type AEX0_W<'a> = crate::BitWriter<'a, u32, ALTEXCONF_SPEC, bool, 16>;
#[doc = "Field `AEX1` reader - ALTEX1 always excite enable"]
pub type AEX1_R = crate::BitReader<bool>;
#[doc = "Field `AEX1` writer - ALTEX1 always excite enable"]
pub type AEX1_W<'a> = crate::BitWriter<'a, u32, ALTEXCONF_SPEC, bool, 17>;
#[doc = "Field `AEX2` reader - ALTEX2 always excite enable"]
pub type AEX2_R = crate::BitReader<bool>;
#[doc = "Field `AEX2` writer - ALTEX2 always excite enable"]
pub type AEX2_W<'a> = crate::BitWriter<'a, u32, ALTEXCONF_SPEC, bool, 18>;
#[doc = "Field `AEX3` reader - ALTEX3 always excite enable"]
pub type AEX3_R = crate::BitReader<bool>;
#[doc = "Field `AEX3` writer - ALTEX3 always excite enable"]
pub type AEX3_W<'a> = crate::BitWriter<'a, u32, ALTEXCONF_SPEC, bool, 19>;
#[doc = "Field `AEX4` reader - ALTEX4 always excite enable"]
pub type AEX4_R = crate::BitReader<bool>;
#[doc = "Field `AEX4` writer - ALTEX4 always excite enable"]
pub type AEX4_W<'a> = crate::BitWriter<'a, u32, ALTEXCONF_SPEC, bool, 20>;
#[doc = "Field `AEX5` reader - ALTEX5 always excite enable"]
pub type AEX5_R = crate::BitReader<bool>;
#[doc = "Field `AEX5` writer - ALTEX5 always excite enable"]
pub type AEX5_W<'a> = crate::BitWriter<'a, u32, ALTEXCONF_SPEC, bool, 21>;
#[doc = "Field `AEX6` reader - ALTEX6 always excite enable"]
pub type AEX6_R = crate::BitReader<bool>;
#[doc = "Field `AEX6` writer - ALTEX6 always excite enable"]
pub type AEX6_W<'a> = crate::BitWriter<'a, u32, ALTEXCONF_SPEC, bool, 22>;
#[doc = "Field `AEX7` reader - ALTEX7 always excite enable"]
pub type AEX7_R = crate::BitReader<bool>;
#[doc = "Field `AEX7` writer - ALTEX7 always excite enable"]
pub type AEX7_W<'a> = crate::BitWriter<'a, u32, ALTEXCONF_SPEC, bool, 23>;
impl R {
    #[doc = "Bits 0:1 - ALTEX0 idle phase configuration"]
    #[inline(always)]
    pub fn idleconf0(&self) -> IDLECONF0_R {
        IDLECONF0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - ALTEX1 idle phase configuration"]
    #[inline(always)]
    pub fn idleconf1(&self) -> IDLECONF1_R {
        IDLECONF1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - ALTEX2 idle phase configuration"]
    #[inline(always)]
    pub fn idleconf2(&self) -> IDLECONF2_R {
        IDLECONF2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - ALTEX3 idle phase configuration"]
    #[inline(always)]
    pub fn idleconf3(&self) -> IDLECONF3_R {
        IDLECONF3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - ALTEX4 idle phase configuration"]
    #[inline(always)]
    pub fn idleconf4(&self) -> IDLECONF4_R {
        IDLECONF4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - ALTEX5 idle phase configuration"]
    #[inline(always)]
    pub fn idleconf5(&self) -> IDLECONF5_R {
        IDLECONF5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - ALTEX6 idle phase configuration"]
    #[inline(always)]
    pub fn idleconf6(&self) -> IDLECONF6_R {
        IDLECONF6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - ALTEX7 idle phase configuration"]
    #[inline(always)]
    pub fn idleconf7(&self) -> IDLECONF7_R {
        IDLECONF7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - ALTEX0 always excite enable"]
    #[inline(always)]
    pub fn aex0(&self) -> AEX0_R {
        AEX0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ALTEX1 always excite enable"]
    #[inline(always)]
    pub fn aex1(&self) -> AEX1_R {
        AEX1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ALTEX2 always excite enable"]
    #[inline(always)]
    pub fn aex2(&self) -> AEX2_R {
        AEX2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ALTEX3 always excite enable"]
    #[inline(always)]
    pub fn aex3(&self) -> AEX3_R {
        AEX3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ALTEX4 always excite enable"]
    #[inline(always)]
    pub fn aex4(&self) -> AEX4_R {
        AEX4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ALTEX5 always excite enable"]
    #[inline(always)]
    pub fn aex5(&self) -> AEX5_R {
        AEX5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ALTEX6 always excite enable"]
    #[inline(always)]
    pub fn aex6(&self) -> AEX6_R {
        AEX6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ALTEX7 always excite enable"]
    #[inline(always)]
    pub fn aex7(&self) -> AEX7_R {
        AEX7_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - ALTEX0 idle phase configuration"]
    #[inline(always)]
    pub fn idleconf0(&mut self) -> IDLECONF0_W {
        IDLECONF0_W::new(self)
    }
    #[doc = "Bits 2:3 - ALTEX1 idle phase configuration"]
    #[inline(always)]
    pub fn idleconf1(&mut self) -> IDLECONF1_W {
        IDLECONF1_W::new(self)
    }
    #[doc = "Bits 4:5 - ALTEX2 idle phase configuration"]
    #[inline(always)]
    pub fn idleconf2(&mut self) -> IDLECONF2_W {
        IDLECONF2_W::new(self)
    }
    #[doc = "Bits 6:7 - ALTEX3 idle phase configuration"]
    #[inline(always)]
    pub fn idleconf3(&mut self) -> IDLECONF3_W {
        IDLECONF3_W::new(self)
    }
    #[doc = "Bits 8:9 - ALTEX4 idle phase configuration"]
    #[inline(always)]
    pub fn idleconf4(&mut self) -> IDLECONF4_W {
        IDLECONF4_W::new(self)
    }
    #[doc = "Bits 10:11 - ALTEX5 idle phase configuration"]
    #[inline(always)]
    pub fn idleconf5(&mut self) -> IDLECONF5_W {
        IDLECONF5_W::new(self)
    }
    #[doc = "Bits 12:13 - ALTEX6 idle phase configuration"]
    #[inline(always)]
    pub fn idleconf6(&mut self) -> IDLECONF6_W {
        IDLECONF6_W::new(self)
    }
    #[doc = "Bits 14:15 - ALTEX7 idle phase configuration"]
    #[inline(always)]
    pub fn idleconf7(&mut self) -> IDLECONF7_W {
        IDLECONF7_W::new(self)
    }
    #[doc = "Bit 16 - ALTEX0 always excite enable"]
    #[inline(always)]
    pub fn aex0(&mut self) -> AEX0_W {
        AEX0_W::new(self)
    }
    #[doc = "Bit 17 - ALTEX1 always excite enable"]
    #[inline(always)]
    pub fn aex1(&mut self) -> AEX1_W {
        AEX1_W::new(self)
    }
    #[doc = "Bit 18 - ALTEX2 always excite enable"]
    #[inline(always)]
    pub fn aex2(&mut self) -> AEX2_W {
        AEX2_W::new(self)
    }
    #[doc = "Bit 19 - ALTEX3 always excite enable"]
    #[inline(always)]
    pub fn aex3(&mut self) -> AEX3_W {
        AEX3_W::new(self)
    }
    #[doc = "Bit 20 - ALTEX4 always excite enable"]
    #[inline(always)]
    pub fn aex4(&mut self) -> AEX4_W {
        AEX4_W::new(self)
    }
    #[doc = "Bit 21 - ALTEX5 always excite enable"]
    #[inline(always)]
    pub fn aex5(&mut self) -> AEX5_W {
        AEX5_W::new(self)
    }
    #[doc = "Bit 22 - ALTEX6 always excite enable"]
    #[inline(always)]
    pub fn aex6(&mut self) -> AEX6_W {
        AEX6_W::new(self)
    }
    #[doc = "Bit 23 - ALTEX7 always excite enable"]
    #[inline(always)]
    pub fn aex7(&mut self) -> AEX7_W {
        AEX7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alternative excite pin configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altexconf](index.html) module"]
pub struct ALTEXCONF_SPEC;
impl crate::RegisterSpec for ALTEXCONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [altexconf::R](R) reader structure"]
impl crate::Readable for ALTEXCONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [altexconf::W](W) writer structure"]
impl crate::Writable for ALTEXCONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALTEXCONF to value 0"]
impl crate::Resettable for ALTEXCONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
