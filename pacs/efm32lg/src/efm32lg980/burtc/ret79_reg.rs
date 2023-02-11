#[doc = "Register `RET79_REG` reader"]
pub struct R(crate::R<RET79_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RET79_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RET79_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RET79_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RET79_REG` writer"]
pub struct W(crate::W<RET79_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RET79_REG_SPEC>;
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
impl From<crate::W<RET79_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RET79_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG` reader - General Purpose Retention Register"]
pub type REG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `REG` writer - General Purpose Retention Register"]
pub type REG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RET79_REG_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - General Purpose Retention Register"]
    #[inline(always)]
    pub fn reg(&self) -> REG_R {
        REG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - General Purpose Retention Register"]
    #[inline(always)]
    #[must_use]
    pub fn reg(&mut self) -> REG_W<0> {
        REG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Retention Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ret79_reg](index.html) module"]
pub struct RET79_REG_SPEC;
impl crate::RegisterSpec for RET79_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ret79_reg::R](R) reader structure"]
impl crate::Readable for RET79_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ret79_reg::W](W) writer structure"]
impl crate::Writable for RET79_REG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RET79_REG to value 0"]
impl crate::Resettable for RET79_REG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
