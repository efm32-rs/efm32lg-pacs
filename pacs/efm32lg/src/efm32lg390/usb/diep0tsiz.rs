#[doc = "Register `DIEP0TSIZ` reader"]
pub struct R(crate::R<DIEP0TSIZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEP0TSIZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEP0TSIZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEP0TSIZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEP0TSIZ` writer"]
pub struct W(crate::W<DIEP0TSIZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEP0TSIZ_SPEC>;
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
impl From<crate::W<DIEP0TSIZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEP0TSIZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFERSIZE` reader - Transfer Size"]
pub type XFERSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XFERSIZE` writer - Transfer Size"]
pub type XFERSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIEP0TSIZ_SPEC, u8, u8, 7, O>;
#[doc = "Field `PKTCNT` reader - Packet Count"]
pub type PKTCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PKTCNT` writer - Packet Count"]
pub type PKTCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIEP0TSIZ_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:6 - Transfer Size"]
    #[inline(always)]
    pub fn xfersize(&self) -> XFERSIZE_R {
        XFERSIZE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 19:20 - Packet Count"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Transfer Size"]
    #[inline(always)]
    #[must_use]
    pub fn xfersize(&mut self) -> XFERSIZE_W<0> {
        XFERSIZE_W::new(self)
    }
    #[doc = "Bits 19:20 - Packet Count"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt(&mut self) -> PKTCNT_W<19> {
        PKTCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device IN Endpoint 0 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep0tsiz](index.html) module"]
pub struct DIEP0TSIZ_SPEC;
impl crate::RegisterSpec for DIEP0TSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diep0tsiz::R](R) reader structure"]
impl crate::Readable for DIEP0TSIZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diep0tsiz::W](W) writer structure"]
impl crate::Writable for DIEP0TSIZ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEP0TSIZ to value 0"]
impl crate::Resettable for DIEP0TSIZ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
