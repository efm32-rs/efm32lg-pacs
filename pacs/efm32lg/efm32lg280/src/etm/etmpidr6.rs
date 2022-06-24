#[doc = "Register `ETMPIDR6` writer"]
pub struct W(crate::W<ETMPIDR6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETMPIDR6_SPEC>;
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
impl From<crate::W<ETMPIDR6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETMPIDR6_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral ID6 Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmpidr6](index.html) module"]
pub struct ETMPIDR6_SPEC;
impl crate::RegisterSpec for ETMPIDR6_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [etmpidr6::W](W) writer structure"]
impl crate::Writable for ETMPIDR6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETMPIDR6 to value 0"]
impl crate::Resettable for ETMPIDR6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
