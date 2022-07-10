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
#[doc = "Field `CH0` writer - Channel 0 Conversion Complete Interrupt Flag Clear"]
pub type CH0_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 0>;
#[doc = "Field `CH1` writer - Channel 1 Conversion Complete Interrupt Flag Clear"]
pub type CH1_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 1>;
#[doc = "Field `CH0UF` writer - Channel 0 Data Underflow Interrupt Flag Clear"]
pub type CH0UF_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 4>;
#[doc = "Field `CH1UF` writer - Channel 1 Data Underflow Interrupt Flag Clear"]
pub type CH1UF_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 5>;
impl W {
    #[doc = "Bit 0 - Channel 0 Conversion Complete Interrupt Flag Clear"]
    #[inline(always)]
    pub fn ch0(&mut self) -> CH0_W {
        CH0_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Conversion Complete Interrupt Flag Clear"]
    #[inline(always)]
    pub fn ch1(&mut self) -> CH1_W {
        CH1_W::new(self)
    }
    #[doc = "Bit 4 - Channel 0 Data Underflow Interrupt Flag Clear"]
    #[inline(always)]
    pub fn ch0uf(&mut self) -> CH0UF_W {
        CH0UF_W::new(self)
    }
    #[doc = "Bit 5 - Channel 1 Data Underflow Interrupt Flag Clear"]
    #[inline(always)]
    pub fn ch1uf(&mut self) -> CH1UF_W {
        CH1UF_W::new(self)
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
