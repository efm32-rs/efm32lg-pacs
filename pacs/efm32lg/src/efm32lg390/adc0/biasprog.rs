#[doc = "Register `BIASPROG` reader"]
pub struct R(crate::R<BIASPROG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BIASPROG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BIASPROG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BIASPROG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BIASPROG` writer"]
pub struct W(crate::W<BIASPROG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BIASPROG_SPEC>;
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
impl From<crate::W<BIASPROG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BIASPROG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BIASPROG` reader - Bias Programming Value"]
pub type BIASPROG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BIASPROG` writer - Bias Programming Value"]
pub type BIASPROG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BIASPROG_SPEC, u8, u8, 4, O>;
#[doc = "Field `HALFBIAS` reader - Half Bias Current"]
pub type HALFBIAS_R = crate::BitReader<bool>;
#[doc = "Field `HALFBIAS` writer - Half Bias Current"]
pub type HALFBIAS_W<'a, const O: u8> = crate::BitWriter<'a, u32, BIASPROG_SPEC, bool, O>;
#[doc = "Field `COMPBIAS` reader - Comparator Bias Value"]
pub type COMPBIAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPBIAS` writer - Comparator Bias Value"]
pub type COMPBIAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BIASPROG_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Bias Programming Value"]
    #[inline(always)]
    pub fn biasprog(&self) -> BIASPROG_R {
        BIASPROG_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Half Bias Current"]
    #[inline(always)]
    pub fn halfbias(&self) -> HALFBIAS_R {
        HALFBIAS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Comparator Bias Value"]
    #[inline(always)]
    pub fn compbias(&self) -> COMPBIAS_R {
        COMPBIAS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Bias Programming Value"]
    #[inline(always)]
    #[must_use]
    pub fn biasprog(&mut self) -> BIASPROG_W<0> {
        BIASPROG_W::new(self)
    }
    #[doc = "Bit 6 - Half Bias Current"]
    #[inline(always)]
    #[must_use]
    pub fn halfbias(&mut self) -> HALFBIAS_W<6> {
        HALFBIAS_W::new(self)
    }
    #[doc = "Bits 8:11 - Comparator Bias Value"]
    #[inline(always)]
    #[must_use]
    pub fn compbias(&mut self) -> COMPBIAS_W<8> {
        COMPBIAS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bias Programming Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [biasprog](index.html) module"]
pub struct BIASPROG_SPEC;
impl crate::RegisterSpec for BIASPROG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [biasprog::R](R) reader structure"]
impl crate::Readable for BIASPROG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [biasprog::W](W) writer structure"]
impl crate::Writable for BIASPROG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BIASPROG to value 0x0747"]
impl crate::Resettable for BIASPROG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0747;
}
