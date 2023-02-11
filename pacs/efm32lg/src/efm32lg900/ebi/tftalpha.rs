#[doc = "Register `TFTALPHA` reader"]
pub struct R(crate::R<TFTALPHA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TFTALPHA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TFTALPHA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TFTALPHA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TFTALPHA` writer"]
pub struct W(crate::W<TFTALPHA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TFTALPHA_SPEC>;
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
impl From<crate::W<TFTALPHA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TFTALPHA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALPHA` reader - TFT Alpha Blending Factor"]
pub type ALPHA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ALPHA` writer - TFT Alpha Blending Factor"]
pub type ALPHA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TFTALPHA_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - TFT Alpha Blending Factor"]
    #[inline(always)]
    pub fn alpha(&self) -> ALPHA_R {
        ALPHA_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - TFT Alpha Blending Factor"]
    #[inline(always)]
    #[must_use]
    pub fn alpha(&mut self) -> ALPHA_W<0> {
        ALPHA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TFT Alpha Blending Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tftalpha](index.html) module"]
pub struct TFTALPHA_SPEC;
impl crate::RegisterSpec for TFTALPHA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tftalpha::R](R) reader structure"]
impl crate::Readable for TFTALPHA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tftalpha::W](W) writer structure"]
impl crate::Writable for TFTALPHA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TFTALPHA to value 0"]
impl crate::Resettable for TFTALPHA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
