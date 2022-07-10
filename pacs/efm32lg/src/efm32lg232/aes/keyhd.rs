#[doc = "Register `KEYHD` reader"]
pub struct R(crate::R<KEYHD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYHD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYHD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYHD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEYHD` writer"]
pub struct W(crate::W<KEYHD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYHD_SPEC>;
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
impl From<crate::W<KEYHD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYHD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEYHD` reader - Key High Access D"]
pub type KEYHD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `KEYHD` writer - Key High Access D"]
pub type KEYHD_W<'a> = crate::FieldWriter<'a, u32, KEYHD_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Key High Access D"]
    #[inline(always)]
    pub fn keyhd(&self) -> KEYHD_R {
        KEYHD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key High Access D"]
    #[inline(always)]
    pub fn keyhd(&mut self) -> KEYHD_W {
        KEYHD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "KEY High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyhd](index.html) module\n\nOne or more dependent resources other than the current register are immediately affected by a read operation."]
pub struct KEYHD_SPEC;
impl crate::RegisterSpec for KEYHD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keyhd::R](R) reader structure"]
impl crate::Readable for KEYHD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keyhd::W](W) writer structure"]
impl crate::Writable for KEYHD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEYHD to value 0"]
impl crate::Resettable for KEYHD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
