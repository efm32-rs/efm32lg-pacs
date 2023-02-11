#[doc = "Register `KEYLA` reader"]
pub struct R(crate::R<KEYLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEYLA` writer"]
pub struct W(crate::W<KEYLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYLA_SPEC>;
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
impl From<crate::W<KEYLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEYLA` reader - Key Low Access A"]
pub type KEYLA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `KEYLA` writer - Key Low Access A"]
pub type KEYLA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEYLA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Key Low Access A"]
    #[inline(always)]
    pub fn keyla(&self) -> KEYLA_R {
        KEYLA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key Low Access A"]
    #[inline(always)]
    #[must_use]
    pub fn keyla(&mut self) -> KEYLA_W<0> {
        KEYLA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "KEY Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyla](index.html) module\n\nOne or more dependent resources other than the current register are immediately affected by a read operation."]
pub struct KEYLA_SPEC;
impl crate::RegisterSpec for KEYLA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keyla::R](R) reader structure"]
impl crate::Readable for KEYLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keyla::W](W) writer structure"]
impl crate::Writable for KEYLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEYLA to value 0"]
impl crate::Resettable for KEYLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
