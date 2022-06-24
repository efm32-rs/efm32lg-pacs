#[doc = "Register `TFTDD` reader"]
pub struct R(crate::R<TFTDD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TFTDD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TFTDD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TFTDD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TFTDD` writer"]
pub struct W(crate::W<TFTDD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TFTDD_SPEC>;
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
impl From<crate::W<TFTDD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TFTDD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - TFT Direct Drive Data from Internal Memory"]
pub type DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DATA` writer - TFT Direct Drive Data from Internal Memory"]
pub type DATA_W<'a> = crate::FieldWriter<'a, u32, TFTDD_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - TFT Direct Drive Data from Internal Memory"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TFT Direct Drive Data from Internal Memory"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TFT Direct Drive Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tftdd](index.html) module"]
pub struct TFTDD_SPEC;
impl crate::RegisterSpec for TFTDD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tftdd::R](R) reader structure"]
impl crate::Readable for TFTDD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tftdd::W](W) writer structure"]
impl crate::Writable for TFTDD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TFTDD to value 0"]
impl crate::Resettable for TFTDD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
