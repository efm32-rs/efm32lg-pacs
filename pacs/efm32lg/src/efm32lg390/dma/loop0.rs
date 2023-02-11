#[doc = "Register `LOOP0` reader"]
pub struct R(crate::R<LOOP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOOP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOOP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOOP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOOP0` writer"]
pub struct W(crate::W<LOOP0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOOP0_SPEC>;
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
impl From<crate::W<LOOP0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOOP0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WIDTH` reader - Loop Width"]
pub type WIDTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WIDTH` writer - Loop Width"]
pub type WIDTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LOOP0_SPEC, u16, u16, 10, O>;
#[doc = "Field `EN` reader - DMA Channel 0 Loop Enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - DMA Channel 0 Loop Enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOOP0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9 - Loop Width"]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 16 - DMA Channel 0 Loop Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Loop Width"]
    #[inline(always)]
    #[must_use]
    pub fn width(&mut self) -> WIDTH_W<0> {
        WIDTH_W::new(self)
    }
    #[doc = "Bit 16 - DMA Channel 0 Loop Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<16> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel 0 Loop Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [loop0](index.html) module"]
pub struct LOOP0_SPEC;
impl crate::RegisterSpec for LOOP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [loop0::R](R) reader structure"]
impl crate::Readable for LOOP0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [loop0::W](W) writer structure"]
impl crate::Writable for LOOP0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOOP0 to value 0"]
impl crate::Resettable for LOOP0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
