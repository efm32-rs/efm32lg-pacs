#[doc = "Register `SEGD0L` reader"]
pub struct R(crate::R<SEGD0L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEGD0L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEGD0L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEGD0L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEGD0L` writer"]
pub struct W(crate::W<SEGD0L_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEGD0L_SPEC>;
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
impl From<crate::W<SEGD0L_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEGD0L_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEGD0L` reader - COM0 Segment Data Low"]
pub type SEGD0L_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SEGD0L` writer - COM0 Segment Data Low"]
pub type SEGD0L_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEGD0L_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - COM0 Segment Data Low"]
    #[inline(always)]
    pub fn segd0l(&self) -> SEGD0L_R {
        SEGD0L_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - COM0 Segment Data Low"]
    #[inline(always)]
    #[must_use]
    pub fn segd0l(&mut self) -> SEGD0L_W<0> {
        SEGD0L_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Segment Data Low Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [segd0l](index.html) module"]
pub struct SEGD0L_SPEC;
impl crate::RegisterSpec for SEGD0L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [segd0l::R](R) reader structure"]
impl crate::Readable for SEGD0L_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [segd0l::W](W) writer structure"]
impl crate::Writable for SEGD0L_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEGD0L to value 0"]
impl crate::Resettable for SEGD0L_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
