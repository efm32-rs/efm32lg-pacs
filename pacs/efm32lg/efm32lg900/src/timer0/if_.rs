#[doc = "Register `IF` reader"]
pub struct R(crate::R<IF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OF` reader - Overflow Interrupt Flag"]
pub type OF_R = crate::BitReader<bool>;
#[doc = "Field `UF` reader - Underflow Interrupt Flag"]
pub type UF_R = crate::BitReader<bool>;
#[doc = "Field `CC0` reader - CC Channel 0 Interrupt Flag"]
pub type CC0_R = crate::BitReader<bool>;
#[doc = "Field `CC1` reader - CC Channel 1 Interrupt Flag"]
pub type CC1_R = crate::BitReader<bool>;
#[doc = "Field `CC2` reader - CC Channel 2 Interrupt Flag"]
pub type CC2_R = crate::BitReader<bool>;
#[doc = "Field `ICBOF0` reader - CC Channel 0 Input Capture Buffer Overflow Interrupt Flag"]
pub type ICBOF0_R = crate::BitReader<bool>;
#[doc = "Field `ICBOF1` reader - CC Channel 1 Input Capture Buffer Overflow Interrupt Flag"]
pub type ICBOF1_R = crate::BitReader<bool>;
#[doc = "Field `ICBOF2` reader - CC Channel 2 Input Capture Buffer Overflow Interrupt Flag"]
pub type ICBOF2_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn of(&self) -> OF_R {
        OF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn uf(&self) -> UF_R {
        UF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - CC Channel 0 Interrupt Flag"]
    #[inline(always)]
    pub fn cc0(&self) -> CC0_R {
        CC0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CC Channel 1 Interrupt Flag"]
    #[inline(always)]
    pub fn cc1(&self) -> CC1_R {
        CC1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CC Channel 2 Interrupt Flag"]
    #[inline(always)]
    pub fn cc2(&self) -> CC2_R {
        CC2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - CC Channel 0 Input Capture Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn icbof0(&self) -> ICBOF0_R {
        ICBOF0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CC Channel 1 Input Capture Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn icbof1(&self) -> ICBOF1_R {
        ICBOF1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CC Channel 2 Input Capture Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn icbof2(&self) -> ICBOF2_R {
        ICBOF2_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](index.html) module"]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if_::R](R) reader structure"]
impl crate::Readable for IF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
