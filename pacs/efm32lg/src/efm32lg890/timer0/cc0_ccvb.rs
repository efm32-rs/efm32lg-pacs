#[doc = "Register `CC0_CCVB` reader"]
pub struct R(crate::R<CC0_CCVB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CC0_CCVB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CC0_CCVB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CC0_CCVB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CC0_CCVB` writer"]
pub struct W(crate::W<CC0_CCVB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CC0_CCVB_SPEC>;
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
impl From<crate::W<CC0_CCVB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CC0_CCVB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCVB` reader - CC Channel Value Buffer"]
pub type CCVB_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CCVB` writer - CC Channel Value Buffer"]
pub type CCVB_W<'a> = crate::FieldWriter<'a, u32, CC0_CCVB_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - CC Channel Value Buffer"]
    #[inline(always)]
    pub fn ccvb(&self) -> CCVB_R {
        CCVB_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CC Channel Value Buffer"]
    #[inline(always)]
    pub fn ccvb(&mut self) -> CCVB_W {
        CCVB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CC Channel Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc0_ccvb](index.html) module"]
pub struct CC0_CCVB_SPEC;
impl crate::RegisterSpec for CC0_CCVB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cc0_ccvb::R](R) reader structure"]
impl crate::Readable for CC0_CCVB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cc0_ccvb::W](W) writer structure"]
impl crate::Writable for CC0_CCVB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CC0_CCVB to value 0"]
impl crate::Resettable for CC0_CCVB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
