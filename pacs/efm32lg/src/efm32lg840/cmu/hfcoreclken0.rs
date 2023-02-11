#[doc = "Register `HFCORECLKEN0` reader"]
pub struct R(crate::R<HFCORECLKEN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFCORECLKEN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFCORECLKEN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFCORECLKEN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFCORECLKEN0` writer"]
pub struct W(crate::W<HFCORECLKEN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFCORECLKEN0_SPEC>;
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
impl From<crate::W<HFCORECLKEN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFCORECLKEN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA` reader - Direct Memory Access Controller Clock Enable"]
pub type DMA_R = crate::BitReader<bool>;
#[doc = "Field `DMA` writer - Direct Memory Access Controller Clock Enable"]
pub type DMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, HFCORECLKEN0_SPEC, bool, O>;
#[doc = "Field `AES` reader - Advanced Encryption Standard Accelerator Clock Enable"]
pub type AES_R = crate::BitReader<bool>;
#[doc = "Field `AES` writer - Advanced Encryption Standard Accelerator Clock Enable"]
pub type AES_W<'a, const O: u8> = crate::BitWriter<'a, u32, HFCORECLKEN0_SPEC, bool, O>;
#[doc = "Field `LE` reader - Low Energy Peripheral Interface Clock Enable"]
pub type LE_R = crate::BitReader<bool>;
#[doc = "Field `LE` writer - Low Energy Peripheral Interface Clock Enable"]
pub type LE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HFCORECLKEN0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Direct Memory Access Controller Clock Enable"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Advanced Encryption Standard Accelerator Clock Enable"]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Low Energy Peripheral Interface Clock Enable"]
    #[inline(always)]
    pub fn le(&self) -> LE_R {
        LE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Direct Memory Access Controller Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<0> {
        DMA_W::new(self)
    }
    #[doc = "Bit 1 - Advanced Encryption Standard Accelerator Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aes(&mut self) -> AES_W<1> {
        AES_W::new(self)
    }
    #[doc = "Bit 4 - Low Energy Peripheral Interface Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn le(&mut self) -> LE_W<4> {
        LE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "High Frequency Core Clock Enable Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfcoreclken0](index.html) module"]
pub struct HFCORECLKEN0_SPEC;
impl crate::RegisterSpec for HFCORECLKEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfcoreclken0::R](R) reader structure"]
impl crate::Readable for HFCORECLKEN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfcoreclken0::W](W) writer structure"]
impl crate::Writable for HFCORECLKEN0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HFCORECLKEN0 to value 0"]
impl crate::Resettable for HFCORECLKEN0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
