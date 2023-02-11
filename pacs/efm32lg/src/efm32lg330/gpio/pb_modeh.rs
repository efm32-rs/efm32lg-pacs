#[doc = "Register `PB_MODEH` reader"]
pub struct R(crate::R<PB_MODEH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PB_MODEH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PB_MODEH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PB_MODEH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PB_MODEH` writer"]
pub struct W(crate::W<PB_MODEH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PB_MODEH_SPEC>;
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
impl From<crate::W<PB_MODEH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PB_MODEH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE8` reader - Pin 8 Mode"]
pub type MODE8_R = crate::FieldReader<u8, MODE8_A>;
#[doc = "Pin 8 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE8_A {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    DISABLED = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    INPUT = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    INPUTPULL = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER = 3,
    #[doc = "4: Push-pull output"]
    PUSHPULL = 4,
    #[doc = "5: Push-pull output with drive-strength set by DRIVEMODE"]
    PUSHPULLDRIVE = 5,
    #[doc = "6: Wired-or output"]
    WIREDOR = 6,
    #[doc = "7: Wired-or output with pull-down"]
    WIREDORPULLDOWN = 7,
    #[doc = "8: Open-drain output"]
    WIREDAND = 8,
    #[doc = "9: Open-drain output with filter"]
    WIREDANDFILTER = 9,
    #[doc = "10: Open-drain output with pullup"]
    WIREDANDPULLUP = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER = 11,
    #[doc = "12: Open-drain output with drive-strength set by DRIVEMODE"]
    WIREDANDDRIVE = 12,
    #[doc = "13: Open-drain output with filter and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEFILTER = 13,
    #[doc = "14: Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUP = 14,
    #[doc = "15: Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUPFILTER = 15,
}
impl From<MODE8_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE8_A) -> Self {
        variant as _
    }
}
impl MODE8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE8_A {
        match self.bits {
            0 => MODE8_A::DISABLED,
            1 => MODE8_A::INPUT,
            2 => MODE8_A::INPUTPULL,
            3 => MODE8_A::INPUTPULLFILTER,
            4 => MODE8_A::PUSHPULL,
            5 => MODE8_A::PUSHPULLDRIVE,
            6 => MODE8_A::WIREDOR,
            7 => MODE8_A::WIREDORPULLDOWN,
            8 => MODE8_A::WIREDAND,
            9 => MODE8_A::WIREDANDFILTER,
            10 => MODE8_A::WIREDANDPULLUP,
            11 => MODE8_A::WIREDANDPULLUPFILTER,
            12 => MODE8_A::WIREDANDDRIVE,
            13 => MODE8_A::WIREDANDDRIVEFILTER,
            14 => MODE8_A::WIREDANDDRIVEPULLUP,
            15 => MODE8_A::WIREDANDDRIVEPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE8_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE8_A::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE8_A::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE8_A::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE8_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLDRIVE`"]
    #[inline(always)]
    pub fn is_pushpulldrive(&self) -> bool {
        *self == MODE8_A::PUSHPULLDRIVE
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE8_A::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE8_A::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE8_A::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE8_A::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE8_A::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE8_A::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVE`"]
    #[inline(always)]
    pub fn is_wiredanddrive(&self) -> bool {
        *self == MODE8_A::WIREDANDDRIVE
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEFILTER`"]
    #[inline(always)]
    pub fn is_wiredanddrivefilter(&self) -> bool {
        *self == MODE8_A::WIREDANDDRIVEFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEPULLUP`"]
    #[inline(always)]
    pub fn is_wiredanddrivepullup(&self) -> bool {
        *self == MODE8_A::WIREDANDDRIVEPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredanddrivepullupfilter(&self) -> bool {
        *self == MODE8_A::WIREDANDDRIVEPULLUPFILTER
    }
}
#[doc = "Field `MODE8` writer - Pin 8 Mode"]
pub type MODE8_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PB_MODEH_SPEC, u8, MODE8_A, 4, O>;
impl<'a, const O: u8> MODE8_W<'a, O> {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE8_A::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE8_A::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE8_A::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE8_A::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE8_A::PUSHPULL)
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn pushpulldrive(self) -> &'a mut W {
        self.variant(MODE8_A::PUSHPULLDRIVE)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE8_A::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE8_A::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE8_A::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE8_A::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE8_A::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE8_A::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrive(self) -> &'a mut W {
        self.variant(MODE8_A::WIREDANDDRIVE)
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivefilter(self) -> &'a mut W {
        self.variant(MODE8_A::WIREDANDDRIVEFILTER)
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullup(self) -> &'a mut W {
        self.variant(MODE8_A::WIREDANDDRIVEPULLUP)
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullupfilter(self) -> &'a mut W {
        self.variant(MODE8_A::WIREDANDDRIVEPULLUPFILTER)
    }
}
#[doc = "Field `MODE9` reader - Pin 9 Mode"]
pub type MODE9_R = crate::FieldReader<u8, MODE9_A>;
#[doc = "Pin 9 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE9_A {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    DISABLED = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    INPUT = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    INPUTPULL = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER = 3,
    #[doc = "4: Push-pull output"]
    PUSHPULL = 4,
    #[doc = "5: Push-pull output with drive-strength set by DRIVEMODE"]
    PUSHPULLDRIVE = 5,
    #[doc = "6: Wired-or output"]
    WIREDOR = 6,
    #[doc = "7: Wired-or output with pull-down"]
    WIREDORPULLDOWN = 7,
    #[doc = "8: Open-drain output"]
    WIREDAND = 8,
    #[doc = "9: Open-drain output with filter"]
    WIREDANDFILTER = 9,
    #[doc = "10: Open-drain output with pullup"]
    WIREDANDPULLUP = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER = 11,
    #[doc = "12: Open-drain output with drive-strength set by DRIVEMODE"]
    WIREDANDDRIVE = 12,
    #[doc = "13: Open-drain output with filter and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEFILTER = 13,
    #[doc = "14: Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUP = 14,
    #[doc = "15: Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUPFILTER = 15,
}
impl From<MODE9_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE9_A) -> Self {
        variant as _
    }
}
impl MODE9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE9_A {
        match self.bits {
            0 => MODE9_A::DISABLED,
            1 => MODE9_A::INPUT,
            2 => MODE9_A::INPUTPULL,
            3 => MODE9_A::INPUTPULLFILTER,
            4 => MODE9_A::PUSHPULL,
            5 => MODE9_A::PUSHPULLDRIVE,
            6 => MODE9_A::WIREDOR,
            7 => MODE9_A::WIREDORPULLDOWN,
            8 => MODE9_A::WIREDAND,
            9 => MODE9_A::WIREDANDFILTER,
            10 => MODE9_A::WIREDANDPULLUP,
            11 => MODE9_A::WIREDANDPULLUPFILTER,
            12 => MODE9_A::WIREDANDDRIVE,
            13 => MODE9_A::WIREDANDDRIVEFILTER,
            14 => MODE9_A::WIREDANDDRIVEPULLUP,
            15 => MODE9_A::WIREDANDDRIVEPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE9_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE9_A::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE9_A::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE9_A::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE9_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLDRIVE`"]
    #[inline(always)]
    pub fn is_pushpulldrive(&self) -> bool {
        *self == MODE9_A::PUSHPULLDRIVE
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE9_A::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE9_A::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE9_A::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE9_A::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE9_A::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE9_A::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVE`"]
    #[inline(always)]
    pub fn is_wiredanddrive(&self) -> bool {
        *self == MODE9_A::WIREDANDDRIVE
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEFILTER`"]
    #[inline(always)]
    pub fn is_wiredanddrivefilter(&self) -> bool {
        *self == MODE9_A::WIREDANDDRIVEFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEPULLUP`"]
    #[inline(always)]
    pub fn is_wiredanddrivepullup(&self) -> bool {
        *self == MODE9_A::WIREDANDDRIVEPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredanddrivepullupfilter(&self) -> bool {
        *self == MODE9_A::WIREDANDDRIVEPULLUPFILTER
    }
}
#[doc = "Field `MODE9` writer - Pin 9 Mode"]
pub type MODE9_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PB_MODEH_SPEC, u8, MODE9_A, 4, O>;
impl<'a, const O: u8> MODE9_W<'a, O> {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE9_A::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE9_A::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE9_A::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE9_A::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE9_A::PUSHPULL)
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn pushpulldrive(self) -> &'a mut W {
        self.variant(MODE9_A::PUSHPULLDRIVE)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE9_A::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE9_A::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE9_A::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE9_A::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE9_A::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE9_A::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrive(self) -> &'a mut W {
        self.variant(MODE9_A::WIREDANDDRIVE)
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivefilter(self) -> &'a mut W {
        self.variant(MODE9_A::WIREDANDDRIVEFILTER)
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullup(self) -> &'a mut W {
        self.variant(MODE9_A::WIREDANDDRIVEPULLUP)
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullupfilter(self) -> &'a mut W {
        self.variant(MODE9_A::WIREDANDDRIVEPULLUPFILTER)
    }
}
#[doc = "Field `MODE10` reader - Pin 10 Mode"]
pub type MODE10_R = crate::FieldReader<u8, MODE10_A>;
#[doc = "Pin 10 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE10_A {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    DISABLED = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    INPUT = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    INPUTPULL = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER = 3,
    #[doc = "4: Push-pull output"]
    PUSHPULL = 4,
    #[doc = "5: Push-pull output with drive-strength set by DRIVEMODE"]
    PUSHPULLDRIVE = 5,
    #[doc = "6: Wired-or output"]
    WIREDOR = 6,
    #[doc = "7: Wired-or output with pull-down"]
    WIREDORPULLDOWN = 7,
    #[doc = "8: Open-drain output"]
    WIREDAND = 8,
    #[doc = "9: Open-drain output with filter"]
    WIREDANDFILTER = 9,
    #[doc = "10: Open-drain output with pullup"]
    WIREDANDPULLUP = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER = 11,
    #[doc = "12: Open-drain output with drive-strength set by DRIVEMODE"]
    WIREDANDDRIVE = 12,
    #[doc = "13: Open-drain output with filter and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEFILTER = 13,
    #[doc = "14: Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUP = 14,
    #[doc = "15: Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUPFILTER = 15,
}
impl From<MODE10_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE10_A) -> Self {
        variant as _
    }
}
impl MODE10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE10_A {
        match self.bits {
            0 => MODE10_A::DISABLED,
            1 => MODE10_A::INPUT,
            2 => MODE10_A::INPUTPULL,
            3 => MODE10_A::INPUTPULLFILTER,
            4 => MODE10_A::PUSHPULL,
            5 => MODE10_A::PUSHPULLDRIVE,
            6 => MODE10_A::WIREDOR,
            7 => MODE10_A::WIREDORPULLDOWN,
            8 => MODE10_A::WIREDAND,
            9 => MODE10_A::WIREDANDFILTER,
            10 => MODE10_A::WIREDANDPULLUP,
            11 => MODE10_A::WIREDANDPULLUPFILTER,
            12 => MODE10_A::WIREDANDDRIVE,
            13 => MODE10_A::WIREDANDDRIVEFILTER,
            14 => MODE10_A::WIREDANDDRIVEPULLUP,
            15 => MODE10_A::WIREDANDDRIVEPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE10_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE10_A::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE10_A::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE10_A::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE10_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLDRIVE`"]
    #[inline(always)]
    pub fn is_pushpulldrive(&self) -> bool {
        *self == MODE10_A::PUSHPULLDRIVE
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE10_A::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE10_A::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE10_A::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE10_A::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE10_A::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE10_A::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVE`"]
    #[inline(always)]
    pub fn is_wiredanddrive(&self) -> bool {
        *self == MODE10_A::WIREDANDDRIVE
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEFILTER`"]
    #[inline(always)]
    pub fn is_wiredanddrivefilter(&self) -> bool {
        *self == MODE10_A::WIREDANDDRIVEFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEPULLUP`"]
    #[inline(always)]
    pub fn is_wiredanddrivepullup(&self) -> bool {
        *self == MODE10_A::WIREDANDDRIVEPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredanddrivepullupfilter(&self) -> bool {
        *self == MODE10_A::WIREDANDDRIVEPULLUPFILTER
    }
}
#[doc = "Field `MODE10` writer - Pin 10 Mode"]
pub type MODE10_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PB_MODEH_SPEC, u8, MODE10_A, 4, O>;
impl<'a, const O: u8> MODE10_W<'a, O> {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE10_A::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE10_A::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE10_A::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE10_A::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE10_A::PUSHPULL)
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn pushpulldrive(self) -> &'a mut W {
        self.variant(MODE10_A::PUSHPULLDRIVE)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE10_A::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE10_A::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE10_A::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE10_A::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE10_A::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE10_A::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrive(self) -> &'a mut W {
        self.variant(MODE10_A::WIREDANDDRIVE)
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivefilter(self) -> &'a mut W {
        self.variant(MODE10_A::WIREDANDDRIVEFILTER)
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullup(self) -> &'a mut W {
        self.variant(MODE10_A::WIREDANDDRIVEPULLUP)
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullupfilter(self) -> &'a mut W {
        self.variant(MODE10_A::WIREDANDDRIVEPULLUPFILTER)
    }
}
#[doc = "Field `MODE11` reader - Pin 11 Mode"]
pub type MODE11_R = crate::FieldReader<u8, MODE11_A>;
#[doc = "Pin 11 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE11_A {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    DISABLED = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    INPUT = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    INPUTPULL = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER = 3,
    #[doc = "4: Push-pull output"]
    PUSHPULL = 4,
    #[doc = "5: Push-pull output with drive-strength set by DRIVEMODE"]
    PUSHPULLDRIVE = 5,
    #[doc = "6: Wired-or output"]
    WIREDOR = 6,
    #[doc = "7: Wired-or output with pull-down"]
    WIREDORPULLDOWN = 7,
    #[doc = "8: Open-drain output"]
    WIREDAND = 8,
    #[doc = "9: Open-drain output with filter"]
    WIREDANDFILTER = 9,
    #[doc = "10: Open-drain output with pullup"]
    WIREDANDPULLUP = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER = 11,
    #[doc = "12: Open-drain output with drive-strength set by DRIVEMODE"]
    WIREDANDDRIVE = 12,
    #[doc = "13: Open-drain output with filter and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEFILTER = 13,
    #[doc = "14: Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUP = 14,
    #[doc = "15: Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUPFILTER = 15,
}
impl From<MODE11_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE11_A) -> Self {
        variant as _
    }
}
impl MODE11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE11_A {
        match self.bits {
            0 => MODE11_A::DISABLED,
            1 => MODE11_A::INPUT,
            2 => MODE11_A::INPUTPULL,
            3 => MODE11_A::INPUTPULLFILTER,
            4 => MODE11_A::PUSHPULL,
            5 => MODE11_A::PUSHPULLDRIVE,
            6 => MODE11_A::WIREDOR,
            7 => MODE11_A::WIREDORPULLDOWN,
            8 => MODE11_A::WIREDAND,
            9 => MODE11_A::WIREDANDFILTER,
            10 => MODE11_A::WIREDANDPULLUP,
            11 => MODE11_A::WIREDANDPULLUPFILTER,
            12 => MODE11_A::WIREDANDDRIVE,
            13 => MODE11_A::WIREDANDDRIVEFILTER,
            14 => MODE11_A::WIREDANDDRIVEPULLUP,
            15 => MODE11_A::WIREDANDDRIVEPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE11_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE11_A::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE11_A::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE11_A::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE11_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLDRIVE`"]
    #[inline(always)]
    pub fn is_pushpulldrive(&self) -> bool {
        *self == MODE11_A::PUSHPULLDRIVE
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE11_A::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE11_A::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE11_A::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE11_A::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE11_A::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE11_A::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVE`"]
    #[inline(always)]
    pub fn is_wiredanddrive(&self) -> bool {
        *self == MODE11_A::WIREDANDDRIVE
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEFILTER`"]
    #[inline(always)]
    pub fn is_wiredanddrivefilter(&self) -> bool {
        *self == MODE11_A::WIREDANDDRIVEFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEPULLUP`"]
    #[inline(always)]
    pub fn is_wiredanddrivepullup(&self) -> bool {
        *self == MODE11_A::WIREDANDDRIVEPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredanddrivepullupfilter(&self) -> bool {
        *self == MODE11_A::WIREDANDDRIVEPULLUPFILTER
    }
}
#[doc = "Field `MODE11` writer - Pin 11 Mode"]
pub type MODE11_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PB_MODEH_SPEC, u8, MODE11_A, 4, O>;
impl<'a, const O: u8> MODE11_W<'a, O> {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE11_A::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE11_A::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE11_A::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE11_A::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE11_A::PUSHPULL)
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn pushpulldrive(self) -> &'a mut W {
        self.variant(MODE11_A::PUSHPULLDRIVE)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE11_A::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE11_A::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE11_A::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE11_A::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE11_A::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE11_A::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrive(self) -> &'a mut W {
        self.variant(MODE11_A::WIREDANDDRIVE)
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivefilter(self) -> &'a mut W {
        self.variant(MODE11_A::WIREDANDDRIVEFILTER)
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullup(self) -> &'a mut W {
        self.variant(MODE11_A::WIREDANDDRIVEPULLUP)
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullupfilter(self) -> &'a mut W {
        self.variant(MODE11_A::WIREDANDDRIVEPULLUPFILTER)
    }
}
#[doc = "Field `MODE12` reader - Pin 12 Mode"]
pub type MODE12_R = crate::FieldReader<u8, MODE12_A>;
#[doc = "Pin 12 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE12_A {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    DISABLED = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    INPUT = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    INPUTPULL = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER = 3,
    #[doc = "4: Push-pull output"]
    PUSHPULL = 4,
    #[doc = "5: Push-pull output with drive-strength set by DRIVEMODE"]
    PUSHPULLDRIVE = 5,
    #[doc = "6: Wired-or output"]
    WIREDOR = 6,
    #[doc = "7: Wired-or output with pull-down"]
    WIREDORPULLDOWN = 7,
    #[doc = "8: Open-drain output"]
    WIREDAND = 8,
    #[doc = "9: Open-drain output with filter"]
    WIREDANDFILTER = 9,
    #[doc = "10: Open-drain output with pullup"]
    WIREDANDPULLUP = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER = 11,
    #[doc = "12: Open-drain output with drive-strength set by DRIVEMODE"]
    WIREDANDDRIVE = 12,
    #[doc = "13: Open-drain output with filter and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEFILTER = 13,
    #[doc = "14: Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUP = 14,
    #[doc = "15: Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUPFILTER = 15,
}
impl From<MODE12_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE12_A) -> Self {
        variant as _
    }
}
impl MODE12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE12_A {
        match self.bits {
            0 => MODE12_A::DISABLED,
            1 => MODE12_A::INPUT,
            2 => MODE12_A::INPUTPULL,
            3 => MODE12_A::INPUTPULLFILTER,
            4 => MODE12_A::PUSHPULL,
            5 => MODE12_A::PUSHPULLDRIVE,
            6 => MODE12_A::WIREDOR,
            7 => MODE12_A::WIREDORPULLDOWN,
            8 => MODE12_A::WIREDAND,
            9 => MODE12_A::WIREDANDFILTER,
            10 => MODE12_A::WIREDANDPULLUP,
            11 => MODE12_A::WIREDANDPULLUPFILTER,
            12 => MODE12_A::WIREDANDDRIVE,
            13 => MODE12_A::WIREDANDDRIVEFILTER,
            14 => MODE12_A::WIREDANDDRIVEPULLUP,
            15 => MODE12_A::WIREDANDDRIVEPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE12_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE12_A::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE12_A::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE12_A::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE12_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLDRIVE`"]
    #[inline(always)]
    pub fn is_pushpulldrive(&self) -> bool {
        *self == MODE12_A::PUSHPULLDRIVE
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE12_A::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE12_A::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE12_A::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE12_A::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE12_A::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE12_A::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVE`"]
    #[inline(always)]
    pub fn is_wiredanddrive(&self) -> bool {
        *self == MODE12_A::WIREDANDDRIVE
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEFILTER`"]
    #[inline(always)]
    pub fn is_wiredanddrivefilter(&self) -> bool {
        *self == MODE12_A::WIREDANDDRIVEFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEPULLUP`"]
    #[inline(always)]
    pub fn is_wiredanddrivepullup(&self) -> bool {
        *self == MODE12_A::WIREDANDDRIVEPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredanddrivepullupfilter(&self) -> bool {
        *self == MODE12_A::WIREDANDDRIVEPULLUPFILTER
    }
}
#[doc = "Field `MODE12` writer - Pin 12 Mode"]
pub type MODE12_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PB_MODEH_SPEC, u8, MODE12_A, 4, O>;
impl<'a, const O: u8> MODE12_W<'a, O> {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE12_A::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE12_A::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE12_A::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE12_A::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE12_A::PUSHPULL)
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn pushpulldrive(self) -> &'a mut W {
        self.variant(MODE12_A::PUSHPULLDRIVE)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE12_A::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE12_A::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE12_A::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE12_A::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE12_A::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE12_A::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrive(self) -> &'a mut W {
        self.variant(MODE12_A::WIREDANDDRIVE)
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivefilter(self) -> &'a mut W {
        self.variant(MODE12_A::WIREDANDDRIVEFILTER)
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullup(self) -> &'a mut W {
        self.variant(MODE12_A::WIREDANDDRIVEPULLUP)
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullupfilter(self) -> &'a mut W {
        self.variant(MODE12_A::WIREDANDDRIVEPULLUPFILTER)
    }
}
#[doc = "Field `MODE13` reader - Pin 13 Mode"]
pub type MODE13_R = crate::FieldReader<u8, MODE13_A>;
#[doc = "Pin 13 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE13_A {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    DISABLED = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    INPUT = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    INPUTPULL = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER = 3,
    #[doc = "4: Push-pull output"]
    PUSHPULL = 4,
    #[doc = "5: Push-pull output with drive-strength set by DRIVEMODE"]
    PUSHPULLDRIVE = 5,
    #[doc = "6: Wired-or output"]
    WIREDOR = 6,
    #[doc = "7: Wired-or output with pull-down"]
    WIREDORPULLDOWN = 7,
    #[doc = "8: Open-drain output"]
    WIREDAND = 8,
    #[doc = "9: Open-drain output with filter"]
    WIREDANDFILTER = 9,
    #[doc = "10: Open-drain output with pullup"]
    WIREDANDPULLUP = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER = 11,
    #[doc = "12: Open-drain output with drive-strength set by DRIVEMODE"]
    WIREDANDDRIVE = 12,
    #[doc = "13: Open-drain output with filter and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEFILTER = 13,
    #[doc = "14: Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUP = 14,
    #[doc = "15: Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUPFILTER = 15,
}
impl From<MODE13_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE13_A) -> Self {
        variant as _
    }
}
impl MODE13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE13_A {
        match self.bits {
            0 => MODE13_A::DISABLED,
            1 => MODE13_A::INPUT,
            2 => MODE13_A::INPUTPULL,
            3 => MODE13_A::INPUTPULLFILTER,
            4 => MODE13_A::PUSHPULL,
            5 => MODE13_A::PUSHPULLDRIVE,
            6 => MODE13_A::WIREDOR,
            7 => MODE13_A::WIREDORPULLDOWN,
            8 => MODE13_A::WIREDAND,
            9 => MODE13_A::WIREDANDFILTER,
            10 => MODE13_A::WIREDANDPULLUP,
            11 => MODE13_A::WIREDANDPULLUPFILTER,
            12 => MODE13_A::WIREDANDDRIVE,
            13 => MODE13_A::WIREDANDDRIVEFILTER,
            14 => MODE13_A::WIREDANDDRIVEPULLUP,
            15 => MODE13_A::WIREDANDDRIVEPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE13_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE13_A::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE13_A::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE13_A::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE13_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLDRIVE`"]
    #[inline(always)]
    pub fn is_pushpulldrive(&self) -> bool {
        *self == MODE13_A::PUSHPULLDRIVE
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE13_A::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE13_A::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE13_A::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE13_A::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE13_A::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE13_A::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVE`"]
    #[inline(always)]
    pub fn is_wiredanddrive(&self) -> bool {
        *self == MODE13_A::WIREDANDDRIVE
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEFILTER`"]
    #[inline(always)]
    pub fn is_wiredanddrivefilter(&self) -> bool {
        *self == MODE13_A::WIREDANDDRIVEFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEPULLUP`"]
    #[inline(always)]
    pub fn is_wiredanddrivepullup(&self) -> bool {
        *self == MODE13_A::WIREDANDDRIVEPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredanddrivepullupfilter(&self) -> bool {
        *self == MODE13_A::WIREDANDDRIVEPULLUPFILTER
    }
}
#[doc = "Field `MODE13` writer - Pin 13 Mode"]
pub type MODE13_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PB_MODEH_SPEC, u8, MODE13_A, 4, O>;
impl<'a, const O: u8> MODE13_W<'a, O> {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE13_A::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE13_A::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE13_A::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE13_A::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE13_A::PUSHPULL)
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn pushpulldrive(self) -> &'a mut W {
        self.variant(MODE13_A::PUSHPULLDRIVE)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE13_A::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE13_A::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE13_A::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE13_A::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE13_A::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE13_A::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrive(self) -> &'a mut W {
        self.variant(MODE13_A::WIREDANDDRIVE)
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivefilter(self) -> &'a mut W {
        self.variant(MODE13_A::WIREDANDDRIVEFILTER)
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullup(self) -> &'a mut W {
        self.variant(MODE13_A::WIREDANDDRIVEPULLUP)
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullupfilter(self) -> &'a mut W {
        self.variant(MODE13_A::WIREDANDDRIVEPULLUPFILTER)
    }
}
#[doc = "Field `MODE14` reader - Pin 14 Mode"]
pub type MODE14_R = crate::FieldReader<u8, MODE14_A>;
#[doc = "Pin 14 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE14_A {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    DISABLED = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    INPUT = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    INPUTPULL = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER = 3,
    #[doc = "4: Push-pull output"]
    PUSHPULL = 4,
    #[doc = "5: Push-pull output with drive-strength set by DRIVEMODE"]
    PUSHPULLDRIVE = 5,
    #[doc = "6: Wired-or output"]
    WIREDOR = 6,
    #[doc = "7: Wired-or output with pull-down"]
    WIREDORPULLDOWN = 7,
    #[doc = "8: Open-drain output"]
    WIREDAND = 8,
    #[doc = "9: Open-drain output with filter"]
    WIREDANDFILTER = 9,
    #[doc = "10: Open-drain output with pullup"]
    WIREDANDPULLUP = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER = 11,
    #[doc = "12: Open-drain output with drive-strength set by DRIVEMODE"]
    WIREDANDDRIVE = 12,
    #[doc = "13: Open-drain output with filter and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEFILTER = 13,
    #[doc = "14: Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUP = 14,
    #[doc = "15: Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUPFILTER = 15,
}
impl From<MODE14_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE14_A) -> Self {
        variant as _
    }
}
impl MODE14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE14_A {
        match self.bits {
            0 => MODE14_A::DISABLED,
            1 => MODE14_A::INPUT,
            2 => MODE14_A::INPUTPULL,
            3 => MODE14_A::INPUTPULLFILTER,
            4 => MODE14_A::PUSHPULL,
            5 => MODE14_A::PUSHPULLDRIVE,
            6 => MODE14_A::WIREDOR,
            7 => MODE14_A::WIREDORPULLDOWN,
            8 => MODE14_A::WIREDAND,
            9 => MODE14_A::WIREDANDFILTER,
            10 => MODE14_A::WIREDANDPULLUP,
            11 => MODE14_A::WIREDANDPULLUPFILTER,
            12 => MODE14_A::WIREDANDDRIVE,
            13 => MODE14_A::WIREDANDDRIVEFILTER,
            14 => MODE14_A::WIREDANDDRIVEPULLUP,
            15 => MODE14_A::WIREDANDDRIVEPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE14_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE14_A::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE14_A::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE14_A::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE14_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLDRIVE`"]
    #[inline(always)]
    pub fn is_pushpulldrive(&self) -> bool {
        *self == MODE14_A::PUSHPULLDRIVE
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE14_A::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE14_A::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE14_A::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE14_A::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE14_A::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE14_A::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVE`"]
    #[inline(always)]
    pub fn is_wiredanddrive(&self) -> bool {
        *self == MODE14_A::WIREDANDDRIVE
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEFILTER`"]
    #[inline(always)]
    pub fn is_wiredanddrivefilter(&self) -> bool {
        *self == MODE14_A::WIREDANDDRIVEFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEPULLUP`"]
    #[inline(always)]
    pub fn is_wiredanddrivepullup(&self) -> bool {
        *self == MODE14_A::WIREDANDDRIVEPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredanddrivepullupfilter(&self) -> bool {
        *self == MODE14_A::WIREDANDDRIVEPULLUPFILTER
    }
}
#[doc = "Field `MODE14` writer - Pin 14 Mode"]
pub type MODE14_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PB_MODEH_SPEC, u8, MODE14_A, 4, O>;
impl<'a, const O: u8> MODE14_W<'a, O> {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE14_A::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE14_A::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE14_A::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE14_A::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE14_A::PUSHPULL)
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn pushpulldrive(self) -> &'a mut W {
        self.variant(MODE14_A::PUSHPULLDRIVE)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE14_A::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE14_A::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE14_A::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE14_A::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE14_A::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE14_A::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrive(self) -> &'a mut W {
        self.variant(MODE14_A::WIREDANDDRIVE)
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivefilter(self) -> &'a mut W {
        self.variant(MODE14_A::WIREDANDDRIVEFILTER)
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullup(self) -> &'a mut W {
        self.variant(MODE14_A::WIREDANDDRIVEPULLUP)
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullupfilter(self) -> &'a mut W {
        self.variant(MODE14_A::WIREDANDDRIVEPULLUPFILTER)
    }
}
#[doc = "Field `MODE15` reader - Pin 15 Mode"]
pub type MODE15_R = crate::FieldReader<u8, MODE15_A>;
#[doc = "Pin 15 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE15_A {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    DISABLED = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    INPUT = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    INPUTPULL = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER = 3,
    #[doc = "4: Push-pull output"]
    PUSHPULL = 4,
    #[doc = "5: Push-pull output with drive-strength set by DRIVEMODE"]
    PUSHPULLDRIVE = 5,
    #[doc = "6: Wired-or output"]
    WIREDOR = 6,
    #[doc = "7: Wired-or output with pull-down"]
    WIREDORPULLDOWN = 7,
    #[doc = "8: Open-drain output"]
    WIREDAND = 8,
    #[doc = "9: Open-drain output with filter"]
    WIREDANDFILTER = 9,
    #[doc = "10: Open-drain output with pullup"]
    WIREDANDPULLUP = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER = 11,
    #[doc = "12: Open-drain output with drive-strength set by DRIVEMODE"]
    WIREDANDDRIVE = 12,
    #[doc = "13: Open-drain output with filter and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEFILTER = 13,
    #[doc = "14: Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUP = 14,
    #[doc = "15: Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUPFILTER = 15,
}
impl From<MODE15_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE15_A) -> Self {
        variant as _
    }
}
impl MODE15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE15_A {
        match self.bits {
            0 => MODE15_A::DISABLED,
            1 => MODE15_A::INPUT,
            2 => MODE15_A::INPUTPULL,
            3 => MODE15_A::INPUTPULLFILTER,
            4 => MODE15_A::PUSHPULL,
            5 => MODE15_A::PUSHPULLDRIVE,
            6 => MODE15_A::WIREDOR,
            7 => MODE15_A::WIREDORPULLDOWN,
            8 => MODE15_A::WIREDAND,
            9 => MODE15_A::WIREDANDFILTER,
            10 => MODE15_A::WIREDANDPULLUP,
            11 => MODE15_A::WIREDANDPULLUPFILTER,
            12 => MODE15_A::WIREDANDDRIVE,
            13 => MODE15_A::WIREDANDDRIVEFILTER,
            14 => MODE15_A::WIREDANDDRIVEPULLUP,
            15 => MODE15_A::WIREDANDDRIVEPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE15_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE15_A::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE15_A::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE15_A::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE15_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLDRIVE`"]
    #[inline(always)]
    pub fn is_pushpulldrive(&self) -> bool {
        *self == MODE15_A::PUSHPULLDRIVE
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE15_A::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE15_A::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE15_A::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE15_A::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE15_A::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE15_A::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVE`"]
    #[inline(always)]
    pub fn is_wiredanddrive(&self) -> bool {
        *self == MODE15_A::WIREDANDDRIVE
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEFILTER`"]
    #[inline(always)]
    pub fn is_wiredanddrivefilter(&self) -> bool {
        *self == MODE15_A::WIREDANDDRIVEFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEPULLUP`"]
    #[inline(always)]
    pub fn is_wiredanddrivepullup(&self) -> bool {
        *self == MODE15_A::WIREDANDDRIVEPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredanddrivepullupfilter(&self) -> bool {
        *self == MODE15_A::WIREDANDDRIVEPULLUPFILTER
    }
}
#[doc = "Field `MODE15` writer - Pin 15 Mode"]
pub type MODE15_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PB_MODEH_SPEC, u8, MODE15_A, 4, O>;
impl<'a, const O: u8> MODE15_W<'a, O> {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE15_A::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE15_A::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE15_A::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE15_A::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE15_A::PUSHPULL)
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn pushpulldrive(self) -> &'a mut W {
        self.variant(MODE15_A::PUSHPULLDRIVE)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE15_A::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE15_A::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE15_A::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE15_A::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE15_A::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE15_A::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrive(self) -> &'a mut W {
        self.variant(MODE15_A::WIREDANDDRIVE)
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivefilter(self) -> &'a mut W {
        self.variant(MODE15_A::WIREDANDDRIVEFILTER)
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullup(self) -> &'a mut W {
        self.variant(MODE15_A::WIREDANDDRIVEPULLUP)
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline(always)]
    pub fn wiredanddrivepullupfilter(self) -> &'a mut W {
        self.variant(MODE15_A::WIREDANDDRIVEPULLUPFILTER)
    }
}
impl R {
    #[doc = "Bits 0:3 - Pin 8 Mode"]
    #[inline(always)]
    pub fn mode8(&self) -> MODE8_R {
        MODE8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Pin 9 Mode"]
    #[inline(always)]
    pub fn mode9(&self) -> MODE9_R {
        MODE9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Pin 10 Mode"]
    #[inline(always)]
    pub fn mode10(&self) -> MODE10_R {
        MODE10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Pin 11 Mode"]
    #[inline(always)]
    pub fn mode11(&self) -> MODE11_R {
        MODE11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Pin 12 Mode"]
    #[inline(always)]
    pub fn mode12(&self) -> MODE12_R {
        MODE12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Pin 13 Mode"]
    #[inline(always)]
    pub fn mode13(&self) -> MODE13_R {
        MODE13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Pin 14 Mode"]
    #[inline(always)]
    pub fn mode14(&self) -> MODE14_R {
        MODE14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Pin 15 Mode"]
    #[inline(always)]
    pub fn mode15(&self) -> MODE15_R {
        MODE15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Pin 8 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode8(&mut self) -> MODE8_W<0> {
        MODE8_W::new(self)
    }
    #[doc = "Bits 4:7 - Pin 9 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode9(&mut self) -> MODE9_W<4> {
        MODE9_W::new(self)
    }
    #[doc = "Bits 8:11 - Pin 10 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode10(&mut self) -> MODE10_W<8> {
        MODE10_W::new(self)
    }
    #[doc = "Bits 12:15 - Pin 11 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode11(&mut self) -> MODE11_W<12> {
        MODE11_W::new(self)
    }
    #[doc = "Bits 16:19 - Pin 12 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode12(&mut self) -> MODE12_W<16> {
        MODE12_W::new(self)
    }
    #[doc = "Bits 20:23 - Pin 13 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode13(&mut self) -> MODE13_W<20> {
        MODE13_W::new(self)
    }
    #[doc = "Bits 24:27 - Pin 14 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode14(&mut self) -> MODE14_W<24> {
        MODE14_W::new(self)
    }
    #[doc = "Bits 28:31 - Pin 15 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode15(&mut self) -> MODE15_W<28> {
        MODE15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Pin Mode High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pb_modeh](index.html) module"]
pub struct PB_MODEH_SPEC;
impl crate::RegisterSpec for PB_MODEH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pb_modeh::R](R) reader structure"]
impl crate::Readable for PB_MODEH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pb_modeh::W](W) writer structure"]
impl crate::Writable for PB_MODEH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PB_MODEH to value 0"]
impl crate::Resettable for PB_MODEH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
