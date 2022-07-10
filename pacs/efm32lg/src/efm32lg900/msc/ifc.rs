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
#[doc = "Field `ERASE` writer - Erase Done Interrupt Clear"]
pub type ERASE_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 0>;
#[doc = "Field `WRITE` writer - Write Done Interrupt Clear"]
pub type WRITE_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 1>;
#[doc = "Field `CHOF` writer - Cache Hits Overflow Interrupt Clear"]
pub type CHOF_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 2>;
#[doc = "Field `CMOF` writer - Cache Misses Overflow Interrupt Clear"]
pub type CMOF_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 3>;
impl W {
    #[doc = "Bit 0 - Erase Done Interrupt Clear"]
    #[inline(always)]
    pub fn erase(&mut self) -> ERASE_W {
        ERASE_W::new(self)
    }
    #[doc = "Bit 1 - Write Done Interrupt Clear"]
    #[inline(always)]
    pub fn write(&mut self) -> WRITE_W {
        WRITE_W::new(self)
    }
    #[doc = "Bit 2 - Cache Hits Overflow Interrupt Clear"]
    #[inline(always)]
    pub fn chof(&mut self) -> CHOF_W {
        CHOF_W::new(self)
    }
    #[doc = "Bit 3 - Cache Misses Overflow Interrupt Clear"]
    #[inline(always)]
    pub fn cmof(&mut self) -> CMOF_W {
        CMOF_W::new(self)
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
