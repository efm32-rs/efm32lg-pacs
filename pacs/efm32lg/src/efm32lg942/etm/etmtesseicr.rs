#[doc = "Register `ETMTESSEICR` reader"]
pub struct R(crate::R<ETMTESSEICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETMTESSEICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETMTESSEICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETMTESSEICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETMTESSEICR` writer"]
pub struct W(crate::W<ETMTESSEICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETMTESSEICR_SPEC>;
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
impl From<crate::W<ETMTESSEICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETMTESSEICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STARTRSEL` reader - Stop Resource Selection"]
pub type STARTRSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STARTRSEL` writer - Stop Resource Selection"]
pub type STARTRSEL_W<'a> = crate::FieldWriter<'a, u32, ETMTESSEICR_SPEC, u8, u8, 4, 0>;
#[doc = "Field `STOPRSEL` reader - Stop Resource Selection"]
pub type STOPRSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STOPRSEL` writer - Stop Resource Selection"]
pub type STOPRSEL_W<'a> = crate::FieldWriter<'a, u32, ETMTESSEICR_SPEC, u8, u8, 4, 16>;
impl R {
    #[doc = "Bits 0:3 - Stop Resource Selection"]
    #[inline(always)]
    pub fn startrsel(&self) -> STARTRSEL_R {
        STARTRSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Stop Resource Selection"]
    #[inline(always)]
    pub fn stoprsel(&self) -> STOPRSEL_R {
        STOPRSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Stop Resource Selection"]
    #[inline(always)]
    pub fn startrsel(&mut self) -> STARTRSEL_W {
        STARTRSEL_W::new(self)
    }
    #[doc = "Bits 16:19 - Stop Resource Selection"]
    #[inline(always)]
    pub fn stoprsel(&mut self) -> STOPRSEL_W {
        STOPRSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TraceEnable Start/Stop EmbeddedICE Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmtesseicr](index.html) module"]
pub struct ETMTESSEICR_SPEC;
impl crate::RegisterSpec for ETMTESSEICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etmtesseicr::R](R) reader structure"]
impl crate::Readable for ETMTESSEICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etmtesseicr::W](W) writer structure"]
impl crate::Writable for ETMTESSEICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETMTESSEICR to value 0"]
impl crate::Resettable for ETMTESSEICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
