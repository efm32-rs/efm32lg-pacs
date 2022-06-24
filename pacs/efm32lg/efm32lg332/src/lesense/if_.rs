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
#[doc = "Field `CH0` reader - "]
pub type CH0_R = crate::BitReader<bool>;
#[doc = "Field `CH1` reader - "]
pub type CH1_R = crate::BitReader<bool>;
#[doc = "Field `CH2` reader - "]
pub type CH2_R = crate::BitReader<bool>;
#[doc = "Field `CH3` reader - "]
pub type CH3_R = crate::BitReader<bool>;
#[doc = "Field `CH4` reader - "]
pub type CH4_R = crate::BitReader<bool>;
#[doc = "Field `CH5` reader - "]
pub type CH5_R = crate::BitReader<bool>;
#[doc = "Field `CH6` reader - "]
pub type CH6_R = crate::BitReader<bool>;
#[doc = "Field `CH7` reader - "]
pub type CH7_R = crate::BitReader<bool>;
#[doc = "Field `CH8` reader - "]
pub type CH8_R = crate::BitReader<bool>;
#[doc = "Field `CH9` reader - "]
pub type CH9_R = crate::BitReader<bool>;
#[doc = "Field `CH10` reader - "]
pub type CH10_R = crate::BitReader<bool>;
#[doc = "Field `CH11` reader - "]
pub type CH11_R = crate::BitReader<bool>;
#[doc = "Field `CH12` reader - "]
pub type CH12_R = crate::BitReader<bool>;
#[doc = "Field `CH13` reader - "]
pub type CH13_R = crate::BitReader<bool>;
#[doc = "Field `CH14` reader - "]
pub type CH14_R = crate::BitReader<bool>;
#[doc = "Field `CH15` reader - "]
pub type CH15_R = crate::BitReader<bool>;
#[doc = "Field `SCANCOMPLETE` reader - "]
pub type SCANCOMPLETE_R = crate::BitReader<bool>;
#[doc = "Field `DEC` reader - "]
pub type DEC_R = crate::BitReader<bool>;
#[doc = "Field `DECERR` reader - "]
pub type DECERR_R = crate::BitReader<bool>;
#[doc = "Field `BUFDATAV` reader - "]
pub type BUFDATAV_R = crate::BitReader<bool>;
#[doc = "Field `BUFLEVEL` reader - "]
pub type BUFLEVEL_R = crate::BitReader<bool>;
#[doc = "Field `BUFOF` reader - "]
pub type BUFOF_R = crate::BitReader<bool>;
#[doc = "Field `CNTOF` reader - "]
pub type CNTOF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ch4(&self) -> CH4_R {
        CH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ch5(&self) -> CH5_R {
        CH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ch6(&self) -> CH6_R {
        CH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ch7(&self) -> CH7_R {
        CH7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ch8(&self) -> CH8_R {
        CH8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ch9(&self) -> CH9_R {
        CH9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ch10(&self) -> CH10_R {
        CH10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ch11(&self) -> CH11_R {
        CH11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ch12(&self) -> CH12_R {
        CH12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ch13(&self) -> CH13_R {
        CH13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ch14(&self) -> CH14_R {
        CH14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ch15(&self) -> CH15_R {
        CH15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn scancomplete(&self) -> SCANCOMPLETE_R {
        SCANCOMPLETE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn dec(&self) -> DEC_R {
        DEC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn decerr(&self) -> DECERR_R {
        DECERR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn bufdatav(&self) -> BUFDATAV_R {
        BUFDATAV_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn buflevel(&self) -> BUFLEVEL_R {
        BUFLEVEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn bufof(&self) -> BUFOF_R {
        BUFOF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn cntof(&self) -> CNTOF_R {
        CNTOF_R::new(((self.bits >> 22) & 1) != 0)
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
