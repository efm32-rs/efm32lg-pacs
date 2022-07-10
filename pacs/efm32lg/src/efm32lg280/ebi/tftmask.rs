#[doc = "Register `TFTMASK` reader"]
pub struct R(crate::R<TFTMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TFTMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TFTMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TFTMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TFTMASK` writer"]
pub struct W(crate::W<TFTMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TFTMASK_SPEC>;
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
impl From<crate::W<TFTMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TFTMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TFTMASK` reader - TFT Mask Value"]
pub type TFTMASK_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TFTMASK` writer - TFT Mask Value"]
pub type TFTMASK_W<'a> = crate::FieldWriter<'a, u32, TFTMASK_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - TFT Mask Value"]
    #[inline(always)]
    pub fn tftmask(&self) -> TFTMASK_R {
        TFTMASK_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TFT Mask Value"]
    #[inline(always)]
    pub fn tftmask(&mut self) -> TFTMASK_W {
        TFTMASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TFT Masking Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tftmask](index.html) module"]
pub struct TFTMASK_SPEC;
impl crate::RegisterSpec for TFTMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tftmask::R](R) reader structure"]
impl crate::Readable for TFTMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tftmask::W](W) writer structure"]
impl crate::Writable for TFTMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TFTMASK to value 0"]
impl crate::Resettable for TFTMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
