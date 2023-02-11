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
#[doc = "Field `FPIOC` reader - FPIOC Interrupt Enable"]
pub type FPIOC_R = crate::BitReader<bool>;
#[doc = "Field `FPIOC` writer - FPIOC Interrupt Enable"]
pub type FPIOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `FPDZC` reader - FPDZC Interrupt Enable"]
pub type FPDZC_R = crate::BitReader<bool>;
#[doc = "Field `FPDZC` writer - FPDZC Interrupt Enable"]
pub type FPDZC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `FPUFC` reader - FPUFC Interrupt Enable"]
pub type FPUFC_R = crate::BitReader<bool>;
#[doc = "Field `FPUFC` writer - FPUFC Interrupt Enable"]
pub type FPUFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `FPOFC` reader - FPOFC Interrupt Enable"]
pub type FPOFC_R = crate::BitReader<bool>;
#[doc = "Field `FPOFC` writer - FPOFC Interrupt Enable"]
pub type FPOFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `FPIDC` reader - FPIDC Interrupt Enable"]
pub type FPIDC_R = crate::BitReader<bool>;
#[doc = "Field `FPIDC` writer - FPIDC Interrupt Enable"]
pub type FPIDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `FPIXC` reader - FPIXC Interrupt Enable"]
pub type FPIXC_R = crate::BitReader<bool>;
#[doc = "Field `FPIXC` writer - FPIXC Interrupt Enable"]
pub type FPIXC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - FPIOC Interrupt Enable"]
    #[inline(always)]
    pub fn fpioc(&self) -> FPIOC_R {
        FPIOC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FPDZC Interrupt Enable"]
    #[inline(always)]
    pub fn fpdzc(&self) -> FPDZC_R {
        FPDZC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FPUFC Interrupt Enable"]
    #[inline(always)]
    pub fn fpufc(&self) -> FPUFC_R {
        FPUFC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FPOFC Interrupt Enable"]
    #[inline(always)]
    pub fn fpofc(&self) -> FPOFC_R {
        FPOFC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FPIDC Interrupt Enable"]
    #[inline(always)]
    pub fn fpidc(&self) -> FPIDC_R {
        FPIDC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FPIXC Interrupt Enable"]
    #[inline(always)]
    pub fn fpixc(&self) -> FPIXC_R {
        FPIXC_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FPIOC Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpioc(&mut self) -> FPIOC_W<0> {
        FPIOC_W::new(self)
    }
    #[doc = "Bit 1 - FPDZC Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpdzc(&mut self) -> FPDZC_W<1> {
        FPDZC_W::new(self)
    }
    #[doc = "Bit 2 - FPUFC Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpufc(&mut self) -> FPUFC_W<2> {
        FPUFC_W::new(self)
    }
    #[doc = "Bit 3 - FPOFC Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpofc(&mut self) -> FPOFC_W<3> {
        FPOFC_W::new(self)
    }
    #[doc = "Bit 4 - FPIDC Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpidc(&mut self) -> FPIDC_W<4> {
        FPIDC_W::new(self)
    }
    #[doc = "Bit 5 - FPIXC Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpixc(&mut self) -> FPIXC_W<5> {
        FPIXC_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
