#[doc = "Register `SEGD6L` reader"]
pub struct R(crate::R<SEGD6L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEGD6L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEGD6L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEGD6L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEGD6L` writer"]
pub struct W(crate::W<SEGD6L_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEGD6L_SPEC>;
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
impl From<crate::W<SEGD6L_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEGD6L_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEGD6L` reader - COM6 Segment Data"]
pub type SEGD6L_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SEGD6L` writer - COM6 Segment Data"]
pub type SEGD6L_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEGD6L_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - COM6 Segment Data"]
    #[inline(always)]
    pub fn segd6l(&self) -> SEGD6L_R {
        SEGD6L_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - COM6 Segment Data"]
    #[inline(always)]
    #[must_use]
    pub fn segd6l(&mut self) -> SEGD6L_W<0> {
        SEGD6L_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Segment Data Low Register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [segd6l](index.html) module"]
pub struct SEGD6L_SPEC;
impl crate::RegisterSpec for SEGD6L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [segd6l::R](R) reader structure"]
impl crate::Readable for SEGD6L_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [segd6l::W](W) writer structure"]
impl crate::Writable for SEGD6L_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEGD6L to value 0"]
impl crate::Resettable for SEGD6L_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
