#[doc = "Register `ETMTEEVR` reader"]
pub struct R(crate::R<ETMTEEVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETMTEEVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETMTEEVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETMTEEVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETMTEEVR` writer"]
pub struct W(crate::W<ETMTEEVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETMTEEVR_SPEC>;
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
impl From<crate::W<ETMTEEVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETMTEEVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESA` reader - ETM Resource A Trace Enable"]
pub type RESA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESA` writer - ETM Resource A Trace Enable"]
pub type RESA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETMTEEVR_SPEC, u8, u8, 7, O>;
#[doc = "Field `RESB` reader - ETM Resource B Trace Enable"]
pub type RESB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESB` writer - ETM Resource B Trace Enable"]
pub type RESB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETMTEEVR_SPEC, u8, u8, 7, O>;
#[doc = "Field `ETMFCNEN` reader - ETM Function Trace Enable"]
pub type ETMFCNEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETMFCNEN` writer - ETM Function Trace Enable"]
pub type ETMFCNEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETMTEEVR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:6 - ETM Resource A Trace Enable"]
    #[inline(always)]
    pub fn resa(&self) -> RESA_R {
        RESA_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - ETM Resource B Trace Enable"]
    #[inline(always)]
    pub fn resb(&self) -> RESB_R {
        RESB_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:16 - ETM Function Trace Enable"]
    #[inline(always)]
    pub fn etmfcnen(&self) -> ETMFCNEN_R {
        ETMFCNEN_R::new(((self.bits >> 14) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - ETM Resource A Trace Enable"]
    #[inline(always)]
    #[must_use]
    pub fn resa(&mut self) -> RESA_W<0> {
        RESA_W::new(self)
    }
    #[doc = "Bits 7:13 - ETM Resource B Trace Enable"]
    #[inline(always)]
    #[must_use]
    pub fn resb(&mut self) -> RESB_W<7> {
        RESB_W::new(self)
    }
    #[doc = "Bits 14:16 - ETM Function Trace Enable"]
    #[inline(always)]
    #[must_use]
    pub fn etmfcnen(&mut self) -> ETMFCNEN_W<14> {
        ETMFCNEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ETM TraceEnable Event Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmteevr](index.html) module"]
pub struct ETMTEEVR_SPEC;
impl crate::RegisterSpec for ETMTEEVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etmteevr::R](R) reader structure"]
impl crate::Readable for ETMTEEVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etmteevr::W](W) writer structure"]
impl crate::Writable for ETMTEEVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETMTEEVR to value 0"]
impl crate::Resettable for ETMTEEVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
