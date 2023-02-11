#[doc = "Register `KEYHC` reader"]
pub struct R(crate::R<KEYHC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYHC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYHC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYHC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEYHC` writer"]
pub struct W(crate::W<KEYHC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYHC_SPEC>;
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
impl From<crate::W<KEYHC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYHC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEYHC` reader - Key High Access C"]
pub type KEYHC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `KEYHC` writer - Key High Access C"]
pub type KEYHC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEYHC_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Key High Access C"]
    #[inline(always)]
    pub fn keyhc(&self) -> KEYHC_R {
        KEYHC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key High Access C"]
    #[inline(always)]
    #[must_use]
    pub fn keyhc(&mut self) -> KEYHC_W<0> {
        KEYHC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "KEY High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyhc](index.html) module\n\nOne or more dependent resources other than the current register are immediately affected by a read operation."]
pub struct KEYHC_SPEC;
impl crate::RegisterSpec for KEYHC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keyhc::R](R) reader structure"]
impl crate::Readable for KEYHC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keyhc::W](W) writer structure"]
impl crate::Writable for KEYHC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEYHC to value 0"]
impl crate::Resettable for KEYHC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
