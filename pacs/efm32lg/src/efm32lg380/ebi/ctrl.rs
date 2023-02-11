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
#[doc = "Field `MODE` reader - Mode"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D8A8 = 0,
    #[doc = "1: EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D16A16ALE = 1,
    #[doc = "2: EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D8A24ALE = 2,
    #[doc = "3: EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D16 = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::D8A8,
            1 => MODE_A::D16A16ALE,
            2 => MODE_A::D8A24ALE,
            3 => MODE_A::D16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `D8A8`"]
    #[inline(always)]
    pub fn is_d8a8(&self) -> bool {
        *self == MODE_A::D8A8
    }
    #[doc = "Checks if the value of the field is `D16A16ALE`"]
    #[inline(always)]
    pub fn is_d16a16ale(&self) -> bool {
        *self == MODE_A::D16A16ALE
    }
    #[doc = "Checks if the value of the field is `D8A24ALE`"]
    #[inline(always)]
    pub fn is_d8a24ale(&self) -> bool {
        *self == MODE_A::D8A24ALE
    }
    #[doc = "Checks if the value of the field is `D16`"]
    #[inline(always)]
    pub fn is_d16(&self) -> bool {
        *self == MODE_A::D16
    }
}
#[doc = "Field `MODE` writer - Mode"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, MODE_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    #[inline(always)]
    pub fn d8a8(self) -> &'a mut W {
        self.variant(MODE_A::D8A8)
    }
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    #[inline(always)]
    pub fn d16a16ale(self) -> &'a mut W {
        self.variant(MODE_A::D16A16ALE)
    }
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    #[inline(always)]
    pub fn d8a24ale(self) -> &'a mut W {
        self.variant(MODE_A::D8A24ALE)
    }
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    #[inline(always)]
    pub fn d16(self) -> &'a mut W {
        self.variant(MODE_A::D16)
    }
}
#[doc = "Field `MODE1` reader - Mode 1"]
pub type MODE1_R = crate::FieldReader<u8, MODE1_A>;
#[doc = "Mode 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE1_A {
    #[doc = "0: EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D8A8 = 0,
    #[doc = "1: EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D16A16ALE = 1,
    #[doc = "2: EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D8A24ALE = 2,
    #[doc = "3: EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D16 = 3,
}
impl From<MODE1_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE1_A) -> Self {
        variant as _
    }
}
impl MODE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE1_A {
        match self.bits {
            0 => MODE1_A::D8A8,
            1 => MODE1_A::D16A16ALE,
            2 => MODE1_A::D8A24ALE,
            3 => MODE1_A::D16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `D8A8`"]
    #[inline(always)]
    pub fn is_d8a8(&self) -> bool {
        *self == MODE1_A::D8A8
    }
    #[doc = "Checks if the value of the field is `D16A16ALE`"]
    #[inline(always)]
    pub fn is_d16a16ale(&self) -> bool {
        *self == MODE1_A::D16A16ALE
    }
    #[doc = "Checks if the value of the field is `D8A24ALE`"]
    #[inline(always)]
    pub fn is_d8a24ale(&self) -> bool {
        *self == MODE1_A::D8A24ALE
    }
    #[doc = "Checks if the value of the field is `D16`"]
    #[inline(always)]
    pub fn is_d16(&self) -> bool {
        *self == MODE1_A::D16
    }
}
#[doc = "Field `MODE1` writer - Mode 1"]
pub type MODE1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, MODE1_A, 2, O>;
impl<'a, const O: u8> MODE1_W<'a, O> {
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    #[inline(always)]
    pub fn d8a8(self) -> &'a mut W {
        self.variant(MODE1_A::D8A8)
    }
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    #[inline(always)]
    pub fn d16a16ale(self) -> &'a mut W {
        self.variant(MODE1_A::D16A16ALE)
    }
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    #[inline(always)]
    pub fn d8a24ale(self) -> &'a mut W {
        self.variant(MODE1_A::D8A24ALE)
    }
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    #[inline(always)]
    pub fn d16(self) -> &'a mut W {
        self.variant(MODE1_A::D16)
    }
}
#[doc = "Field `MODE2` reader - Mode 2"]
pub type MODE2_R = crate::FieldReader<u8, MODE2_A>;
#[doc = "Mode 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE2_A {
    #[doc = "0: EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D8A8 = 0,
    #[doc = "1: EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D16A16ALE = 1,
    #[doc = "2: EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D8A24ALE = 2,
    #[doc = "3: EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D16 = 3,
}
impl From<MODE2_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE2_A) -> Self {
        variant as _
    }
}
impl MODE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE2_A {
        match self.bits {
            0 => MODE2_A::D8A8,
            1 => MODE2_A::D16A16ALE,
            2 => MODE2_A::D8A24ALE,
            3 => MODE2_A::D16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `D8A8`"]
    #[inline(always)]
    pub fn is_d8a8(&self) -> bool {
        *self == MODE2_A::D8A8
    }
    #[doc = "Checks if the value of the field is `D16A16ALE`"]
    #[inline(always)]
    pub fn is_d16a16ale(&self) -> bool {
        *self == MODE2_A::D16A16ALE
    }
    #[doc = "Checks if the value of the field is `D8A24ALE`"]
    #[inline(always)]
    pub fn is_d8a24ale(&self) -> bool {
        *self == MODE2_A::D8A24ALE
    }
    #[doc = "Checks if the value of the field is `D16`"]
    #[inline(always)]
    pub fn is_d16(&self) -> bool {
        *self == MODE2_A::D16
    }
}
#[doc = "Field `MODE2` writer - Mode 2"]
pub type MODE2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, MODE2_A, 2, O>;
impl<'a, const O: u8> MODE2_W<'a, O> {
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    #[inline(always)]
    pub fn d8a8(self) -> &'a mut W {
        self.variant(MODE2_A::D8A8)
    }
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    #[inline(always)]
    pub fn d16a16ale(self) -> &'a mut W {
        self.variant(MODE2_A::D16A16ALE)
    }
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    #[inline(always)]
    pub fn d8a24ale(self) -> &'a mut W {
        self.variant(MODE2_A::D8A24ALE)
    }
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    #[inline(always)]
    pub fn d16(self) -> &'a mut W {
        self.variant(MODE2_A::D16)
    }
}
#[doc = "Field `MODE3` reader - Mode 3"]
pub type MODE3_R = crate::FieldReader<u8, MODE3_A>;
#[doc = "Mode 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE3_A {
    #[doc = "0: EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D8A8 = 0,
    #[doc = "1: EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D16A16ALE = 1,
    #[doc = "2: EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D8A24ALE = 2,
    #[doc = "3: EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D16 = 3,
}
impl From<MODE3_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE3_A) -> Self {
        variant as _
    }
}
impl MODE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE3_A {
        match self.bits {
            0 => MODE3_A::D8A8,
            1 => MODE3_A::D16A16ALE,
            2 => MODE3_A::D8A24ALE,
            3 => MODE3_A::D16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `D8A8`"]
    #[inline(always)]
    pub fn is_d8a8(&self) -> bool {
        *self == MODE3_A::D8A8
    }
    #[doc = "Checks if the value of the field is `D16A16ALE`"]
    #[inline(always)]
    pub fn is_d16a16ale(&self) -> bool {
        *self == MODE3_A::D16A16ALE
    }
    #[doc = "Checks if the value of the field is `D8A24ALE`"]
    #[inline(always)]
    pub fn is_d8a24ale(&self) -> bool {
        *self == MODE3_A::D8A24ALE
    }
    #[doc = "Checks if the value of the field is `D16`"]
    #[inline(always)]
    pub fn is_d16(&self) -> bool {
        *self == MODE3_A::D16
    }
}
#[doc = "Field `MODE3` writer - Mode 3"]
pub type MODE3_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, MODE3_A, 2, O>;
impl<'a, const O: u8> MODE3_W<'a, O> {
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    #[inline(always)]
    pub fn d8a8(self) -> &'a mut W {
        self.variant(MODE3_A::D8A8)
    }
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    #[inline(always)]
    pub fn d16a16ale(self) -> &'a mut W {
        self.variant(MODE3_A::D16A16ALE)
    }
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    #[inline(always)]
    pub fn d8a24ale(self) -> &'a mut W {
        self.variant(MODE3_A::D8A24ALE)
    }
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    #[inline(always)]
    pub fn d16(self) -> &'a mut W {
        self.variant(MODE3_A::D16)
    }
}
#[doc = "Field `BANK0EN` reader - Bank 0 Enable"]
pub type BANK0EN_R = crate::BitReader<bool>;
#[doc = "Field `BANK0EN` writer - Bank 0 Enable"]
pub type BANK0EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `BANK1EN` reader - Bank 1 Enable"]
pub type BANK1EN_R = crate::BitReader<bool>;
#[doc = "Field `BANK1EN` writer - Bank 1 Enable"]
pub type BANK1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `BANK2EN` reader - Bank 2 Enable"]
pub type BANK2EN_R = crate::BitReader<bool>;
#[doc = "Field `BANK2EN` writer - Bank 2 Enable"]
pub type BANK2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `BANK3EN` reader - Bank 3 Enable"]
pub type BANK3EN_R = crate::BitReader<bool>;
#[doc = "Field `BANK3EN` writer - Bank 3 Enable"]
pub type BANK3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `NOIDLE` reader - No idle cycle insertion on bank 0."]
pub type NOIDLE_R = crate::BitReader<bool>;
#[doc = "Field `NOIDLE` writer - No idle cycle insertion on bank 0."]
pub type NOIDLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `NOIDLE1` reader - No idle cycle insertion on bank 1."]
pub type NOIDLE1_R = crate::BitReader<bool>;
#[doc = "Field `NOIDLE1` writer - No idle cycle insertion on bank 1."]
pub type NOIDLE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `NOIDLE2` reader - No idle cycle insertion on bank 2."]
pub type NOIDLE2_R = crate::BitReader<bool>;
#[doc = "Field `NOIDLE2` writer - No idle cycle insertion on bank 2."]
pub type NOIDLE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `NOIDLE3` reader - No idle cycle insertion on bank 3."]
pub type NOIDLE3_R = crate::BitReader<bool>;
#[doc = "Field `NOIDLE3` writer - No idle cycle insertion on bank 3."]
pub type NOIDLE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ARDYEN` reader - ARDY Enable"]
pub type ARDYEN_R = crate::BitReader<bool>;
#[doc = "Field `ARDYEN` writer - ARDY Enable"]
pub type ARDYEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ARDYTODIS` reader - ARDY Timeout Disable"]
pub type ARDYTODIS_R = crate::BitReader<bool>;
#[doc = "Field `ARDYTODIS` writer - ARDY Timeout Disable"]
pub type ARDYTODIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ARDY1EN` reader - ARDY Enable for bank 1"]
pub type ARDY1EN_R = crate::BitReader<bool>;
#[doc = "Field `ARDY1EN` writer - ARDY Enable for bank 1"]
pub type ARDY1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ARDYTO1DIS` reader - ARDY Timeout Disable for bank 1"]
pub type ARDYTO1DIS_R = crate::BitReader<bool>;
#[doc = "Field `ARDYTO1DIS` writer - ARDY Timeout Disable for bank 1"]
pub type ARDYTO1DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ARDY2EN` reader - ARDY Enable for bank 2"]
pub type ARDY2EN_R = crate::BitReader<bool>;
#[doc = "Field `ARDY2EN` writer - ARDY Enable for bank 2"]
pub type ARDY2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ARDYTO2DIS` reader - ARDY Timeout Disable for bank 2"]
pub type ARDYTO2DIS_R = crate::BitReader<bool>;
#[doc = "Field `ARDYTO2DIS` writer - ARDY Timeout Disable for bank 2"]
pub type ARDYTO2DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ARDY3EN` reader - ARDY Enable for bank 3"]
pub type ARDY3EN_R = crate::BitReader<bool>;
#[doc = "Field `ARDY3EN` writer - ARDY Enable for bank 3"]
pub type ARDY3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ARDYTO3DIS` reader - ARDY Timeout Disable for bank 3"]
pub type ARDYTO3DIS_R = crate::BitReader<bool>;
#[doc = "Field `ARDYTO3DIS` writer - ARDY Timeout Disable for bank 3"]
pub type ARDYTO3DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `BL` reader - Byte Lane Enable for bank 0"]
pub type BL_R = crate::BitReader<bool>;
#[doc = "Field `BL` writer - Byte Lane Enable for bank 0"]
pub type BL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `BL1` reader - Byte Lane Enable for bank 1"]
pub type BL1_R = crate::BitReader<bool>;
#[doc = "Field `BL1` writer - Byte Lane Enable for bank 1"]
pub type BL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `BL2` reader - Byte Lane Enable for bank 2"]
pub type BL2_R = crate::BitReader<bool>;
#[doc = "Field `BL2` writer - Byte Lane Enable for bank 2"]
pub type BL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `BL3` reader - Byte Lane Enable for bank 3"]
pub type BL3_R = crate::BitReader<bool>;
#[doc = "Field `BL3` writer - Byte Lane Enable for bank 3"]
pub type BL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ITS` reader - Individual Timing Set, Line Polarity and Mode Definition Enable"]
pub type ITS_R = crate::BitReader<bool>;
#[doc = "Field `ITS` writer - Individual Timing Set, Line Polarity and Mode Definition Enable"]
pub type ITS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ALTMAP` reader - Alternative Address Map Enable"]
pub type ALTMAP_R = crate::BitReader<bool>;
#[doc = "Field `ALTMAP` writer - Alternative Address Map Enable"]
pub type ALTMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Mode 1"]
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Mode 2"]
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Mode 3"]
    #[inline(always)]
    pub fn mode3(&self) -> MODE3_R {
        MODE3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Bank 0 Enable"]
    #[inline(always)]
    pub fn bank0en(&self) -> BANK0EN_R {
        BANK0EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Bank 1 Enable"]
    #[inline(always)]
    pub fn bank1en(&self) -> BANK1EN_R {
        BANK1EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bank 2 Enable"]
    #[inline(always)]
    pub fn bank2en(&self) -> BANK2EN_R {
        BANK2EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bank 3 Enable"]
    #[inline(always)]
    pub fn bank3en(&self) -> BANK3EN_R {
        BANK3EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - No idle cycle insertion on bank 0."]
    #[inline(always)]
    pub fn noidle(&self) -> NOIDLE_R {
        NOIDLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - No idle cycle insertion on bank 1."]
    #[inline(always)]
    pub fn noidle1(&self) -> NOIDLE1_R {
        NOIDLE1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - No idle cycle insertion on bank 2."]
    #[inline(always)]
    pub fn noidle2(&self) -> NOIDLE2_R {
        NOIDLE2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - No idle cycle insertion on bank 3."]
    #[inline(always)]
    pub fn noidle3(&self) -> NOIDLE3_R {
        NOIDLE3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ARDY Enable"]
    #[inline(always)]
    pub fn ardyen(&self) -> ARDYEN_R {
        ARDYEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ARDY Timeout Disable"]
    #[inline(always)]
    pub fn ardytodis(&self) -> ARDYTODIS_R {
        ARDYTODIS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ARDY Enable for bank 1"]
    #[inline(always)]
    pub fn ardy1en(&self) -> ARDY1EN_R {
        ARDY1EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ARDY Timeout Disable for bank 1"]
    #[inline(always)]
    pub fn ardyto1dis(&self) -> ARDYTO1DIS_R {
        ARDYTO1DIS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ARDY Enable for bank 2"]
    #[inline(always)]
    pub fn ardy2en(&self) -> ARDY2EN_R {
        ARDY2EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ARDY Timeout Disable for bank 2"]
    #[inline(always)]
    pub fn ardyto2dis(&self) -> ARDYTO2DIS_R {
        ARDYTO2DIS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ARDY Enable for bank 3"]
    #[inline(always)]
    pub fn ardy3en(&self) -> ARDY3EN_R {
        ARDY3EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ARDY Timeout Disable for bank 3"]
    #[inline(always)]
    pub fn ardyto3dis(&self) -> ARDYTO3DIS_R {
        ARDYTO3DIS_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Byte Lane Enable for bank 0"]
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Byte Lane Enable for bank 1"]
    #[inline(always)]
    pub fn bl1(&self) -> BL1_R {
        BL1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Byte Lane Enable for bank 2"]
    #[inline(always)]
    pub fn bl2(&self) -> BL2_R {
        BL2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Byte Lane Enable for bank 3"]
    #[inline(always)]
    pub fn bl3(&self) -> BL3_R {
        BL3_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - Individual Timing Set, Line Polarity and Mode Definition Enable"]
    #[inline(always)]
    pub fn its(&self) -> ITS_R {
        ITS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Alternative Address Map Enable"]
    #[inline(always)]
    pub fn altmap(&self) -> ALTMAP_R {
        ALTMAP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bits 2:3 - Mode 1"]
    #[inline(always)]
    #[must_use]
    pub fn mode1(&mut self) -> MODE1_W<2> {
        MODE1_W::new(self)
    }
    #[doc = "Bits 4:5 - Mode 2"]
    #[inline(always)]
    #[must_use]
    pub fn mode2(&mut self) -> MODE2_W<4> {
        MODE2_W::new(self)
    }
    #[doc = "Bits 6:7 - Mode 3"]
    #[inline(always)]
    #[must_use]
    pub fn mode3(&mut self) -> MODE3_W<6> {
        MODE3_W::new(self)
    }
    #[doc = "Bit 8 - Bank 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bank0en(&mut self) -> BANK0EN_W<8> {
        BANK0EN_W::new(self)
    }
    #[doc = "Bit 9 - Bank 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bank1en(&mut self) -> BANK1EN_W<9> {
        BANK1EN_W::new(self)
    }
    #[doc = "Bit 10 - Bank 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bank2en(&mut self) -> BANK2EN_W<10> {
        BANK2EN_W::new(self)
    }
    #[doc = "Bit 11 - Bank 3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bank3en(&mut self) -> BANK3EN_W<11> {
        BANK3EN_W::new(self)
    }
    #[doc = "Bit 12 - No idle cycle insertion on bank 0."]
    #[inline(always)]
    #[must_use]
    pub fn noidle(&mut self) -> NOIDLE_W<12> {
        NOIDLE_W::new(self)
    }
    #[doc = "Bit 13 - No idle cycle insertion on bank 1."]
    #[inline(always)]
    #[must_use]
    pub fn noidle1(&mut self) -> NOIDLE1_W<13> {
        NOIDLE1_W::new(self)
    }
    #[doc = "Bit 14 - No idle cycle insertion on bank 2."]
    #[inline(always)]
    #[must_use]
    pub fn noidle2(&mut self) -> NOIDLE2_W<14> {
        NOIDLE2_W::new(self)
    }
    #[doc = "Bit 15 - No idle cycle insertion on bank 3."]
    #[inline(always)]
    #[must_use]
    pub fn noidle3(&mut self) -> NOIDLE3_W<15> {
        NOIDLE3_W::new(self)
    }
    #[doc = "Bit 16 - ARDY Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ardyen(&mut self) -> ARDYEN_W<16> {
        ARDYEN_W::new(self)
    }
    #[doc = "Bit 17 - ARDY Timeout Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ardytodis(&mut self) -> ARDYTODIS_W<17> {
        ARDYTODIS_W::new(self)
    }
    #[doc = "Bit 18 - ARDY Enable for bank 1"]
    #[inline(always)]
    #[must_use]
    pub fn ardy1en(&mut self) -> ARDY1EN_W<18> {
        ARDY1EN_W::new(self)
    }
    #[doc = "Bit 19 - ARDY Timeout Disable for bank 1"]
    #[inline(always)]
    #[must_use]
    pub fn ardyto1dis(&mut self) -> ARDYTO1DIS_W<19> {
        ARDYTO1DIS_W::new(self)
    }
    #[doc = "Bit 20 - ARDY Enable for bank 2"]
    #[inline(always)]
    #[must_use]
    pub fn ardy2en(&mut self) -> ARDY2EN_W<20> {
        ARDY2EN_W::new(self)
    }
    #[doc = "Bit 21 - ARDY Timeout Disable for bank 2"]
    #[inline(always)]
    #[must_use]
    pub fn ardyto2dis(&mut self) -> ARDYTO2DIS_W<21> {
        ARDYTO2DIS_W::new(self)
    }
    #[doc = "Bit 22 - ARDY Enable for bank 3"]
    #[inline(always)]
    #[must_use]
    pub fn ardy3en(&mut self) -> ARDY3EN_W<22> {
        ARDY3EN_W::new(self)
    }
    #[doc = "Bit 23 - ARDY Timeout Disable for bank 3"]
    #[inline(always)]
    #[must_use]
    pub fn ardyto3dis(&mut self) -> ARDYTO3DIS_W<23> {
        ARDYTO3DIS_W::new(self)
    }
    #[doc = "Bit 24 - Byte Lane Enable for bank 0"]
    #[inline(always)]
    #[must_use]
    pub fn bl(&mut self) -> BL_W<24> {
        BL_W::new(self)
    }
    #[doc = "Bit 25 - Byte Lane Enable for bank 1"]
    #[inline(always)]
    #[must_use]
    pub fn bl1(&mut self) -> BL1_W<25> {
        BL1_W::new(self)
    }
    #[doc = "Bit 26 - Byte Lane Enable for bank 2"]
    #[inline(always)]
    #[must_use]
    pub fn bl2(&mut self) -> BL2_W<26> {
        BL2_W::new(self)
    }
    #[doc = "Bit 27 - Byte Lane Enable for bank 3"]
    #[inline(always)]
    #[must_use]
    pub fn bl3(&mut self) -> BL3_W<27> {
        BL3_W::new(self)
    }
    #[doc = "Bit 30 - Individual Timing Set, Line Polarity and Mode Definition Enable"]
    #[inline(always)]
    #[must_use]
    pub fn its(&mut self) -> ITS_W<30> {
        ITS_W::new(self)
    }
    #[doc = "Bit 31 - Alternative Address Map Enable"]
    #[inline(always)]
    #[must_use]
    pub fn altmap(&mut self) -> ALTMAP_W<31> {
        ALTMAP_W::new(self)
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
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
