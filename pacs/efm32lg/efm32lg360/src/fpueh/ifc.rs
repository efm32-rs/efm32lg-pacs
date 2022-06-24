#[doc = "Register `IFC` reader"]
pub struct R(crate::R<IFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IFC_SPEC>) -> Self {
        R(reader)
    }
}
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
#[doc = "Field `FPIOC` reader - Clear FPIOC Interrupt Flag"]
pub type FPIOC_R = crate::BitReader<bool>;
#[doc = "Field `FPIOC` writer - Clear FPIOC Interrupt Flag"]
pub type FPIOC_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 0>;
#[doc = "Field `FPDZC` reader - Clear FPDZC Interrupt Flag"]
pub type FPDZC_R = crate::BitReader<bool>;
#[doc = "Field `FPDZC` writer - Clear FPDZC Interrupt Flag"]
pub type FPDZC_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 1>;
#[doc = "Field `FPUFC` reader - Clear FPUFC Interrupt Flag"]
pub type FPUFC_R = crate::BitReader<bool>;
#[doc = "Field `FPUFC` writer - Clear FPUFC Interrupt Flag"]
pub type FPUFC_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 2>;
#[doc = "Field `FPOFC` reader - Clear FPOFC Interrupt Flag"]
pub type FPOFC_R = crate::BitReader<bool>;
#[doc = "Field `FPOFC` writer - Clear FPOFC Interrupt Flag"]
pub type FPOFC_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 3>;
#[doc = "Field `FPIDC` reader - Clear FPIDC Interrupt Flag"]
pub type FPIDC_R = crate::BitReader<bool>;
#[doc = "Field `FPIDC` writer - Clear FPIDC Interrupt Flag"]
pub type FPIDC_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 4>;
#[doc = "Field `FPIXC` reader - Clear FPIXC Interrupt Flag"]
pub type FPIXC_R = crate::BitReader<bool>;
#[doc = "Field `FPIXC` writer - Clear FPIXC Interrupt Flag"]
pub type FPIXC_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 5>;
impl R {
    #[doc = "Bit 0 - Clear FPIOC Interrupt Flag"]
    #[inline(always)]
    pub fn fpioc(&self) -> FPIOC_R {
        FPIOC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear FPDZC Interrupt Flag"]
    #[inline(always)]
    pub fn fpdzc(&self) -> FPDZC_R {
        FPDZC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear FPUFC Interrupt Flag"]
    #[inline(always)]
    pub fn fpufc(&self) -> FPUFC_R {
        FPUFC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear FPOFC Interrupt Flag"]
    #[inline(always)]
    pub fn fpofc(&self) -> FPOFC_R {
        FPOFC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear FPIDC Interrupt Flag"]
    #[inline(always)]
    pub fn fpidc(&self) -> FPIDC_R {
        FPIDC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clear FPIXC Interrupt Flag"]
    #[inline(always)]
    pub fn fpixc(&self) -> FPIXC_R {
        FPIXC_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear FPIOC Interrupt Flag"]
    #[inline(always)]
    pub fn fpioc(&mut self) -> FPIOC_W {
        FPIOC_W::new(self)
    }
    #[doc = "Bit 1 - Clear FPDZC Interrupt Flag"]
    #[inline(always)]
    pub fn fpdzc(&mut self) -> FPDZC_W {
        FPDZC_W::new(self)
    }
    #[doc = "Bit 2 - Clear FPUFC Interrupt Flag"]
    #[inline(always)]
    pub fn fpufc(&mut self) -> FPUFC_W {
        FPUFC_W::new(self)
    }
    #[doc = "Bit 3 - Clear FPOFC Interrupt Flag"]
    #[inline(always)]
    pub fn fpofc(&mut self) -> FPOFC_W {
        FPOFC_W::new(self)
    }
    #[doc = "Bit 4 - Clear FPIDC Interrupt Flag"]
    #[inline(always)]
    pub fn fpidc(&mut self) -> FPIDC_W {
        FPIDC_W::new(self)
    }
    #[doc = "Bit 5 - Clear FPIXC Interrupt Flag"]
    #[inline(always)]
    pub fn fpixc(&mut self) -> FPIXC_W {
        FPIXC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifc](index.html) module"]
pub struct IFC_SPEC;
impl crate::RegisterSpec for IFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ifc::R](R) reader structure"]
impl crate::Readable for IFC_SPEC {
    type Reader = R;
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
