#[doc = "Register `DOEP0TSIZ` reader"]
pub struct R(crate::R<DOEP0TSIZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEP0TSIZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEP0TSIZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEP0TSIZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEP0TSIZ` writer"]
pub struct W(crate::W<DOEP0TSIZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEP0TSIZ_SPEC>;
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
impl From<crate::W<DOEP0TSIZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEP0TSIZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFERSIZE` reader - Transfer Size"]
pub type XFERSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XFERSIZE` writer - Transfer Size"]
pub type XFERSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOEP0TSIZ_SPEC, u8, u8, 7, O>;
#[doc = "Field `PKTCNT` reader - Packet Count"]
pub type PKTCNT_R = crate::BitReader<bool>;
#[doc = "Field `PKTCNT` writer - Packet Count"]
pub type PKTCNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEP0TSIZ_SPEC, bool, O>;
#[doc = "Field `SUPCNT` reader - SETUP Packet Count"]
pub type SUPCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SUPCNT` writer - SETUP Packet Count"]
pub type SUPCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOEP0TSIZ_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:6 - Transfer Size"]
    #[inline(always)]
    pub fn xfersize(&self) -> XFERSIZE_R {
        XFERSIZE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 19 - Packet Count"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 29:30 - SETUP Packet Count"]
    #[inline(always)]
    pub fn supcnt(&self) -> SUPCNT_R {
        SUPCNT_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Transfer Size"]
    #[inline(always)]
    #[must_use]
    pub fn xfersize(&mut self) -> XFERSIZE_W<0> {
        XFERSIZE_W::new(self)
    }
    #[doc = "Bit 19 - Packet Count"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt(&mut self) -> PKTCNT_W<19> {
        PKTCNT_W::new(self)
    }
    #[doc = "Bits 29:30 - SETUP Packet Count"]
    #[inline(always)]
    #[must_use]
    pub fn supcnt(&mut self) -> SUPCNT_W<29> {
        SUPCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device OUT Endpoint 0 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep0tsiz](index.html) module"]
pub struct DOEP0TSIZ_SPEC;
impl crate::RegisterSpec for DOEP0TSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doep0tsiz::R](R) reader structure"]
impl crate::Readable for DOEP0TSIZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doep0tsiz::W](W) writer structure"]
impl crate::Writable for DOEP0TSIZ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEP0TSIZ to value 0"]
impl crate::Resettable for DOEP0TSIZ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
