#[doc = "Register `KEYLC` reader"]
pub struct R(crate::R<KEYLC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYLC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYLC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYLC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEYLC` writer"]
pub struct W(crate::W<KEYLC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYLC_SPEC>;
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
impl From<crate::W<KEYLC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYLC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEYLC` reader - Key Low Access C"]
pub type KEYLC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `KEYLC` writer - Key Low Access C"]
pub type KEYLC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEYLC_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Key Low Access C"]
    #[inline(always)]
    pub fn keylc(&self) -> KEYLC_R {
        KEYLC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key Low Access C"]
    #[inline(always)]
    #[must_use]
    pub fn keylc(&mut self) -> KEYLC_W<0> {
        KEYLC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "KEY Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keylc](index.html) module\n\nOne or more dependent resources other than the current register are immediately affected by a read operation."]
pub struct KEYLC_SPEC;
impl crate::RegisterSpec for KEYLC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keylc::R](R) reader structure"]
impl crate::Readable for KEYLC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keylc::W](W) writer structure"]
impl crate::Writable for KEYLC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEYLC to value 0"]
impl crate::Resettable for KEYLC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
