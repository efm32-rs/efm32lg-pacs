#[doc = "Register `ETMISCIN` reader"]
pub struct R(crate::R<ETMISCIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETMISCIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETMISCIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETMISCIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETMISCIN` writer"]
pub struct W(crate::W<ETMISCIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETMISCIN_SPEC>;
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
impl From<crate::W<ETMISCIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETMISCIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTIN` reader - EXTIN Value"]
pub type EXTIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTIN` writer - EXTIN Value"]
pub type EXTIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETMISCIN_SPEC, u8, u8, 2, O>;
#[doc = "Field `COREHALT` reader - Core Halt"]
pub type COREHALT_R = crate::BitReader<bool>;
#[doc = "Field `COREHALT` writer - Core Halt"]
pub type COREHALT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETMISCIN_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - EXTIN Value"]
    #[inline(always)]
    pub fn extin(&self) -> EXTIN_R {
        EXTIN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Core Halt"]
    #[inline(always)]
    pub fn corehalt(&self) -> COREHALT_R {
        COREHALT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - EXTIN Value"]
    #[inline(always)]
    #[must_use]
    pub fn extin(&mut self) -> EXTIN_W<0> {
        EXTIN_W::new(self)
    }
    #[doc = "Bit 4 - Core Halt"]
    #[inline(always)]
    #[must_use]
    pub fn corehalt(&mut self) -> COREHALT_W<4> {
        COREHALT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Integration Test Miscellaneous Inputs Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmiscin](index.html) module"]
pub struct ETMISCIN_SPEC;
impl crate::RegisterSpec for ETMISCIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etmiscin::R](R) reader structure"]
impl crate::Readable for ETMISCIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etmiscin::W](W) writer structure"]
impl crate::Writable for ETMISCIN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETMISCIN to value 0"]
impl crate::Resettable for ETMISCIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
