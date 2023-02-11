#[doc = "Register `IRCTRL` reader"]
pub struct R(crate::R<IRCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRCTRL` writer"]
pub struct W(crate::W<IRCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRCTRL_SPEC>;
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
impl From<crate::W<IRCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IREN` reader - Enable IrDA Module"]
pub type IREN_R = crate::BitReader<bool>;
#[doc = "Field `IREN` writer - Enable IrDA Module"]
pub type IREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRCTRL_SPEC, bool, O>;
#[doc = "Field `IRPW` reader - IrDA TX Pulse Width"]
pub type IRPW_R = crate::FieldReader<u8, IRPW_A>;
#[doc = "IrDA TX Pulse Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IRPW_A {
    #[doc = "0: IrDA pulse width is 1/16 for OVS=0 and 1/8 for OVS=1"]
    ONE = 0,
    #[doc = "1: IrDA pulse width is 2/16 for OVS=0 and 2/8 for OVS=1"]
    TWO = 1,
    #[doc = "2: IrDA pulse width is 3/16 for OVS=0 and 3/8 for OVS=1"]
    THREE = 2,
    #[doc = "3: IrDA pulse width is 4/16 for OVS=0 and 4/8 for OVS=1"]
    FOUR = 3,
}
impl From<IRPW_A> for u8 {
    #[inline(always)]
    fn from(variant: IRPW_A) -> Self {
        variant as _
    }
}
impl IRPW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRPW_A {
        match self.bits {
            0 => IRPW_A::ONE,
            1 => IRPW_A::TWO,
            2 => IRPW_A::THREE,
            3 => IRPW_A::FOUR,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == IRPW_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == IRPW_A::TWO
    }
    #[doc = "Checks if the value of the field is `THREE`"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == IRPW_A::THREE
    }
    #[doc = "Checks if the value of the field is `FOUR`"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == IRPW_A::FOUR
    }
}
#[doc = "Field `IRPW` writer - IrDA TX Pulse Width"]
pub type IRPW_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, IRCTRL_SPEC, u8, IRPW_A, 2, O>;
impl<'a, const O: u8> IRPW_W<'a, O> {
    #[doc = "IrDA pulse width is 1/16 for OVS=0 and 1/8 for OVS=1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(IRPW_A::ONE)
    }
    #[doc = "IrDA pulse width is 2/16 for OVS=0 and 2/8 for OVS=1"]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(IRPW_A::TWO)
    }
    #[doc = "IrDA pulse width is 3/16 for OVS=0 and 3/8 for OVS=1"]
    #[inline(always)]
    pub fn three(self) -> &'a mut W {
        self.variant(IRPW_A::THREE)
    }
    #[doc = "IrDA pulse width is 4/16 for OVS=0 and 4/8 for OVS=1"]
    #[inline(always)]
    pub fn four(self) -> &'a mut W {
        self.variant(IRPW_A::FOUR)
    }
}
#[doc = "Field `IRFILT` reader - IrDA RX Filter"]
pub type IRFILT_R = crate::BitReader<bool>;
#[doc = "Field `IRFILT` writer - IrDA RX Filter"]
pub type IRFILT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRCTRL_SPEC, bool, O>;
#[doc = "Field `IRPRSSEL` reader - IrDA PRS Channel Select"]
pub type IRPRSSEL_R = crate::FieldReader<u8, IRPRSSEL_A>;
#[doc = "IrDA PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IRPRSSEL_A {
    #[doc = "0: PRS Channel 0 selected"]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected"]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected"]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected"]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected"]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected"]
    PRSCH5 = 5,
    #[doc = "6: PRS Channel 6 selected"]
    PRSCH6 = 6,
    #[doc = "7: PRS Channel 7 selected"]
    PRSCH7 = 7,
}
impl From<IRPRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: IRPRSSEL_A) -> Self {
        variant as _
    }
}
impl IRPRSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRPRSSEL_A {
        match self.bits {
            0 => IRPRSSEL_A::PRSCH0,
            1 => IRPRSSEL_A::PRSCH1,
            2 => IRPRSSEL_A::PRSCH2,
            3 => IRPRSSEL_A::PRSCH3,
            4 => IRPRSSEL_A::PRSCH4,
            5 => IRPRSSEL_A::PRSCH5,
            6 => IRPRSSEL_A::PRSCH6,
            7 => IRPRSSEL_A::PRSCH7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH7
    }
}
#[doc = "Field `IRPRSSEL` writer - IrDA PRS Channel Select"]
pub type IRPRSSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, IRCTRL_SPEC, u8, IRPRSSEL_A, 3, O>;
impl<'a, const O: u8> IRPRSSEL_W<'a, O> {
    #[doc = "PRS Channel 0 selected"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH7)
    }
}
#[doc = "Field `IRPRSEN` reader - IrDA PRS Channel Enable"]
pub type IRPRSEN_R = crate::BitReader<bool>;
#[doc = "Field `IRPRSEN` writer - IrDA PRS Channel Enable"]
pub type IRPRSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable IrDA Module"]
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - IrDA TX Pulse Width"]
    #[inline(always)]
    pub fn irpw(&self) -> IRPW_R {
        IRPW_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - IrDA RX Filter"]
    #[inline(always)]
    pub fn irfilt(&self) -> IRFILT_R {
        IRFILT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - IrDA PRS Channel Select"]
    #[inline(always)]
    pub fn irprssel(&self) -> IRPRSSEL_R {
        IRPRSSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - IrDA PRS Channel Enable"]
    #[inline(always)]
    pub fn irprsen(&self) -> IRPRSEN_R {
        IRPRSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable IrDA Module"]
    #[inline(always)]
    #[must_use]
    pub fn iren(&mut self) -> IREN_W<0> {
        IREN_W::new(self)
    }
    #[doc = "Bits 1:2 - IrDA TX Pulse Width"]
    #[inline(always)]
    #[must_use]
    pub fn irpw(&mut self) -> IRPW_W<1> {
        IRPW_W::new(self)
    }
    #[doc = "Bit 3 - IrDA RX Filter"]
    #[inline(always)]
    #[must_use]
    pub fn irfilt(&mut self) -> IRFILT_W<3> {
        IRFILT_W::new(self)
    }
    #[doc = "Bits 4:6 - IrDA PRS Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn irprssel(&mut self) -> IRPRSSEL_W<4> {
        IRPRSSEL_W::new(self)
    }
    #[doc = "Bit 7 - IrDA PRS Channel Enable"]
    #[inline(always)]
    #[must_use]
    pub fn irprsen(&mut self) -> IRPRSEN_W<7> {
        IRPRSEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IrDA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irctrl](index.html) module"]
pub struct IRCTRL_SPEC;
impl crate::RegisterSpec for IRCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irctrl::R](R) reader structure"]
impl crate::Readable for IRCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irctrl::W](W) writer structure"]
impl crate::Writable for IRCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRCTRL to value 0"]
impl crate::Resettable for IRCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
