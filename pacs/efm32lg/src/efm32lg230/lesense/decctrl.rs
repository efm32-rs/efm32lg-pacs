#[doc = "Register `DECCTRL` reader"]
pub struct R(crate::R<DECCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DECCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DECCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DECCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DECCTRL` writer"]
pub struct W(crate::W<DECCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DECCTRL_SPEC>;
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
impl From<crate::W<DECCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DECCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DISABLE` reader - Disable the decoder"]
pub type DISABLE_R = crate::BitReader<bool>;
#[doc = "Field `DISABLE` writer - Disable the decoder"]
pub type DISABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DECCTRL_SPEC, bool, O>;
#[doc = "Field `ERRCHK` reader - Enable check of current state"]
pub type ERRCHK_R = crate::BitReader<bool>;
#[doc = "Field `ERRCHK` writer - Enable check of current state"]
pub type ERRCHK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DECCTRL_SPEC, bool, O>;
#[doc = "Field `INTMAP` reader - Enable decoder to channel interrupt mapping"]
pub type INTMAP_R = crate::BitReader<bool>;
#[doc = "Field `INTMAP` writer - Enable decoder to channel interrupt mapping"]
pub type INTMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DECCTRL_SPEC, bool, O>;
#[doc = "Field `HYSTPRS0` reader - Enable decoder hysteresis on PRS0 output"]
pub type HYSTPRS0_R = crate::BitReader<bool>;
#[doc = "Field `HYSTPRS0` writer - Enable decoder hysteresis on PRS0 output"]
pub type HYSTPRS0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DECCTRL_SPEC, bool, O>;
#[doc = "Field `HYSTPRS1` reader - Enable decoder hysteresis on PRS1 output"]
pub type HYSTPRS1_R = crate::BitReader<bool>;
#[doc = "Field `HYSTPRS1` writer - Enable decoder hysteresis on PRS1 output"]
pub type HYSTPRS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DECCTRL_SPEC, bool, O>;
#[doc = "Field `HYSTPRS2` reader - Enable decoder hysteresis on PRS2 output"]
pub type HYSTPRS2_R = crate::BitReader<bool>;
#[doc = "Field `HYSTPRS2` writer - Enable decoder hysteresis on PRS2 output"]
pub type HYSTPRS2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DECCTRL_SPEC, bool, O>;
#[doc = "Field `HYSTIRQ` reader - Enable decoder hysteresis on interrupt requests"]
pub type HYSTIRQ_R = crate::BitReader<bool>;
#[doc = "Field `HYSTIRQ` writer - Enable decoder hysteresis on interrupt requests"]
pub type HYSTIRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, DECCTRL_SPEC, bool, O>;
#[doc = "Field `PRSCNT` reader - Enable count mode on decoder PRS channels 0 and 1"]
pub type PRSCNT_R = crate::BitReader<bool>;
#[doc = "Field `PRSCNT` writer - Enable count mode on decoder PRS channels 0 and 1"]
pub type PRSCNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DECCTRL_SPEC, bool, O>;
#[doc = "Field `INPUT` reader - "]
pub type INPUT_R = crate::BitReader<bool>;
#[doc = "Field `INPUT` writer - "]
pub type INPUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DECCTRL_SPEC, bool, O>;
#[doc = "Field `PRSSEL0` reader - "]
pub type PRSSEL0_R = crate::FieldReader<u8, PRSSEL0_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSEL0_A {
    #[doc = "0: PRS Channel 0 selected as input"]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected as input"]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected as input"]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected as input"]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected as input"]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected as input"]
    PRSCH5 = 5,
    #[doc = "6: PRS Channel 6 selected as input"]
    PRSCH6 = 6,
    #[doc = "7: PRS Channel 7 selected as input"]
    PRSCH7 = 7,
    #[doc = "8: PRS Channel 8 selected as input"]
    PRSCH8 = 8,
    #[doc = "9: PRS Channel 9 selected as input"]
    PRSCH9 = 9,
    #[doc = "10: PRS Channel 10 selected as input"]
    PRSCH10 = 10,
    #[doc = "11: PRS Channel 11 selected as input"]
    PRSCH11 = 11,
}
impl From<PRSSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSEL0_A) -> Self {
        variant as _
    }
}
impl PRSSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRSSEL0_A> {
        match self.bits {
            0 => Some(PRSSEL0_A::PRSCH0),
            1 => Some(PRSSEL0_A::PRSCH1),
            2 => Some(PRSSEL0_A::PRSCH2),
            3 => Some(PRSSEL0_A::PRSCH3),
            4 => Some(PRSSEL0_A::PRSCH4),
            5 => Some(PRSSEL0_A::PRSCH5),
            6 => Some(PRSSEL0_A::PRSCH6),
            7 => Some(PRSSEL0_A::PRSCH7),
            8 => Some(PRSSEL0_A::PRSCH8),
            9 => Some(PRSSEL0_A::PRSCH9),
            10 => Some(PRSSEL0_A::PRSCH10),
            11 => Some(PRSSEL0_A::PRSCH11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL0_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL0_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL0_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL0_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL0_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL0_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL0_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL0_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL0_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL0_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL0_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL0_A::PRSCH11
    }
}
#[doc = "Field `PRSSEL0` writer - "]
pub type PRSSEL0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DECCTRL_SPEC, u8, PRSSEL0_A, 4, O>;
impl<'a, const O: u8> PRSSEL0_W<'a, O> {
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH11)
    }
}
#[doc = "Field `PRSSEL1` reader - "]
pub type PRSSEL1_R = crate::FieldReader<u8, PRSSEL1_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSEL1_A {
    #[doc = "0: PRS Channel 0 selected as input"]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected as input"]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected as input"]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected as input"]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected as input"]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected as input"]
    PRSCH5 = 5,
    #[doc = "6: PRS Channel 6 selected as input"]
    PRSCH6 = 6,
    #[doc = "7: PRS Channel 7 selected as input"]
    PRSCH7 = 7,
    #[doc = "8: PRS Channel 8 selected as input"]
    PRSCH8 = 8,
    #[doc = "9: PRS Channel 9 selected as input"]
    PRSCH9 = 9,
    #[doc = "10: PRS Channel 10 selected as input"]
    PRSCH10 = 10,
    #[doc = "11: PRS Channel 11 selected as input"]
    PRSCH11 = 11,
}
impl From<PRSSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSEL1_A) -> Self {
        variant as _
    }
}
impl PRSSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRSSEL1_A> {
        match self.bits {
            0 => Some(PRSSEL1_A::PRSCH0),
            1 => Some(PRSSEL1_A::PRSCH1),
            2 => Some(PRSSEL1_A::PRSCH2),
            3 => Some(PRSSEL1_A::PRSCH3),
            4 => Some(PRSSEL1_A::PRSCH4),
            5 => Some(PRSSEL1_A::PRSCH5),
            6 => Some(PRSSEL1_A::PRSCH6),
            7 => Some(PRSSEL1_A::PRSCH7),
            8 => Some(PRSSEL1_A::PRSCH8),
            9 => Some(PRSSEL1_A::PRSCH9),
            10 => Some(PRSSEL1_A::PRSCH10),
            11 => Some(PRSSEL1_A::PRSCH11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL1_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL1_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL1_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL1_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL1_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL1_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL1_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL1_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL1_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL1_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL1_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL1_A::PRSCH11
    }
}
#[doc = "Field `PRSSEL1` writer - "]
pub type PRSSEL1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DECCTRL_SPEC, u8, PRSSEL1_A, 4, O>;
impl<'a, const O: u8> PRSSEL1_W<'a, O> {
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH11)
    }
}
#[doc = "Field `PRSSEL2` reader - "]
pub type PRSSEL2_R = crate::FieldReader<u8, PRSSEL2_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSEL2_A {
    #[doc = "0: PRS Channel 0 selected as input"]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected as input"]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected as input"]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected as input"]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected as input"]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected as input"]
    PRSCH5 = 5,
    #[doc = "6: PRS Channel 6 selected as input"]
    PRSCH6 = 6,
    #[doc = "7: PRS Channel 7 selected as input"]
    PRSCH7 = 7,
    #[doc = "8: PRS Channel 8 selected as input"]
    PRSCH8 = 8,
    #[doc = "9: PRS Channel 9 selected as input"]
    PRSCH9 = 9,
    #[doc = "10: PRS Channel 10 selected as input"]
    PRSCH10 = 10,
    #[doc = "11: PRS Channel 11 selected as input"]
    PRSCH11 = 11,
}
impl From<PRSSEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSEL2_A) -> Self {
        variant as _
    }
}
impl PRSSEL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRSSEL2_A> {
        match self.bits {
            0 => Some(PRSSEL2_A::PRSCH0),
            1 => Some(PRSSEL2_A::PRSCH1),
            2 => Some(PRSSEL2_A::PRSCH2),
            3 => Some(PRSSEL2_A::PRSCH3),
            4 => Some(PRSSEL2_A::PRSCH4),
            5 => Some(PRSSEL2_A::PRSCH5),
            6 => Some(PRSSEL2_A::PRSCH6),
            7 => Some(PRSSEL2_A::PRSCH7),
            8 => Some(PRSSEL2_A::PRSCH8),
            9 => Some(PRSSEL2_A::PRSCH9),
            10 => Some(PRSSEL2_A::PRSCH10),
            11 => Some(PRSSEL2_A::PRSCH11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL2_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL2_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL2_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL2_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL2_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL2_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL2_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL2_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL2_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL2_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL2_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL2_A::PRSCH11
    }
}
#[doc = "Field `PRSSEL2` writer - "]
pub type PRSSEL2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DECCTRL_SPEC, u8, PRSSEL2_A, 4, O>;
impl<'a, const O: u8> PRSSEL2_W<'a, O> {
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH11)
    }
}
#[doc = "Field `PRSSEL3` reader - "]
pub type PRSSEL3_R = crate::FieldReader<u8, PRSSEL3_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSEL3_A {
    #[doc = "0: PRS Channel 0 selected as input"]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected as input"]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected as input"]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected as input"]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected as input"]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected as input"]
    PRSCH5 = 5,
    #[doc = "6: PRS Channel 6 selected as input"]
    PRSCH6 = 6,
    #[doc = "7: PRS Channel 7 selected as input"]
    PRSCH7 = 7,
    #[doc = "8: PRS Channel 8 selected as input"]
    PRSCH8 = 8,
    #[doc = "9: PRS Channel 9 selected as input"]
    PRSCH9 = 9,
    #[doc = "10: PRS Channel 10 selected as input"]
    PRSCH10 = 10,
    #[doc = "11: PRS Channel 11 selected as input"]
    PRSCH11 = 11,
}
impl From<PRSSEL3_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSEL3_A) -> Self {
        variant as _
    }
}
impl PRSSEL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRSSEL3_A> {
        match self.bits {
            0 => Some(PRSSEL3_A::PRSCH0),
            1 => Some(PRSSEL3_A::PRSCH1),
            2 => Some(PRSSEL3_A::PRSCH2),
            3 => Some(PRSSEL3_A::PRSCH3),
            4 => Some(PRSSEL3_A::PRSCH4),
            5 => Some(PRSSEL3_A::PRSCH5),
            6 => Some(PRSSEL3_A::PRSCH6),
            7 => Some(PRSSEL3_A::PRSCH7),
            8 => Some(PRSSEL3_A::PRSCH8),
            9 => Some(PRSSEL3_A::PRSCH9),
            10 => Some(PRSSEL3_A::PRSCH10),
            11 => Some(PRSSEL3_A::PRSCH11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL3_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL3_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL3_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL3_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL3_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL3_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL3_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL3_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL3_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL3_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL3_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL3_A::PRSCH11
    }
}
#[doc = "Field `PRSSEL3` writer - "]
pub type PRSSEL3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DECCTRL_SPEC, u8, PRSSEL3_A, 4, O>;
impl<'a, const O: u8> PRSSEL3_W<'a, O> {
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH11)
    }
}
impl R {
    #[doc = "Bit 0 - Disable the decoder"]
    #[inline(always)]
    pub fn disable(&self) -> DISABLE_R {
        DISABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable check of current state"]
    #[inline(always)]
    pub fn errchk(&self) -> ERRCHK_R {
        ERRCHK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable decoder to channel interrupt mapping"]
    #[inline(always)]
    pub fn intmap(&self) -> INTMAP_R {
        INTMAP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable decoder hysteresis on PRS0 output"]
    #[inline(always)]
    pub fn hystprs0(&self) -> HYSTPRS0_R {
        HYSTPRS0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable decoder hysteresis on PRS1 output"]
    #[inline(always)]
    pub fn hystprs1(&self) -> HYSTPRS1_R {
        HYSTPRS1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable decoder hysteresis on PRS2 output"]
    #[inline(always)]
    pub fn hystprs2(&self) -> HYSTPRS2_R {
        HYSTPRS2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable decoder hysteresis on interrupt requests"]
    #[inline(always)]
    pub fn hystirq(&self) -> HYSTIRQ_R {
        HYSTIRQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable count mode on decoder PRS channels 0 and 1"]
    #[inline(always)]
    pub fn prscnt(&self) -> PRSCNT_R {
        PRSCNT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn input(&self) -> INPUT_R {
        INPUT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:13"]
    #[inline(always)]
    pub fn prssel0(&self) -> PRSSEL0_R {
        PRSSEL0_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:17"]
    #[inline(always)]
    pub fn prssel1(&self) -> PRSSEL1_R {
        PRSSEL1_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn prssel2(&self) -> PRSSEL2_R {
        PRSSEL2_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn prssel3(&self) -> PRSSEL3_R {
        PRSSEL3_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Disable the decoder"]
    #[inline(always)]
    #[must_use]
    pub fn disable(&mut self) -> DISABLE_W<0> {
        DISABLE_W::new(self)
    }
    #[doc = "Bit 1 - Enable check of current state"]
    #[inline(always)]
    #[must_use]
    pub fn errchk(&mut self) -> ERRCHK_W<1> {
        ERRCHK_W::new(self)
    }
    #[doc = "Bit 2 - Enable decoder to channel interrupt mapping"]
    #[inline(always)]
    #[must_use]
    pub fn intmap(&mut self) -> INTMAP_W<2> {
        INTMAP_W::new(self)
    }
    #[doc = "Bit 3 - Enable decoder hysteresis on PRS0 output"]
    #[inline(always)]
    #[must_use]
    pub fn hystprs0(&mut self) -> HYSTPRS0_W<3> {
        HYSTPRS0_W::new(self)
    }
    #[doc = "Bit 4 - Enable decoder hysteresis on PRS1 output"]
    #[inline(always)]
    #[must_use]
    pub fn hystprs1(&mut self) -> HYSTPRS1_W<4> {
        HYSTPRS1_W::new(self)
    }
    #[doc = "Bit 5 - Enable decoder hysteresis on PRS2 output"]
    #[inline(always)]
    #[must_use]
    pub fn hystprs2(&mut self) -> HYSTPRS2_W<5> {
        HYSTPRS2_W::new(self)
    }
    #[doc = "Bit 6 - Enable decoder hysteresis on interrupt requests"]
    #[inline(always)]
    #[must_use]
    pub fn hystirq(&mut self) -> HYSTIRQ_W<6> {
        HYSTIRQ_W::new(self)
    }
    #[doc = "Bit 7 - Enable count mode on decoder PRS channels 0 and 1"]
    #[inline(always)]
    #[must_use]
    pub fn prscnt(&mut self) -> PRSCNT_W<7> {
        PRSCNT_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn input(&mut self) -> INPUT_W<8> {
        INPUT_W::new(self)
    }
    #[doc = "Bits 10:13"]
    #[inline(always)]
    #[must_use]
    pub fn prssel0(&mut self) -> PRSSEL0_W<10> {
        PRSSEL0_W::new(self)
    }
    #[doc = "Bits 14:17"]
    #[inline(always)]
    #[must_use]
    pub fn prssel1(&mut self) -> PRSSEL1_W<14> {
        PRSSEL1_W::new(self)
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    #[must_use]
    pub fn prssel2(&mut self) -> PRSSEL2_W<18> {
        PRSSEL2_W::new(self)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    #[must_use]
    pub fn prssel3(&mut self) -> PRSSEL3_W<22> {
        PRSSEL3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Decoder control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [decctrl](index.html) module"]
pub struct DECCTRL_SPEC;
impl crate::RegisterSpec for DECCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [decctrl::R](R) reader structure"]
impl crate::Readable for DECCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [decctrl::W](W) writer structure"]
impl crate::Writable for DECCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DECCTRL to value 0"]
impl crate::Resettable for DECCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
