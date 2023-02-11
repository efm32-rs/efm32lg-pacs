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
#[doc = "Field `EN` reader - LCD Enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - LCD Enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `UDCTRL` reader - Update Data Control"]
pub type UDCTRL_R = crate::FieldReader<u8, UDCTRL_A>;
#[doc = "Update Data Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UDCTRL_A {
    #[doc = "0: The data transfer is controlled by SW. Transfer is performed as soon as possible"]
    REGULAR = 0,
    #[doc = "1: The data transfer is done at the next event triggered by the Frame Counter"]
    FCEVENT = 1,
    #[doc = "2: The data transfer is done continuously at every LCD frame start"]
    FRAMESTART = 2,
}
impl From<UDCTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: UDCTRL_A) -> Self {
        variant as _
    }
}
impl UDCTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UDCTRL_A> {
        match self.bits {
            0 => Some(UDCTRL_A::REGULAR),
            1 => Some(UDCTRL_A::FCEVENT),
            2 => Some(UDCTRL_A::FRAMESTART),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `REGULAR`"]
    #[inline(always)]
    pub fn is_regular(&self) -> bool {
        *self == UDCTRL_A::REGULAR
    }
    #[doc = "Checks if the value of the field is `FCEVENT`"]
    #[inline(always)]
    pub fn is_fcevent(&self) -> bool {
        *self == UDCTRL_A::FCEVENT
    }
    #[doc = "Checks if the value of the field is `FRAMESTART`"]
    #[inline(always)]
    pub fn is_framestart(&self) -> bool {
        *self == UDCTRL_A::FRAMESTART
    }
}
#[doc = "Field `UDCTRL` writer - Update Data Control"]
pub type UDCTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, UDCTRL_A, 2, O>;
impl<'a, const O: u8> UDCTRL_W<'a, O> {
    #[doc = "The data transfer is controlled by SW. Transfer is performed as soon as possible"]
    #[inline(always)]
    pub fn regular(self) -> &'a mut W {
        self.variant(UDCTRL_A::REGULAR)
    }
    #[doc = "The data transfer is done at the next event triggered by the Frame Counter"]
    #[inline(always)]
    pub fn fcevent(self) -> &'a mut W {
        self.variant(UDCTRL_A::FCEVENT)
    }
    #[doc = "The data transfer is done continuously at every LCD frame start"]
    #[inline(always)]
    pub fn framestart(self) -> &'a mut W {
        self.variant(UDCTRL_A::FRAMESTART)
    }
}
#[doc = "Field `DSC` reader - Direct Segment Control"]
pub type DSC_R = crate::BitReader<bool>;
#[doc = "Field `DSC` writer - Direct Segment Control"]
pub type DSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - LCD Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Update Data Control"]
    #[inline(always)]
    pub fn udctrl(&self) -> UDCTRL_R {
        UDCTRL_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 23 - Direct Segment Control"]
    #[inline(always)]
    pub fn dsc(&self) -> DSC_R {
        DSC_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bits 1:2 - Update Data Control"]
    #[inline(always)]
    #[must_use]
    pub fn udctrl(&mut self) -> UDCTRL_W<1> {
        UDCTRL_W::new(self)
    }
    #[doc = "Bit 23 - Direct Segment Control"]
    #[inline(always)]
    #[must_use]
    pub fn dsc(&mut self) -> DSC_W<23> {
        DSC_W::new(self)
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
