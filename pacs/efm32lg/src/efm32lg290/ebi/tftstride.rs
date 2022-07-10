#[doc = "Register `TFTSTRIDE` reader"]
pub struct R(crate::R<TFTSTRIDE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TFTSTRIDE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TFTSTRIDE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TFTSTRIDE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TFTSTRIDE` writer"]
pub struct W(crate::W<TFTSTRIDE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TFTSTRIDE_SPEC>;
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
impl From<crate::W<TFTSTRIDE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TFTSTRIDE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSTRIDE` reader - Horizontal Stride"]
pub type HSTRIDE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HSTRIDE` writer - Horizontal Stride"]
pub type HSTRIDE_W<'a> = crate::FieldWriter<'a, u32, TFTSTRIDE_SPEC, u16, u16, 12, 0>;
impl R {
    #[doc = "Bits 0:11 - Horizontal Stride"]
    #[inline(always)]
    pub fn hstride(&self) -> HSTRIDE_R {
        HSTRIDE_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Horizontal Stride"]
    #[inline(always)]
    pub fn hstride(&mut self) -> HSTRIDE_W {
        HSTRIDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TFT Stride Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tftstride](index.html) module"]
pub struct TFTSTRIDE_SPEC;
impl crate::RegisterSpec for TFTSTRIDE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tftstride::R](R) reader structure"]
impl crate::Readable for TFTSTRIDE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tftstride::W](W) writer structure"]
impl crate::Writable for TFTSTRIDE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TFTSTRIDE to value 0"]
impl crate::Resettable for TFTSTRIDE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
