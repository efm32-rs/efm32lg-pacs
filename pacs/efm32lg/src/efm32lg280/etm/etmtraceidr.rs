#[doc = "Register `ETMTRACEIDR` reader"]
pub struct R(crate::R<ETMTRACEIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETMTRACEIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETMTRACEIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETMTRACEIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETMTRACEIDR` writer"]
pub struct W(crate::W<ETMTRACEIDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETMTRACEIDR_SPEC>;
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
impl From<crate::W<ETMTRACEIDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETMTRACEIDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRACEID` reader - Trace ID"]
pub type TRACEID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRACEID` writer - Trace ID"]
pub type TRACEID_W<'a> = crate::FieldWriter<'a, u32, ETMTRACEIDR_SPEC, u8, u8, 7, 0>;
impl R {
    #[doc = "Bits 0:6 - Trace ID"]
    #[inline(always)]
    pub fn traceid(&self) -> TRACEID_R {
        TRACEID_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Trace ID"]
    #[inline(always)]
    pub fn traceid(&mut self) -> TRACEID_W {
        TRACEID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CoreSight Trace ID Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmtraceidr](index.html) module"]
pub struct ETMTRACEIDR_SPEC;
impl crate::RegisterSpec for ETMTRACEIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etmtraceidr::R](R) reader structure"]
impl crate::Readable for ETMTRACEIDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etmtraceidr::W](W) writer structure"]
impl crate::Writable for ETMTRACEIDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETMTRACEIDR to value 0"]
impl crate::Resettable for ETMTRACEIDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
