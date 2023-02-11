#[doc = "Register `TFTFRAMEBASE` reader"]
pub struct R(crate::R<TFTFRAMEBASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TFTFRAMEBASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TFTFRAMEBASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TFTFRAMEBASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TFTFRAMEBASE` writer"]
pub struct W(crate::W<TFTFRAMEBASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TFTFRAMEBASE_SPEC>;
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
impl From<crate::W<TFTFRAMEBASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TFTFRAMEBASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAMEBASE` reader - Frame Base Address"]
pub type FRAMEBASE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FRAMEBASE` writer - Frame Base Address"]
pub type FRAMEBASE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TFTFRAMEBASE_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 0:27 - Frame Base Address"]
    #[inline(always)]
    pub fn framebase(&self) -> FRAMEBASE_R {
        FRAMEBASE_R::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - Frame Base Address"]
    #[inline(always)]
    #[must_use]
    pub fn framebase(&mut self) -> FRAMEBASE_W<0> {
        FRAMEBASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TFT Frame Base Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tftframebase](index.html) module"]
pub struct TFTFRAMEBASE_SPEC;
impl crate::RegisterSpec for TFTFRAMEBASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tftframebase::R](R) reader structure"]
impl crate::Readable for TFTFRAMEBASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tftframebase::W](W) writer structure"]
impl crate::Writable for TFTFRAMEBASE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TFTFRAMEBASE to value 0"]
impl crate::Resettable for TFTFRAMEBASE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
