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
#[doc = "Field `CH0` reader - Channel 0 Conversion Complete Interrupt Flag"]
pub type CH0_R = crate::BitReader<bool>;
#[doc = "Field `CH1` reader - Channel 1 Conversion Complete Interrupt Flag"]
pub type CH1_R = crate::BitReader<bool>;
#[doc = "Field `CH0UF` reader - Channel 0 Data Underflow Interrupt Flag"]
pub type CH0UF_R = crate::BitReader<bool>;
#[doc = "Field `CH1UF` reader - Channel 1 Data Underflow Interrupt Flag"]
pub type CH1UF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Channel 0 Conversion Complete Interrupt Flag"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Conversion Complete Interrupt Flag"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 0 Data Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn ch0uf(&self) -> CH0UF_R {
        CH0UF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 1 Data Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn ch1uf(&self) -> CH1UF_R {
        CH1UF_R::new(((self.bits >> 5) & 1) != 0)
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
