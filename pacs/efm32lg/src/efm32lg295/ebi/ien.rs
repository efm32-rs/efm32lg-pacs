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
#[doc = "Field `VSYNC` reader - Vertical Sync Interrupt Enable"]
pub type VSYNC_R = crate::BitReader<bool>;
#[doc = "Field `VSYNC` writer - Vertical Sync Interrupt Enable"]
pub type VSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `HSYNC` reader - Horizontal Sync Interrupt Enable"]
pub type HSYNC_R = crate::BitReader<bool>;
#[doc = "Field `HSYNC` writer - Horizontal Sync Interrupt Enable"]
pub type HSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `VBPORCH` reader - Vertical Back Porch Interrupt Enable"]
pub type VBPORCH_R = crate::BitReader<bool>;
#[doc = "Field `VBPORCH` writer - Vertical Back Porch Interrupt Enable"]
pub type VBPORCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `VFPORCH` reader - Vertical Front Porch Interrupt Enable"]
pub type VFPORCH_R = crate::BitReader<bool>;
#[doc = "Field `VFPORCH` writer - Vertical Front Porch Interrupt Enable"]
pub type VFPORCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `DDEMPTY` reader - Direct Drive Data Empty Interrupt Enable"]
pub type DDEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `DDEMPTY` writer - Direct Drive Data Empty Interrupt Enable"]
pub type DDEMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `DDJIT` reader - Direct Drive Jitter Interrupt Enable"]
pub type DDJIT_R = crate::BitReader<bool>;
#[doc = "Field `DDJIT` writer - Direct Drive Jitter Interrupt Enable"]
pub type DDJIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Vertical Sync Interrupt Enable"]
    #[inline(always)]
    pub fn vsync(&self) -> VSYNC_R {
        VSYNC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Horizontal Sync Interrupt Enable"]
    #[inline(always)]
    pub fn hsync(&self) -> HSYNC_R {
        HSYNC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Vertical Back Porch Interrupt Enable"]
    #[inline(always)]
    pub fn vbporch(&self) -> VBPORCH_R {
        VBPORCH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Vertical Front Porch Interrupt Enable"]
    #[inline(always)]
    pub fn vfporch(&self) -> VFPORCH_R {
        VFPORCH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Direct Drive Data Empty Interrupt Enable"]
    #[inline(always)]
    pub fn ddempty(&self) -> DDEMPTY_R {
        DDEMPTY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Direct Drive Jitter Interrupt Enable"]
    #[inline(always)]
    pub fn ddjit(&self) -> DDJIT_R {
        DDJIT_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Vertical Sync Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vsync(&mut self) -> VSYNC_W<0> {
        VSYNC_W::new(self)
    }
    #[doc = "Bit 1 - Horizontal Sync Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsync(&mut self) -> HSYNC_W<1> {
        HSYNC_W::new(self)
    }
    #[doc = "Bit 2 - Vertical Back Porch Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbporch(&mut self) -> VBPORCH_W<2> {
        VBPORCH_W::new(self)
    }
    #[doc = "Bit 3 - Vertical Front Porch Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vfporch(&mut self) -> VFPORCH_W<3> {
        VFPORCH_W::new(self)
    }
    #[doc = "Bit 4 - Direct Drive Data Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ddempty(&mut self) -> DDEMPTY_W<4> {
        DDEMPTY_W::new(self)
    }
    #[doc = "Bit 5 - Direct Drive Jitter Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ddjit(&mut self) -> DDJIT_W<5> {
        DDJIT_W::new(self)
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
