#[doc = "Register `SEGD0H` reader"]
pub struct R(crate::R<SEGD0H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEGD0H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEGD0H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEGD0H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEGD0H` writer"]
pub struct W(crate::W<SEGD0H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEGD0H_SPEC>;
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
impl From<crate::W<SEGD0H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEGD0H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEGD0H` reader - COM0 Segment Data High"]
pub type SEGD0H_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEGD0H` writer - COM0 Segment Data High"]
pub type SEGD0H_W<'a> = crate::FieldWriter<'a, u32, SEGD0H_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - COM0 Segment Data High"]
    #[inline(always)]
    pub fn segd0h(&self) -> SEGD0H_R {
        SEGD0H_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - COM0 Segment Data High"]
    #[inline(always)]
    pub fn segd0h(&mut self) -> SEGD0H_W {
        SEGD0H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Segment Data High Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [segd0h](index.html) module"]
pub struct SEGD0H_SPEC;
impl crate::RegisterSpec for SEGD0H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [segd0h::R](R) reader structure"]
impl crate::Readable for SEGD0H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [segd0h::W](W) writer structure"]
impl crate::Writable for SEGD0H_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEGD0H to value 0"]
impl crate::Resettable for SEGD0H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
