#[doc = "Register `PC_DOUTSET` writer"]
pub struct W(crate::W<PC_DOUTSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PC_DOUTSET_SPEC>;
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
impl From<crate::W<PC_DOUTSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PC_DOUTSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOUTSET` writer - Data Out Set"]
pub type DOUTSET_W<'a> = crate::FieldWriter<'a, u32, PC_DOUTSET_SPEC, u16, u16, 16, 0>;
impl W {
    #[doc = "Bits 0:15 - Data Out Set"]
    #[inline(always)]
    pub fn doutset(&mut self) -> DOUTSET_W {
        DOUTSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Data Out Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pc_doutset](index.html) module"]
pub struct PC_DOUTSET_SPEC;
impl crate::RegisterSpec for PC_DOUTSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pc_doutset::W](W) writer structure"]
impl crate::Writable for PC_DOUTSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PC_DOUTSET to value 0"]
impl crate::Resettable for PC_DOUTSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
