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
#[doc = "Field `FPIOC` reader - FPU invalid operation"]
pub type FPIOC_R = crate::BitReader<bool>;
#[doc = "Field `FPDZC` reader - FPU divide-by-zero exception"]
pub type FPDZC_R = crate::BitReader<bool>;
#[doc = "Field `FPUFC` reader - FPU underflow exception"]
pub type FPUFC_R = crate::BitReader<bool>;
#[doc = "Field `FPOFC` reader - FPU overflow exception"]
pub type FPOFC_R = crate::BitReader<bool>;
#[doc = "Field `FPIDC` reader - FPU input denormal exception"]
pub type FPIDC_R = crate::BitReader<bool>;
#[doc = "Field `FPIXC` reader - FPU inexact exception"]
pub type FPIXC_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - FPU invalid operation"]
    #[inline(always)]
    pub fn fpioc(&self) -> FPIOC_R {
        FPIOC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FPU divide-by-zero exception"]
    #[inline(always)]
    pub fn fpdzc(&self) -> FPDZC_R {
        FPDZC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FPU underflow exception"]
    #[inline(always)]
    pub fn fpufc(&self) -> FPUFC_R {
        FPUFC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FPU overflow exception"]
    #[inline(always)]
    pub fn fpofc(&self) -> FPOFC_R {
        FPOFC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FPU input denormal exception"]
    #[inline(always)]
    pub fn fpidc(&self) -> FPIDC_R {
        FPIDC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FPU inexact exception"]
    #[inline(always)]
    pub fn fpixc(&self) -> FPIXC_R {
        FPIXC_R::new(((self.bits >> 5) & 1) != 0)
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
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
