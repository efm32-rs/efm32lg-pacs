#[doc = "Register `SEGD3L` reader"]
pub struct R(crate::R<SEGD3L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEGD3L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEGD3L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEGD3L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEGD3L` writer"]
pub struct W(crate::W<SEGD3L_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEGD3L_SPEC>;
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
impl From<crate::W<SEGD3L_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEGD3L_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEGD3L` reader - COM3 Segment Data Low"]
pub type SEGD3L_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SEGD3L` writer - COM3 Segment Data Low"]
pub type SEGD3L_W<'a> = crate::FieldWriter<'a, u32, SEGD3L_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - COM3 Segment Data Low"]
    #[inline(always)]
    pub fn segd3l(&self) -> SEGD3L_R {
        SEGD3L_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - COM3 Segment Data Low"]
    #[inline(always)]
    pub fn segd3l(&mut self) -> SEGD3L_W {
        SEGD3L_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Segment Data Low Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [segd3l](index.html) module"]
pub struct SEGD3L_SPEC;
impl crate::RegisterSpec for SEGD3L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [segd3l::R](R) reader structure"]
impl crate::Readable for SEGD3L_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [segd3l::W](W) writer structure"]
impl crate::Writable for SEGD3L_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEGD3L to value 0"]
impl crate::Resettable for SEGD3L_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
