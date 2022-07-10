#[doc = "Register `TFTHPORCH` reader"]
pub struct R(crate::R<TFTHPORCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TFTHPORCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TFTHPORCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TFTHPORCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TFTHPORCH` writer"]
pub struct W(crate::W<TFTHPORCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TFTHPORCH_SPEC>;
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
impl From<crate::W<TFTHPORCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TFTHPORCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSYNC` reader - Horizontal Synchronization Pulse Width"]
pub type HSYNC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSYNC` writer - Horizontal Synchronization Pulse Width"]
pub type HSYNC_W<'a> = crate::FieldWriter<'a, u32, TFTHPORCH_SPEC, u8, u8, 7, 0>;
#[doc = "Field `HFPORCH` reader - Horizontal Front Porch Size"]
pub type HFPORCH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HFPORCH` writer - Horizontal Front Porch Size"]
pub type HFPORCH_W<'a> = crate::FieldWriter<'a, u32, TFTHPORCH_SPEC, u8, u8, 8, 8>;
#[doc = "Field `HBPORCH` reader - Horizontal Back Porch Size"]
pub type HBPORCH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HBPORCH` writer - Horizontal Back Porch Size"]
pub type HBPORCH_W<'a> = crate::FieldWriter<'a, u32, TFTHPORCH_SPEC, u8, u8, 8, 18>;
#[doc = "Field `HSYNCSTART` reader - HSYNC Start Delay"]
pub type HSYNCSTART_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSYNCSTART` writer - HSYNC Start Delay"]
pub type HSYNCSTART_W<'a> = crate::FieldWriter<'a, u32, TFTHPORCH_SPEC, u8, u8, 2, 28>;
impl R {
    #[doc = "Bits 0:6 - Horizontal Synchronization Pulse Width"]
    #[inline(always)]
    pub fn hsync(&self) -> HSYNC_R {
        HSYNC_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - Horizontal Front Porch Size"]
    #[inline(always)]
    pub fn hfporch(&self) -> HFPORCH_R {
        HFPORCH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 18:25 - Horizontal Back Porch Size"]
    #[inline(always)]
    pub fn hbporch(&self) -> HBPORCH_R {
        HBPORCH_R::new(((self.bits >> 18) & 0xff) as u8)
    }
    #[doc = "Bits 28:29 - HSYNC Start Delay"]
    #[inline(always)]
    pub fn hsyncstart(&self) -> HSYNCSTART_R {
        HSYNCSTART_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Horizontal Synchronization Pulse Width"]
    #[inline(always)]
    pub fn hsync(&mut self) -> HSYNC_W {
        HSYNC_W::new(self)
    }
    #[doc = "Bits 8:15 - Horizontal Front Porch Size"]
    #[inline(always)]
    pub fn hfporch(&mut self) -> HFPORCH_W {
        HFPORCH_W::new(self)
    }
    #[doc = "Bits 18:25 - Horizontal Back Porch Size"]
    #[inline(always)]
    pub fn hbporch(&mut self) -> HBPORCH_W {
        HBPORCH_W::new(self)
    }
    #[doc = "Bits 28:29 - HSYNC Start Delay"]
    #[inline(always)]
    pub fn hsyncstart(&mut self) -> HSYNCSTART_W {
        HSYNCSTART_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TFT Horizontal Porch Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tfthporch](index.html) module"]
pub struct TFTHPORCH_SPEC;
impl crate::RegisterSpec for TFTHPORCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tfthporch::R](R) reader structure"]
impl crate::Readable for TFTHPORCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tfthporch::W](W) writer structure"]
impl crate::Writable for TFTHPORCH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TFTHPORCH to value 0"]
impl crate::Resettable for TFTHPORCH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
