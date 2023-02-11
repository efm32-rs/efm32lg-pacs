#[doc = "Register `ETMCLAIMSET` reader"]
pub struct R(crate::R<ETMCLAIMSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETMCLAIMSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETMCLAIMSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETMCLAIMSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETMCLAIMSET` writer"]
pub struct W(crate::W<ETMCLAIMSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETMCLAIMSET_SPEC>;
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
impl From<crate::W<ETMCLAIMSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETMCLAIMSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SETTAG` reader - Tag Bits"]
pub type SETTAG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SETTAG` writer - Tag Bits"]
pub type SETTAG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETMCLAIMSET_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Tag Bits"]
    #[inline(always)]
    pub fn settag(&self) -> SETTAG_R {
        SETTAG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Tag Bits"]
    #[inline(always)]
    #[must_use]
    pub fn settag(&mut self) -> SETTAG_W<0> {
        SETTAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ETM Claim Tag Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmclaimset](index.html) module"]
pub struct ETMCLAIMSET_SPEC;
impl crate::RegisterSpec for ETMCLAIMSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etmclaimset::R](R) reader structure"]
impl crate::Readable for ETMCLAIMSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etmclaimset::W](W) writer structure"]
impl crate::Writable for ETMCLAIMSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETMCLAIMSET to value 0x0f"]
impl crate::Resettable for ETMCLAIMSET_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
