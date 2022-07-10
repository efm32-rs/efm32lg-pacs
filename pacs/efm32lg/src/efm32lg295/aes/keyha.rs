#[doc = "Register `KEYHA` reader"]
pub struct R(crate::R<KEYHA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYHA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYHA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYHA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEYHA` writer"]
pub struct W(crate::W<KEYHA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYHA_SPEC>;
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
impl From<crate::W<KEYHA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYHA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEYHA` reader - Key High Access A"]
pub type KEYHA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `KEYHA` writer - Key High Access A"]
pub type KEYHA_W<'a> = crate::FieldWriter<'a, u32, KEYHA_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Key High Access A"]
    #[inline(always)]
    pub fn keyha(&self) -> KEYHA_R {
        KEYHA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key High Access A"]
    #[inline(always)]
    pub fn keyha(&mut self) -> KEYHA_W {
        KEYHA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "KEY High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyha](index.html) module\n\nOne or more dependent resources other than the current register are immediately affected by a read operation."]
pub struct KEYHA_SPEC;
impl crate::RegisterSpec for KEYHA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keyha::R](R) reader structure"]
impl crate::Readable for KEYHA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keyha::W](W) writer structure"]
impl crate::Writable for KEYHA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEYHA to value 0"]
impl crate::Resettable for KEYHA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
