#[doc = "Register `ETMTRIGGER` reader"]
pub struct R(crate::R<ETMTRIGGER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETMTRIGGER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETMTRIGGER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETMTRIGGER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETMTRIGGER` writer"]
pub struct W(crate::W<ETMTRIGGER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETMTRIGGER_SPEC>;
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
impl From<crate::W<ETMTRIGGER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETMTRIGGER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESA` reader - ETM Resource A"]
pub type RESA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESA` writer - ETM Resource A"]
pub type RESA_W<'a> = crate::FieldWriter<'a, u32, ETMTRIGGER_SPEC, u8, u8, 7, 0>;
#[doc = "Field `RESB` reader - ETM Resource B"]
pub type RESB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESB` writer - ETM Resource B"]
pub type RESB_W<'a> = crate::FieldWriter<'a, u32, ETMTRIGGER_SPEC, u8, u8, 7, 7>;
#[doc = "Field `ETMFCN` reader - ETM Function"]
pub type ETMFCN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETMFCN` writer - ETM Function"]
pub type ETMFCN_W<'a> = crate::FieldWriter<'a, u32, ETMTRIGGER_SPEC, u8, u8, 3, 14>;
impl R {
    #[doc = "Bits 0:6 - ETM Resource A"]
    #[inline(always)]
    pub fn resa(&self) -> RESA_R {
        RESA_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - ETM Resource B"]
    #[inline(always)]
    pub fn resb(&self) -> RESB_R {
        RESB_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:16 - ETM Function"]
    #[inline(always)]
    pub fn etmfcn(&self) -> ETMFCN_R {
        ETMFCN_R::new(((self.bits >> 14) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - ETM Resource A"]
    #[inline(always)]
    pub fn resa(&mut self) -> RESA_W {
        RESA_W::new(self)
    }
    #[doc = "Bits 7:13 - ETM Resource B"]
    #[inline(always)]
    pub fn resb(&mut self) -> RESB_W {
        RESB_W::new(self)
    }
    #[doc = "Bits 14:16 - ETM Function"]
    #[inline(always)]
    pub fn etmfcn(&mut self) -> ETMFCN_W {
        ETMFCN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ETM Trigger Event Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmtrigger](index.html) module"]
pub struct ETMTRIGGER_SPEC;
impl crate::RegisterSpec for ETMTRIGGER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etmtrigger::R](R) reader structure"]
impl crate::Readable for ETMTRIGGER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etmtrigger::W](W) writer structure"]
impl crate::Writable for ETMTRIGGER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETMTRIGGER to value 0"]
impl crate::Resettable for ETMTRIGGER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
