#[doc = "Register `POWERDOWN` reader"]
pub struct R(crate::R<POWERDOWN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POWERDOWN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POWERDOWN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POWERDOWN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POWERDOWN` writer"]
pub struct W(crate::W<POWERDOWN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POWERDOWN_SPEC>;
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
impl From<crate::W<POWERDOWN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POWERDOWN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAM` reader - Retention RAM power-down"]
pub type RAM_R = crate::BitReader<bool>;
#[doc = "Field `RAM` writer - Retention RAM power-down"]
pub type RAM_W<'a> = crate::BitWriter<'a, u32, POWERDOWN_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Retention RAM power-down"]
    #[inline(always)]
    pub fn ram(&self) -> RAM_R {
        RAM_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Retention RAM power-down"]
    #[inline(always)]
    pub fn ram(&mut self) -> RAM_W {
        RAM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Retention RAM power-down Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [powerdown](index.html) module"]
pub struct POWERDOWN_SPEC;
impl crate::RegisterSpec for POWERDOWN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [powerdown::R](R) reader structure"]
impl crate::Readable for POWERDOWN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [powerdown::W](W) writer structure"]
impl crate::Writable for POWERDOWN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POWERDOWN to value 0"]
impl crate::Resettable for POWERDOWN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
