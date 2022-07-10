#[doc = "Register `IEN` reader"]
pub struct R(crate::R<IEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IEN` writer"]
pub struct W(crate::W<IEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IEN_SPEC>;
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
impl From<crate::W<IEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HFRCORDY` reader - HFRCO Ready Interrupt Enable"]
pub type HFRCORDY_R = crate::BitReader<bool>;
#[doc = "Field `HFRCORDY` writer - HFRCO Ready Interrupt Enable"]
pub type HFRCORDY_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 0>;
#[doc = "Field `HFXORDY` reader - HFXO Ready Interrupt Enable"]
pub type HFXORDY_R = crate::BitReader<bool>;
#[doc = "Field `HFXORDY` writer - HFXO Ready Interrupt Enable"]
pub type HFXORDY_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 1>;
#[doc = "Field `LFRCORDY` reader - LFRCO Ready Interrupt Enable"]
pub type LFRCORDY_R = crate::BitReader<bool>;
#[doc = "Field `LFRCORDY` writer - LFRCO Ready Interrupt Enable"]
pub type LFRCORDY_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 2>;
#[doc = "Field `LFXORDY` reader - LFXO Ready Interrupt Enable"]
pub type LFXORDY_R = crate::BitReader<bool>;
#[doc = "Field `LFXORDY` writer - LFXO Ready Interrupt Enable"]
pub type LFXORDY_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 3>;
#[doc = "Field `AUXHFRCORDY` reader - AUXHFRCO Ready Interrupt Enable"]
pub type AUXHFRCORDY_R = crate::BitReader<bool>;
#[doc = "Field `AUXHFRCORDY` writer - AUXHFRCO Ready Interrupt Enable"]
pub type AUXHFRCORDY_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 4>;
#[doc = "Field `CALRDY` reader - Calibration Ready Interrupt Enable"]
pub type CALRDY_R = crate::BitReader<bool>;
#[doc = "Field `CALRDY` writer - Calibration Ready Interrupt Enable"]
pub type CALRDY_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 5>;
#[doc = "Field `CALOF` reader - Calibration Overflow Interrupt Enable"]
pub type CALOF_R = crate::BitReader<bool>;
#[doc = "Field `CALOF` writer - Calibration Overflow Interrupt Enable"]
pub type CALOF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 6>;
impl R {
    #[doc = "Bit 0 - HFRCO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn hfrcordy(&self) -> HFRCORDY_R {
        HFRCORDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HFXO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn hfxordy(&self) -> HFXORDY_R {
        HFXORDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LFRCO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn lfrcordy(&self) -> LFRCORDY_R {
        LFRCORDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LFXO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn lfxordy(&self) -> LFXORDY_R {
        LFXORDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AUXHFRCO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn auxhfrcordy(&self) -> AUXHFRCORDY_R {
        AUXHFRCORDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Calibration Ready Interrupt Enable"]
    #[inline(always)]
    pub fn calrdy(&self) -> CALRDY_R {
        CALRDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Calibration Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn calof(&self) -> CALOF_R {
        CALOF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HFRCO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn hfrcordy(&mut self) -> HFRCORDY_W {
        HFRCORDY_W::new(self)
    }
    #[doc = "Bit 1 - HFXO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn hfxordy(&mut self) -> HFXORDY_W {
        HFXORDY_W::new(self)
    }
    #[doc = "Bit 2 - LFRCO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn lfrcordy(&mut self) -> LFRCORDY_W {
        LFRCORDY_W::new(self)
    }
    #[doc = "Bit 3 - LFXO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn lfxordy(&mut self) -> LFXORDY_W {
        LFXORDY_W::new(self)
    }
    #[doc = "Bit 4 - AUXHFRCO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn auxhfrcordy(&mut self) -> AUXHFRCORDY_W {
        AUXHFRCORDY_W::new(self)
    }
    #[doc = "Bit 5 - Calibration Ready Interrupt Enable"]
    #[inline(always)]
    pub fn calrdy(&mut self) -> CALRDY_W {
        CALRDY_W::new(self)
    }
    #[doc = "Bit 6 - Calibration Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn calof(&mut self) -> CALOF_W {
        CALOF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](index.html) module"]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ien::R](R) reader structure"]
impl crate::Readable for IEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ien::W](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
