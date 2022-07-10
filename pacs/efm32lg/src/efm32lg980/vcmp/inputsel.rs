#[doc = "Register `INPUTSEL` reader"]
pub struct R(crate::R<INPUTSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INPUTSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INPUTSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INPUTSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INPUTSEL` writer"]
pub struct W(crate::W<INPUTSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INPUTSEL_SPEC>;
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
impl From<crate::W<INPUTSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INPUTSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIGLEVEL` reader - Trigger Level"]
pub type TRIGLEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIGLEVEL` writer - Trigger Level"]
pub type TRIGLEVEL_W<'a> = crate::FieldWriter<'a, u32, INPUTSEL_SPEC, u8, u8, 6, 0>;
#[doc = "Field `LPREF` reader - Low Power Reference"]
pub type LPREF_R = crate::BitReader<bool>;
#[doc = "Field `LPREF` writer - Low Power Reference"]
pub type LPREF_W<'a> = crate::BitWriter<'a, u32, INPUTSEL_SPEC, bool, 8>;
impl R {
    #[doc = "Bits 0:5 - Trigger Level"]
    #[inline(always)]
    pub fn triglevel(&self) -> TRIGLEVEL_R {
        TRIGLEVEL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Low Power Reference"]
    #[inline(always)]
    pub fn lpref(&self) -> LPREF_R {
        LPREF_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Trigger Level"]
    #[inline(always)]
    pub fn triglevel(&mut self) -> TRIGLEVEL_W {
        TRIGLEVEL_W::new(self)
    }
    #[doc = "Bit 8 - Low Power Reference"]
    #[inline(always)]
    pub fn lpref(&mut self) -> LPREF_W {
        LPREF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inputsel](index.html) module"]
pub struct INPUTSEL_SPEC;
impl crate::RegisterSpec for INPUTSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inputsel::R](R) reader structure"]
impl crate::Readable for INPUTSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inputsel::W](W) writer structure"]
impl crate::Writable for INPUTSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INPUTSEL to value 0"]
impl crate::Resettable for INPUTSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
