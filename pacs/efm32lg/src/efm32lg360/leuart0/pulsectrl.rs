#[doc = "Register `PULSECTRL` reader"]
pub struct R(crate::R<PULSECTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PULSECTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PULSECTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PULSECTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PULSECTRL` writer"]
pub struct W(crate::W<PULSECTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PULSECTRL_SPEC>;
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
impl From<crate::W<PULSECTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PULSECTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PULSEW` reader - Pulse Width"]
pub type PULSEW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PULSEW` writer - Pulse Width"]
pub type PULSEW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PULSECTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `PULSEEN` reader - Pulse Generator/Extender Enable"]
pub type PULSEEN_R = crate::BitReader<bool>;
#[doc = "Field `PULSEEN` writer - Pulse Generator/Extender Enable"]
pub type PULSEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PULSECTRL_SPEC, bool, O>;
#[doc = "Field `PULSEFILT` reader - Pulse Filter"]
pub type PULSEFILT_R = crate::BitReader<bool>;
#[doc = "Field `PULSEFILT` writer - Pulse Filter"]
pub type PULSEFILT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PULSECTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Pulse Width"]
    #[inline(always)]
    pub fn pulsew(&self) -> PULSEW_R {
        PULSEW_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Pulse Generator/Extender Enable"]
    #[inline(always)]
    pub fn pulseen(&self) -> PULSEEN_R {
        PULSEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pulse Filter"]
    #[inline(always)]
    pub fn pulsefilt(&self) -> PULSEFILT_R {
        PULSEFILT_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Pulse Width"]
    #[inline(always)]
    #[must_use]
    pub fn pulsew(&mut self) -> PULSEW_W<0> {
        PULSEW_W::new(self)
    }
    #[doc = "Bit 4 - Pulse Generator/Extender Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pulseen(&mut self) -> PULSEEN_W<4> {
        PULSEEN_W::new(self)
    }
    #[doc = "Bit 5 - Pulse Filter"]
    #[inline(always)]
    #[must_use]
    pub fn pulsefilt(&mut self) -> PULSEFILT_W<5> {
        PULSEFILT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pulse Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pulsectrl](index.html) module"]
pub struct PULSECTRL_SPEC;
impl crate::RegisterSpec for PULSECTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pulsectrl::R](R) reader structure"]
impl crate::Readable for PULSECTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pulsectrl::W](W) writer structure"]
impl crate::Writable for PULSECTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PULSECTRL to value 0"]
impl crate::Resettable for PULSECTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
