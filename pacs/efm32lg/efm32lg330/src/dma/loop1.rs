#[doc = "Register `LOOP1` reader"]
pub struct R(crate::R<LOOP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOOP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOOP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOOP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOOP1` writer"]
pub struct W(crate::W<LOOP1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOOP1_SPEC>;
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
impl From<crate::W<LOOP1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOOP1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WIDTH` reader - DMA Channel 1 Loop Width"]
pub type WIDTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WIDTH` writer - DMA Channel 1 Loop Width"]
pub type WIDTH_W<'a> = crate::FieldWriter<'a, u32, LOOP1_SPEC, u16, u16, 10, 0>;
#[doc = "Field `EN` reader - DMA Channel 1 Loop Enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - DMA Channel 1 Loop Enable"]
pub type EN_W<'a> = crate::BitWriter<'a, u32, LOOP1_SPEC, bool, 16>;
impl R {
    #[doc = "Bits 0:9 - DMA Channel 1 Loop Width"]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 16 - DMA Channel 1 Loop Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - DMA Channel 1 Loop Width"]
    #[inline(always)]
    pub fn width(&mut self) -> WIDTH_W {
        WIDTH_W::new(self)
    }
    #[doc = "Bit 16 - DMA Channel 1 Loop Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel 1 Loop Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [loop1](index.html) module"]
pub struct LOOP1_SPEC;
impl crate::RegisterSpec for LOOP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [loop1::R](R) reader structure"]
impl crate::Readable for LOOP1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [loop1::W](W) writer structure"]
impl crate::Writable for LOOP1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LOOP1 to value 0"]
impl crate::Resettable for LOOP1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
