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
#[doc = "Repeat Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REPMODE_A {
    #[doc = "0: When started, the LETIMER counts down until it is stopped by software."]
    FREE = 0,
    #[doc = "1: The counter counts REP0 times. When REP0 reaches zero, the counter stops."]
    ONESHOT = 1,
    #[doc = "2: The counter counts REP0 times. If REP1 has been written, it is loaded into REP0 when REP0 reaches zero. Else the counter stops"]
    BUFFERED = 2,
    #[doc = "3: Both REP0 and REP1 are decremented when the LETIMER wraps around. The LETIMER counts until both REP0 and REP1 are zero"]
    DOUBLE = 3,
}
impl From<REPMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: REPMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `REPMODE` reader - Repeat Mode"]
pub type REPMODE_R = crate::FieldReader<u8, REPMODE_A>;
impl REPMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REPMODE_A {
        match self.bits {
            0 => REPMODE_A::FREE,
            1 => REPMODE_A::ONESHOT,
            2 => REPMODE_A::BUFFERED,
            3 => REPMODE_A::DOUBLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FREE`"]
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        *self == REPMODE_A::FREE
    }
    #[doc = "Checks if the value of the field is `ONESHOT`"]
    #[inline(always)]
    pub fn is_oneshot(&self) -> bool {
        *self == REPMODE_A::ONESHOT
    }
    #[doc = "Checks if the value of the field is `BUFFERED`"]
    #[inline(always)]
    pub fn is_buffered(&self) -> bool {
        *self == REPMODE_A::BUFFERED
    }
    #[doc = "Checks if the value of the field is `DOUBLE`"]
    #[inline(always)]
    pub fn is_double(&self) -> bool {
        *self == REPMODE_A::DOUBLE
    }
}
#[doc = "Field `REPMODE` writer - Repeat Mode"]
pub type REPMODE_W<'a> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, REPMODE_A, 2, 0>;
impl<'a> REPMODE_W<'a> {
    #[doc = "When started, the LETIMER counts down until it is stopped by software."]
    #[inline(always)]
    pub fn free(self) -> &'a mut W {
        self.variant(REPMODE_A::FREE)
    }
    #[doc = "The counter counts REP0 times. When REP0 reaches zero, the counter stops."]
    #[inline(always)]
    pub fn oneshot(self) -> &'a mut W {
        self.variant(REPMODE_A::ONESHOT)
    }
    #[doc = "The counter counts REP0 times. If REP1 has been written, it is loaded into REP0 when REP0 reaches zero. Else the counter stops"]
    #[inline(always)]
    pub fn buffered(self) -> &'a mut W {
        self.variant(REPMODE_A::BUFFERED)
    }
    #[doc = "Both REP0 and REP1 are decremented when the LETIMER wraps around. The LETIMER counts until both REP0 and REP1 are zero"]
    #[inline(always)]
    pub fn double(self) -> &'a mut W {
        self.variant(REPMODE_A::DOUBLE)
    }
}
#[doc = "Underflow Output Action 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UFOA0_A {
    #[doc = "0: LETn_O0 is held at its idle value as defined by OPOL0."]
    NONE = 0,
    #[doc = "1: LETn_O0 is toggled on CNT underflow."]
    TOGGLE = 1,
    #[doc = "2: LETn_O0 is held active for one LFACLKLETIMER0 clock cycle on CNT underflow. The output then returns to its idle value as defined by OPOL0."]
    PULSE = 2,
    #[doc = "3: LETn_O0 is set idle on CNT underflow, and active on compare match with COMP1"]
    PWM = 3,
}
impl From<UFOA0_A> for u8 {
    #[inline(always)]
    fn from(variant: UFOA0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `UFOA0` reader - Underflow Output Action 0"]
pub type UFOA0_R = crate::FieldReader<u8, UFOA0_A>;
impl UFOA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UFOA0_A {
        match self.bits {
            0 => UFOA0_A::NONE,
            1 => UFOA0_A::TOGGLE,
            2 => UFOA0_A::PULSE,
            3 => UFOA0_A::PWM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == UFOA0_A::NONE
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == UFOA0_A::TOGGLE
    }
    #[doc = "Checks if the value of the field is `PULSE`"]
    #[inline(always)]
    pub fn is_pulse(&self) -> bool {
        *self == UFOA0_A::PULSE
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == UFOA0_A::PWM
    }
}
#[doc = "Field `UFOA0` writer - Underflow Output Action 0"]
pub type UFOA0_W<'a> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, UFOA0_A, 2, 2>;
impl<'a> UFOA0_W<'a> {
    #[doc = "LETn_O0 is held at its idle value as defined by OPOL0."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(UFOA0_A::NONE)
    }
    #[doc = "LETn_O0 is toggled on CNT underflow."]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(UFOA0_A::TOGGLE)
    }
    #[doc = "LETn_O0 is held active for one LFACLKLETIMER0 clock cycle on CNT underflow. The output then returns to its idle value as defined by OPOL0."]
    #[inline(always)]
    pub fn pulse(self) -> &'a mut W {
        self.variant(UFOA0_A::PULSE)
    }
    #[doc = "LETn_O0 is set idle on CNT underflow, and active on compare match with COMP1"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut W {
        self.variant(UFOA0_A::PWM)
    }
}
#[doc = "Underflow Output Action 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UFOA1_A {
    #[doc = "0: LETn_O1 is held at its idle value as defined by OPOL1."]
    NONE = 0,
    #[doc = "1: LETn_O1 is toggled on CNT underflow."]
    TOGGLE = 1,
    #[doc = "2: LETn_O1 is held active for one LFACLKLETIMER0 clock cycle on CNT underflow. The output then returns to its idle value as defined by OPOL1."]
    PULSE = 2,
    #[doc = "3: LETn_O1 is set idle on CNT underflow, and active on compare match with COMP1"]
    PWM = 3,
}
impl From<UFOA1_A> for u8 {
    #[inline(always)]
    fn from(variant: UFOA1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `UFOA1` reader - Underflow Output Action 1"]
pub type UFOA1_R = crate::FieldReader<u8, UFOA1_A>;
impl UFOA1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UFOA1_A {
        match self.bits {
            0 => UFOA1_A::NONE,
            1 => UFOA1_A::TOGGLE,
            2 => UFOA1_A::PULSE,
            3 => UFOA1_A::PWM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == UFOA1_A::NONE
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == UFOA1_A::TOGGLE
    }
    #[doc = "Checks if the value of the field is `PULSE`"]
    #[inline(always)]
    pub fn is_pulse(&self) -> bool {
        *self == UFOA1_A::PULSE
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == UFOA1_A::PWM
    }
}
#[doc = "Field `UFOA1` writer - Underflow Output Action 1"]
pub type UFOA1_W<'a> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, UFOA1_A, 2, 4>;
impl<'a> UFOA1_W<'a> {
    #[doc = "LETn_O1 is held at its idle value as defined by OPOL1."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(UFOA1_A::NONE)
    }
    #[doc = "LETn_O1 is toggled on CNT underflow."]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(UFOA1_A::TOGGLE)
    }
    #[doc = "LETn_O1 is held active for one LFACLKLETIMER0 clock cycle on CNT underflow. The output then returns to its idle value as defined by OPOL1."]
    #[inline(always)]
    pub fn pulse(self) -> &'a mut W {
        self.variant(UFOA1_A::PULSE)
    }
    #[doc = "LETn_O1 is set idle on CNT underflow, and active on compare match with COMP1"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut W {
        self.variant(UFOA1_A::PWM)
    }
}
#[doc = "Field `OPOL0` reader - Output 0 Polarity"]
pub type OPOL0_R = crate::BitReader<bool>;
#[doc = "Field `OPOL0` writer - Output 0 Polarity"]
pub type OPOL0_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 6>;
#[doc = "Field `OPOL1` reader - Output 1 Polarity"]
pub type OPOL1_R = crate::BitReader<bool>;
#[doc = "Field `OPOL1` writer - Output 1 Polarity"]
pub type OPOL1_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 7>;
#[doc = "Field `BUFTOP` reader - Buffered Top"]
pub type BUFTOP_R = crate::BitReader<bool>;
#[doc = "Field `BUFTOP` writer - Buffered Top"]
pub type BUFTOP_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 8>;
#[doc = "Field `COMP0TOP` reader - Compare Value 0 Is Top Value"]
pub type COMP0TOP_R = crate::BitReader<bool>;
#[doc = "Field `COMP0TOP` writer - Compare Value 0 Is Top Value"]
pub type COMP0TOP_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 9>;
#[doc = "Field `RTCC0TEN` reader - RTC Compare 0 Trigger Enable"]
pub type RTCC0TEN_R = crate::BitReader<bool>;
#[doc = "Field `RTCC0TEN` writer - RTC Compare 0 Trigger Enable"]
pub type RTCC0TEN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 10>;
#[doc = "Field `RTCC1TEN` reader - RTC Compare 1 Trigger Enable"]
pub type RTCC1TEN_R = crate::BitReader<bool>;
#[doc = "Field `RTCC1TEN` writer - RTC Compare 1 Trigger Enable"]
pub type RTCC1TEN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 11>;
#[doc = "Field `DEBUGRUN` reader - Debug Mode Run Enable"]
pub type DEBUGRUN_R = crate::BitReader<bool>;
#[doc = "Field `DEBUGRUN` writer - Debug Mode Run Enable"]
pub type DEBUGRUN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 12>;
impl R {
    #[doc = "Bits 0:1 - Repeat Mode"]
    #[inline(always)]
    pub fn repmode(&self) -> REPMODE_R {
        REPMODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Underflow Output Action 0"]
    #[inline(always)]
    pub fn ufoa0(&self) -> UFOA0_R {
        UFOA0_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Underflow Output Action 1"]
    #[inline(always)]
    pub fn ufoa1(&self) -> UFOA1_R {
        UFOA1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Output 0 Polarity"]
    #[inline(always)]
    pub fn opol0(&self) -> OPOL0_R {
        OPOL0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Output 1 Polarity"]
    #[inline(always)]
    pub fn opol1(&self) -> OPOL1_R {
        OPOL1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Buffered Top"]
    #[inline(always)]
    pub fn buftop(&self) -> BUFTOP_R {
        BUFTOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Compare Value 0 Is Top Value"]
    #[inline(always)]
    pub fn comp0top(&self) -> COMP0TOP_R {
        COMP0TOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC Compare 0 Trigger Enable"]
    #[inline(always)]
    pub fn rtcc0ten(&self) -> RTCC0TEN_R {
        RTCC0TEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RTC Compare 1 Trigger Enable"]
    #[inline(always)]
    pub fn rtcc1ten(&self) -> RTCC1TEN_R {
        RTCC1TEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&self) -> DEBUGRUN_R {
        DEBUGRUN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Repeat Mode"]
    #[inline(always)]
    pub fn repmode(&mut self) -> REPMODE_W {
        REPMODE_W::new(self)
    }
    #[doc = "Bits 2:3 - Underflow Output Action 0"]
    #[inline(always)]
    pub fn ufoa0(&mut self) -> UFOA0_W {
        UFOA0_W::new(self)
    }
    #[doc = "Bits 4:5 - Underflow Output Action 1"]
    #[inline(always)]
    pub fn ufoa1(&mut self) -> UFOA1_W {
        UFOA1_W::new(self)
    }
    #[doc = "Bit 6 - Output 0 Polarity"]
    #[inline(always)]
    pub fn opol0(&mut self) -> OPOL0_W {
        OPOL0_W::new(self)
    }
    #[doc = "Bit 7 - Output 1 Polarity"]
    #[inline(always)]
    pub fn opol1(&mut self) -> OPOL1_W {
        OPOL1_W::new(self)
    }
    #[doc = "Bit 8 - Buffered Top"]
    #[inline(always)]
    pub fn buftop(&mut self) -> BUFTOP_W {
        BUFTOP_W::new(self)
    }
    #[doc = "Bit 9 - Compare Value 0 Is Top Value"]
    #[inline(always)]
    pub fn comp0top(&mut self) -> COMP0TOP_W {
        COMP0TOP_W::new(self)
    }
    #[doc = "Bit 10 - RTC Compare 0 Trigger Enable"]
    #[inline(always)]
    pub fn rtcc0ten(&mut self) -> RTCC0TEN_W {
        RTCC0TEN_W::new(self)
    }
    #[doc = "Bit 11 - RTC Compare 1 Trigger Enable"]
    #[inline(always)]
    pub fn rtcc1ten(&mut self) -> RTCC1TEN_W {
        RTCC1TEN_W::new(self)
    }
    #[doc = "Bit 12 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&mut self) -> DEBUGRUN_W {
        DEBUGRUN_W::new(self)
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
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
