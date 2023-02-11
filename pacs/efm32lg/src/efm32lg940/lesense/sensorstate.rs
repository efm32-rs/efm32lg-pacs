#[doc = "Register `SENSORSTATE` reader"]
pub struct R(crate::R<SENSORSTATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SENSORSTATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SENSORSTATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SENSORSTATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SENSORSTATE` writer"]
pub struct W(crate::W<SENSORSTATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SENSORSTATE_SPEC>;
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
impl From<crate::W<SENSORSTATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SENSORSTATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SENSORSTATE` reader - Shows the status of sensors chosen as input to the decoder"]
pub type SENSORSTATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SENSORSTATE` writer - Shows the status of sensors chosen as input to the decoder"]
pub type SENSORSTATE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SENSORSTATE_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Shows the status of sensors chosen as input to the decoder"]
    #[inline(always)]
    pub fn sensorstate(&self) -> SENSORSTATE_R {
        SENSORSTATE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Shows the status of sensors chosen as input to the decoder"]
    #[inline(always)]
    #[must_use]
    pub fn sensorstate(&mut self) -> SENSORSTATE_W<0> {
        SENSORSTATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Decoder input register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensorstate](index.html) module"]
pub struct SENSORSTATE_SPEC;
impl crate::RegisterSpec for SENSORSTATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sensorstate::R](R) reader structure"]
impl crate::Readable for SENSORSTATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sensorstate::W](W) writer structure"]
impl crate::Writable for SENSORSTATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SENSORSTATE to value 0"]
impl crate::Resettable for SENSORSTATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
