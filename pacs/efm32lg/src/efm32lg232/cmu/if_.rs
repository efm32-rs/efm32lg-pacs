#[doc = "Register `IF` reader"]
pub struct R(crate::R<IF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HFRCORDY` reader - HFRCO Ready Interrupt Flag"]
pub type HFRCORDY_R = crate::BitReader<bool>;
#[doc = "Field `HFXORDY` reader - HFXO Ready Interrupt Flag"]
pub type HFXORDY_R = crate::BitReader<bool>;
#[doc = "Field `LFRCORDY` reader - LFRCO Ready Interrupt Flag"]
pub type LFRCORDY_R = crate::BitReader<bool>;
#[doc = "Field `LFXORDY` reader - LFXO Ready Interrupt Flag"]
pub type LFXORDY_R = crate::BitReader<bool>;
#[doc = "Field `AUXHFRCORDY` reader - AUXHFRCO Ready Interrupt Flag"]
pub type AUXHFRCORDY_R = crate::BitReader<bool>;
#[doc = "Field `CALRDY` reader - Calibration Ready Interrupt Flag"]
pub type CALRDY_R = crate::BitReader<bool>;
#[doc = "Field `CALOF` reader - Calibration Overflow Interrupt Flag"]
pub type CALOF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - HFRCO Ready Interrupt Flag"]
    #[inline(always)]
    pub fn hfrcordy(&self) -> HFRCORDY_R {
        HFRCORDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HFXO Ready Interrupt Flag"]
    #[inline(always)]
    pub fn hfxordy(&self) -> HFXORDY_R {
        HFXORDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LFRCO Ready Interrupt Flag"]
    #[inline(always)]
    pub fn lfrcordy(&self) -> LFRCORDY_R {
        LFRCORDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LFXO Ready Interrupt Flag"]
    #[inline(always)]
    pub fn lfxordy(&self) -> LFXORDY_R {
        LFXORDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AUXHFRCO Ready Interrupt Flag"]
    #[inline(always)]
    pub fn auxhfrcordy(&self) -> AUXHFRCORDY_R {
        AUXHFRCORDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Calibration Ready Interrupt Flag"]
    #[inline(always)]
    pub fn calrdy(&self) -> CALRDY_R {
        CALRDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Calibration Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn calof(&self) -> CALOF_R {
        CALOF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](index.html) module"]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if_::R](R) reader structure"]
impl crate::Readable for IF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IF to value 0x01"]
impl crate::Resettable for IF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
