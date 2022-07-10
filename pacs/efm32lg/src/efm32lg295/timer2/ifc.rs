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
#[doc = "Field `OF` writer - Overflow Interrupt Flag Clear"]
pub type OF_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 0>;
#[doc = "Field `UF` writer - Underflow Interrupt Flag Clear"]
pub type UF_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 1>;
#[doc = "Field `CC0` writer - CC Channel 0 Interrupt Flag Clear"]
pub type CC0_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 4>;
#[doc = "Field `CC1` writer - CC Channel 1 Interrupt Flag Clear"]
pub type CC1_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 5>;
#[doc = "Field `CC2` writer - CC Channel 2 Interrupt Flag Clear"]
pub type CC2_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 6>;
#[doc = "Field `ICBOF0` writer - CC Channel 0 Input Capture Buffer Overflow Interrupt Flag Clear"]
pub type ICBOF0_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 8>;
#[doc = "Field `ICBOF1` writer - CC Channel 1 Input Capture Buffer Overflow Interrupt Flag Clear"]
pub type ICBOF1_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 9>;
#[doc = "Field `ICBOF2` writer - CC Channel 2 Input Capture Buffer Overflow Interrupt Flag Clear"]
pub type ICBOF2_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 10>;
impl W {
    #[doc = "Bit 0 - Overflow Interrupt Flag Clear"]
    #[inline(always)]
    pub fn of(&mut self) -> OF_W {
        OF_W::new(self)
    }
    #[doc = "Bit 1 - Underflow Interrupt Flag Clear"]
    #[inline(always)]
    pub fn uf(&mut self) -> UF_W {
        UF_W::new(self)
    }
    #[doc = "Bit 4 - CC Channel 0 Interrupt Flag Clear"]
    #[inline(always)]
    pub fn cc0(&mut self) -> CC0_W {
        CC0_W::new(self)
    }
    #[doc = "Bit 5 - CC Channel 1 Interrupt Flag Clear"]
    #[inline(always)]
    pub fn cc1(&mut self) -> CC1_W {
        CC1_W::new(self)
    }
    #[doc = "Bit 6 - CC Channel 2 Interrupt Flag Clear"]
    #[inline(always)]
    pub fn cc2(&mut self) -> CC2_W {
        CC2_W::new(self)
    }
    #[doc = "Bit 8 - CC Channel 0 Input Capture Buffer Overflow Interrupt Flag Clear"]
    #[inline(always)]
    pub fn icbof0(&mut self) -> ICBOF0_W {
        ICBOF0_W::new(self)
    }
    #[doc = "Bit 9 - CC Channel 1 Input Capture Buffer Overflow Interrupt Flag Clear"]
    #[inline(always)]
    pub fn icbof1(&mut self) -> ICBOF1_W {
        ICBOF1_W::new(self)
    }
    #[doc = "Bit 10 - CC Channel 2 Input Capture Buffer Overflow Interrupt Flag Clear"]
    #[inline(always)]
    pub fn icbof2(&mut self) -> ICBOF2_W {
        ICBOF2_W::new(self)
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
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IFC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
