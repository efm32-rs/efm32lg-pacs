#[doc = "Register `SEGD5L` reader"]
pub struct R(crate::R<SEGD5L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEGD5L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEGD5L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEGD5L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEGD5L` writer"]
pub struct W(crate::W<SEGD5L_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEGD5L_SPEC>;
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
impl From<crate::W<SEGD5L_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEGD5L_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEGD5L` reader - COM5 Segment Data"]
pub type SEGD5L_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SEGD5L` writer - COM5 Segment Data"]
pub type SEGD5L_W<'a> = crate::FieldWriter<'a, u32, SEGD5L_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - COM5 Segment Data"]
    #[inline(always)]
    pub fn segd5l(&self) -> SEGD5L_R {
        SEGD5L_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - COM5 Segment Data"]
    #[inline(always)]
    pub fn segd5l(&mut self) -> SEGD5L_W {
        SEGD5L_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Segment Data Low Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [segd5l](index.html) module"]
pub struct SEGD5L_SPEC;
impl crate::RegisterSpec for SEGD5L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [segd5l::R](R) reader structure"]
impl crate::Readable for SEGD5L_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [segd5l::W](W) writer structure"]
impl crate::Writable for SEGD5L_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEGD5L to value 0"]
impl crate::Resettable for SEGD5L_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
