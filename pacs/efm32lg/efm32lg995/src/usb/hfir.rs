#[doc = "Register `HFIR` reader"]
pub struct R(crate::R<HFIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFIR` writer"]
pub struct W(crate::W<HFIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFIR_SPEC>;
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
impl From<crate::W<HFIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRINT` reader - Frame Interval"]
pub type FRINT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FRINT` writer - Frame Interval"]
pub type FRINT_W<'a> = crate::FieldWriter<'a, u32, HFIR_SPEC, u16, u16, 16, 0>;
#[doc = "Field `HFIRRLDCTRL` reader - Reload Control"]
pub type HFIRRLDCTRL_R = crate::BitReader<bool>;
#[doc = "Field `HFIRRLDCTRL` writer - Reload Control"]
pub type HFIRRLDCTRL_W<'a> = crate::BitWriter<'a, u32, HFIR_SPEC, bool, 16>;
impl R {
    #[doc = "Bits 0:15 - Frame Interval"]
    #[inline(always)]
    pub fn frint(&self) -> FRINT_R {
        FRINT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Reload Control"]
    #[inline(always)]
    pub fn hfirrldctrl(&self) -> HFIRRLDCTRL_R {
        HFIRRLDCTRL_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Frame Interval"]
    #[inline(always)]
    pub fn frint(&mut self) -> FRINT_W {
        FRINT_W::new(self)
    }
    #[doc = "Bit 16 - Reload Control"]
    #[inline(always)]
    pub fn hfirrldctrl(&mut self) -> HFIRRLDCTRL_W {
        HFIRRLDCTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Frame Interval Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfir](index.html) module"]
pub struct HFIR_SPEC;
impl crate::RegisterSpec for HFIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfir::R](R) reader structure"]
impl crate::Readable for HFIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfir::W](W) writer structure"]
impl crate::Writable for HFIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HFIR to value 0x17d7"]
impl crate::Resettable for HFIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x17d7
    }
}
