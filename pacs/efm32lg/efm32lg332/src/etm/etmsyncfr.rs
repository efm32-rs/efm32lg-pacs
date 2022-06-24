#[doc = "Register `ETMSYNCFR` reader"]
pub struct R(crate::R<ETMSYNCFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETMSYNCFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETMSYNCFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETMSYNCFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETMSYNCFR` writer"]
pub struct W(crate::W<ETMSYNCFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETMSYNCFR_SPEC>;
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
impl From<crate::W<ETMSYNCFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETMSYNCFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FREQ` reader - Synchronisation Frequency Value"]
pub type FREQ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FREQ` writer - Synchronisation Frequency Value"]
pub type FREQ_W<'a> = crate::FieldWriter<'a, u32, ETMSYNCFR_SPEC, u16, u16, 12, 0>;
impl R {
    #[doc = "Bits 0:11 - Synchronisation Frequency Value"]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Synchronisation Frequency Value"]
    #[inline(always)]
    pub fn freq(&mut self) -> FREQ_W {
        FREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Synchronisation Frequency Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmsyncfr](index.html) module"]
pub struct ETMSYNCFR_SPEC;
impl crate::RegisterSpec for ETMSYNCFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etmsyncfr::R](R) reader structure"]
impl crate::Readable for ETMSYNCFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etmsyncfr::W](W) writer structure"]
impl crate::Writable for ETMSYNCFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETMSYNCFR to value 0x0400"]
impl crate::Resettable for ETMSYNCFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0400
    }
}
