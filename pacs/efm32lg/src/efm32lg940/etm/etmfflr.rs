#[doc = "Register `ETMFFLR` reader"]
pub struct R(crate::R<ETMFFLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETMFFLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETMFFLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETMFFLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETMFFLR` writer"]
pub struct W(crate::W<ETMFFLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETMFFLR_SPEC>;
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
impl From<crate::W<ETMFFLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETMFFLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYTENUM` reader - Bytes left in FIFO"]
pub type BYTENUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BYTENUM` writer - Bytes left in FIFO"]
pub type BYTENUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETMFFLR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Bytes left in FIFO"]
    #[inline(always)]
    pub fn bytenum(&self) -> BYTENUM_R {
        BYTENUM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bytes left in FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn bytenum(&mut self) -> BYTENUM_W<0> {
        BYTENUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ETM Fifo Full Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmfflr](index.html) module"]
pub struct ETMFFLR_SPEC;
impl crate::RegisterSpec for ETMFFLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etmfflr::R](R) reader structure"]
impl crate::Readable for ETMFFLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etmfflr::W](W) writer structure"]
impl crate::Writable for ETMFFLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETMFFLR to value 0"]
impl crate::Resettable for ETMFFLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
