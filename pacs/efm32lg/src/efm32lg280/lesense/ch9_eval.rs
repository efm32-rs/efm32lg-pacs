#[doc = "Register `CH9_EVAL` reader"]
pub struct R(crate::R<CH9_EVAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH9_EVAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH9_EVAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH9_EVAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH9_EVAL` writer"]
pub struct W(crate::W<CH9_EVAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH9_EVAL_SPEC>;
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
impl From<crate::W<CH9_EVAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH9_EVAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPTHRES` reader - Decision threshold for counter"]
pub type COMPTHRES_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COMPTHRES` writer - Decision threshold for counter"]
pub type COMPTHRES_W<'a> = crate::FieldWriter<'a, u32, CH9_EVAL_SPEC, u16, u16, 16, 0>;
#[doc = "Field `COMP` reader - Select mode for counter comparison"]
pub type COMP_R = crate::BitReader<bool>;
#[doc = "Field `COMP` writer - Select mode for counter comparison"]
pub type COMP_W<'a> = crate::BitWriter<'a, u32, CH9_EVAL_SPEC, bool, 16>;
#[doc = "Field `DECODE` reader - Send result to decoder"]
pub type DECODE_R = crate::BitReader<bool>;
#[doc = "Field `DECODE` writer - Send result to decoder"]
pub type DECODE_W<'a> = crate::BitWriter<'a, u32, CH9_EVAL_SPEC, bool, 17>;
#[doc = "Field `STRSAMPLE` reader - Select if counter result should be stored"]
pub type STRSAMPLE_R = crate::BitReader<bool>;
#[doc = "Field `STRSAMPLE` writer - Select if counter result should be stored"]
pub type STRSAMPLE_W<'a> = crate::BitWriter<'a, u32, CH9_EVAL_SPEC, bool, 18>;
#[doc = "Field `SCANRESINV` reader - Enable inversion of result"]
pub type SCANRESINV_R = crate::BitReader<bool>;
#[doc = "Field `SCANRESINV` writer - Enable inversion of result"]
pub type SCANRESINV_W<'a> = crate::BitWriter<'a, u32, CH9_EVAL_SPEC, bool, 19>;
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
    pub fn compthres(&mut self) -> COMPTHRES_W {
        COMPTHRES_W::new(self)
    }
    #[doc = "Bit 16 - Select mode for counter comparison"]
    #[inline(always)]
    pub fn comp(&mut self) -> COMP_W {
        COMP_W::new(self)
    }
    #[doc = "Bit 17 - Send result to decoder"]
    #[inline(always)]
    pub fn decode(&mut self) -> DECODE_W {
        DECODE_W::new(self)
    }
    #[doc = "Bit 18 - Select if counter result should be stored"]
    #[inline(always)]
    pub fn strsample(&mut self) -> STRSAMPLE_W {
        STRSAMPLE_W::new(self)
    }
    #[doc = "Bit 19 - Enable inversion of result"]
    #[inline(always)]
    pub fn scanresinv(&mut self) -> SCANRESINV_W {
        SCANRESINV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scan configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch9_eval](index.html) module"]
pub struct CH9_EVAL_SPEC;
impl crate::RegisterSpec for CH9_EVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch9_eval::R](R) reader structure"]
impl crate::Readable for CH9_EVAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch9_eval::W](W) writer structure"]
impl crate::Writable for CH9_EVAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH9_EVAL to value 0"]
impl crate::Resettable for CH9_EVAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
