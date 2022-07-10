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
#[doc = "Field `SINGLE` writer - Single Conversion Complete Interrupt Flag Clear"]
pub type SINGLE_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 0>;
#[doc = "Field `SCAN` writer - Scan Conversion Complete Interrupt Flag Clear"]
pub type SCAN_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 1>;
#[doc = "Field `SINGLEOF` writer - Single Result Overflow Interrupt Flag Clear"]
pub type SINGLEOF_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 8>;
#[doc = "Field `SCANOF` writer - Scan Result Overflow Interrupt Flag Clear"]
pub type SCANOF_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 9>;
impl W {
    #[doc = "Bit 0 - Single Conversion Complete Interrupt Flag Clear"]
    #[inline(always)]
    pub fn single(&mut self) -> SINGLE_W {
        SINGLE_W::new(self)
    }
    #[doc = "Bit 1 - Scan Conversion Complete Interrupt Flag Clear"]
    #[inline(always)]
    pub fn scan(&mut self) -> SCAN_W {
        SCAN_W::new(self)
    }
    #[doc = "Bit 8 - Single Result Overflow Interrupt Flag Clear"]
    #[inline(always)]
    pub fn singleof(&mut self) -> SINGLEOF_W {
        SINGLEOF_W::new(self)
    }
    #[doc = "Bit 9 - Scan Result Overflow Interrupt Flag Clear"]
    #[inline(always)]
    pub fn scanof(&mut self) -> SCANOF_W {
        SCANOF_W::new(self)
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
