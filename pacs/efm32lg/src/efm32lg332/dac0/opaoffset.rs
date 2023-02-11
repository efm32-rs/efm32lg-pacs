#[doc = "Register `OPAOFFSET` reader"]
pub struct R(crate::R<OPAOFFSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPAOFFSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPAOFFSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPAOFFSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPAOFFSET` writer"]
pub struct W(crate::W<OPAOFFSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPAOFFSET_SPEC>;
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
impl From<crate::W<OPAOFFSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPAOFFSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPA2OFFSET` reader - OPA2 Offset Configuration Value"]
pub type OPA2OFFSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OPA2OFFSET` writer - OPA2 Offset Configuration Value"]
pub type OPA2OFFSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPAOFFSET_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - OPA2 Offset Configuration Value"]
    #[inline(always)]
    pub fn opa2offset(&self) -> OPA2OFFSET_R {
        OPA2OFFSET_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - OPA2 Offset Configuration Value"]
    #[inline(always)]
    #[must_use]
    pub fn opa2offset(&mut self) -> OPA2OFFSET_W<0> {
        OPA2OFFSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Operational Amplifier Offset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opaoffset](index.html) module"]
pub struct OPAOFFSET_SPEC;
impl crate::RegisterSpec for OPAOFFSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opaoffset::R](R) reader structure"]
impl crate::Readable for OPAOFFSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opaoffset::W](W) writer structure"]
impl crate::Writable for OPAOFFSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPAOFFSET to value 0x20"]
impl crate::Resettable for OPAOFFSET_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
