#[doc = "Register `CLKDIV` reader"]
pub struct R(crate::R<CLKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKDIV` writer"]
pub struct W(crate::W<CLKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKDIV_SPEC>;
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
impl From<crate::W<CLKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV` reader - Fractional Clock Divider"]
pub type DIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DIV` writer - Fractional Clock Divider"]
pub type DIV_W<'a> = crate::FieldWriter<'a, u32, CLKDIV_SPEC, u16, u16, 15, 6>;
impl R {
    #[doc = "Bits 6:20 - Fractional Clock Divider"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 6) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 6:20 - Fractional Clock Divider"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkdiv](index.html) module"]
pub struct CLKDIV_SPEC;
impl crate::RegisterSpec for CLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkdiv::R](R) reader structure"]
impl crate::Readable for CLKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkdiv::W](W) writer structure"]
impl crate::Writable for CLKDIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKDIV to value 0"]
impl crate::Resettable for CLKDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
