#[doc = "Register `SEGD6H` reader"]
pub struct R(crate::R<SEGD6H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEGD6H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEGD6H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEGD6H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEGD6H` writer"]
pub struct W(crate::W<SEGD6H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEGD6H_SPEC>;
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
impl From<crate::W<SEGD6H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEGD6H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEGD6H` reader - COM2 Segment Data High"]
pub type SEGD6H_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEGD6H` writer - COM2 Segment Data High"]
pub type SEGD6H_W<'a> = crate::FieldWriter<'a, u32, SEGD6H_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - COM2 Segment Data High"]
    #[inline(always)]
    pub fn segd6h(&self) -> SEGD6H_R {
        SEGD6H_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - COM2 Segment Data High"]
    #[inline(always)]
    pub fn segd6h(&mut self) -> SEGD6H_W {
        SEGD6H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Segment Data High Register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [segd6h](index.html) module"]
pub struct SEGD6H_SPEC;
impl crate::RegisterSpec for SEGD6H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [segd6h::R](R) reader structure"]
impl crate::Readable for SEGD6H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [segd6h::W](W) writer structure"]
impl crate::Writable for SEGD6H_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEGD6H to value 0"]
impl crate::Resettable for SEGD6H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
