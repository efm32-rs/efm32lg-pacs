#[doc = "Register `RSTCAUSE` reader"]
pub struct R(crate::R<RSTCAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTCAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTCAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTCAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PORST` reader - Power On Reset"]
pub type PORST_R = crate::BitReader<bool>;
#[doc = "Field `BODUNREGRST` reader - Brown Out Detector Unregulated Domain Reset"]
pub type BODUNREGRST_R = crate::BitReader<bool>;
#[doc = "Field `BODREGRST` reader - Brown Out Detector Regulated Domain Reset"]
pub type BODREGRST_R = crate::BitReader<bool>;
#[doc = "Field `EXTRST` reader - External Pin Reset"]
pub type EXTRST_R = crate::BitReader<bool>;
#[doc = "Field `WDOGRST` reader - Watchdog Reset"]
pub type WDOGRST_R = crate::BitReader<bool>;
#[doc = "Field `LOCKUPRST` reader - LOCKUP Reset"]
pub type LOCKUPRST_R = crate::BitReader<bool>;
#[doc = "Field `SYSREQRST` reader - System Request Reset"]
pub type SYSREQRST_R = crate::BitReader<bool>;
#[doc = "Field `EM4RST` reader - EM4 Reset"]
pub type EM4RST_R = crate::BitReader<bool>;
#[doc = "Field `EM4WURST` reader - EM4 Wake-up Reset"]
pub type EM4WURST_R = crate::BitReader<bool>;
#[doc = "Field `BODAVDD0` reader - AVDD0 Bod Reset"]
pub type BODAVDD0_R = crate::BitReader<bool>;
#[doc = "Field `BODAVDD1` reader - AVDD1 Bod Reset"]
pub type BODAVDD1_R = crate::BitReader<bool>;
#[doc = "Field `BUBODVDDDREG` reader - Backup Brown Out Detector, VDD_DREG"]
pub type BUBODVDDDREG_R = crate::BitReader<bool>;
#[doc = "Field `BUBODBUVIN` reader - Backup Brown Out Detector, BU_VIN"]
pub type BUBODBUVIN_R = crate::BitReader<bool>;
#[doc = "Field `BUBODUNREG` reader - Backup Brown Out Detector Unregulated Domain"]
pub type BUBODUNREG_R = crate::BitReader<bool>;
#[doc = "Field `BUBODREG` reader - Backup Brown Out Detector Regulated Domain"]
pub type BUBODREG_R = crate::BitReader<bool>;
#[doc = "Field `BUMODERST` reader - Backup mode reset"]
pub type BUMODERST_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Power On Reset"]
    #[inline(always)]
    pub fn porst(&self) -> PORST_R {
        PORST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Brown Out Detector Unregulated Domain Reset"]
    #[inline(always)]
    pub fn bodunregrst(&self) -> BODUNREGRST_R {
        BODUNREGRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Brown Out Detector Regulated Domain Reset"]
    #[inline(always)]
    pub fn bodregrst(&self) -> BODREGRST_R {
        BODREGRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External Pin Reset"]
    #[inline(always)]
    pub fn extrst(&self) -> EXTRST_R {
        EXTRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Watchdog Reset"]
    #[inline(always)]
    pub fn wdogrst(&self) -> WDOGRST_R {
        WDOGRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LOCKUP Reset"]
    #[inline(always)]
    pub fn lockuprst(&self) -> LOCKUPRST_R {
        LOCKUPRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - System Request Reset"]
    #[inline(always)]
    pub fn sysreqrst(&self) -> SYSREQRST_R {
        SYSREQRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EM4 Reset"]
    #[inline(always)]
    pub fn em4rst(&self) -> EM4RST_R {
        EM4RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - EM4 Wake-up Reset"]
    #[inline(always)]
    pub fn em4wurst(&self) -> EM4WURST_R {
        EM4WURST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AVDD0 Bod Reset"]
    #[inline(always)]
    pub fn bodavdd0(&self) -> BODAVDD0_R {
        BODAVDD0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AVDD1 Bod Reset"]
    #[inline(always)]
    pub fn bodavdd1(&self) -> BODAVDD1_R {
        BODAVDD1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Backup Brown Out Detector, VDD_DREG"]
    #[inline(always)]
    pub fn bubodvdddreg(&self) -> BUBODVDDDREG_R {
        BUBODVDDDREG_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Backup Brown Out Detector, BU_VIN"]
    #[inline(always)]
    pub fn bubodbuvin(&self) -> BUBODBUVIN_R {
        BUBODBUVIN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Backup Brown Out Detector Unregulated Domain"]
    #[inline(always)]
    pub fn bubodunreg(&self) -> BUBODUNREG_R {
        BUBODUNREG_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Backup Brown Out Detector Regulated Domain"]
    #[inline(always)]
    pub fn bubodreg(&self) -> BUBODREG_R {
        BUBODREG_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Backup mode reset"]
    #[inline(always)]
    pub fn bumoderst(&self) -> BUMODERST_R {
        BUMODERST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Reset Cause Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstcause](index.html) module"]
pub struct RSTCAUSE_SPEC;
impl crate::RegisterSpec for RSTCAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstcause::R](R) reader structure"]
impl crate::Readable for RSTCAUSE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSTCAUSE to value 0"]
impl crate::Resettable for RSTCAUSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
