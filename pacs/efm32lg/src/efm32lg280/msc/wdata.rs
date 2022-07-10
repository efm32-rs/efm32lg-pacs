#[doc = "Register `WDATA` reader"]
pub struct R(crate::R<WDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDATA` writer"]
pub struct W(crate::W<WDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDATA_SPEC>;
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
impl From<crate::W<WDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDATA` reader - Write Data"]
pub type WDATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WDATA` writer - Write Data"]
pub type WDATA_W<'a> = crate::FieldWriter<'a, u32, WDATA_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Write Data"]
    #[inline(always)]
    pub fn wdata(&self) -> WDATA_R {
        WDATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write Data"]
    #[inline(always)]
    pub fn wdata(&mut self) -> WDATA_W {
        WDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdata](index.html) module"]
pub struct WDATA_SPEC;
impl crate::RegisterSpec for WDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdata::R](R) reader structure"]
impl crate::Readable for WDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdata::W](W) writer structure"]
impl crate::Writable for WDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDATA to value 0"]
impl crate::Resettable for WDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
