#[doc = "Register `ETMLAR` reader"]
pub struct R(crate::R<ETMLAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETMLAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETMLAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETMLAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETMLAR` writer"]
pub struct W(crate::W<ETMLAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETMLAR_SPEC>;
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
impl From<crate::W<ETMLAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETMLAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY` reader - Key Value"]
pub type KEY_R = crate::BitReader<bool>;
#[doc = "Field `KEY` writer - Key Value"]
pub type KEY_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETMLAR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Key Value"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Key Value"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<0> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ETM Lock Access Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmlar](index.html) module"]
pub struct ETMLAR_SPEC;
impl crate::RegisterSpec for ETMLAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etmlar::R](R) reader structure"]
impl crate::Readable for ETMLAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etmlar::W](W) writer structure"]
impl crate::Writable for ETMLAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETMLAR to value 0"]
impl crate::Resettable for ETMLAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
