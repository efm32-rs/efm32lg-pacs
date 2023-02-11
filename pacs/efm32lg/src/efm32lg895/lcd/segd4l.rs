#[doc = "Register `SEGD4L` reader"]
pub struct R(crate::R<SEGD4L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEGD4L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEGD4L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEGD4L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEGD4L` writer"]
pub struct W(crate::W<SEGD4L_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEGD4L_SPEC>;
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
impl From<crate::W<SEGD4L_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEGD4L_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEGD4L` reader - COM4 Segment Data"]
pub type SEGD4L_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SEGD4L` writer - COM4 Segment Data"]
pub type SEGD4L_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEGD4L_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - COM4 Segment Data"]
    #[inline(always)]
    pub fn segd4l(&self) -> SEGD4L_R {
        SEGD4L_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - COM4 Segment Data"]
    #[inline(always)]
    #[must_use]
    pub fn segd4l(&mut self) -> SEGD4L_W<0> {
        SEGD4L_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Segment Data Low Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [segd4l](index.html) module"]
pub struct SEGD4L_SPEC;
impl crate::RegisterSpec for SEGD4L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [segd4l::R](R) reader structure"]
impl crate::Readable for SEGD4L_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [segd4l::W](W) writer structure"]
impl crate::Writable for SEGD4L_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEGD4L to value 0"]
impl crate::Resettable for SEGD4L_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
