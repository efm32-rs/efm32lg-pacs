#[doc = "Register `DOEP0DMAADDR` reader"]
pub struct R(crate::R<DOEP0DMAADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEP0DMAADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEP0DMAADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEP0DMAADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEP0DMAADDR` writer"]
pub struct W(crate::W<DOEP0DMAADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEP0DMAADDR_SPEC>;
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
impl From<crate::W<DOEP0DMAADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEP0DMAADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOEP0DMAADDR` reader - DMA Address"]
pub type DOEP0DMAADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DOEP0DMAADDR` writer - DMA Address"]
pub type DOEP0DMAADDR_W<'a> = crate::FieldWriter<'a, u32, DOEP0DMAADDR_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    pub fn doep0dmaaddr(&self) -> DOEP0DMAADDR_R {
        DOEP0DMAADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    pub fn doep0dmaaddr(&mut self) -> DOEP0DMAADDR_W {
        DOEP0DMAADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device OUT Endpoint 0 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep0dmaaddr](index.html) module"]
pub struct DOEP0DMAADDR_SPEC;
impl crate::RegisterSpec for DOEP0DMAADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doep0dmaaddr::R](R) reader structure"]
impl crate::Readable for DOEP0DMAADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doep0dmaaddr::W](W) writer structure"]
impl crate::Writable for DOEP0DMAADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOEP0DMAADDR to value 0"]
impl crate::Resettable for DOEP0DMAADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
