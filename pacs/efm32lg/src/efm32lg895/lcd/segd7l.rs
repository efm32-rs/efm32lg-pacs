#[doc = "Register `SEGD7L` reader"]
pub struct R(crate::R<SEGD7L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEGD7L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEGD7L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEGD7L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEGD7L` writer"]
pub struct W(crate::W<SEGD7L_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEGD7L_SPEC>;
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
impl From<crate::W<SEGD7L_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEGD7L_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEGD7L` reader - COM7 Segment Data"]
pub type SEGD7L_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SEGD7L` writer - COM7 Segment Data"]
pub type SEGD7L_W<'a> = crate::FieldWriter<'a, u32, SEGD7L_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - COM7 Segment Data"]
    #[inline(always)]
    pub fn segd7l(&self) -> SEGD7L_R {
        SEGD7L_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - COM7 Segment Data"]
    #[inline(always)]
    pub fn segd7l(&mut self) -> SEGD7L_W {
        SEGD7L_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Segment Data Low Register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [segd7l](index.html) module"]
pub struct SEGD7L_SPEC;
impl crate::RegisterSpec for SEGD7L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [segd7l::R](R) reader structure"]
impl crate::Readable for SEGD7L_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [segd7l::W](W) writer structure"]
impl crate::Writable for SEGD7L_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEGD7L to value 0"]
impl crate::Resettable for SEGD7L_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
