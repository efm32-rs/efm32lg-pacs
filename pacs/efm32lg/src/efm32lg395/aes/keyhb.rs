#[doc = "Register `KEYHB` reader"]
pub struct R(crate::R<KEYHB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYHB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYHB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYHB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEYHB` writer"]
pub struct W(crate::W<KEYHB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYHB_SPEC>;
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
impl From<crate::W<KEYHB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYHB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEYHB` reader - Key High Access B"]
pub type KEYHB_R = crate::FieldReader<u32, u32>;
#[doc = "Field `KEYHB` writer - Key High Access B"]
pub type KEYHB_W<'a> = crate::FieldWriter<'a, u32, KEYHB_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Key High Access B"]
    #[inline(always)]
    pub fn keyhb(&self) -> KEYHB_R {
        KEYHB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key High Access B"]
    #[inline(always)]
    pub fn keyhb(&mut self) -> KEYHB_W {
        KEYHB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "KEY High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyhb](index.html) module\n\nOne or more dependent resources other than the current register are immediately affected by a read operation."]
pub struct KEYHB_SPEC;
impl crate::RegisterSpec for KEYHB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keyhb::R](R) reader structure"]
impl crate::Readable for KEYHB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keyhb::W](W) writer structure"]
impl crate::Writable for KEYHB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEYHB to value 0"]
impl crate::Resettable for KEYHB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
