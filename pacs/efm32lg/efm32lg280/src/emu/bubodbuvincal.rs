#[doc = "Register `BUBODBUVINCAL` reader"]
pub struct R(crate::R<BUBODBUVINCAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUBODBUVINCAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUBODBUVINCAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUBODBUVINCAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUBODBUVINCAL` writer"]
pub struct W(crate::W<BUBODBUVINCAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUBODBUVINCAL_SPEC>;
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
impl From<crate::W<BUBODBUVINCAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUBODBUVINCAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THRES` reader - "]
pub type THRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `THRES` writer - "]
pub type THRES_W<'a> = crate::FieldWriter<'a, u32, BUBODBUVINCAL_SPEC, u8, u8, 3, 0>;
#[doc = "Field `RANGE` reader - "]
pub type RANGE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RANGE` writer - "]
pub type RANGE_W<'a> = crate::FieldWriter<'a, u32, BUBODBUVINCAL_SPEC, u8, u8, 2, 3>;
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
    pub fn thres(&mut self) -> THRES_W {
        THRES_W::new(self)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn range(&mut self) -> RANGE_W {
        RANGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BU_VIN Backup BOD calibration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bubodbuvincal](index.html) module"]
pub struct BUBODBUVINCAL_SPEC;
impl crate::RegisterSpec for BUBODBUVINCAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bubodbuvincal::R](R) reader structure"]
impl crate::Readable for BUBODBUVINCAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bubodbuvincal::W](W) writer structure"]
impl crate::Writable for BUBODBUVINCAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BUBODBUVINCAL to value 0x0b"]
impl crate::Resettable for BUBODBUVINCAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0b
    }
}
