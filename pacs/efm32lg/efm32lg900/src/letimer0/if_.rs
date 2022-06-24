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
#[doc = "Field `COMP0` reader - Compare Match 0 Interrupt Flag"]
pub type COMP0_R = crate::BitReader<bool>;
#[doc = "Field `COMP1` reader - Compare Match 1 Interrupt Flag"]
pub type COMP1_R = crate::BitReader<bool>;
#[doc = "Field `UF` reader - Underflow Interrupt Flag"]
pub type UF_R = crate::BitReader<bool>;
#[doc = "Field `REP0` reader - Repeat Counter 0 Interrupt Flag"]
pub type REP0_R = crate::BitReader<bool>;
#[doc = "Field `REP1` reader - Repeat Counter 1 Interrupt Flag"]
pub type REP1_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Compare Match 0 Interrupt Flag"]
    #[inline(always)]
    pub fn comp0(&self) -> COMP0_R {
        COMP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare Match 1 Interrupt Flag"]
    #[inline(always)]
    pub fn comp1(&self) -> COMP1_R {
        COMP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn uf(&self) -> UF_R {
        UF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Repeat Counter 0 Interrupt Flag"]
    #[inline(always)]
    pub fn rep0(&self) -> REP0_R {
        REP0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Repeat Counter 1 Interrupt Flag"]
    #[inline(always)]
    pub fn rep1(&self) -> REP1_R {
        REP1_R::new(((self.bits >> 4) & 1) != 0)
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
