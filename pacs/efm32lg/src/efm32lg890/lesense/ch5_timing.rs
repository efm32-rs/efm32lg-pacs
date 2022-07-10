#[doc = "Register `CH5_TIMING` reader"]
pub struct R(crate::R<CH5_TIMING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH5_TIMING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH5_TIMING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH5_TIMING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH5_TIMING` writer"]
pub struct W(crate::W<CH5_TIMING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH5_TIMING_SPEC>;
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
impl From<crate::W<CH5_TIMING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH5_TIMING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTIME` reader - Set excitation time"]
pub type EXTIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTIME` writer - Set excitation time"]
pub type EXTIME_W<'a> = crate::FieldWriter<'a, u32, CH5_TIMING_SPEC, u8, u8, 6, 0>;
#[doc = "Field `SAMPLEDLY` reader - Set sample delay"]
pub type SAMPLEDLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMPLEDLY` writer - Set sample delay"]
pub type SAMPLEDLY_W<'a> = crate::FieldWriter<'a, u32, CH5_TIMING_SPEC, u8, u8, 7, 6>;
#[doc = "Field `MEASUREDLY` reader - Set measure delay"]
pub type MEASUREDLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MEASUREDLY` writer - Set measure delay"]
pub type MEASUREDLY_W<'a> = crate::FieldWriter<'a, u32, CH5_TIMING_SPEC, u8, u8, 7, 13>;
impl R {
    #[doc = "Bits 0:5 - Set excitation time"]
    #[inline(always)]
    pub fn extime(&self) -> EXTIME_R {
        EXTIME_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:12 - Set sample delay"]
    #[inline(always)]
    pub fn sampledly(&self) -> SAMPLEDLY_R {
        SAMPLEDLY_R::new(((self.bits >> 6) & 0x7f) as u8)
    }
    #[doc = "Bits 13:19 - Set measure delay"]
    #[inline(always)]
    pub fn measuredly(&self) -> MEASUREDLY_R {
        MEASUREDLY_R::new(((self.bits >> 13) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Set excitation time"]
    #[inline(always)]
    pub fn extime(&mut self) -> EXTIME_W {
        EXTIME_W::new(self)
    }
    #[doc = "Bits 6:12 - Set sample delay"]
    #[inline(always)]
    pub fn sampledly(&mut self) -> SAMPLEDLY_W {
        SAMPLEDLY_W::new(self)
    }
    #[doc = "Bits 13:19 - Set measure delay"]
    #[inline(always)]
    pub fn measuredly(&mut self) -> MEASUREDLY_W {
        MEASUREDLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scan configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_timing](index.html) module"]
pub struct CH5_TIMING_SPEC;
impl crate::RegisterSpec for CH5_TIMING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch5_timing::R](R) reader structure"]
impl crate::Readable for CH5_TIMING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch5_timing::W](W) writer structure"]
impl crate::Writable for CH5_TIMING_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH5_TIMING to value 0"]
impl crate::Resettable for CH5_TIMING_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
