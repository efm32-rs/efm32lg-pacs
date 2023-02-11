#[doc = "Register `RECT0` reader"]
pub struct R(crate::R<RECT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RECT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RECT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RECT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RECT0` writer"]
pub struct W(crate::W<RECT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RECT0_SPEC>;
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
impl From<crate::W<RECT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RECT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HEIGHT` reader - DMA Channel 0 Rectangle Height"]
pub type HEIGHT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HEIGHT` writer - DMA Channel 0 Rectangle Height"]
pub type HEIGHT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RECT0_SPEC, u16, u16, 10, O>;
#[doc = "Field `SRCSTRIDE` reader - DMA Channel 0 Source Stride"]
pub type SRCSTRIDE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SRCSTRIDE` writer - DMA Channel 0 Source Stride"]
pub type SRCSTRIDE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RECT0_SPEC, u16, u16, 11, O>;
#[doc = "Field `DSTSTRIDE` reader - DMA Channel 0 Destination Stride"]
pub type DSTSTRIDE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DSTSTRIDE` writer - DMA Channel 0 Destination Stride"]
pub type DSTSTRIDE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RECT0_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:9 - DMA Channel 0 Rectangle Height"]
    #[inline(always)]
    pub fn height(&self) -> HEIGHT_R {
        HEIGHT_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:20 - DMA Channel 0 Source Stride"]
    #[inline(always)]
    pub fn srcstride(&self) -> SRCSTRIDE_R {
        SRCSTRIDE_R::new(((self.bits >> 10) & 0x07ff) as u16)
    }
    #[doc = "Bits 21:31 - DMA Channel 0 Destination Stride"]
    #[inline(always)]
    pub fn dststride(&self) -> DSTSTRIDE_R {
        DSTSTRIDE_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - DMA Channel 0 Rectangle Height"]
    #[inline(always)]
    #[must_use]
    pub fn height(&mut self) -> HEIGHT_W<0> {
        HEIGHT_W::new(self)
    }
    #[doc = "Bits 10:20 - DMA Channel 0 Source Stride"]
    #[inline(always)]
    #[must_use]
    pub fn srcstride(&mut self) -> SRCSTRIDE_W<10> {
        SRCSTRIDE_W::new(self)
    }
    #[doc = "Bits 21:31 - DMA Channel 0 Destination Stride"]
    #[inline(always)]
    #[must_use]
    pub fn dststride(&mut self) -> DSTSTRIDE_W<21> {
        DSTSTRIDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel 0 Rectangle Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rect0](index.html) module"]
pub struct RECT0_SPEC;
impl crate::RegisterSpec for RECT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rect0::R](R) reader structure"]
impl crate::Readable for RECT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rect0::W](W) writer structure"]
impl crate::Writable for RECT0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RECT0 to value 0"]
impl crate::Resettable for RECT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
