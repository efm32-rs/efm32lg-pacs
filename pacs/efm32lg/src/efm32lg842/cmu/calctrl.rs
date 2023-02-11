#[doc = "Register `CALCTRL` reader"]
pub struct R(crate::R<CALCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALCTRL` writer"]
pub struct W(crate::W<CALCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALCTRL_SPEC>;
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
impl From<crate::W<CALCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UPSEL` reader - Calibration Up-counter Select"]
pub type UPSEL_R = crate::FieldReader<u8, UPSEL_A>;
#[doc = "Calibration Up-counter Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UPSEL_A {
    #[doc = "0: Select HFXO as up-counter."]
    HFXO = 0,
    #[doc = "1: Select LFXO as up-counter."]
    LFXO = 1,
    #[doc = "2: Select HFRCO as up-counter."]
    HFRCO = 2,
    #[doc = "3: Select LFRCO as up-counter."]
    LFRCO = 3,
    #[doc = "4: Select AUXHFRCO as up-counter."]
    AUXHFRCO = 4,
}
impl From<UPSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: UPSEL_A) -> Self {
        variant as _
    }
}
impl UPSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UPSEL_A> {
        match self.bits {
            0 => Some(UPSEL_A::HFXO),
            1 => Some(UPSEL_A::LFXO),
            2 => Some(UPSEL_A::HFRCO),
            3 => Some(UPSEL_A::LFRCO),
            4 => Some(UPSEL_A::AUXHFRCO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == UPSEL_A::HFXO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == UPSEL_A::LFXO
    }
    #[doc = "Checks if the value of the field is `HFRCO`"]
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        *self == UPSEL_A::HFRCO
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == UPSEL_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `AUXHFRCO`"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == UPSEL_A::AUXHFRCO
    }
}
#[doc = "Field `UPSEL` writer - Calibration Up-counter Select"]
pub type UPSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CALCTRL_SPEC, u8, UPSEL_A, 3, O>;
impl<'a, const O: u8> UPSEL_W<'a, O> {
    #[doc = "Select HFXO as up-counter."]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(UPSEL_A::HFXO)
    }
    #[doc = "Select LFXO as up-counter."]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(UPSEL_A::LFXO)
    }
    #[doc = "Select HFRCO as up-counter."]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut W {
        self.variant(UPSEL_A::HFRCO)
    }
    #[doc = "Select LFRCO as up-counter."]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(UPSEL_A::LFRCO)
    }
    #[doc = "Select AUXHFRCO as up-counter."]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut W {
        self.variant(UPSEL_A::AUXHFRCO)
    }
}
#[doc = "Field `DOWNSEL` reader - Calibration Down-counter Select"]
pub type DOWNSEL_R = crate::FieldReader<u8, DOWNSEL_A>;
#[doc = "Calibration Down-counter Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DOWNSEL_A {
    #[doc = "0: Select HFCLK for down-counter."]
    HFCLK = 0,
    #[doc = "1: Select HFXO for down-counter."]
    HFXO = 1,
    #[doc = "2: Select LFXO for down-counter."]
    LFXO = 2,
    #[doc = "3: Select HFRCO for down-counter."]
    HFRCO = 3,
    #[doc = "4: Select LFRCO for down-counter."]
    LFRCO = 4,
    #[doc = "5: Select AUXHFRCO for down-counter."]
    AUXHFRCO = 5,
}
impl From<DOWNSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DOWNSEL_A) -> Self {
        variant as _
    }
}
impl DOWNSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DOWNSEL_A> {
        match self.bits {
            0 => Some(DOWNSEL_A::HFCLK),
            1 => Some(DOWNSEL_A::HFXO),
            2 => Some(DOWNSEL_A::LFXO),
            3 => Some(DOWNSEL_A::HFRCO),
            4 => Some(DOWNSEL_A::LFRCO),
            5 => Some(DOWNSEL_A::AUXHFRCO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HFCLK`"]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        *self == DOWNSEL_A::HFCLK
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == DOWNSEL_A::HFXO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == DOWNSEL_A::LFXO
    }
    #[doc = "Checks if the value of the field is `HFRCO`"]
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        *self == DOWNSEL_A::HFRCO
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == DOWNSEL_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `AUXHFRCO`"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == DOWNSEL_A::AUXHFRCO
    }
}
#[doc = "Field `DOWNSEL` writer - Calibration Down-counter Select"]
pub type DOWNSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CALCTRL_SPEC, u8, DOWNSEL_A, 3, O>;
impl<'a, const O: u8> DOWNSEL_W<'a, O> {
    #[doc = "Select HFCLK for down-counter."]
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut W {
        self.variant(DOWNSEL_A::HFCLK)
    }
    #[doc = "Select HFXO for down-counter."]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(DOWNSEL_A::HFXO)
    }
    #[doc = "Select LFXO for down-counter."]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(DOWNSEL_A::LFXO)
    }
    #[doc = "Select HFRCO for down-counter."]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut W {
        self.variant(DOWNSEL_A::HFRCO)
    }
    #[doc = "Select LFRCO for down-counter."]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(DOWNSEL_A::LFRCO)
    }
    #[doc = "Select AUXHFRCO for down-counter."]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut W {
        self.variant(DOWNSEL_A::AUXHFRCO)
    }
}
#[doc = "Field `CONT` reader - Continuous Calibration"]
pub type CONT_R = crate::BitReader<bool>;
#[doc = "Field `CONT` writer - Continuous Calibration"]
pub type CONT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CALCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Calibration Up-counter Select"]
    #[inline(always)]
    pub fn upsel(&self) -> UPSEL_R {
        UPSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Calibration Down-counter Select"]
    #[inline(always)]
    pub fn downsel(&self) -> DOWNSEL_R {
        DOWNSEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - Continuous Calibration"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Calibration Up-counter Select"]
    #[inline(always)]
    #[must_use]
    pub fn upsel(&mut self) -> UPSEL_W<0> {
        UPSEL_W::new(self)
    }
    #[doc = "Bits 3:5 - Calibration Down-counter Select"]
    #[inline(always)]
    #[must_use]
    pub fn downsel(&mut self) -> DOWNSEL_W<3> {
        DOWNSEL_W::new(self)
    }
    #[doc = "Bit 6 - Continuous Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> CONT_W<6> {
        CONT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calibration Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calctrl](index.html) module"]
pub struct CALCTRL_SPEC;
impl crate::RegisterSpec for CALCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [calctrl::R](R) reader structure"]
impl crate::Readable for CALCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [calctrl::W](W) writer structure"]
impl crate::Writable for CALCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CALCTRL to value 0"]
impl crate::Resettable for CALCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
