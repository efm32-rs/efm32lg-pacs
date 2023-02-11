#[doc = "Register `HFNUM` reader"]
pub struct R(crate::R<HFNUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFNUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFNUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFNUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FRNUM` reader - Frame Number"]
pub type FRNUM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FRREM` reader - Frame Time Remaining"]
pub type FRREM_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Frame Number"]
    #[inline(always)]
    pub fn frnum(&self) -> FRNUM_R {
        FRNUM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Frame Time Remaining"]
    #[inline(always)]
    pub fn frrem(&self) -> FRREM_R {
        FRREM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Host Frame Number/Frame Time Remaining Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfnum](index.html) module"]
pub struct HFNUM_SPEC;
impl crate::RegisterSpec for HFNUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfnum::R](R) reader structure"]
impl crate::Readable for HFNUM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HFNUM to value 0x3fff"]
impl crate::Resettable for HFNUM_SPEC {
    const RESET_VALUE: Self::Ux = 0x3fff;
}
