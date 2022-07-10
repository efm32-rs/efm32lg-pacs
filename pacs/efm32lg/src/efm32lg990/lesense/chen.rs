#[doc = "Register `CHEN` reader"]
pub struct R(crate::R<CHEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHEN` writer"]
pub struct W(crate::W<CHEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHEN_SPEC>;
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
impl From<crate::W<CHEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHEN` reader - Enable scan channel"]
pub type CHEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CHEN` writer - Enable scan channel"]
pub type CHEN_W<'a> = crate::FieldWriter<'a, u32, CHEN_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Enable scan channel"]
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Enable scan channel"]
    #[inline(always)]
    pub fn chen(&mut self) -> CHEN_W {
        CHEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chen](index.html) module"]
pub struct CHEN_SPEC;
impl crate::RegisterSpec for CHEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chen::R](R) reader structure"]
impl crate::Readable for CHEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chen::W](W) writer structure"]
impl crate::Writable for CHEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHEN to value 0"]
impl crate::Resettable for CHEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
