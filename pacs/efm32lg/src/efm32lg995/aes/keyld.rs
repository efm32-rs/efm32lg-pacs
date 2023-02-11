#[doc = "Register `KEYLD` reader"]
pub struct R(crate::R<KEYLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEYLD` writer"]
pub struct W(crate::W<KEYLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYLD_SPEC>;
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
impl From<crate::W<KEYLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEYLD` reader - Key Low Access D"]
pub type KEYLD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `KEYLD` writer - Key Low Access D"]
pub type KEYLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEYLD_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Key Low Access D"]
    #[inline(always)]
    pub fn keyld(&self) -> KEYLD_R {
        KEYLD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key Low Access D"]
    #[inline(always)]
    #[must_use]
    pub fn keyld(&mut self) -> KEYLD_W<0> {
        KEYLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "KEY Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyld](index.html) module\n\nOne or more dependent resources other than the current register are immediately affected by a read operation."]
pub struct KEYLD_SPEC;
impl crate::RegisterSpec for KEYLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keyld::R](R) reader structure"]
impl crate::Readable for KEYLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keyld::W](W) writer structure"]
impl crate::Writable for KEYLD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEYLD to value 0"]
impl crate::Resettable for KEYLD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
