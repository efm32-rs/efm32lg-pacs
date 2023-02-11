#[doc = "Register `TFTPOLARITY` reader"]
pub struct R(crate::R<TFTPOLARITY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TFTPOLARITY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TFTPOLARITY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TFTPOLARITY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TFTPOLARITY` writer"]
pub struct W(crate::W<TFTPOLARITY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TFTPOLARITY_SPEC>;
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
impl From<crate::W<TFTPOLARITY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TFTPOLARITY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSPOL` reader - TFT Chip Select Polarity"]
pub type CSPOL_R = crate::BitReader<bool>;
#[doc = "Field `CSPOL` writer - TFT Chip Select Polarity"]
pub type CSPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TFTPOLARITY_SPEC, bool, O>;
#[doc = "Field `DCLKPOL` reader - TFT DCLK Polarity"]
pub type DCLKPOL_R = crate::BitReader<bool>;
#[doc = "Field `DCLKPOL` writer - TFT DCLK Polarity"]
pub type DCLKPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TFTPOLARITY_SPEC, bool, O>;
#[doc = "Field `DATAENPOL` reader - TFT DATAEN Polarity"]
pub type DATAENPOL_R = crate::BitReader<bool>;
#[doc = "Field `DATAENPOL` writer - TFT DATAEN Polarity"]
pub type DATAENPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TFTPOLARITY_SPEC, bool, O>;
#[doc = "Field `HSYNCPOL` reader - Address Latch Polarity"]
pub type HSYNCPOL_R = crate::BitReader<bool>;
#[doc = "Field `HSYNCPOL` writer - Address Latch Polarity"]
pub type HSYNCPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TFTPOLARITY_SPEC, bool, O>;
#[doc = "Field `VSYNCPOL` reader - VSYNC Polarity"]
pub type VSYNCPOL_R = crate::BitReader<bool>;
#[doc = "Field `VSYNCPOL` writer - VSYNC Polarity"]
pub type VSYNCPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TFTPOLARITY_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TFT Chip Select Polarity"]
    #[inline(always)]
    pub fn cspol(&self) -> CSPOL_R {
        CSPOL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TFT DCLK Polarity"]
    #[inline(always)]
    pub fn dclkpol(&self) -> DCLKPOL_R {
        DCLKPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TFT DATAEN Polarity"]
    #[inline(always)]
    pub fn dataenpol(&self) -> DATAENPOL_R {
        DATAENPOL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Address Latch Polarity"]
    #[inline(always)]
    pub fn hsyncpol(&self) -> HSYNCPOL_R {
        HSYNCPOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VSYNC Polarity"]
    #[inline(always)]
    pub fn vsyncpol(&self) -> VSYNCPOL_R {
        VSYNCPOL_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TFT Chip Select Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cspol(&mut self) -> CSPOL_W<0> {
        CSPOL_W::new(self)
    }
    #[doc = "Bit 1 - TFT DCLK Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn dclkpol(&mut self) -> DCLKPOL_W<1> {
        DCLKPOL_W::new(self)
    }
    #[doc = "Bit 2 - TFT DATAEN Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn dataenpol(&mut self) -> DATAENPOL_W<2> {
        DATAENPOL_W::new(self)
    }
    #[doc = "Bit 3 - Address Latch Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn hsyncpol(&mut self) -> HSYNCPOL_W<3> {
        HSYNCPOL_W::new(self)
    }
    #[doc = "Bit 4 - VSYNC Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn vsyncpol(&mut self) -> VSYNCPOL_W<4> {
        VSYNCPOL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TFT Polarity Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tftpolarity](index.html) module"]
pub struct TFTPOLARITY_SPEC;
impl crate::RegisterSpec for TFTPOLARITY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tftpolarity::R](R) reader structure"]
impl crate::Readable for TFTPOLARITY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tftpolarity::W](W) writer structure"]
impl crate::Writable for TFTPOLARITY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TFTPOLARITY to value 0"]
impl crate::Resettable for TFTPOLARITY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
