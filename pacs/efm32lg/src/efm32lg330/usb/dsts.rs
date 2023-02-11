#[doc = "Register `DSTS` reader"]
pub struct R(crate::R<DSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SUSPSTS` reader - Suspend Status"]
pub type SUSPSTS_R = crate::BitReader<bool>;
#[doc = "Field `ENUMSPD` reader - Enumerated Speed"]
pub type ENUMSPD_R = crate::FieldReader<u8, ENUMSPD_A>;
#[doc = "Enumerated Speed\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ENUMSPD_A {
    #[doc = "2: Low speed (PHY clock is running at 6 MHz)."]
    LS = 2,
    #[doc = "3: Full speed (PHY clock is running at 48 MHz)."]
    FS = 3,
}
impl From<ENUMSPD_A> for u8 {
    #[inline(always)]
    fn from(variant: ENUMSPD_A) -> Self {
        variant as _
    }
}
impl ENUMSPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ENUMSPD_A> {
        match self.bits {
            2 => Some(ENUMSPD_A::LS),
            3 => Some(ENUMSPD_A::FS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LS`"]
    #[inline(always)]
    pub fn is_ls(&self) -> bool {
        *self == ENUMSPD_A::LS
    }
    #[doc = "Checks if the value of the field is `FS`"]
    #[inline(always)]
    pub fn is_fs(&self) -> bool {
        *self == ENUMSPD_A::FS
    }
}
#[doc = "Field `ERRTICERR` reader - Erratic Error"]
pub type ERRTICERR_R = crate::BitReader<bool>;
#[doc = "Field `SOFFN` reader - Frame Number of the Received SOF"]
pub type SOFFN_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0 - Suspend Status"]
    #[inline(always)]
    pub fn suspsts(&self) -> SUSPSTS_R {
        SUSPSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Enumerated Speed"]
    #[inline(always)]
    pub fn enumspd(&self) -> ENUMSPD_R {
        ENUMSPD_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Erratic Error"]
    #[inline(always)]
    pub fn errticerr(&self) -> ERRTICERR_R {
        ERRTICERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:21 - Frame Number of the Received SOF"]
    #[inline(always)]
    pub fn soffn(&self) -> SOFFN_R {
        SOFFN_R::new(((self.bits >> 8) & 0x3fff) as u16)
    }
}
#[doc = "Device Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsts](index.html) module"]
pub struct DSTS_SPEC;
impl crate::RegisterSpec for DSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsts::R](R) reader structure"]
impl crate::Readable for DSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DSTS to value 0x02"]
impl crate::Resettable for DSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
