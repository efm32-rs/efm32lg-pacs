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
#[doc = "Field `VSYNC` reader - Vertical Sync Interrupt Flag"]
pub type VSYNC_R = crate::BitReader<bool>;
#[doc = "Field `HSYNC` reader - Horizontal Sync Interrupt Flag"]
pub type HSYNC_R = crate::BitReader<bool>;
#[doc = "Field `VBPORCH` reader - Vertical Back Porch Interrupt Flag"]
pub type VBPORCH_R = crate::BitReader<bool>;
#[doc = "Field `VFPORCH` reader - Vertical Front Porch Interrupt Flag"]
pub type VFPORCH_R = crate::BitReader<bool>;
#[doc = "Field `DDEMPTY` reader - Direct Drive Data Empty Interrupt Flag"]
pub type DDEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `DDJIT` reader - Direct Drive Jitter Interrupt Flag"]
pub type DDJIT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Vertical Sync Interrupt Flag"]
    #[inline(always)]
    pub fn vsync(&self) -> VSYNC_R {
        VSYNC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Horizontal Sync Interrupt Flag"]
    #[inline(always)]
    pub fn hsync(&self) -> HSYNC_R {
        HSYNC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Vertical Back Porch Interrupt Flag"]
    #[inline(always)]
    pub fn vbporch(&self) -> VBPORCH_R {
        VBPORCH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Vertical Front Porch Interrupt Flag"]
    #[inline(always)]
    pub fn vfporch(&self) -> VFPORCH_R {
        VFPORCH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Direct Drive Data Empty Interrupt Flag"]
    #[inline(always)]
    pub fn ddempty(&self) -> DDEMPTY_R {
        DDEMPTY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Direct Drive Jitter Interrupt Flag"]
    #[inline(always)]
    pub fn ddjit(&self) -> DDJIT_R {
        DDJIT_R::new(((self.bits >> 5) & 1) != 0)
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
