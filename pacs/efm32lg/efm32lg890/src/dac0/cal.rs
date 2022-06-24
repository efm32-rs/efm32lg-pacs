#[doc = "Register `CAL` reader"]
pub struct R(crate::R<CAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAL` writer"]
pub struct W(crate::W<CAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAL_SPEC>;
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
impl From<crate::W<CAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0OFFSET` reader - Channel 0 Offset Calibration Value"]
pub type CH0OFFSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH0OFFSET` writer - Channel 0 Offset Calibration Value"]
pub type CH0OFFSET_W<'a> = crate::FieldWriter<'a, u32, CAL_SPEC, u8, u8, 6, 0>;
#[doc = "Field `CH1OFFSET` reader - Channel 1 Offset Calibration Value"]
pub type CH1OFFSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH1OFFSET` writer - Channel 1 Offset Calibration Value"]
pub type CH1OFFSET_W<'a> = crate::FieldWriter<'a, u32, CAL_SPEC, u8, u8, 6, 8>;
#[doc = "Field `GAIN` reader - Gain Calibration Value"]
pub type GAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GAIN` writer - Gain Calibration Value"]
pub type GAIN_W<'a> = crate::FieldWriter<'a, u32, CAL_SPEC, u8, u8, 7, 16>;
impl R {
    #[doc = "Bits 0:5 - Channel 0 Offset Calibration Value"]
    #[inline(always)]
    pub fn ch0offset(&self) -> CH0OFFSET_R {
        CH0OFFSET_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Channel 1 Offset Calibration Value"]
    #[inline(always)]
    pub fn ch1offset(&self) -> CH1OFFSET_R {
        CH1OFFSET_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:22 - Gain Calibration Value"]
    #[inline(always)]
    pub fn gain(&self) -> GAIN_R {
        GAIN_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Channel 0 Offset Calibration Value"]
    #[inline(always)]
    pub fn ch0offset(&mut self) -> CH0OFFSET_W {
        CH0OFFSET_W::new(self)
    }
    #[doc = "Bits 8:13 - Channel 1 Offset Calibration Value"]
    #[inline(always)]
    pub fn ch1offset(&mut self) -> CH1OFFSET_W {
        CH1OFFSET_W::new(self)
    }
    #[doc = "Bits 16:22 - Gain Calibration Value"]
    #[inline(always)]
    pub fn gain(&mut self) -> GAIN_W {
        GAIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calibration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal](index.html) module"]
pub struct CAL_SPEC;
impl crate::RegisterSpec for CAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cal::R](R) reader structure"]
impl crate::Readable for CAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cal::W](W) writer structure"]
impl crate::Writable for CAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAL to value 0x0040_0000"]
impl crate::Resettable for CAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0040_0000
    }
}
