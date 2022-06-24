#[doc = "Register `BUCTRL` reader"]
pub struct R(crate::R<BUCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUCTRL` writer"]
pub struct W(crate::W<BUCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<BUCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Enable backup mode"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Enable backup mode"]
pub type EN_W<'a> = crate::BitWriter<'a, u32, BUCTRL_SPEC, bool, 0>;
#[doc = "Field `STATEN` reader - Enable backup mode status export"]
pub type STATEN_R = crate::BitReader<bool>;
#[doc = "Field `STATEN` writer - Enable backup mode status export"]
pub type STATEN_W<'a> = crate::BitWriter<'a, u32, BUCTRL_SPEC, bool, 1>;
#[doc = "Field `BODCAL` reader - Enable BOD calibration mode"]
pub type BODCAL_R = crate::BitReader<bool>;
#[doc = "Field `BODCAL` writer - Enable BOD calibration mode"]
pub type BODCAL_W<'a> = crate::BitWriter<'a, u32, BUCTRL_SPEC, bool, 2>;
#[doc = "Field `BUMODEBODEN` reader - Enable brown out detection on BU_VIN when in backup mode"]
pub type BUMODEBODEN_R = crate::BitReader<bool>;
#[doc = "Field `BUMODEBODEN` writer - Enable brown out detection on BU_VIN when in backup mode"]
pub type BUMODEBODEN_W<'a> = crate::BitWriter<'a, u32, BUCTRL_SPEC, bool, 3>;
#[doc = "Voltage probe select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PROBE_A {
    #[doc = "0: Disable voltage probe."]
    DISABLE = 0,
    #[doc = "1: Connect probe to VDD_DREG."]
    VDDDREG = 1,
    #[doc = "2: Connect probe to BU_IN."]
    BUIN = 2,
    #[doc = "3: Connect probe to BU_OUT."]
    BUOUT = 3,
}
impl From<PROBE_A> for u8 {
    #[inline(always)]
    fn from(variant: PROBE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PROBE` reader - Voltage probe select"]
pub type PROBE_R = crate::FieldReader<u8, PROBE_A>;
impl PROBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROBE_A {
        match self.bits {
            0 => PROBE_A::DISABLE,
            1 => PROBE_A::VDDDREG,
            2 => PROBE_A::BUIN,
            3 => PROBE_A::BUOUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PROBE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `VDDDREG`"]
    #[inline(always)]
    pub fn is_vdddreg(&self) -> bool {
        *self == PROBE_A::VDDDREG
    }
    #[doc = "Checks if the value of the field is `BUIN`"]
    #[inline(always)]
    pub fn is_buin(&self) -> bool {
        *self == PROBE_A::BUIN
    }
    #[doc = "Checks if the value of the field is `BUOUT`"]
    #[inline(always)]
    pub fn is_buout(&self) -> bool {
        *self == PROBE_A::BUOUT
    }
}
#[doc = "Field `PROBE` writer - Voltage probe select"]
pub type PROBE_W<'a> = crate::FieldWriterSafe<'a, u32, BUCTRL_SPEC, u8, PROBE_A, 2, 5>;
impl<'a> PROBE_W<'a> {
    #[doc = "Disable voltage probe."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PROBE_A::DISABLE)
    }
    #[doc = "Connect probe to VDD_DREG."]
    #[inline(always)]
    pub fn vdddreg(self) -> &'a mut W {
        self.variant(PROBE_A::VDDDREG)
    }
    #[doc = "Connect probe to BU_IN."]
    #[inline(always)]
    pub fn buin(self) -> &'a mut W {
        self.variant(PROBE_A::BUIN)
    }
    #[doc = "Connect probe to BU_OUT."]
    #[inline(always)]
    pub fn buout(self) -> &'a mut W {
        self.variant(PROBE_A::BUOUT)
    }
}
impl R {
    #[doc = "Bit 0 - Enable backup mode"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable backup mode status export"]
    #[inline(always)]
    pub fn staten(&self) -> STATEN_R {
        STATEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable BOD calibration mode"]
    #[inline(always)]
    pub fn bodcal(&self) -> BODCAL_R {
        BODCAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable brown out detection on BU_VIN when in backup mode"]
    #[inline(always)]
    pub fn bumodeboden(&self) -> BUMODEBODEN_R {
        BUMODEBODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Voltage probe select"]
    #[inline(always)]
    pub fn probe(&self) -> PROBE_R {
        PROBE_R::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable backup mode"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Enable backup mode status export"]
    #[inline(always)]
    pub fn staten(&mut self) -> STATEN_W {
        STATEN_W::new(self)
    }
    #[doc = "Bit 2 - Enable BOD calibration mode"]
    #[inline(always)]
    pub fn bodcal(&mut self) -> BODCAL_W {
        BODCAL_W::new(self)
    }
    #[doc = "Bit 3 - Enable brown out detection on BU_VIN when in backup mode"]
    #[inline(always)]
    pub fn bumodeboden(&mut self) -> BUMODEBODEN_W {
        BUMODEBODEN_W::new(self)
    }
    #[doc = "Bits 5:6 - Voltage probe select"]
    #[inline(always)]
    pub fn probe(&mut self) -> PROBE_W {
        PROBE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup Power configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buctrl](index.html) module"]
pub struct BUCTRL_SPEC;
impl crate::RegisterSpec for BUCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buctrl::R](R) reader structure"]
impl crate::Readable for BUCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buctrl::W](W) writer structure"]
impl crate::Writable for BUCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BUCTRL to value 0"]
impl crate::Resettable for BUCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
