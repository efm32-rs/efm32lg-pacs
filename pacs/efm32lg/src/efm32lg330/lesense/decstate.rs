#[doc = "Register `DECSTATE` reader"]
pub struct R(crate::R<DECSTATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DECSTATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DECSTATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DECSTATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DECSTATE` writer"]
pub struct W(crate::W<DECSTATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DECSTATE_SPEC>;
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
impl From<crate::W<DECSTATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DECSTATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DECSTATE` reader - Shows the current decoder state"]
pub type DECSTATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DECSTATE` writer - Shows the current decoder state"]
pub type DECSTATE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DECSTATE_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Shows the current decoder state"]
    #[inline(always)]
    pub fn decstate(&self) -> DECSTATE_R {
        DECSTATE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Shows the current decoder state"]
    #[inline(always)]
    #[must_use]
    pub fn decstate(&mut self) -> DECSTATE_W<0> {
        DECSTATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Current decoder state\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [decstate](index.html) module"]
pub struct DECSTATE_SPEC;
impl crate::RegisterSpec for DECSTATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [decstate::R](R) reader structure"]
impl crate::Readable for DECSTATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [decstate::W](W) writer structure"]
impl crate::Writable for DECSTATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DECSTATE to value 0"]
impl crate::Resettable for DECSTATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
