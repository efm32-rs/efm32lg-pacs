#[doc = "Register `ETMCLAIMCLR` reader"]
pub struct R(crate::R<ETMCLAIMCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETMCLAIMCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETMCLAIMCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETMCLAIMCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETMCLAIMCLR` writer"]
pub struct W(crate::W<ETMCLAIMCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETMCLAIMCLR_SPEC>;
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
impl From<crate::W<ETMCLAIMCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETMCLAIMCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLRTAG` reader - Tag Bits"]
pub type CLRTAG_R = crate::BitReader<bool>;
#[doc = "Field `CLRTAG` writer - Tag Bits"]
pub type CLRTAG_W<'a> = crate::BitWriter<'a, u32, ETMCLAIMCLR_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Tag Bits"]
    #[inline(always)]
    pub fn clrtag(&self) -> CLRTAG_R {
        CLRTAG_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tag Bits"]
    #[inline(always)]
    pub fn clrtag(&mut self) -> CLRTAG_W {
        CLRTAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ETM Claim Tag Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmclaimclr](index.html) module"]
pub struct ETMCLAIMCLR_SPEC;
impl crate::RegisterSpec for ETMCLAIMCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etmclaimclr::R](R) reader structure"]
impl crate::Readable for ETMCLAIMCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etmclaimclr::W](W) writer structure"]
impl crate::Writable for ETMCLAIMCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETMCLAIMCLR to value 0"]
impl crate::Resettable for ETMCLAIMCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
