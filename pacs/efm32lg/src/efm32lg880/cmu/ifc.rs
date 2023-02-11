#[doc = "Register `IFC` writer"]
pub struct W(crate::W<IFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFC_SPEC>;
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
impl From<crate::W<IFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HFRCORDY` writer - HFRCO Ready Interrupt Flag Clear"]
pub type HFRCORDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `HFXORDY` writer - HFXO Ready Interrupt Flag Clear"]
pub type HFXORDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `LFRCORDY` writer - LFRCO Ready Interrupt Flag Clear"]
pub type LFRCORDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `LFXORDY` writer - LFXO Ready Interrupt Flag Clear"]
pub type LFXORDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `AUXHFRCORDY` writer - AUXHFRCO Ready Interrupt Flag Clear"]
pub type AUXHFRCORDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CALRDY` writer - Calibration Ready Interrupt Flag Clear"]
pub type CALRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CALOF` writer - Calibration Overflow Interrupt Flag Clear"]
pub type CALOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - HFRCO Ready Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hfrcordy(&mut self) -> HFRCORDY_W<0> {
        HFRCORDY_W::new(self)
    }
    #[doc = "Bit 1 - HFXO Ready Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hfxordy(&mut self) -> HFXORDY_W<1> {
        HFXORDY_W::new(self)
    }
    #[doc = "Bit 2 - LFRCO Ready Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn lfrcordy(&mut self) -> LFRCORDY_W<2> {
        LFRCORDY_W::new(self)
    }
    #[doc = "Bit 3 - LFXO Ready Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn lfxordy(&mut self) -> LFXORDY_W<3> {
        LFXORDY_W::new(self)
    }
    #[doc = "Bit 4 - AUXHFRCO Ready Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn auxhfrcordy(&mut self) -> AUXHFRCORDY_W<4> {
        AUXHFRCORDY_W::new(self)
    }
    #[doc = "Bit 5 - Calibration Ready Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn calrdy(&mut self) -> CALRDY_W<5> {
        CALRDY_W::new(self)
    }
    #[doc = "Bit 6 - Calibration Overflow Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn calof(&mut self) -> CALOF_W<6> {
        CALOF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifc](index.html) module"]
pub struct IFC_SPEC;
impl crate::RegisterSpec for IFC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ifc::W](W) writer structure"]
impl crate::Writable for IFC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
