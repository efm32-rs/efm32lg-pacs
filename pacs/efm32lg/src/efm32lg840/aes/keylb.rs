#[doc = "Register `KEYLB` reader"]
pub struct R(crate::R<KEYLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEYLB` writer"]
pub struct W(crate::W<KEYLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYLB_SPEC>;
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
impl From<crate::W<KEYLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEYLB` reader - Key Low Access B"]
pub type KEYLB_R = crate::FieldReader<u32, u32>;
#[doc = "Field `KEYLB` writer - Key Low Access B"]
pub type KEYLB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEYLB_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Key Low Access B"]
    #[inline(always)]
    pub fn keylb(&self) -> KEYLB_R {
        KEYLB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key Low Access B"]
    #[inline(always)]
    #[must_use]
    pub fn keylb(&mut self) -> KEYLB_W<0> {
        KEYLB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "KEY Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keylb](index.html) module\n\nOne or more dependent resources other than the current register are immediately affected by a read operation."]
pub struct KEYLB_SPEC;
impl crate::RegisterSpec for KEYLB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keylb::R](R) reader structure"]
impl crate::Readable for KEYLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keylb::W](W) writer structure"]
impl crate::Writable for KEYLB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEYLB to value 0"]
impl crate::Resettable for KEYLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
