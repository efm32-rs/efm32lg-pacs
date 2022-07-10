#[doc = "Register `PB_DOUTTGL` writer"]
pub struct W(crate::W<PB_DOUTTGL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PB_DOUTTGL_SPEC>;
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
impl From<crate::W<PB_DOUTTGL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PB_DOUTTGL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOUTTGL` writer - Data Out Toggle"]
pub type DOUTTGL_W<'a> = crate::FieldWriter<'a, u32, PB_DOUTTGL_SPEC, u16, u16, 16, 0>;
impl W {
    #[doc = "Bits 0:15 - Data Out Toggle"]
    #[inline(always)]
    pub fn douttgl(&mut self) -> DOUTTGL_W {
        DOUTTGL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Data Out Toggle Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pb_douttgl](index.html) module"]
pub struct PB_DOUTTGL_SPEC;
impl crate::RegisterSpec for PB_DOUTTGL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pb_douttgl::W](W) writer structure"]
impl crate::Writable for PB_DOUTTGL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PB_DOUTTGL to value 0"]
impl crate::Resettable for PB_DOUTTGL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
