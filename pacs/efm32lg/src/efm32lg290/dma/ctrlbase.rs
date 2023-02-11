#[doc = "Register `CTRLBASE` reader"]
pub struct R(crate::R<CTRLBASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLBASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLBASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLBASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLBASE` writer"]
pub struct W(crate::W<CTRLBASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLBASE_SPEC>;
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
impl From<crate::W<CTRLBASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLBASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTRLBASE` reader - Channel Control Data Base Pointer"]
pub type CTRLBASE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CTRLBASE` writer - Channel Control Data Base Pointer"]
pub type CTRLBASE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLBASE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Channel Control Data Base Pointer"]
    #[inline(always)]
    pub fn ctrlbase(&self) -> CTRLBASE_R {
        CTRLBASE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel Control Data Base Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn ctrlbase(&mut self) -> CTRLBASE_W<0> {
        CTRLBASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Control Data Base Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlbase](index.html) module"]
pub struct CTRLBASE_SPEC;
impl crate::RegisterSpec for CTRLBASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrlbase::R](R) reader structure"]
impl crate::Readable for CTRLBASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlbase::W](W) writer structure"]
impl crate::Writable for CTRLBASE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLBASE to value 0"]
impl crate::Resettable for CTRLBASE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
