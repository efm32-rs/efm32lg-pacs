#[doc = "Register `PA_PINLOCKN` reader"]
pub struct R(crate::R<PA_PINLOCKN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PA_PINLOCKN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PA_PINLOCKN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PA_PINLOCKN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PA_PINLOCKN` writer"]
pub struct W(crate::W<PA_PINLOCKN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PA_PINLOCKN_SPEC>;
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
impl From<crate::W<PA_PINLOCKN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PA_PINLOCKN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PINLOCKN` reader - Unlocked Pins"]
pub type PINLOCKN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PINLOCKN` writer - Unlocked Pins"]
pub type PINLOCKN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PA_PINLOCKN_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Unlocked Pins"]
    #[inline(always)]
    pub fn pinlockn(&self) -> PINLOCKN_R {
        PINLOCKN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Unlocked Pins"]
    #[inline(always)]
    #[must_use]
    pub fn pinlockn(&mut self) -> PINLOCKN_W<0> {
        PINLOCKN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Unlocked Pins Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pa_pinlockn](index.html) module"]
pub struct PA_PINLOCKN_SPEC;
impl crate::RegisterSpec for PA_PINLOCKN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pa_pinlockn::R](R) reader structure"]
impl crate::Readable for PA_PINLOCKN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pa_pinlockn::W](W) writer structure"]
impl crate::Writable for PA_PINLOCKN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PA_PINLOCKN to value 0xffff"]
impl crate::Resettable for PA_PINLOCKN_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
