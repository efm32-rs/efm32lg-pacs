#[doc = "Register `FREEZE` reader"]
pub struct R(crate::R<FREEZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FREEZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FREEZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FREEZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FREEZE` writer"]
pub struct W(crate::W<FREEZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FREEZE_SPEC>;
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
impl From<crate::W<FREEZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FREEZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGFREEZE` reader - Register Update Freeze"]
pub type REGFREEZE_R = crate::BitReader<bool>;
#[doc = "Field `REGFREEZE` writer - Register Update Freeze"]
pub type REGFREEZE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FREEZE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Register Update Freeze"]
    #[inline(always)]
    pub fn regfreeze(&self) -> REGFREEZE_R {
        REGFREEZE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Register Update Freeze"]
    #[inline(always)]
    #[must_use]
    pub fn regfreeze(&mut self) -> REGFREEZE_W<0> {
        REGFREEZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Freeze Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [freeze](index.html) module"]
pub struct FREEZE_SPEC;
impl crate::RegisterSpec for FREEZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [freeze::R](R) reader structure"]
impl crate::Readable for FREEZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [freeze::W](W) writer structure"]
impl crate::Writable for FREEZE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FREEZE to value 0"]
impl crate::Resettable for FREEZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
