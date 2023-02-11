#[doc = "Register `TFTPIXEL` reader"]
pub struct R(crate::R<TFTPIXEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TFTPIXEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TFTPIXEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TFTPIXEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - Alpha Blending Result"]
pub type DATA_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Alpha Blending Result"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "TFT Alpha Blending Result Pixel Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tftpixel](index.html) module"]
pub struct TFTPIXEL_SPEC;
impl crate::RegisterSpec for TFTPIXEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tftpixel::R](R) reader structure"]
impl crate::Readable for TFTPIXEL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TFTPIXEL to value 0"]
impl crate::Resettable for TFTPIXEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
