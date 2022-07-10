#[doc = "Register `TFTSIZE` reader"]
pub struct R(crate::R<TFTSIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TFTSIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TFTSIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TFTSIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TFTSIZE` writer"]
pub struct W(crate::W<TFTSIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TFTSIZE_SPEC>;
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
impl From<crate::W<TFTSIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TFTSIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSZ` reader - Horizontal Size (excluding porches)"]
pub type HSZ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HSZ` writer - Horizontal Size (excluding porches)"]
pub type HSZ_W<'a> = crate::FieldWriter<'a, u32, TFTSIZE_SPEC, u16, u16, 10, 0>;
#[doc = "Field `VSZ` reader - Vertical Size (excluding porches)"]
pub type VSZ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VSZ` writer - Vertical Size (excluding porches)"]
pub type VSZ_W<'a> = crate::FieldWriter<'a, u32, TFTSIZE_SPEC, u16, u16, 10, 16>;
impl R {
    #[doc = "Bits 0:9 - Horizontal Size (excluding porches)"]
    #[inline(always)]
    pub fn hsz(&self) -> HSZ_R {
        HSZ_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Vertical Size (excluding porches)"]
    #[inline(always)]
    pub fn vsz(&self) -> VSZ_R {
        VSZ_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Horizontal Size (excluding porches)"]
    #[inline(always)]
    pub fn hsz(&mut self) -> HSZ_W {
        HSZ_W::new(self)
    }
    #[doc = "Bits 16:25 - Vertical Size (excluding porches)"]
    #[inline(always)]
    pub fn vsz(&mut self) -> VSZ_W {
        VSZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TFT Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tftsize](index.html) module"]
pub struct TFTSIZE_SPEC;
impl crate::RegisterSpec for TFTSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tftsize::R](R) reader structure"]
impl crate::Readable for TFTSIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tftsize::W](W) writer structure"]
impl crate::Writable for TFTSIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TFTSIZE to value 0"]
impl crate::Resettable for TFTSIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
