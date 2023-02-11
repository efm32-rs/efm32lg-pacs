#[doc = "Register `ETMITCTRL` reader"]
pub struct R(crate::R<ETMITCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETMITCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETMITCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETMITCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETMITCTRL` writer"]
pub struct W(crate::W<ETMITCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETMITCTRL_SPEC>;
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
impl From<crate::W<ETMITCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETMITCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ITEN` reader - Integration Mode Enable"]
pub type ITEN_R = crate::BitReader<bool>;
#[doc = "Field `ITEN` writer - Integration Mode Enable"]
pub type ITEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETMITCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Integration Mode Enable"]
    #[inline(always)]
    pub fn iten(&self) -> ITEN_R {
        ITEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Integration Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn iten(&mut self) -> ITEN_W<0> {
        ITEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ETM Integration Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmitctrl](index.html) module"]
pub struct ETMITCTRL_SPEC;
impl crate::RegisterSpec for ETMITCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etmitctrl::R](R) reader structure"]
impl crate::Readable for ETMITCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etmitctrl::W](W) writer structure"]
impl crate::Writable for ETMITCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETMITCTRL to value 0"]
impl crate::Resettable for ETMITCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
