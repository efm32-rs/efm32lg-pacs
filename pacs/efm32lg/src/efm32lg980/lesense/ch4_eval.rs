#[doc = "Register `CH4_EVAL` reader"]
pub struct R(crate::R<CH4_EVAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH4_EVAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH4_EVAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH4_EVAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH4_EVAL` writer"]
pub struct W(crate::W<CH4_EVAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH4_EVAL_SPEC>;
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
impl From<crate::W<CH4_EVAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH4_EVAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPTHRES` reader - Decision threshold for counter"]
pub type COMPTHRES_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COMPTHRES` writer - Decision threshold for counter"]
pub type COMPTHRES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH4_EVAL_SPEC, u16, u16, 16, O>;
#[doc = "Field `COMP` reader - Select mode for counter comparison"]
pub type COMP_R = crate::BitReader<bool>;
#[doc = "Field `COMP` writer - Select mode for counter comparison"]
pub type COMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH4_EVAL_SPEC, bool, O>;
#[doc = "Field `DECODE` reader - Send result to decoder"]
pub type DECODE_R = crate::BitReader<bool>;
#[doc = "Field `DECODE` writer - Send result to decoder"]
pub type DECODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH4_EVAL_SPEC, bool, O>;
#[doc = "Field `STRSAMPLE` reader - Select if counter result should be stored"]
pub type STRSAMPLE_R = crate::BitReader<bool>;
#[doc = "Field `STRSAMPLE` writer - Select if counter result should be stored"]
pub type STRSAMPLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH4_EVAL_SPEC, bool, O>;
#[doc = "Field `SCANRESINV` reader - Enable inversion of result"]
pub type SCANRESINV_R = crate::BitReader<bool>;
#[doc = "Field `SCANRESINV` writer - Enable inversion of result"]
pub type SCANRESINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH4_EVAL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - Decision threshold for counter"]
    #[inline(always)]
    pub fn compthres(&self) -> COMPTHRES_R {
        COMPTHRES_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Select mode for counter comparison"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Send result to decoder"]
    #[inline(always)]
    pub fn decode(&self) -> DECODE_R {
        DECODE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Select if counter result should be stored"]
    #[inline(always)]
    pub fn strsample(&self) -> STRSAMPLE_R {
        STRSAMPLE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable inversion of result"]
    #[inline(always)]
    pub fn scanresinv(&self) -> SCANRESINV_R {
        SCANRESINV_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Decision threshold for counter"]
    #[inline(always)]
    #[must_use]
    pub fn compthres(&mut self) -> COMPTHRES_W<0> {
        COMPTHRES_W::new(self)
    }
    #[doc = "Bit 16 - Select mode for counter comparison"]
    #[inline(always)]
    #[must_use]
    pub fn comp(&mut self) -> COMP_W<16> {
        COMP_W::new(self)
    }
    #[doc = "Bit 17 - Send result to decoder"]
    #[inline(always)]
    #[must_use]
    pub fn decode(&mut self) -> DECODE_W<17> {
        DECODE_W::new(self)
    }
    #[doc = "Bit 18 - Select if counter result should be stored"]
    #[inline(always)]
    #[must_use]
    pub fn strsample(&mut self) -> STRSAMPLE_W<18> {
        STRSAMPLE_W::new(self)
    }
    #[doc = "Bit 19 - Enable inversion of result"]
    #[inline(always)]
    #[must_use]
    pub fn scanresinv(&mut self) -> SCANRESINV_W<19> {
        SCANRESINV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scan configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_eval](index.html) module"]
pub struct CH4_EVAL_SPEC;
impl crate::RegisterSpec for CH4_EVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch4_eval::R](R) reader structure"]
impl crate::Readable for CH4_EVAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch4_eval::W](W) writer structure"]
impl crate::Writable for CH4_EVAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH4_EVAL to value 0"]
impl crate::Resettable for CH4_EVAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
