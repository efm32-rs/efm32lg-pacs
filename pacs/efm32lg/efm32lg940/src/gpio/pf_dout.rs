#[doc = "Register `PF_DOUT` reader"]
pub struct R(crate::R<PF_DOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PF_DOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PF_DOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PF_DOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PF_DOUT` writer"]
pub struct W(crate::W<PF_DOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PF_DOUT_SPEC>;
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
impl From<crate::W<PF_DOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PF_DOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOUT` reader - Data Out"]
pub type DOUT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DOUT` writer - Data Out"]
pub type DOUT_W<'a> = crate::FieldWriter<'a, u32, PF_DOUT_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Data Out"]
    #[inline(always)]
    pub fn dout(&self) -> DOUT_R {
        DOUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Out"]
    #[inline(always)]
    pub fn dout(&mut self) -> DOUT_W {
        DOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Data Out Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pf_dout](index.html) module"]
pub struct PF_DOUT_SPEC;
impl crate::RegisterSpec for PF_DOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pf_dout::R](R) reader structure"]
impl crate::Readable for PF_DOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pf_dout::W](W) writer structure"]
impl crate::Writable for PF_DOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PF_DOUT to value 0"]
impl crate::Resettable for PF_DOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
