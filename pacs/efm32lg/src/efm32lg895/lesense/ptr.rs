#[doc = "Register `PTR` reader"]
pub struct R(crate::R<PTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RD` reader - Result buffer read pointer."]
pub type RD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WR` reader - Result buffer write pointer."]
pub type WR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Result buffer read pointer."]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 5:8 - Result buffer write pointer."]
    #[inline(always)]
    pub fn wr(&self) -> WR_R {
        WR_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
}
#[doc = "Result buffer pointers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptr](index.html) module"]
pub struct PTR_SPEC;
impl crate::RegisterSpec for PTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptr::R](R) reader structure"]
impl crate::Readable for PTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PTR to value 0"]
impl crate::Resettable for PTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
