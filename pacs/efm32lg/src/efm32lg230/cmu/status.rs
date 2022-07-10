#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HFRCOENS` reader - HFRCO Enable Status"]
pub type HFRCOENS_R = crate::BitReader<bool>;
#[doc = "Field `HFRCORDY` reader - HFRCO Ready"]
pub type HFRCORDY_R = crate::BitReader<bool>;
#[doc = "Field `HFXOENS` reader - HFXO Enable Status"]
pub type HFXOENS_R = crate::BitReader<bool>;
#[doc = "Field `HFXORDY` reader - HFXO Ready"]
pub type HFXORDY_R = crate::BitReader<bool>;
#[doc = "Field `AUXHFRCOENS` reader - AUXHFRCO Enable Status"]
pub type AUXHFRCOENS_R = crate::BitReader<bool>;
#[doc = "Field `AUXHFRCORDY` reader - AUXHFRCO Ready"]
pub type AUXHFRCORDY_R = crate::BitReader<bool>;
#[doc = "Field `LFRCOENS` reader - LFRCO Enable Status"]
pub type LFRCOENS_R = crate::BitReader<bool>;
#[doc = "Field `LFRCORDY` reader - LFRCO Ready"]
pub type LFRCORDY_R = crate::BitReader<bool>;
#[doc = "Field `LFXOENS` reader - LFXO Enable Status"]
pub type LFXOENS_R = crate::BitReader<bool>;
#[doc = "Field `LFXORDY` reader - LFXO Ready"]
pub type LFXORDY_R = crate::BitReader<bool>;
#[doc = "Field `HFRCOSEL` reader - HFRCO Selected"]
pub type HFRCOSEL_R = crate::BitReader<bool>;
#[doc = "Field `HFXOSEL` reader - HFXO Selected"]
pub type HFXOSEL_R = crate::BitReader<bool>;
#[doc = "Field `LFRCOSEL` reader - LFRCO Selected"]
pub type LFRCOSEL_R = crate::BitReader<bool>;
#[doc = "Field `LFXOSEL` reader - LFXO Selected"]
pub type LFXOSEL_R = crate::BitReader<bool>;
#[doc = "Field `CALBSY` reader - Calibration Busy"]
pub type CALBSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - HFRCO Enable Status"]
    #[inline(always)]
    pub fn hfrcoens(&self) -> HFRCOENS_R {
        HFRCOENS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HFRCO Ready"]
    #[inline(always)]
    pub fn hfrcordy(&self) -> HFRCORDY_R {
        HFRCORDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HFXO Enable Status"]
    #[inline(always)]
    pub fn hfxoens(&self) -> HFXOENS_R {
        HFXOENS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HFXO Ready"]
    #[inline(always)]
    pub fn hfxordy(&self) -> HFXORDY_R {
        HFXORDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AUXHFRCO Enable Status"]
    #[inline(always)]
    pub fn auxhfrcoens(&self) -> AUXHFRCOENS_R {
        AUXHFRCOENS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AUXHFRCO Ready"]
    #[inline(always)]
    pub fn auxhfrcordy(&self) -> AUXHFRCORDY_R {
        AUXHFRCORDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LFRCO Enable Status"]
    #[inline(always)]
    pub fn lfrcoens(&self) -> LFRCOENS_R {
        LFRCOENS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LFRCO Ready"]
    #[inline(always)]
    pub fn lfrcordy(&self) -> LFRCORDY_R {
        LFRCORDY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LFXO Enable Status"]
    #[inline(always)]
    pub fn lfxoens(&self) -> LFXOENS_R {
        LFXOENS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LFXO Ready"]
    #[inline(always)]
    pub fn lfxordy(&self) -> LFXORDY_R {
        LFXORDY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HFRCO Selected"]
    #[inline(always)]
    pub fn hfrcosel(&self) -> HFRCOSEL_R {
        HFRCOSEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HFXO Selected"]
    #[inline(always)]
    pub fn hfxosel(&self) -> HFXOSEL_R {
        HFXOSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LFRCO Selected"]
    #[inline(always)]
    pub fn lfrcosel(&self) -> LFRCOSEL_R {
        LFRCOSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LFXO Selected"]
    #[inline(always)]
    pub fn lfxosel(&self) -> LFXOSEL_R {
        LFXOSEL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Calibration Busy"]
    #[inline(always)]
    pub fn calbsy(&self) -> CALBSY_R {
        CALBSY_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0x0403"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0403
    }
}
