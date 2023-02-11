#[doc = "Register `ETMCNTRLDVR1` reader"]
pub struct R(crate::R<ETMCNTRLDVR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETMCNTRLDVR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETMCNTRLDVR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETMCNTRLDVR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETMCNTRLDVR1` writer"]
pub struct W(crate::W<ETMCNTRLDVR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETMCNTRLDVR1_SPEC>;
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
impl From<crate::W<ETMCNTRLDVR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETMCNTRLDVR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT` reader - Free running counter reload value"]
pub type COUNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COUNT` writer - Free running counter reload value"]
pub type COUNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETMCNTRLDVR1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Free running counter reload value"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Free running counter reload value"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<0> {
        COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter Reload Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmcntrldvr1](index.html) module"]
pub struct ETMCNTRLDVR1_SPEC;
impl crate::RegisterSpec for ETMCNTRLDVR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etmcntrldvr1::R](R) reader structure"]
impl crate::Readable for ETMCNTRLDVR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etmcntrldvr1::W](W) writer structure"]
impl crate::Writable for ETMCNTRLDVR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETMCNTRLDVR1 to value 0"]
impl crate::Resettable for ETMCNTRLDVR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
