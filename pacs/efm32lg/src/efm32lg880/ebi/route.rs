#[doc = "Register `ROUTE` reader"]
pub struct R(crate::R<ROUTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROUTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROUTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROUTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROUTE` writer"]
pub struct W(crate::W<ROUTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROUTE_SPEC>;
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
impl From<crate::W<ROUTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROUTE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EBIPEN` reader - EBI Pin Enable"]
pub type EBIPEN_R = crate::BitReader<bool>;
#[doc = "Field `EBIPEN` writer - EBI Pin Enable"]
pub type EBIPEN_W<'a> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, 0>;
#[doc = "Field `CS0PEN` reader - EBI_CS0 Pin Enable"]
pub type CS0PEN_R = crate::BitReader<bool>;
#[doc = "Field `CS0PEN` writer - EBI_CS0 Pin Enable"]
pub type CS0PEN_W<'a> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, 1>;
#[doc = "Field `CS1PEN` reader - EBI_CS1 Pin Enable"]
pub type CS1PEN_R = crate::BitReader<bool>;
#[doc = "Field `CS1PEN` writer - EBI_CS1 Pin Enable"]
pub type CS1PEN_W<'a> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, 2>;
#[doc = "Field `CS2PEN` reader - EBI_CS2 Pin Enable"]
pub type CS2PEN_R = crate::BitReader<bool>;
#[doc = "Field `CS2PEN` writer - EBI_CS2 Pin Enable"]
pub type CS2PEN_W<'a> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, 3>;
#[doc = "Field `CS3PEN` reader - EBI_CS3 Pin Enable"]
pub type CS3PEN_R = crate::BitReader<bool>;
#[doc = "Field `CS3PEN` writer - EBI_CS3 Pin Enable"]
pub type CS3PEN_W<'a> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, 4>;
#[doc = "Field `ALEPEN` reader - EBI_ALE Pin Enable"]
pub type ALEPEN_R = crate::BitReader<bool>;
#[doc = "Field `ALEPEN` writer - EBI_ALE Pin Enable"]
pub type ALEPEN_W<'a> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, 5>;
#[doc = "Field `ARDYPEN` reader - EBI_ARDY Pin Enable"]
pub type ARDYPEN_R = crate::BitReader<bool>;
#[doc = "Field `ARDYPEN` writer - EBI_ARDY Pin Enable"]
pub type ARDYPEN_W<'a> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, 6>;
#[doc = "Field `BLPEN` reader - EBI_BL\\[1:0\\]
Pin Enable"]
pub type BLPEN_R = crate::BitReader<bool>;
#[doc = "Field `BLPEN` writer - EBI_BL\\[1:0\\]
Pin Enable"]
pub type BLPEN_W<'a> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, 7>;
#[doc = "Field `NANDPEN` reader - NANDRE and NANDWE Pin Enable"]
pub type NANDPEN_R = crate::BitReader<bool>;
#[doc = "Field `NANDPEN` writer - NANDRE and NANDWE Pin Enable"]
pub type NANDPEN_W<'a> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, 12>;
#[doc = "Sets the lower bound for EBI_A enabling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ALB_A {
    #[doc = "0: Address lines from EBI_A\\[0\\]
and upwards can be enabled via APEN."]
    A0 = 0,
    #[doc = "1: Address lines from EBI_A\\[8\\]
and upwards can be enabled via APEN."]
    A8 = 1,
    #[doc = "2: Address lines from EBI_A\\[16\\]
and upwards can be enabled via APEN."]
    A16 = 2,
    #[doc = "3: Address lines from EBI_A\\[24\\]
and upwards can be enabled via APEN."]
    A24 = 3,
}
impl From<ALB_A> for u8 {
    #[inline(always)]
    fn from(variant: ALB_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ALB` reader - Sets the lower bound for EBI_A enabling"]
pub type ALB_R = crate::FieldReader<u8, ALB_A>;
impl ALB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALB_A {
        match self.bits {
            0 => ALB_A::A0,
            1 => ALB_A::A8,
            2 => ALB_A::A16,
            3 => ALB_A::A24,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A0`"]
    #[inline(always)]
    pub fn is_a0(&self) -> bool {
        *self == ALB_A::A0
    }
    #[doc = "Checks if the value of the field is `A8`"]
    #[inline(always)]
    pub fn is_a8(&self) -> bool {
        *self == ALB_A::A8
    }
    #[doc = "Checks if the value of the field is `A16`"]
    #[inline(always)]
    pub fn is_a16(&self) -> bool {
        *self == ALB_A::A16
    }
    #[doc = "Checks if the value of the field is `A24`"]
    #[inline(always)]
    pub fn is_a24(&self) -> bool {
        *self == ALB_A::A24
    }
}
#[doc = "Field `ALB` writer - Sets the lower bound for EBI_A enabling"]
pub type ALB_W<'a> = crate::FieldWriterSafe<'a, u32, ROUTE_SPEC, u8, ALB_A, 2, 16>;
impl<'a> ALB_W<'a> {
    #[doc = "Address lines from EBI_A\\[0\\]
and upwards can be enabled via APEN."]
    #[inline(always)]
    pub fn a0(self) -> &'a mut W {
        self.variant(ALB_A::A0)
    }
    #[doc = "Address lines from EBI_A\\[8\\]
and upwards can be enabled via APEN."]
    #[inline(always)]
    pub fn a8(self) -> &'a mut W {
        self.variant(ALB_A::A8)
    }
    #[doc = "Address lines from EBI_A\\[16\\]
and upwards can be enabled via APEN."]
    #[inline(always)]
    pub fn a16(self) -> &'a mut W {
        self.variant(ALB_A::A16)
    }
    #[doc = "Address lines from EBI_A\\[24\\]
and upwards can be enabled via APEN."]
    #[inline(always)]
    pub fn a24(self) -> &'a mut W {
        self.variant(ALB_A::A24)
    }
}
#[doc = "EBI_A Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum APEN_A {
    #[doc = "0: All EBI_A pins are disabled."]
    A0 = 0,
    #[doc = "5: EBI_A\\[4:L\\]
pins enabled."]
    A5 = 5,
    #[doc = "6: EBI_A\\[5:L\\]
pins enabled."]
    A6 = 6,
    #[doc = "7: EBI_A\\[6:L\\]
pins enabled."]
    A7 = 7,
    #[doc = "8: EBI_A\\[7:L\\]
pins enabled."]
    A8 = 8,
    #[doc = "9: EBI_A\\[8:L\\]
pins enabled."]
    A9 = 9,
    #[doc = "10: EBI_A\\[9:L\\]
pins enabled."]
    A10 = 10,
    #[doc = "11: EBI_A\\[10:L\\]
pins enabled."]
    A11 = 11,
    #[doc = "12: EBI_A\\[11:L\\]
pins enabled."]
    A12 = 12,
    #[doc = "13: EBI_A\\[12:L\\]
pins enabled."]
    A13 = 13,
    #[doc = "14: EBI_A\\[13:L\\]
pins enabled."]
    A14 = 14,
    #[doc = "15: EBI_A\\[14:L\\]
pins enabled."]
    A15 = 15,
    #[doc = "16: EBI_A\\[15:L\\]
pins enabled."]
    A16 = 16,
    #[doc = "17: EBI_A\\[16:L\\]
pins enabled."]
    A17 = 17,
    #[doc = "18: EBI_A\\[17:L\\]
pins enabled."]
    A18 = 18,
    #[doc = "19: EBI_A\\[18:L\\]
pins enabled."]
    A19 = 19,
    #[doc = "20: EBI_A\\[19:L\\]
pins enabled."]
    A20 = 20,
    #[doc = "21: EBI_A\\[20:L\\]
pins enabled."]
    A21 = 21,
    #[doc = "22: EBI_A\\[21:L\\]
pins enabled."]
    A22 = 22,
    #[doc = "23: EBI_A\\[22:L\\]
pins enabled."]
    A23 = 23,
    #[doc = "24: EBI_A\\[23:L\\]
pins enabled."]
    A24 = 24,
    #[doc = "25: EBI_A\\[24:L\\]
pins enabled."]
    A25 = 25,
    #[doc = "26: EBI_A\\[25:L\\]
pins enabled."]
    A26 = 26,
    #[doc = "27: EBI_A\\[26:L\\]
pins enabled."]
    A27 = 27,
    #[doc = "28: EBI_A\\[27:L\\]
pins enabled."]
    A28 = 28,
}
impl From<APEN_A> for u8 {
    #[inline(always)]
    fn from(variant: APEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `APEN` reader - EBI_A Pin Enable"]
pub type APEN_R = crate::FieldReader<u8, APEN_A>;
impl APEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<APEN_A> {
        match self.bits {
            0 => Some(APEN_A::A0),
            5 => Some(APEN_A::A5),
            6 => Some(APEN_A::A6),
            7 => Some(APEN_A::A7),
            8 => Some(APEN_A::A8),
            9 => Some(APEN_A::A9),
            10 => Some(APEN_A::A10),
            11 => Some(APEN_A::A11),
            12 => Some(APEN_A::A12),
            13 => Some(APEN_A::A13),
            14 => Some(APEN_A::A14),
            15 => Some(APEN_A::A15),
            16 => Some(APEN_A::A16),
            17 => Some(APEN_A::A17),
            18 => Some(APEN_A::A18),
            19 => Some(APEN_A::A19),
            20 => Some(APEN_A::A20),
            21 => Some(APEN_A::A21),
            22 => Some(APEN_A::A22),
            23 => Some(APEN_A::A23),
            24 => Some(APEN_A::A24),
            25 => Some(APEN_A::A25),
            26 => Some(APEN_A::A26),
            27 => Some(APEN_A::A27),
            28 => Some(APEN_A::A28),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `A0`"]
    #[inline(always)]
    pub fn is_a0(&self) -> bool {
        *self == APEN_A::A0
    }
    #[doc = "Checks if the value of the field is `A5`"]
    #[inline(always)]
    pub fn is_a5(&self) -> bool {
        *self == APEN_A::A5
    }
    #[doc = "Checks if the value of the field is `A6`"]
    #[inline(always)]
    pub fn is_a6(&self) -> bool {
        *self == APEN_A::A6
    }
    #[doc = "Checks if the value of the field is `A7`"]
    #[inline(always)]
    pub fn is_a7(&self) -> bool {
        *self == APEN_A::A7
    }
    #[doc = "Checks if the value of the field is `A8`"]
    #[inline(always)]
    pub fn is_a8(&self) -> bool {
        *self == APEN_A::A8
    }
    #[doc = "Checks if the value of the field is `A9`"]
    #[inline(always)]
    pub fn is_a9(&self) -> bool {
        *self == APEN_A::A9
    }
    #[doc = "Checks if the value of the field is `A10`"]
    #[inline(always)]
    pub fn is_a10(&self) -> bool {
        *self == APEN_A::A10
    }
    #[doc = "Checks if the value of the field is `A11`"]
    #[inline(always)]
    pub fn is_a11(&self) -> bool {
        *self == APEN_A::A11
    }
    #[doc = "Checks if the value of the field is `A12`"]
    #[inline(always)]
    pub fn is_a12(&self) -> bool {
        *self == APEN_A::A12
    }
    #[doc = "Checks if the value of the field is `A13`"]
    #[inline(always)]
    pub fn is_a13(&self) -> bool {
        *self == APEN_A::A13
    }
    #[doc = "Checks if the value of the field is `A14`"]
    #[inline(always)]
    pub fn is_a14(&self) -> bool {
        *self == APEN_A::A14
    }
    #[doc = "Checks if the value of the field is `A15`"]
    #[inline(always)]
    pub fn is_a15(&self) -> bool {
        *self == APEN_A::A15
    }
    #[doc = "Checks if the value of the field is `A16`"]
    #[inline(always)]
    pub fn is_a16(&self) -> bool {
        *self == APEN_A::A16
    }
    #[doc = "Checks if the value of the field is `A17`"]
    #[inline(always)]
    pub fn is_a17(&self) -> bool {
        *self == APEN_A::A17
    }
    #[doc = "Checks if the value of the field is `A18`"]
    #[inline(always)]
    pub fn is_a18(&self) -> bool {
        *self == APEN_A::A18
    }
    #[doc = "Checks if the value of the field is `A19`"]
    #[inline(always)]
    pub fn is_a19(&self) -> bool {
        *self == APEN_A::A19
    }
    #[doc = "Checks if the value of the field is `A20`"]
    #[inline(always)]
    pub fn is_a20(&self) -> bool {
        *self == APEN_A::A20
    }
    #[doc = "Checks if the value of the field is `A21`"]
    #[inline(always)]
    pub fn is_a21(&self) -> bool {
        *self == APEN_A::A21
    }
    #[doc = "Checks if the value of the field is `A22`"]
    #[inline(always)]
    pub fn is_a22(&self) -> bool {
        *self == APEN_A::A22
    }
    #[doc = "Checks if the value of the field is `A23`"]
    #[inline(always)]
    pub fn is_a23(&self) -> bool {
        *self == APEN_A::A23
    }
    #[doc = "Checks if the value of the field is `A24`"]
    #[inline(always)]
    pub fn is_a24(&self) -> bool {
        *self == APEN_A::A24
    }
    #[doc = "Checks if the value of the field is `A25`"]
    #[inline(always)]
    pub fn is_a25(&self) -> bool {
        *self == APEN_A::A25
    }
    #[doc = "Checks if the value of the field is `A26`"]
    #[inline(always)]
    pub fn is_a26(&self) -> bool {
        *self == APEN_A::A26
    }
    #[doc = "Checks if the value of the field is `A27`"]
    #[inline(always)]
    pub fn is_a27(&self) -> bool {
        *self == APEN_A::A27
    }
    #[doc = "Checks if the value of the field is `A28`"]
    #[inline(always)]
    pub fn is_a28(&self) -> bool {
        *self == APEN_A::A28
    }
}
#[doc = "Field `APEN` writer - EBI_A Pin Enable"]
pub type APEN_W<'a> = crate::FieldWriter<'a, u32, ROUTE_SPEC, u8, APEN_A, 5, 18>;
impl<'a> APEN_W<'a> {
    #[doc = "All EBI_A pins are disabled."]
    #[inline(always)]
    pub fn a0(self) -> &'a mut W {
        self.variant(APEN_A::A0)
    }
    #[doc = "EBI_A\\[4:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a5(self) -> &'a mut W {
        self.variant(APEN_A::A5)
    }
    #[doc = "EBI_A\\[5:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a6(self) -> &'a mut W {
        self.variant(APEN_A::A6)
    }
    #[doc = "EBI_A\\[6:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a7(self) -> &'a mut W {
        self.variant(APEN_A::A7)
    }
    #[doc = "EBI_A\\[7:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a8(self) -> &'a mut W {
        self.variant(APEN_A::A8)
    }
    #[doc = "EBI_A\\[8:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a9(self) -> &'a mut W {
        self.variant(APEN_A::A9)
    }
    #[doc = "EBI_A\\[9:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a10(self) -> &'a mut W {
        self.variant(APEN_A::A10)
    }
    #[doc = "EBI_A\\[10:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a11(self) -> &'a mut W {
        self.variant(APEN_A::A11)
    }
    #[doc = "EBI_A\\[11:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a12(self) -> &'a mut W {
        self.variant(APEN_A::A12)
    }
    #[doc = "EBI_A\\[12:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a13(self) -> &'a mut W {
        self.variant(APEN_A::A13)
    }
    #[doc = "EBI_A\\[13:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a14(self) -> &'a mut W {
        self.variant(APEN_A::A14)
    }
    #[doc = "EBI_A\\[14:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a15(self) -> &'a mut W {
        self.variant(APEN_A::A15)
    }
    #[doc = "EBI_A\\[15:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a16(self) -> &'a mut W {
        self.variant(APEN_A::A16)
    }
    #[doc = "EBI_A\\[16:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a17(self) -> &'a mut W {
        self.variant(APEN_A::A17)
    }
    #[doc = "EBI_A\\[17:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a18(self) -> &'a mut W {
        self.variant(APEN_A::A18)
    }
    #[doc = "EBI_A\\[18:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a19(self) -> &'a mut W {
        self.variant(APEN_A::A19)
    }
    #[doc = "EBI_A\\[19:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a20(self) -> &'a mut W {
        self.variant(APEN_A::A20)
    }
    #[doc = "EBI_A\\[20:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a21(self) -> &'a mut W {
        self.variant(APEN_A::A21)
    }
    #[doc = "EBI_A\\[21:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a22(self) -> &'a mut W {
        self.variant(APEN_A::A22)
    }
    #[doc = "EBI_A\\[22:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a23(self) -> &'a mut W {
        self.variant(APEN_A::A23)
    }
    #[doc = "EBI_A\\[23:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a24(self) -> &'a mut W {
        self.variant(APEN_A::A24)
    }
    #[doc = "EBI_A\\[24:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a25(self) -> &'a mut W {
        self.variant(APEN_A::A25)
    }
    #[doc = "EBI_A\\[25:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a26(self) -> &'a mut W {
        self.variant(APEN_A::A26)
    }
    #[doc = "EBI_A\\[26:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a27(self) -> &'a mut W {
        self.variant(APEN_A::A27)
    }
    #[doc = "EBI_A\\[27:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a28(self) -> &'a mut W {
        self.variant(APEN_A::A28)
    }
}
#[doc = "Field `TFTPEN` reader - EBI_TFT Pin Enable"]
pub type TFTPEN_R = crate::BitReader<bool>;
#[doc = "Field `TFTPEN` writer - EBI_TFT Pin Enable"]
pub type TFTPEN_W<'a> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, 24>;
#[doc = "Field `DATAENPEN` reader - EBI_TFT Pin Enable"]
pub type DATAENPEN_R = crate::BitReader<bool>;
#[doc = "Field `DATAENPEN` writer - EBI_TFT Pin Enable"]
pub type DATAENPEN_W<'a> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, 25>;
#[doc = "Field `CSTFTPEN` reader - EBI_CSTFT Pin Enable"]
pub type CSTFTPEN_R = crate::BitReader<bool>;
#[doc = "Field `CSTFTPEN` writer - EBI_CSTFT Pin Enable"]
pub type CSTFTPEN_W<'a> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, 26>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LOCATION_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<LOCATION_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCATION_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LOCATION` reader - I/O Location"]
pub type LOCATION_R = crate::FieldReader<u8, LOCATION_A>;
impl LOCATION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOCATION_A> {
        match self.bits {
            0 => Some(LOCATION_A::LOC0),
            1 => Some(LOCATION_A::LOC1),
            2 => Some(LOCATION_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == LOCATION_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == LOCATION_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == LOCATION_A::LOC2
    }
}
#[doc = "Field `LOCATION` writer - I/O Location"]
pub type LOCATION_W<'a> = crate::FieldWriter<'a, u32, ROUTE_SPEC, u8, LOCATION_A, 3, 28>;
impl<'a> LOCATION_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(LOCATION_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(LOCATION_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(LOCATION_A::LOC2)
    }
}
impl R {
    #[doc = "Bit 0 - EBI Pin Enable"]
    #[inline(always)]
    pub fn ebipen(&self) -> EBIPEN_R {
        EBIPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EBI_CS0 Pin Enable"]
    #[inline(always)]
    pub fn cs0pen(&self) -> CS0PEN_R {
        CS0PEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EBI_CS1 Pin Enable"]
    #[inline(always)]
    pub fn cs1pen(&self) -> CS1PEN_R {
        CS1PEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EBI_CS2 Pin Enable"]
    #[inline(always)]
    pub fn cs2pen(&self) -> CS2PEN_R {
        CS2PEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EBI_CS3 Pin Enable"]
    #[inline(always)]
    pub fn cs3pen(&self) -> CS3PEN_R {
        CS3PEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EBI_ALE Pin Enable"]
    #[inline(always)]
    pub fn alepen(&self) -> ALEPEN_R {
        ALEPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EBI_ARDY Pin Enable"]
    #[inline(always)]
    pub fn ardypen(&self) -> ARDYPEN_R {
        ARDYPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EBI_BL\\[1:0\\]
Pin Enable"]
    #[inline(always)]
    pub fn blpen(&self) -> BLPEN_R {
        BLPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - NANDRE and NANDWE Pin Enable"]
    #[inline(always)]
    pub fn nandpen(&self) -> NANDPEN_R {
        NANDPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Sets the lower bound for EBI_A enabling"]
    #[inline(always)]
    pub fn alb(&self) -> ALB_R {
        ALB_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:22 - EBI_A Pin Enable"]
    #[inline(always)]
    pub fn apen(&self) -> APEN_R {
        APEN_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - EBI_TFT Pin Enable"]
    #[inline(always)]
    pub fn tftpen(&self) -> TFTPEN_R {
        TFTPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - EBI_TFT Pin Enable"]
    #[inline(always)]
    pub fn dataenpen(&self) -> DATAENPEN_R {
        DATAENPEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - EBI_CSTFT Pin Enable"]
    #[inline(always)]
    pub fn cstftpen(&self) -> CSTFTPEN_R {
        CSTFTPEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 28:30 - I/O Location"]
    #[inline(always)]
    pub fn location(&self) -> LOCATION_R {
        LOCATION_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - EBI Pin Enable"]
    #[inline(always)]
    pub fn ebipen(&mut self) -> EBIPEN_W {
        EBIPEN_W::new(self)
    }
    #[doc = "Bit 1 - EBI_CS0 Pin Enable"]
    #[inline(always)]
    pub fn cs0pen(&mut self) -> CS0PEN_W {
        CS0PEN_W::new(self)
    }
    #[doc = "Bit 2 - EBI_CS1 Pin Enable"]
    #[inline(always)]
    pub fn cs1pen(&mut self) -> CS1PEN_W {
        CS1PEN_W::new(self)
    }
    #[doc = "Bit 3 - EBI_CS2 Pin Enable"]
    #[inline(always)]
    pub fn cs2pen(&mut self) -> CS2PEN_W {
        CS2PEN_W::new(self)
    }
    #[doc = "Bit 4 - EBI_CS3 Pin Enable"]
    #[inline(always)]
    pub fn cs3pen(&mut self) -> CS3PEN_W {
        CS3PEN_W::new(self)
    }
    #[doc = "Bit 5 - EBI_ALE Pin Enable"]
    #[inline(always)]
    pub fn alepen(&mut self) -> ALEPEN_W {
        ALEPEN_W::new(self)
    }
    #[doc = "Bit 6 - EBI_ARDY Pin Enable"]
    #[inline(always)]
    pub fn ardypen(&mut self) -> ARDYPEN_W {
        ARDYPEN_W::new(self)
    }
    #[doc = "Bit 7 - EBI_BL\\[1:0\\]
Pin Enable"]
    #[inline(always)]
    pub fn blpen(&mut self) -> BLPEN_W {
        BLPEN_W::new(self)
    }
    #[doc = "Bit 12 - NANDRE and NANDWE Pin Enable"]
    #[inline(always)]
    pub fn nandpen(&mut self) -> NANDPEN_W {
        NANDPEN_W::new(self)
    }
    #[doc = "Bits 16:17 - Sets the lower bound for EBI_A enabling"]
    #[inline(always)]
    pub fn alb(&mut self) -> ALB_W {
        ALB_W::new(self)
    }
    #[doc = "Bits 18:22 - EBI_A Pin Enable"]
    #[inline(always)]
    pub fn apen(&mut self) -> APEN_W {
        APEN_W::new(self)
    }
    #[doc = "Bit 24 - EBI_TFT Pin Enable"]
    #[inline(always)]
    pub fn tftpen(&mut self) -> TFTPEN_W {
        TFTPEN_W::new(self)
    }
    #[doc = "Bit 25 - EBI_TFT Pin Enable"]
    #[inline(always)]
    pub fn dataenpen(&mut self) -> DATAENPEN_W {
        DATAENPEN_W::new(self)
    }
    #[doc = "Bit 26 - EBI_CSTFT Pin Enable"]
    #[inline(always)]
    pub fn cstftpen(&mut self) -> CSTFTPEN_W {
        CSTFTPEN_W::new(self)
    }
    #[doc = "Bits 28:30 - I/O Location"]
    #[inline(always)]
    pub fn location(&mut self) -> LOCATION_W {
        LOCATION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O Routing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [route](index.html) module"]
pub struct ROUTE_SPEC;
impl crate::RegisterSpec for ROUTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [route::R](R) reader structure"]
impl crate::Readable for ROUTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [route::W](W) writer structure"]
impl crate::Writable for ROUTE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROUTE to value 0"]
impl crate::Resettable for ROUTE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
