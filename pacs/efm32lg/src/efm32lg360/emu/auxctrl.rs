#[doc = "Register `AUXCTRL` reader"]
pub struct R(crate::R<AUXCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUXCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUXCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUXCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUXCTRL` writer"]
pub struct W(crate::W<AUXCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUXCTRL_SPEC>;
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
impl From<crate::W<AUXCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUXCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HRCCLR` reader - Hard Reset Cause Clear"]
pub type HRCCLR_R = crate::BitReader<bool>;
#[doc = "Field `HRCCLR` writer - Hard Reset Cause Clear"]
pub type HRCCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUXCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Hard Reset Cause Clear"]
    #[inline(always)]
    pub fn hrcclr(&self) -> HRCCLR_R {
        HRCCLR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hard Reset Cause Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hrcclr(&mut self) -> HRCCLR_W<0> {
        HRCCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Auxiliary Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auxctrl](index.html) module"]
pub struct AUXCTRL_SPEC;
impl crate::RegisterSpec for AUXCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [auxctrl::R](R) reader structure"]
impl crate::Readable for AUXCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [auxctrl::W](W) writer structure"]
impl crate::Writable for AUXCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AUXCTRL to value 0"]
impl crate::Resettable for AUXCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
