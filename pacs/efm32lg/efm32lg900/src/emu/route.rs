#[doc = "Register `ROUTE` reader"]
pub struct R(crate::R<ROUTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROUTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROUTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROUTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROUTE` writer"]
pub struct W(crate::W<ROUTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROUTE_SPEC>;
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
impl From<crate::W<ROUTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROUTE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUVINPEN` reader - BU_VIN Pin Enable"]
pub type BUVINPEN_R = crate::BitReader<bool>;
#[doc = "Field `BUVINPEN` writer - BU_VIN Pin Enable"]
pub type BUVINPEN_W<'a> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - BU_VIN Pin Enable"]
    #[inline(always)]
    pub fn buvinpen(&self) -> BUVINPEN_R {
        BUVINPEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BU_VIN Pin Enable"]
    #[inline(always)]
    pub fn buvinpen(&mut self) -> BUVINPEN_W {
        BUVINPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O Routing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [route](index.html) module"]
pub struct ROUTE_SPEC;
impl crate::RegisterSpec for ROUTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [route::R](R) reader structure"]
impl crate::Readable for ROUTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [route::W](W) writer structure"]
impl crate::Writable for ROUTE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROUTE to value 0x01"]
impl crate::Resettable for ROUTE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
