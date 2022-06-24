#[doc = "Register `PB_DIN` reader"]
pub struct R(crate::R<PB_DIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PB_DIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PB_DIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PB_DIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIN` reader - Data In"]
pub type DIN_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data In"]
    #[inline(always)]
    pub fn din(&self) -> DIN_R {
        DIN_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Port Data In Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pb_din](index.html) module"]
pub struct PB_DIN_SPEC;
impl crate::RegisterSpec for PB_DIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pb_din::R](R) reader structure"]
impl crate::Readable for PB_DIN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PB_DIN to value 0"]
impl crate::Resettable for PB_DIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
