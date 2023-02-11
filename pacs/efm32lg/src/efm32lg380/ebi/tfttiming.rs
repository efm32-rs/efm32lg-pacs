#[doc = "Register `TFTTIMING` reader"]
pub struct R(crate::R<TFTTIMING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TFTTIMING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TFTTIMING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TFTTIMING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TFTTIMING` writer"]
pub struct W(crate::W<TFTTIMING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TFTTIMING_SPEC>;
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
impl From<crate::W<TFTTIMING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TFTTIMING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCLKPERIOD` reader - TFT Direct Drive Transaction (EBI_DCLK) Period"]
pub type DCLKPERIOD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DCLKPERIOD` writer - TFT Direct Drive Transaction (EBI_DCLK) Period"]
pub type DCLKPERIOD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TFTTIMING_SPEC, u16, u16, 11, O>;
#[doc = "Field `TFTSTART` reader - TFT Direct Drive Transaction Start"]
pub type TFTSTART_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TFTSTART` writer - TFT Direct Drive Transaction Start"]
pub type TFTSTART_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TFTTIMING_SPEC, u16, u16, 11, O>;
#[doc = "Field `TFTSETUP` reader - TFT Setup Time"]
pub type TFTSETUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TFTSETUP` writer - TFT Setup Time"]
pub type TFTSETUP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TFTTIMING_SPEC, u8, u8, 2, O>;
#[doc = "Field `TFTHOLD` reader - TFT Hold Time"]
pub type TFTHOLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TFTHOLD` writer - TFT Hold Time"]
pub type TFTHOLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TFTTIMING_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:10 - TFT Direct Drive Transaction (EBI_DCLK) Period"]
    #[inline(always)]
    pub fn dclkperiod(&self) -> DCLKPERIOD_R {
        DCLKPERIOD_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 12:22 - TFT Direct Drive Transaction Start"]
    #[inline(always)]
    pub fn tftstart(&self) -> TFTSTART_R {
        TFTSTART_R::new(((self.bits >> 12) & 0x07ff) as u16)
    }
    #[doc = "Bits 24:25 - TFT Setup Time"]
    #[inline(always)]
    pub fn tftsetup(&self) -> TFTSETUP_R {
        TFTSETUP_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - TFT Hold Time"]
    #[inline(always)]
    pub fn tfthold(&self) -> TFTHOLD_R {
        TFTHOLD_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - TFT Direct Drive Transaction (EBI_DCLK) Period"]
    #[inline(always)]
    #[must_use]
    pub fn dclkperiod(&mut self) -> DCLKPERIOD_W<0> {
        DCLKPERIOD_W::new(self)
    }
    #[doc = "Bits 12:22 - TFT Direct Drive Transaction Start"]
    #[inline(always)]
    #[must_use]
    pub fn tftstart(&mut self) -> TFTSTART_W<12> {
        TFTSTART_W::new(self)
    }
    #[doc = "Bits 24:25 - TFT Setup Time"]
    #[inline(always)]
    #[must_use]
    pub fn tftsetup(&mut self) -> TFTSETUP_W<24> {
        TFTSETUP_W::new(self)
    }
    #[doc = "Bits 28:29 - TFT Hold Time"]
    #[inline(always)]
    #[must_use]
    pub fn tfthold(&mut self) -> TFTHOLD_W<28> {
        TFTHOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TFT Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tfttiming](index.html) module"]
pub struct TFTTIMING_SPEC;
impl crate::RegisterSpec for TFTTIMING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tfttiming::R](R) reader structure"]
impl crate::Readable for TFTTIMING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tfttiming::W](W) writer structure"]
impl crate::Writable for TFTTIMING_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TFTTIMING to value 0"]
impl crate::Resettable for TFTTIMING_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
