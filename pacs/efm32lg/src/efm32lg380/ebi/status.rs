#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AHBACT` reader - EBI Busy with AHB Transaction."]
pub type AHBACT_R = crate::BitReader<bool>;
#[doc = "Field `ECCACT` reader - EBI ECC Generation Active."]
pub type ECCACT_R = crate::BitReader<bool>;
#[doc = "Field `TFTPIXEL0EMPTY` reader - EBI_TFTPIXEL0 is empty."]
pub type TFTPIXEL0EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `TFTPIXEL1EMPTY` reader - EBI_TFTPIXEL1 is empty."]
pub type TFTPIXEL1EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `TFTPIXELFULL` reader - EBI_TFTPIXEL0 is full."]
pub type TFTPIXELFULL_R = crate::BitReader<bool>;
#[doc = "Field `DDACT` reader - EBI Busy with Direct Drive Transactions."]
pub type DDACT_R = crate::BitReader<bool>;
#[doc = "Field `TFTDDEMPTY` reader - EBI_TFTDD register is empty."]
pub type TFTDDEMPTY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - EBI Busy with AHB Transaction."]
    #[inline(always)]
    pub fn ahbact(&self) -> AHBACT_R {
        AHBACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - EBI ECC Generation Active."]
    #[inline(always)]
    pub fn eccact(&self) -> ECCACT_R {
        ECCACT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - EBI_TFTPIXEL0 is empty."]
    #[inline(always)]
    pub fn tftpixel0empty(&self) -> TFTPIXEL0EMPTY_R {
        TFTPIXEL0EMPTY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EBI_TFTPIXEL1 is empty."]
    #[inline(always)]
    pub fn tftpixel1empty(&self) -> TFTPIXEL1EMPTY_R {
        TFTPIXEL1EMPTY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EBI_TFTPIXEL0 is full."]
    #[inline(always)]
    pub fn tftpixelfull(&self) -> TFTPIXELFULL_R {
        TFTPIXELFULL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - EBI Busy with Direct Drive Transactions."]
    #[inline(always)]
    pub fn ddact(&self) -> DDACT_R {
        DDACT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - EBI_TFTDD register is empty."]
    #[inline(always)]
    pub fn tftddempty(&self) -> TFTDDEMPTY_R {
        TFTDDEMPTY_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
