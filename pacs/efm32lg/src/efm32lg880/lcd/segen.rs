#[doc = "Register `SEGEN` reader"]
pub struct R(crate::R<SEGEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEGEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEGEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEGEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEGEN` writer"]
pub struct W(crate::W<SEGEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEGEN_SPEC>;
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
impl From<crate::W<SEGEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEGEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEGEN` reader - Segment Enable"]
pub type SEGEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SEGEN` writer - Segment Enable"]
pub type SEGEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEGEN_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Segment Enable"]
    #[inline(always)]
    pub fn segen(&self) -> SEGEN_R {
        SEGEN_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Segment Enable"]
    #[inline(always)]
    #[must_use]
    pub fn segen(&mut self) -> SEGEN_W<0> {
        SEGEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Segment Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [segen](index.html) module"]
pub struct SEGEN_SPEC;
impl crate::RegisterSpec for SEGEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [segen::R](R) reader structure"]
impl crate::Readable for SEGEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [segen::W](W) writer structure"]
impl crate::Writable for SEGEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEGEN to value 0"]
impl crate::Resettable for SEGEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
