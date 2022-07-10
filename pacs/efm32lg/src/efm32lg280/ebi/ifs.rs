#[doc = "Register `IFS` writer"]
pub struct W(crate::W<IFS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFS_SPEC>;
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
impl From<crate::W<IFS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VSYNC` writer - Vertical Sync Interrupt Flag Set"]
pub type VSYNC_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 0>;
#[doc = "Field `HSYNC` writer - Horizontal Sync Interrupt Flag Set"]
pub type HSYNC_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 1>;
#[doc = "Field `VBPORCH` writer - Vertical Back Porch Interrupt Flag Set"]
pub type VBPORCH_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 2>;
#[doc = "Field `VFPORCH` writer - Vertical Front Porch Interrupt Flag Set"]
pub type VFPORCH_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 3>;
#[doc = "Field `DDEMPTY` writer - Direct Drive Data Empty Interrupt Flag Set"]
pub type DDEMPTY_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 4>;
#[doc = "Field `DDJIT` writer - Direct Drive Jitter Interrupt Flag Set"]
pub type DDJIT_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 5>;
impl W {
    #[doc = "Bit 0 - Vertical Sync Interrupt Flag Set"]
    #[inline(always)]
    pub fn vsync(&mut self) -> VSYNC_W {
        VSYNC_W::new(self)
    }
    #[doc = "Bit 1 - Horizontal Sync Interrupt Flag Set"]
    #[inline(always)]
    pub fn hsync(&mut self) -> HSYNC_W {
        HSYNC_W::new(self)
    }
    #[doc = "Bit 2 - Vertical Back Porch Interrupt Flag Set"]
    #[inline(always)]
    pub fn vbporch(&mut self) -> VBPORCH_W {
        VBPORCH_W::new(self)
    }
    #[doc = "Bit 3 - Vertical Front Porch Interrupt Flag Set"]
    #[inline(always)]
    pub fn vfporch(&mut self) -> VFPORCH_W {
        VFPORCH_W::new(self)
    }
    #[doc = "Bit 4 - Direct Drive Data Empty Interrupt Flag Set"]
    #[inline(always)]
    pub fn ddempty(&mut self) -> DDEMPTY_W {
        DDEMPTY_W::new(self)
    }
    #[doc = "Bit 5 - Direct Drive Jitter Interrupt Flag Set"]
    #[inline(always)]
    pub fn ddjit(&mut self) -> DDJIT_W {
        DDJIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifs](index.html) module"]
pub struct IFS_SPEC;
impl crate::RegisterSpec for IFS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ifs::W](W) writer structure"]
impl crate::Writable for IFS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
