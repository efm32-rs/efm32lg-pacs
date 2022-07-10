#[doc = "Register `PA_DOUTCLR` writer"]
pub struct W(crate::W<PA_DOUTCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PA_DOUTCLR_SPEC>;
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
impl From<crate::W<PA_DOUTCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PA_DOUTCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOUTCLR` writer - Data Out Clear"]
pub type DOUTCLR_W<'a> = crate::FieldWriter<'a, u32, PA_DOUTCLR_SPEC, u16, u16, 16, 0>;
impl W {
    #[doc = "Bits 0:15 - Data Out Clear"]
    #[inline(always)]
    pub fn doutclr(&mut self) -> DOUTCLR_W {
        DOUTCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Data Out Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pa_doutclr](index.html) module"]
pub struct PA_DOUTCLR_SPEC;
impl crate::RegisterSpec for PA_DOUTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pa_doutclr::W](W) writer structure"]
impl crate::Writable for PA_DOUTCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PA_DOUTCLR to value 0"]
impl crate::Resettable for PA_DOUTCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
