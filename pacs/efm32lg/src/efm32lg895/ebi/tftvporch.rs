#[doc = "Register `TFTVPORCH` reader"]
pub struct R(crate::R<TFTVPORCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TFTVPORCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TFTVPORCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TFTVPORCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TFTVPORCH` writer"]
pub struct W(crate::W<TFTVPORCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TFTVPORCH_SPEC>;
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
impl From<crate::W<TFTVPORCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TFTVPORCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VSYNC` reader - Vertical Synchronization Pulse Width"]
pub type VSYNC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VSYNC` writer - Vertical Synchronization Pulse Width"]
pub type VSYNC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TFTVPORCH_SPEC, u8, u8, 7, O>;
#[doc = "Field `VFPORCH` reader - Vertical Front Porch Size"]
pub type VFPORCH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VFPORCH` writer - Vertical Front Porch Size"]
pub type VFPORCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TFTVPORCH_SPEC, u8, u8, 8, O>;
#[doc = "Field `VBPORCH` reader - Vertical Back Porch Size"]
pub type VBPORCH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VBPORCH` writer - Vertical Back Porch Size"]
pub type VBPORCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TFTVPORCH_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:6 - Vertical Synchronization Pulse Width"]
    #[inline(always)]
    pub fn vsync(&self) -> VSYNC_R {
        VSYNC_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - Vertical Front Porch Size"]
    #[inline(always)]
    pub fn vfporch(&self) -> VFPORCH_R {
        VFPORCH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 18:25 - Vertical Back Porch Size"]
    #[inline(always)]
    pub fn vbporch(&self) -> VBPORCH_R {
        VBPORCH_R::new(((self.bits >> 18) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Vertical Synchronization Pulse Width"]
    #[inline(always)]
    #[must_use]
    pub fn vsync(&mut self) -> VSYNC_W<0> {
        VSYNC_W::new(self)
    }
    #[doc = "Bits 8:15 - Vertical Front Porch Size"]
    #[inline(always)]
    #[must_use]
    pub fn vfporch(&mut self) -> VFPORCH_W<8> {
        VFPORCH_W::new(self)
    }
    #[doc = "Bits 18:25 - Vertical Back Porch Size"]
    #[inline(always)]
    #[must_use]
    pub fn vbporch(&mut self) -> VBPORCH_W<18> {
        VBPORCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TFT Vertical Porch Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tftvporch](index.html) module"]
pub struct TFTVPORCH_SPEC;
impl crate::RegisterSpec for TFTVPORCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tftvporch::R](R) reader structure"]
impl crate::Readable for TFTVPORCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tftvporch::W](W) writer structure"]
impl crate::Writable for TFTVPORCH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TFTVPORCH to value 0"]
impl crate::Resettable for TFTVPORCH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
