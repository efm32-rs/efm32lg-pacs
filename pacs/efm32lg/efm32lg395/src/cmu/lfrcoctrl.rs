#[doc = "Register `LFRCOCTRL` reader"]
pub struct R(crate::R<LFRCOCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LFRCOCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LFRCOCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LFRCOCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LFRCOCTRL` writer"]
pub struct W(crate::W<LFRCOCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LFRCOCTRL_SPEC>;
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
impl From<crate::W<LFRCOCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LFRCOCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TUNING` reader - LFRCO Tuning Value"]
pub type TUNING_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TUNING` writer - LFRCO Tuning Value"]
pub type TUNING_W<'a> = crate::FieldWriter<'a, u32, LFRCOCTRL_SPEC, u8, u8, 7, 0>;
impl R {
    #[doc = "Bits 0:6 - LFRCO Tuning Value"]
    #[inline(always)]
    pub fn tuning(&self) -> TUNING_R {
        TUNING_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - LFRCO Tuning Value"]
    #[inline(always)]
    pub fn tuning(&mut self) -> TUNING_W {
        TUNING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LFRCO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfrcoctrl](index.html) module"]
pub struct LFRCOCTRL_SPEC;
impl crate::RegisterSpec for LFRCOCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lfrcoctrl::R](R) reader structure"]
impl crate::Readable for LFRCOCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lfrcoctrl::W](W) writer structure"]
impl crate::Writable for LFRCOCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LFRCOCTRL to value 0x40"]
impl crate::Resettable for LFRCOCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x40
    }
}
