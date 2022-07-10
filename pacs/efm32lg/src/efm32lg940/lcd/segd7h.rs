#[doc = "Register `SEGD7H` reader"]
pub struct R(crate::R<SEGD7H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEGD7H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEGD7H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEGD7H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEGD7H` writer"]
pub struct W(crate::W<SEGD7H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEGD7H_SPEC>;
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
impl From<crate::W<SEGD7H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEGD7H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEGD7H` reader - COM3 Segment Data High"]
pub type SEGD7H_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEGD7H` writer - COM3 Segment Data High"]
pub type SEGD7H_W<'a> = crate::FieldWriter<'a, u32, SEGD7H_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - COM3 Segment Data High"]
    #[inline(always)]
    pub fn segd7h(&self) -> SEGD7H_R {
        SEGD7H_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - COM3 Segment Data High"]
    #[inline(always)]
    pub fn segd7h(&mut self) -> SEGD7H_W {
        SEGD7H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Segment Data High Register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [segd7h](index.html) module"]
pub struct SEGD7H_SPEC;
impl crate::RegisterSpec for SEGD7H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [segd7h::R](R) reader structure"]
impl crate::Readable for SEGD7H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [segd7h::W](W) writer structure"]
impl crate::Writable for SEGD7H_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEGD7H to value 0"]
impl crate::Resettable for SEGD7H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
