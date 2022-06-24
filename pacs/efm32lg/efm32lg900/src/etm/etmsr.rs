#[doc = "Register `ETMSR` reader"]
pub struct R(crate::R<ETMSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETMSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETMSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETMSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETMSR` writer"]
pub struct W(crate::W<ETMSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETMSR_SPEC>;
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
impl From<crate::W<ETMSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETMSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETHOF` reader - ETM Overflow"]
pub type ETHOF_R = crate::BitReader<bool>;
#[doc = "Field `ETMPROGBIT` reader - ETM Programming Bit Status"]
pub type ETMPROGBIT_R = crate::BitReader<bool>;
#[doc = "Field `TRACESTAT` reader - Trace Start/Stop Status"]
pub type TRACESTAT_R = crate::BitReader<bool>;
#[doc = "Field `TRACESTAT` writer - Trace Start/Stop Status"]
pub type TRACESTAT_W<'a> = crate::BitWriter<'a, u32, ETMSR_SPEC, bool, 2>;
#[doc = "Field `TRIGBIT` reader - Trigger Bit"]
pub type TRIGBIT_R = crate::BitReader<bool>;
#[doc = "Field `TRIGBIT` writer - Trigger Bit"]
pub type TRIGBIT_W<'a> = crate::BitWriter<'a, u32, ETMSR_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - ETM Overflow"]
    #[inline(always)]
    pub fn ethof(&self) -> ETHOF_R {
        ETHOF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ETM Programming Bit Status"]
    #[inline(always)]
    pub fn etmprogbit(&self) -> ETMPROGBIT_R {
        ETMPROGBIT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Trace Start/Stop Status"]
    #[inline(always)]
    pub fn tracestat(&self) -> TRACESTAT_R {
        TRACESTAT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Trigger Bit"]
    #[inline(always)]
    pub fn trigbit(&self) -> TRIGBIT_R {
        TRIGBIT_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Trace Start/Stop Status"]
    #[inline(always)]
    pub fn tracestat(&mut self) -> TRACESTAT_W {
        TRACESTAT_W::new(self)
    }
    #[doc = "Bit 3 - Trigger Bit"]
    #[inline(always)]
    pub fn trigbit(&mut self) -> TRIGBIT_W {
        TRIGBIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ETM Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmsr](index.html) module"]
pub struct ETMSR_SPEC;
impl crate::RegisterSpec for ETMSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etmsr::R](R) reader structure"]
impl crate::Readable for ETMSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etmsr::W](W) writer structure"]
impl crate::Writable for ETMSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETMSR to value 0x02"]
impl crate::Resettable for ETMSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
