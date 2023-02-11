#[doc = "Register `ETMTSEVR` reader"]
pub struct R(crate::R<ETMTSEVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETMTSEVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETMTSEVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETMTSEVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETMTSEVR` writer"]
pub struct W(crate::W<ETMTSEVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETMTSEVR_SPEC>;
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
impl From<crate::W<ETMTSEVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETMTSEVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESAEVT` reader - ETM Resource A Event"]
pub type RESAEVT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESAEVT` writer - ETM Resource A Event"]
pub type RESAEVT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETMTSEVR_SPEC, u8, u8, 7, O>;
#[doc = "Field `RESBEVT` reader - ETM Resource B Event"]
pub type RESBEVT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESBEVT` writer - ETM Resource B Event"]
pub type RESBEVT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETMTSEVR_SPEC, u8, u8, 7, O>;
#[doc = "Field `ETMFCNEVT` reader - ETM Function Event"]
pub type ETMFCNEVT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETMFCNEVT` writer - ETM Function Event"]
pub type ETMFCNEVT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETMTSEVR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:6 - ETM Resource A Event"]
    #[inline(always)]
    pub fn resaevt(&self) -> RESAEVT_R {
        RESAEVT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - ETM Resource B Event"]
    #[inline(always)]
    pub fn resbevt(&self) -> RESBEVT_R {
        RESBEVT_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:16 - ETM Function Event"]
    #[inline(always)]
    pub fn etmfcnevt(&self) -> ETMFCNEVT_R {
        ETMFCNEVT_R::new(((self.bits >> 14) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - ETM Resource A Event"]
    #[inline(always)]
    #[must_use]
    pub fn resaevt(&mut self) -> RESAEVT_W<0> {
        RESAEVT_W::new(self)
    }
    #[doc = "Bits 7:13 - ETM Resource B Event"]
    #[inline(always)]
    #[must_use]
    pub fn resbevt(&mut self) -> RESBEVT_W<7> {
        RESBEVT_W::new(self)
    }
    #[doc = "Bits 14:16 - ETM Function Event"]
    #[inline(always)]
    #[must_use]
    pub fn etmfcnevt(&mut self) -> ETMFCNEVT_W<14> {
        ETMFCNEVT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timestamp Event Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmtsevr](index.html) module"]
pub struct ETMTSEVR_SPEC;
impl crate::RegisterSpec for ETMTSEVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etmtsevr::R](R) reader structure"]
impl crate::Readable for ETMTSEVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etmtsevr::W](W) writer structure"]
impl crate::Writable for ETMTSEVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETMTSEVR to value 0"]
impl crate::Resettable for ETMTSEVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
