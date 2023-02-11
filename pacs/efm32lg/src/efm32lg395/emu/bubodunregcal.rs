#[doc = "Register `BUBODUNREGCAL` reader"]
pub struct R(crate::R<BUBODUNREGCAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUBODUNREGCAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUBODUNREGCAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUBODUNREGCAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUBODUNREGCAL` writer"]
pub struct W(crate::W<BUBODUNREGCAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUBODUNREGCAL_SPEC>;
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
impl From<crate::W<BUBODUNREGCAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUBODUNREGCAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THRES` reader - "]
pub type THRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `THRES` writer - "]
pub type THRES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUBODUNREGCAL_SPEC, u8, u8, 3, O>;
#[doc = "Field `RANGE` reader - "]
pub type RANGE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RANGE` writer - "]
pub type RANGE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUBODUNREGCAL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn thres(&self) -> THRES_R {
        THRES_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn range(&self) -> RANGE_R {
        RANGE_R::new(((self.bits >> 3) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn thres(&mut self) -> THRES_W<0> {
        THRES_W::new(self)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    #[must_use]
    pub fn range(&mut self) -> RANGE_W<3> {
        RANGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Unregulated power Backup BOD calibration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bubodunregcal](index.html) module"]
pub struct BUBODUNREGCAL_SPEC;
impl crate::RegisterSpec for BUBODUNREGCAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bubodunregcal::R](R) reader structure"]
impl crate::Readable for BUBODUNREGCAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bubodunregcal::W](W) writer structure"]
impl crate::Writable for BUBODUNREGCAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUBODUNREGCAL to value 0x0b"]
impl crate::Resettable for BUBODUNREGCAL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0b;
}
