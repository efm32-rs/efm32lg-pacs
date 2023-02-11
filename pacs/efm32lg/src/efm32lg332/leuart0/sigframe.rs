#[doc = "Register `SIGFRAME` reader"]
pub struct R(crate::R<SIGFRAME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGFRAME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGFRAME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGFRAME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIGFRAME` writer"]
pub struct W(crate::W<SIGFRAME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIGFRAME_SPEC>;
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
impl From<crate::W<SIGFRAME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIGFRAME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIGFRAME` reader - Signal Frame"]
pub type SIGFRAME_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SIGFRAME` writer - Signal Frame"]
pub type SIGFRAME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SIGFRAME_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - Signal Frame"]
    #[inline(always)]
    pub fn sigframe(&self) -> SIGFRAME_R {
        SIGFRAME_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Signal Frame"]
    #[inline(always)]
    #[must_use]
    pub fn sigframe(&mut self) -> SIGFRAME_W<0> {
        SIGFRAME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Signal Frame Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigframe](index.html) module"]
pub struct SIGFRAME_SPEC;
impl crate::RegisterSpec for SIGFRAME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigframe::R](R) reader structure"]
impl crate::Readable for SIGFRAME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sigframe::W](W) writer structure"]
impl crate::Writable for SIGFRAME_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIGFRAME to value 0"]
impl crate::Resettable for SIGFRAME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
