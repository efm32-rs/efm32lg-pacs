#[doc = "Register `CC1_CTRL` reader"]
pub struct R(crate::R<CC1_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CC1_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CC1_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CC1_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CC1_CTRL` writer"]
pub struct W(crate::W<CC1_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CC1_CTRL_SPEC>;
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
impl From<crate::W<CC1_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CC1_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CC Channel Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Compare/Capture channel turned off"]
    OFF = 0,
    #[doc = "1: Input capture"]
    INPUTCAPTURE = 1,
    #[doc = "2: Output compare"]
    OUTPUTCOMPARE = 2,
    #[doc = "3: Pulse-Width Modulation"]
    PWM = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - CC Channel Mode"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::OFF,
            1 => MODE_A::INPUTCAPTURE,
            2 => MODE_A::OUTPUTCOMPARE,
            3 => MODE_A::PWM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == MODE_A::OFF
    }
    #[doc = "Checks if the value of the field is `INPUTCAPTURE`"]
    #[inline(always)]
    pub fn is_inputcapture(&self) -> bool {
        *self == MODE_A::INPUTCAPTURE
    }
    #[doc = "Checks if the value of the field is `OUTPUTCOMPARE`"]
    #[inline(always)]
    pub fn is_outputcompare(&self) -> bool {
        *self == MODE_A::OUTPUTCOMPARE
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == MODE_A::PWM
    }
}
#[doc = "Field `MODE` writer - CC Channel Mode"]
pub type MODE_W<'a> = crate::FieldWriterSafe<'a, u32, CC1_CTRL_SPEC, u8, MODE_A, 2, 0>;
impl<'a> MODE_W<'a> {
    #[doc = "Compare/Capture channel turned off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(MODE_A::OFF)
    }
    #[doc = "Input capture"]
    #[inline(always)]
    pub fn inputcapture(self) -> &'a mut W {
        self.variant(MODE_A::INPUTCAPTURE)
    }
    #[doc = "Output compare"]
    #[inline(always)]
    pub fn outputcompare(self) -> &'a mut W {
        self.variant(MODE_A::OUTPUTCOMPARE)
    }
    #[doc = "Pulse-Width Modulation"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut W {
        self.variant(MODE_A::PWM)
    }
}
#[doc = "Field `OUTINV` reader - Output Invert"]
pub type OUTINV_R = crate::BitReader<bool>;
#[doc = "Field `OUTINV` writer - Output Invert"]
pub type OUTINV_W<'a> = crate::BitWriter<'a, u32, CC1_CTRL_SPEC, bool, 2>;
#[doc = "Field `COIST` reader - Compare Output Initial State"]
pub type COIST_R = crate::BitReader<bool>;
#[doc = "Field `COIST` writer - Compare Output Initial State"]
pub type COIST_W<'a> = crate::BitWriter<'a, u32, CC1_CTRL_SPEC, bool, 4>;
#[doc = "Compare Match Output Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMOA_A {
    #[doc = "0: No action on compare match"]
    NONE = 0,
    #[doc = "1: Toggle output on compare match"]
    TOGGLE = 1,
    #[doc = "2: Clear output on compare match"]
    CLEAR = 2,
    #[doc = "3: Set output on compare match"]
    SET = 3,
}
impl From<CMOA_A> for u8 {
    #[inline(always)]
    fn from(variant: CMOA_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMOA` reader - Compare Match Output Action"]
pub type CMOA_R = crate::FieldReader<u8, CMOA_A>;
impl CMOA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMOA_A {
        match self.bits {
            0 => CMOA_A::NONE,
            1 => CMOA_A::TOGGLE,
            2 => CMOA_A::CLEAR,
            3 => CMOA_A::SET,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CMOA_A::NONE
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == CMOA_A::TOGGLE
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CMOA_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CMOA_A::SET
    }
}
#[doc = "Field `CMOA` writer - Compare Match Output Action"]
pub type CMOA_W<'a> = crate::FieldWriterSafe<'a, u32, CC1_CTRL_SPEC, u8, CMOA_A, 2, 8>;
impl<'a> CMOA_W<'a> {
    #[doc = "No action on compare match"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(CMOA_A::NONE)
    }
    #[doc = "Toggle output on compare match"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(CMOA_A::TOGGLE)
    }
    #[doc = "Clear output on compare match"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CMOA_A::CLEAR)
    }
    #[doc = "Set output on compare match"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CMOA_A::SET)
    }
}
#[doc = "Counter Overflow Output Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COFOA_A {
    #[doc = "0: No action on counter overflow"]
    NONE = 0,
    #[doc = "1: Toggle output on counter overflow"]
    TOGGLE = 1,
    #[doc = "2: Clear output on counter overflow"]
    CLEAR = 2,
    #[doc = "3: Set output on counter overflow"]
    SET = 3,
}
impl From<COFOA_A> for u8 {
    #[inline(always)]
    fn from(variant: COFOA_A) -> Self {
        variant as _
    }
}
#[doc = "Field `COFOA` reader - Counter Overflow Output Action"]
pub type COFOA_R = crate::FieldReader<u8, COFOA_A>;
impl COFOA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COFOA_A {
        match self.bits {
            0 => COFOA_A::NONE,
            1 => COFOA_A::TOGGLE,
            2 => COFOA_A::CLEAR,
            3 => COFOA_A::SET,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == COFOA_A::NONE
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == COFOA_A::TOGGLE
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == COFOA_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == COFOA_A::SET
    }
}
#[doc = "Field `COFOA` writer - Counter Overflow Output Action"]
pub type COFOA_W<'a> = crate::FieldWriterSafe<'a, u32, CC1_CTRL_SPEC, u8, COFOA_A, 2, 10>;
impl<'a> COFOA_W<'a> {
    #[doc = "No action on counter overflow"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(COFOA_A::NONE)
    }
    #[doc = "Toggle output on counter overflow"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(COFOA_A::TOGGLE)
    }
    #[doc = "Clear output on counter overflow"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(COFOA_A::CLEAR)
    }
    #[doc = "Set output on counter overflow"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(COFOA_A::SET)
    }
}
#[doc = "Counter Underflow Output Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CUFOA_A {
    #[doc = "0: No action on counter underflow"]
    NONE = 0,
    #[doc = "1: Toggle output on counter underflow"]
    TOGGLE = 1,
    #[doc = "2: Clear output on counter underflow"]
    CLEAR = 2,
    #[doc = "3: Set output on counter underflow"]
    SET = 3,
}
impl From<CUFOA_A> for u8 {
    #[inline(always)]
    fn from(variant: CUFOA_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CUFOA` reader - Counter Underflow Output Action"]
pub type CUFOA_R = crate::FieldReader<u8, CUFOA_A>;
impl CUFOA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CUFOA_A {
        match self.bits {
            0 => CUFOA_A::NONE,
            1 => CUFOA_A::TOGGLE,
            2 => CUFOA_A::CLEAR,
            3 => CUFOA_A::SET,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CUFOA_A::NONE
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == CUFOA_A::TOGGLE
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CUFOA_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CUFOA_A::SET
    }
}
#[doc = "Field `CUFOA` writer - Counter Underflow Output Action"]
pub type CUFOA_W<'a> = crate::FieldWriterSafe<'a, u32, CC1_CTRL_SPEC, u8, CUFOA_A, 2, 12>;
impl<'a> CUFOA_W<'a> {
    #[doc = "No action on counter underflow"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(CUFOA_A::NONE)
    }
    #[doc = "Toggle output on counter underflow"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(CUFOA_A::TOGGLE)
    }
    #[doc = "Clear output on counter underflow"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CUFOA_A::CLEAR)
    }
    #[doc = "Set output on counter underflow"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CUFOA_A::SET)
    }
}
#[doc = "Compare/Capture Channel PRS Input Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRSSEL_A {
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
impl From<PRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRSSEL` reader - Compare/Capture Channel PRS Input Channel Selection"]
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
#[doc = "Field `PRSSEL` writer - Compare/Capture Channel PRS Input Channel Selection"]
pub type PRSSEL_W<'a> = crate::FieldWriter<'a, u32, CC1_CTRL_SPEC, u8, PRSSEL_A, 4, 16>;
impl<'a> PRSSEL_W<'a> {
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH11)
    }
}
#[doc = "Field `INSEL` reader - Input Selection"]
pub type INSEL_R = crate::BitReader<bool>;
#[doc = "Field `INSEL` writer - Input Selection"]
pub type INSEL_W<'a> = crate::BitWriter<'a, u32, CC1_CTRL_SPEC, bool, 20>;
#[doc = "Field `FILT` reader - Digital Filter"]
pub type FILT_R = crate::BitReader<bool>;
#[doc = "Field `FILT` writer - Digital Filter"]
pub type FILT_W<'a> = crate::BitWriter<'a, u32, CC1_CTRL_SPEC, bool, 21>;
#[doc = "Input Capture Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ICEDGE_A {
    #[doc = "0: Rising edges detected"]
    RISING = 0,
    #[doc = "1: Falling edges detected"]
    FALLING = 1,
    #[doc = "2: Both edges detected"]
    BOTH = 2,
    #[doc = "3: No edge detection, signal is left as it is"]
    NONE = 3,
}
impl From<ICEDGE_A> for u8 {
    #[inline(always)]
    fn from(variant: ICEDGE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ICEDGE` reader - Input Capture Edge Select"]
pub type ICEDGE_R = crate::FieldReader<u8, ICEDGE_A>;
impl ICEDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICEDGE_A {
        match self.bits {
            0 => ICEDGE_A::RISING,
            1 => ICEDGE_A::FALLING,
            2 => ICEDGE_A::BOTH,
            3 => ICEDGE_A::NONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == ICEDGE_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == ICEDGE_A::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == ICEDGE_A::BOTH
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ICEDGE_A::NONE
    }
}
#[doc = "Field `ICEDGE` writer - Input Capture Edge Select"]
pub type ICEDGE_W<'a> = crate::FieldWriterSafe<'a, u32, CC1_CTRL_SPEC, u8, ICEDGE_A, 2, 24>;
impl<'a> ICEDGE_W<'a> {
    #[doc = "Rising edges detected"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(ICEDGE_A::RISING)
    }
    #[doc = "Falling edges detected"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(ICEDGE_A::FALLING)
    }
    #[doc = "Both edges detected"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(ICEDGE_A::BOTH)
    }
    #[doc = "No edge detection, signal is left as it is"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ICEDGE_A::NONE)
    }
}
#[doc = "Input Capture Event Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ICEVCTRL_A {
    #[doc = "0: PRS output pulse, interrupt flag and DMA request set on every capture"]
    EVERYEDGE = 0,
    #[doc = "1: PRS output pulse, interrupt flag and DMA request set on every second capture"]
    EVERYSECONDEDGE = 1,
    #[doc = "2: PRS output pulse, interrupt flag and DMA request set on rising edge only (if ICEDGE = BOTH)"]
    RISING = 2,
    #[doc = "3: PRS output pulse, interrupt flag and DMA request set on falling edge only (if ICEDGE = BOTH)"]
    FALLING = 3,
}
impl From<ICEVCTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: ICEVCTRL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ICEVCTRL` reader - Input Capture Event Control"]
pub type ICEVCTRL_R = crate::FieldReader<u8, ICEVCTRL_A>;
impl ICEVCTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICEVCTRL_A {
        match self.bits {
            0 => ICEVCTRL_A::EVERYEDGE,
            1 => ICEVCTRL_A::EVERYSECONDEDGE,
            2 => ICEVCTRL_A::RISING,
            3 => ICEVCTRL_A::FALLING,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EVERYEDGE`"]
    #[inline(always)]
    pub fn is_everyedge(&self) -> bool {
        *self == ICEVCTRL_A::EVERYEDGE
    }
    #[doc = "Checks if the value of the field is `EVERYSECONDEDGE`"]
    #[inline(always)]
    pub fn is_everysecondedge(&self) -> bool {
        *self == ICEVCTRL_A::EVERYSECONDEDGE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == ICEVCTRL_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == ICEVCTRL_A::FALLING
    }
}
#[doc = "Field `ICEVCTRL` writer - Input Capture Event Control"]
pub type ICEVCTRL_W<'a> = crate::FieldWriterSafe<'a, u32, CC1_CTRL_SPEC, u8, ICEVCTRL_A, 2, 26>;
impl<'a> ICEVCTRL_W<'a> {
    #[doc = "PRS output pulse, interrupt flag and DMA request set on every capture"]
    #[inline(always)]
    pub fn everyedge(self) -> &'a mut W {
        self.variant(ICEVCTRL_A::EVERYEDGE)
    }
    #[doc = "PRS output pulse, interrupt flag and DMA request set on every second capture"]
    #[inline(always)]
    pub fn everysecondedge(self) -> &'a mut W {
        self.variant(ICEVCTRL_A::EVERYSECONDEDGE)
    }
    #[doc = "PRS output pulse, interrupt flag and DMA request set on rising edge only (if ICEDGE = BOTH)"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(ICEVCTRL_A::RISING)
    }
    #[doc = "PRS output pulse, interrupt flag and DMA request set on falling edge only (if ICEDGE = BOTH)"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(ICEVCTRL_A::FALLING)
    }
}
impl R {
    #[doc = "Bits 0:1 - CC Channel Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Output Invert"]
    #[inline(always)]
    pub fn outinv(&self) -> OUTINV_R {
        OUTINV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Compare Output Initial State"]
    #[inline(always)]
    pub fn coist(&self) -> COIST_R {
        COIST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Compare Match Output Action"]
    #[inline(always)]
    pub fn cmoa(&self) -> CMOA_R {
        CMOA_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Counter Overflow Output Action"]
    #[inline(always)]
    pub fn cofoa(&self) -> COFOA_R {
        COFOA_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Counter Underflow Output Action"]
    #[inline(always)]
    pub fn cufoa(&self) -> CUFOA_R {
        CUFOA_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Compare/Capture Channel PRS Input Channel Selection"]
    #[inline(always)]
    pub fn prssel(&self) -> PRSSEL_R {
        PRSSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Input Selection"]
    #[inline(always)]
    pub fn insel(&self) -> INSEL_R {
        INSEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Digital Filter"]
    #[inline(always)]
    pub fn filt(&self) -> FILT_R {
        FILT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Input Capture Edge Select"]
    #[inline(always)]
    pub fn icedge(&self) -> ICEDGE_R {
        ICEDGE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Input Capture Event Control"]
    #[inline(always)]
    pub fn icevctrl(&self) -> ICEVCTRL_R {
        ICEVCTRL_R::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CC Channel Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W::new(self)
    }
    #[doc = "Bit 2 - Output Invert"]
    #[inline(always)]
    pub fn outinv(&mut self) -> OUTINV_W {
        OUTINV_W::new(self)
    }
    #[doc = "Bit 4 - Compare Output Initial State"]
    #[inline(always)]
    pub fn coist(&mut self) -> COIST_W {
        COIST_W::new(self)
    }
    #[doc = "Bits 8:9 - Compare Match Output Action"]
    #[inline(always)]
    pub fn cmoa(&mut self) -> CMOA_W {
        CMOA_W::new(self)
    }
    #[doc = "Bits 10:11 - Counter Overflow Output Action"]
    #[inline(always)]
    pub fn cofoa(&mut self) -> COFOA_W {
        COFOA_W::new(self)
    }
    #[doc = "Bits 12:13 - Counter Underflow Output Action"]
    #[inline(always)]
    pub fn cufoa(&mut self) -> CUFOA_W {
        CUFOA_W::new(self)
    }
    #[doc = "Bits 16:19 - Compare/Capture Channel PRS Input Channel Selection"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PRSSEL_W {
        PRSSEL_W::new(self)
    }
    #[doc = "Bit 20 - Input Selection"]
    #[inline(always)]
    pub fn insel(&mut self) -> INSEL_W {
        INSEL_W::new(self)
    }
    #[doc = "Bit 21 - Digital Filter"]
    #[inline(always)]
    pub fn filt(&mut self) -> FILT_W {
        FILT_W::new(self)
    }
    #[doc = "Bits 24:25 - Input Capture Edge Select"]
    #[inline(always)]
    pub fn icedge(&mut self) -> ICEDGE_W {
        ICEDGE_W::new(self)
    }
    #[doc = "Bits 26:27 - Input Capture Event Control"]
    #[inline(always)]
    pub fn icevctrl(&mut self) -> ICEVCTRL_W {
        ICEVCTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CC Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc1_ctrl](index.html) module"]
pub struct CC1_CTRL_SPEC;
impl crate::RegisterSpec for CC1_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cc1_ctrl::R](R) reader structure"]
impl crate::Readable for CC1_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cc1_ctrl::W](W) writer structure"]
impl crate::Writable for CC1_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CC1_CTRL to value 0"]
impl crate::Resettable for CC1_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
