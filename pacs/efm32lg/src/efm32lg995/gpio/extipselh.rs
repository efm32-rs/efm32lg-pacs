#[doc = "Register `EXTIPSELH` reader"]
pub struct R(crate::R<EXTIPSELH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTIPSELH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTIPSELH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTIPSELH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTIPSELH` writer"]
pub struct W(crate::W<EXTIPSELH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTIPSELH_SPEC>;
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
impl From<crate::W<EXTIPSELH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTIPSELH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTIPSEL8` reader - External Interrupt 8 Port Select"]
pub type EXTIPSEL8_R = crate::FieldReader<u8, EXTIPSEL8_A>;
#[doc = "External Interrupt 8 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL8_A {
    #[doc = "0: Port A pin 8 selected for external interrupt 8"]
    PORTA = 0,
    #[doc = "1: Port B pin 8 selected for external interrupt 8"]
    PORTB = 1,
    #[doc = "2: Port C pin 8 selected for external interrupt 8"]
    PORTC = 2,
    #[doc = "3: Port D pin 8 selected for external interrupt 8"]
    PORTD = 3,
    #[doc = "4: Port E pin 8 selected for external interrupt 8"]
    PORTE = 4,
    #[doc = "5: Port F pin 8 selected for external interrupt 8"]
    PORTF = 5,
}
impl From<EXTIPSEL8_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL8_A) -> Self {
        variant as _
    }
}
impl EXTIPSEL8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTIPSEL8_A> {
        match self.bits {
            0 => Some(EXTIPSEL8_A::PORTA),
            1 => Some(EXTIPSEL8_A::PORTB),
            2 => Some(EXTIPSEL8_A::PORTC),
            3 => Some(EXTIPSEL8_A::PORTD),
            4 => Some(EXTIPSEL8_A::PORTE),
            5 => Some(EXTIPSEL8_A::PORTF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL8_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL8_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL8_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL8_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL8_A::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL8_A::PORTF
    }
}
#[doc = "Field `EXTIPSEL8` writer - External Interrupt 8 Port Select"]
pub type EXTIPSEL8_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EXTIPSELH_SPEC, u8, EXTIPSEL8_A, 3, O>;
impl<'a, const O: u8> EXTIPSEL8_W<'a, O> {
    #[doc = "Port A pin 8 selected for external interrupt 8"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL8_A::PORTA)
    }
    #[doc = "Port B pin 8 selected for external interrupt 8"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL8_A::PORTB)
    }
    #[doc = "Port C pin 8 selected for external interrupt 8"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL8_A::PORTC)
    }
    #[doc = "Port D pin 8 selected for external interrupt 8"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL8_A::PORTD)
    }
    #[doc = "Port E pin 8 selected for external interrupt 8"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL8_A::PORTE)
    }
    #[doc = "Port F pin 8 selected for external interrupt 8"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL8_A::PORTF)
    }
}
#[doc = "Field `EXTIPSEL9` reader - External Interrupt 9 Port Select"]
pub type EXTIPSEL9_R = crate::FieldReader<u8, EXTIPSEL9_A>;
#[doc = "External Interrupt 9 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL9_A {
    #[doc = "0: Port A pin 9 selected for external interrupt 9"]
    PORTA = 0,
    #[doc = "1: Port B pin 9 selected for external interrupt 9"]
    PORTB = 1,
    #[doc = "2: Port C pin 9 selected for external interrupt 9"]
    PORTC = 2,
    #[doc = "3: Port D pin 9 selected for external interrupt 9"]
    PORTD = 3,
    #[doc = "4: Port E pin 9 selected for external interrupt 9"]
    PORTE = 4,
    #[doc = "5: Port F pin 9 selected for external interrupt 9"]
    PORTF = 5,
}
impl From<EXTIPSEL9_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL9_A) -> Self {
        variant as _
    }
}
impl EXTIPSEL9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTIPSEL9_A> {
        match self.bits {
            0 => Some(EXTIPSEL9_A::PORTA),
            1 => Some(EXTIPSEL9_A::PORTB),
            2 => Some(EXTIPSEL9_A::PORTC),
            3 => Some(EXTIPSEL9_A::PORTD),
            4 => Some(EXTIPSEL9_A::PORTE),
            5 => Some(EXTIPSEL9_A::PORTF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL9_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL9_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL9_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL9_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL9_A::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL9_A::PORTF
    }
}
#[doc = "Field `EXTIPSEL9` writer - External Interrupt 9 Port Select"]
pub type EXTIPSEL9_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EXTIPSELH_SPEC, u8, EXTIPSEL9_A, 3, O>;
impl<'a, const O: u8> EXTIPSEL9_W<'a, O> {
    #[doc = "Port A pin 9 selected for external interrupt 9"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL9_A::PORTA)
    }
    #[doc = "Port B pin 9 selected for external interrupt 9"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL9_A::PORTB)
    }
    #[doc = "Port C pin 9 selected for external interrupt 9"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL9_A::PORTC)
    }
    #[doc = "Port D pin 9 selected for external interrupt 9"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL9_A::PORTD)
    }
    #[doc = "Port E pin 9 selected for external interrupt 9"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL9_A::PORTE)
    }
    #[doc = "Port F pin 9 selected for external interrupt 9"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL9_A::PORTF)
    }
}
#[doc = "Field `EXTIPSEL10` reader - External Interrupt 10 Port Select"]
pub type EXTIPSEL10_R = crate::FieldReader<u8, EXTIPSEL10_A>;
#[doc = "External Interrupt 10 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL10_A {
    #[doc = "0: Port A pin 10 selected for external interrupt 10"]
    PORTA = 0,
    #[doc = "1: Port B pin 10 selected for external interrupt 10"]
    PORTB = 1,
    #[doc = "2: Port C pin 10 selected for external interrupt 10"]
    PORTC = 2,
    #[doc = "3: Port D pin 10 selected for external interrupt 10"]
    PORTD = 3,
    #[doc = "4: Port E pin 10 selected for external interrupt 10"]
    PORTE = 4,
    #[doc = "5: Port F pin 10 selected for external interrupt 10"]
    PORTF = 5,
}
impl From<EXTIPSEL10_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL10_A) -> Self {
        variant as _
    }
}
impl EXTIPSEL10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTIPSEL10_A> {
        match self.bits {
            0 => Some(EXTIPSEL10_A::PORTA),
            1 => Some(EXTIPSEL10_A::PORTB),
            2 => Some(EXTIPSEL10_A::PORTC),
            3 => Some(EXTIPSEL10_A::PORTD),
            4 => Some(EXTIPSEL10_A::PORTE),
            5 => Some(EXTIPSEL10_A::PORTF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL10_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL10_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL10_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL10_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL10_A::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL10_A::PORTF
    }
}
#[doc = "Field `EXTIPSEL10` writer - External Interrupt 10 Port Select"]
pub type EXTIPSEL10_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EXTIPSELH_SPEC, u8, EXTIPSEL10_A, 3, O>;
impl<'a, const O: u8> EXTIPSEL10_W<'a, O> {
    #[doc = "Port A pin 10 selected for external interrupt 10"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL10_A::PORTA)
    }
    #[doc = "Port B pin 10 selected for external interrupt 10"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL10_A::PORTB)
    }
    #[doc = "Port C pin 10 selected for external interrupt 10"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL10_A::PORTC)
    }
    #[doc = "Port D pin 10 selected for external interrupt 10"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL10_A::PORTD)
    }
    #[doc = "Port E pin 10 selected for external interrupt 10"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL10_A::PORTE)
    }
    #[doc = "Port F pin 10 selected for external interrupt 10"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL10_A::PORTF)
    }
}
#[doc = "Field `EXTIPSEL11` reader - External Interrupt 11 Port Select"]
pub type EXTIPSEL11_R = crate::FieldReader<u8, EXTIPSEL11_A>;
#[doc = "External Interrupt 11 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL11_A {
    #[doc = "0: Port A pin 11 selected for external interrupt 11"]
    PORTA = 0,
    #[doc = "1: Port B pin 11 selected for external interrupt 11"]
    PORTB = 1,
    #[doc = "2: Port C pin 11 selected for external interrupt 11"]
    PORTC = 2,
    #[doc = "3: Port D pin 11 selected for external interrupt 11"]
    PORTD = 3,
    #[doc = "4: Port E pin 11 selected for external interrupt 11"]
    PORTE = 4,
    #[doc = "5: Port F pin 11 selected for external interrupt 11"]
    PORTF = 5,
}
impl From<EXTIPSEL11_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL11_A) -> Self {
        variant as _
    }
}
impl EXTIPSEL11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTIPSEL11_A> {
        match self.bits {
            0 => Some(EXTIPSEL11_A::PORTA),
            1 => Some(EXTIPSEL11_A::PORTB),
            2 => Some(EXTIPSEL11_A::PORTC),
            3 => Some(EXTIPSEL11_A::PORTD),
            4 => Some(EXTIPSEL11_A::PORTE),
            5 => Some(EXTIPSEL11_A::PORTF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL11_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL11_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL11_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL11_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL11_A::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL11_A::PORTF
    }
}
#[doc = "Field `EXTIPSEL11` writer - External Interrupt 11 Port Select"]
pub type EXTIPSEL11_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EXTIPSELH_SPEC, u8, EXTIPSEL11_A, 3, O>;
impl<'a, const O: u8> EXTIPSEL11_W<'a, O> {
    #[doc = "Port A pin 11 selected for external interrupt 11"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL11_A::PORTA)
    }
    #[doc = "Port B pin 11 selected for external interrupt 11"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL11_A::PORTB)
    }
    #[doc = "Port C pin 11 selected for external interrupt 11"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL11_A::PORTC)
    }
    #[doc = "Port D pin 11 selected for external interrupt 11"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL11_A::PORTD)
    }
    #[doc = "Port E pin 11 selected for external interrupt 11"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL11_A::PORTE)
    }
    #[doc = "Port F pin 11 selected for external interrupt 11"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL11_A::PORTF)
    }
}
#[doc = "Field `EXTIPSEL12` reader - External Interrupt 12 Port Select"]
pub type EXTIPSEL12_R = crate::FieldReader<u8, EXTIPSEL12_A>;
#[doc = "External Interrupt 12 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL12_A {
    #[doc = "0: Port A pin 12 selected for external interrupt 12"]
    PORTA = 0,
    #[doc = "1: Port B pin 12 selected for external interrupt 12"]
    PORTB = 1,
    #[doc = "2: Port C pin 12 selected for external interrupt 12"]
    PORTC = 2,
    #[doc = "3: Port D pin 12 selected for external interrupt 12"]
    PORTD = 3,
    #[doc = "4: Port E pin 12 selected for external interrupt 12"]
    PORTE = 4,
    #[doc = "5: Port F pin 12 selected for external interrupt 12"]
    PORTF = 5,
}
impl From<EXTIPSEL12_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL12_A) -> Self {
        variant as _
    }
}
impl EXTIPSEL12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTIPSEL12_A> {
        match self.bits {
            0 => Some(EXTIPSEL12_A::PORTA),
            1 => Some(EXTIPSEL12_A::PORTB),
            2 => Some(EXTIPSEL12_A::PORTC),
            3 => Some(EXTIPSEL12_A::PORTD),
            4 => Some(EXTIPSEL12_A::PORTE),
            5 => Some(EXTIPSEL12_A::PORTF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL12_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL12_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL12_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL12_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL12_A::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL12_A::PORTF
    }
}
#[doc = "Field `EXTIPSEL12` writer - External Interrupt 12 Port Select"]
pub type EXTIPSEL12_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EXTIPSELH_SPEC, u8, EXTIPSEL12_A, 3, O>;
impl<'a, const O: u8> EXTIPSEL12_W<'a, O> {
    #[doc = "Port A pin 12 selected for external interrupt 12"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL12_A::PORTA)
    }
    #[doc = "Port B pin 12 selected for external interrupt 12"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL12_A::PORTB)
    }
    #[doc = "Port C pin 12 selected for external interrupt 12"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL12_A::PORTC)
    }
    #[doc = "Port D pin 12 selected for external interrupt 12"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL12_A::PORTD)
    }
    #[doc = "Port E pin 12 selected for external interrupt 12"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL12_A::PORTE)
    }
    #[doc = "Port F pin 12 selected for external interrupt 12"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL12_A::PORTF)
    }
}
#[doc = "Field `EXTIPSEL13` reader - External Interrupt 13 Port Select"]
pub type EXTIPSEL13_R = crate::FieldReader<u8, EXTIPSEL13_A>;
#[doc = "External Interrupt 13 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL13_A {
    #[doc = "0: Port A pin 13 selected for external interrupt 13"]
    PORTA = 0,
    #[doc = "1: Port B pin 13 selected for external interrupt 13"]
    PORTB = 1,
    #[doc = "2: Port C pin 13 selected for external interrupt 13"]
    PORTC = 2,
    #[doc = "3: Port D pin 13 selected for external interrupt 13"]
    PORTD = 3,
    #[doc = "4: Port E pin 13 selected for external interrupt 13"]
    PORTE = 4,
    #[doc = "5: Port F pin 13 selected for external interrupt 13"]
    PORTF = 5,
}
impl From<EXTIPSEL13_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL13_A) -> Self {
        variant as _
    }
}
impl EXTIPSEL13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTIPSEL13_A> {
        match self.bits {
            0 => Some(EXTIPSEL13_A::PORTA),
            1 => Some(EXTIPSEL13_A::PORTB),
            2 => Some(EXTIPSEL13_A::PORTC),
            3 => Some(EXTIPSEL13_A::PORTD),
            4 => Some(EXTIPSEL13_A::PORTE),
            5 => Some(EXTIPSEL13_A::PORTF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL13_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL13_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL13_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL13_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL13_A::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL13_A::PORTF
    }
}
#[doc = "Field `EXTIPSEL13` writer - External Interrupt 13 Port Select"]
pub type EXTIPSEL13_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EXTIPSELH_SPEC, u8, EXTIPSEL13_A, 3, O>;
impl<'a, const O: u8> EXTIPSEL13_W<'a, O> {
    #[doc = "Port A pin 13 selected for external interrupt 13"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL13_A::PORTA)
    }
    #[doc = "Port B pin 13 selected for external interrupt 13"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL13_A::PORTB)
    }
    #[doc = "Port C pin 13 selected for external interrupt 13"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL13_A::PORTC)
    }
    #[doc = "Port D pin 13 selected for external interrupt 13"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL13_A::PORTD)
    }
    #[doc = "Port E pin 13 selected for external interrupt 13"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL13_A::PORTE)
    }
    #[doc = "Port F pin 13 selected for external interrupt 13"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL13_A::PORTF)
    }
}
#[doc = "Field `EXTIPSEL14` reader - External Interrupt 14 Port Select"]
pub type EXTIPSEL14_R = crate::FieldReader<u8, EXTIPSEL14_A>;
#[doc = "External Interrupt 14 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL14_A {
    #[doc = "0: Port A pin 14 selected for external interrupt 14"]
    PORTA = 0,
    #[doc = "1: Port B pin 14 selected for external interrupt 14"]
    PORTB = 1,
    #[doc = "2: Port C pin 14 selected for external interrupt 14"]
    PORTC = 2,
    #[doc = "3: Port D pin 14 selected for external interrupt 14"]
    PORTD = 3,
    #[doc = "4: Port E pin 14 selected for external interrupt 14"]
    PORTE = 4,
    #[doc = "5: Port F pin 14 selected for external interrupt 14"]
    PORTF = 5,
}
impl From<EXTIPSEL14_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL14_A) -> Self {
        variant as _
    }
}
impl EXTIPSEL14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTIPSEL14_A> {
        match self.bits {
            0 => Some(EXTIPSEL14_A::PORTA),
            1 => Some(EXTIPSEL14_A::PORTB),
            2 => Some(EXTIPSEL14_A::PORTC),
            3 => Some(EXTIPSEL14_A::PORTD),
            4 => Some(EXTIPSEL14_A::PORTE),
            5 => Some(EXTIPSEL14_A::PORTF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL14_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL14_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL14_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL14_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL14_A::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL14_A::PORTF
    }
}
#[doc = "Field `EXTIPSEL14` writer - External Interrupt 14 Port Select"]
pub type EXTIPSEL14_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EXTIPSELH_SPEC, u8, EXTIPSEL14_A, 3, O>;
impl<'a, const O: u8> EXTIPSEL14_W<'a, O> {
    #[doc = "Port A pin 14 selected for external interrupt 14"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL14_A::PORTA)
    }
    #[doc = "Port B pin 14 selected for external interrupt 14"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL14_A::PORTB)
    }
    #[doc = "Port C pin 14 selected for external interrupt 14"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL14_A::PORTC)
    }
    #[doc = "Port D pin 14 selected for external interrupt 14"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL14_A::PORTD)
    }
    #[doc = "Port E pin 14 selected for external interrupt 14"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL14_A::PORTE)
    }
    #[doc = "Port F pin 14 selected for external interrupt 14"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL14_A::PORTF)
    }
}
#[doc = "Field `EXTIPSEL15` reader - External Interrupt 15 Port Select"]
pub type EXTIPSEL15_R = crate::FieldReader<u8, EXTIPSEL15_A>;
#[doc = "External Interrupt 15 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL15_A {
    #[doc = "0: Port A pin 15 selected for external interrupt 15"]
    PORTA = 0,
    #[doc = "1: Port B pin 15 selected for external interrupt 15"]
    PORTB = 1,
    #[doc = "2: Port C pin 15 selected for external interrupt 15"]
    PORTC = 2,
    #[doc = "3: Port D pin 15 selected for external interrupt 15"]
    PORTD = 3,
    #[doc = "4: Port E pin 15 selected for external interrupt 15"]
    PORTE = 4,
    #[doc = "5: Port F pin 15 selected for external interrupt 15"]
    PORTF = 5,
}
impl From<EXTIPSEL15_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL15_A) -> Self {
        variant as _
    }
}
impl EXTIPSEL15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTIPSEL15_A> {
        match self.bits {
            0 => Some(EXTIPSEL15_A::PORTA),
            1 => Some(EXTIPSEL15_A::PORTB),
            2 => Some(EXTIPSEL15_A::PORTC),
            3 => Some(EXTIPSEL15_A::PORTD),
            4 => Some(EXTIPSEL15_A::PORTE),
            5 => Some(EXTIPSEL15_A::PORTF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL15_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL15_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL15_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL15_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL15_A::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL15_A::PORTF
    }
}
#[doc = "Field `EXTIPSEL15` writer - External Interrupt 15 Port Select"]
pub type EXTIPSEL15_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EXTIPSELH_SPEC, u8, EXTIPSEL15_A, 3, O>;
impl<'a, const O: u8> EXTIPSEL15_W<'a, O> {
    #[doc = "Port A pin 15 selected for external interrupt 15"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL15_A::PORTA)
    }
    #[doc = "Port B pin 15 selected for external interrupt 15"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL15_A::PORTB)
    }
    #[doc = "Port C pin 15 selected for external interrupt 15"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL15_A::PORTC)
    }
    #[doc = "Port D pin 15 selected for external interrupt 15"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL15_A::PORTD)
    }
    #[doc = "Port E pin 15 selected for external interrupt 15"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL15_A::PORTE)
    }
    #[doc = "Port F pin 15 selected for external interrupt 15"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL15_A::PORTF)
    }
}
impl R {
    #[doc = "Bits 0:2 - External Interrupt 8 Port Select"]
    #[inline(always)]
    pub fn extipsel8(&self) -> EXTIPSEL8_R {
        EXTIPSEL8_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - External Interrupt 9 Port Select"]
    #[inline(always)]
    pub fn extipsel9(&self) -> EXTIPSEL9_R {
        EXTIPSEL9_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - External Interrupt 10 Port Select"]
    #[inline(always)]
    pub fn extipsel10(&self) -> EXTIPSEL10_R {
        EXTIPSEL10_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - External Interrupt 11 Port Select"]
    #[inline(always)]
    pub fn extipsel11(&self) -> EXTIPSEL11_R {
        EXTIPSEL11_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - External Interrupt 12 Port Select"]
    #[inline(always)]
    pub fn extipsel12(&self) -> EXTIPSEL12_R {
        EXTIPSEL12_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - External Interrupt 13 Port Select"]
    #[inline(always)]
    pub fn extipsel13(&self) -> EXTIPSEL13_R {
        EXTIPSEL13_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - External Interrupt 14 Port Select"]
    #[inline(always)]
    pub fn extipsel14(&self) -> EXTIPSEL14_R {
        EXTIPSEL14_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - External Interrupt 15 Port Select"]
    #[inline(always)]
    pub fn extipsel15(&self) -> EXTIPSEL15_R {
        EXTIPSEL15_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - External Interrupt 8 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel8(&mut self) -> EXTIPSEL8_W<0> {
        EXTIPSEL8_W::new(self)
    }
    #[doc = "Bits 4:6 - External Interrupt 9 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel9(&mut self) -> EXTIPSEL9_W<4> {
        EXTIPSEL9_W::new(self)
    }
    #[doc = "Bits 8:10 - External Interrupt 10 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel10(&mut self) -> EXTIPSEL10_W<8> {
        EXTIPSEL10_W::new(self)
    }
    #[doc = "Bits 12:14 - External Interrupt 11 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel11(&mut self) -> EXTIPSEL11_W<12> {
        EXTIPSEL11_W::new(self)
    }
    #[doc = "Bits 16:18 - External Interrupt 12 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel12(&mut self) -> EXTIPSEL12_W<16> {
        EXTIPSEL12_W::new(self)
    }
    #[doc = "Bits 20:22 - External Interrupt 13 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel13(&mut self) -> EXTIPSEL13_W<20> {
        EXTIPSEL13_W::new(self)
    }
    #[doc = "Bits 24:26 - External Interrupt 14 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel14(&mut self) -> EXTIPSEL14_W<24> {
        EXTIPSEL14_W::new(self)
    }
    #[doc = "Bits 28:30 - External Interrupt 15 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel15(&mut self) -> EXTIPSEL15_W<28> {
        EXTIPSEL15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Interrupt Port Select High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extipselh](index.html) module"]
pub struct EXTIPSELH_SPEC;
impl crate::RegisterSpec for EXTIPSELH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extipselh::R](R) reader structure"]
impl crate::Readable for EXTIPSELH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extipselh::W](W) writer structure"]
impl crate::Writable for EXTIPSELH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTIPSELH to value 0"]
impl crate::Resettable for EXTIPSELH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
