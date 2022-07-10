#[doc = "Register `HPRT` reader"]
pub struct R(crate::R<HPRT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPRT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPRT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPRT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HPRT` writer"]
pub struct W(crate::W<HPRT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HPRT_SPEC>;
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
impl From<crate::W<HPRT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HPRT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRTCONNSTS` reader - Port Connect Status"]
pub type PRTCONNSTS_R = crate::BitReader<bool>;
#[doc = "Field `PRTCONNDET` reader - Port Connect Detected"]
pub type PRTCONNDET_R = crate::BitReader<bool>;
#[doc = "Field `PRTCONNDET` writer - Port Connect Detected"]
pub type PRTCONNDET_W<'a> = crate::BitWriter<'a, u32, HPRT_SPEC, bool, 1>;
#[doc = "Field `PRTENA` reader - Port Enable"]
pub type PRTENA_R = crate::BitReader<bool>;
#[doc = "Field `PRTENA` writer - Port Enable"]
pub type PRTENA_W<'a> = crate::BitWriter<'a, u32, HPRT_SPEC, bool, 2>;
#[doc = "Field `PRTENCHNG` reader - Port Enable/Disable Change"]
pub type PRTENCHNG_R = crate::BitReader<bool>;
#[doc = "Field `PRTENCHNG` writer - Port Enable/Disable Change"]
pub type PRTENCHNG_W<'a> = crate::BitWriter<'a, u32, HPRT_SPEC, bool, 3>;
#[doc = "Field `PRTOVRCURRACT` reader - Port Overcurrent Active"]
pub type PRTOVRCURRACT_R = crate::BitReader<bool>;
#[doc = "Field `PRTOVRCURRCHNG` reader - Port Overcurrent Change"]
pub type PRTOVRCURRCHNG_R = crate::BitReader<bool>;
#[doc = "Field `PRTOVRCURRCHNG` writer - Port Overcurrent Change"]
pub type PRTOVRCURRCHNG_W<'a> = crate::BitWriter<'a, u32, HPRT_SPEC, bool, 5>;
#[doc = "Field `PRTRES` reader - Port Resume"]
pub type PRTRES_R = crate::BitReader<bool>;
#[doc = "Field `PRTRES` writer - Port Resume"]
pub type PRTRES_W<'a> = crate::BitWriter<'a, u32, HPRT_SPEC, bool, 6>;
#[doc = "Field `PRTSUSP` reader - Port Suspend"]
pub type PRTSUSP_R = crate::BitReader<bool>;
#[doc = "Field `PRTSUSP` writer - Port Suspend"]
pub type PRTSUSP_W<'a> = crate::BitWriter<'a, u32, HPRT_SPEC, bool, 7>;
#[doc = "Field `PRTRST` reader - Port Reset"]
pub type PRTRST_R = crate::BitReader<bool>;
#[doc = "Field `PRTRST` writer - Port Reset"]
pub type PRTRST_W<'a> = crate::BitWriter<'a, u32, HPRT_SPEC, bool, 8>;
#[doc = "Field `PRTLNSTS` reader - Port Line Status"]
pub type PRTLNSTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRTPWR` reader - Port Power"]
pub type PRTPWR_R = crate::BitReader<bool>;
#[doc = "Field `PRTPWR` writer - Port Power"]
pub type PRTPWR_W<'a> = crate::BitWriter<'a, u32, HPRT_SPEC, bool, 12>;
#[doc = "Port Test Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRTTSTCTL_A {
    #[doc = "0: Test mode disabled."]
    DISABLE = 0,
    #[doc = "1: Test_J mode."]
    J = 1,
    #[doc = "2: Test_K mode."]
    K = 2,
    #[doc = "3: Test_SE0_NAK mode."]
    SE0NAK = 3,
    #[doc = "4: Test_Packet mode."]
    PACKET = 4,
    #[doc = "5: Test_Force_Enable."]
    FORCE = 5,
}
impl From<PRTTSTCTL_A> for u8 {
    #[inline(always)]
    fn from(variant: PRTTSTCTL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRTTSTCTL` reader - Port Test Control"]
pub type PRTTSTCTL_R = crate::FieldReader<u8, PRTTSTCTL_A>;
impl PRTTSTCTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRTTSTCTL_A> {
        match self.bits {
            0 => Some(PRTTSTCTL_A::DISABLE),
            1 => Some(PRTTSTCTL_A::J),
            2 => Some(PRTTSTCTL_A::K),
            3 => Some(PRTTSTCTL_A::SE0NAK),
            4 => Some(PRTTSTCTL_A::PACKET),
            5 => Some(PRTTSTCTL_A::FORCE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PRTTSTCTL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `J`"]
    #[inline(always)]
    pub fn is_j(&self) -> bool {
        *self == PRTTSTCTL_A::J
    }
    #[doc = "Checks if the value of the field is `K`"]
    #[inline(always)]
    pub fn is_k(&self) -> bool {
        *self == PRTTSTCTL_A::K
    }
    #[doc = "Checks if the value of the field is `SE0NAK`"]
    #[inline(always)]
    pub fn is_se0nak(&self) -> bool {
        *self == PRTTSTCTL_A::SE0NAK
    }
    #[doc = "Checks if the value of the field is `PACKET`"]
    #[inline(always)]
    pub fn is_packet(&self) -> bool {
        *self == PRTTSTCTL_A::PACKET
    }
    #[doc = "Checks if the value of the field is `FORCE`"]
    #[inline(always)]
    pub fn is_force(&self) -> bool {
        *self == PRTTSTCTL_A::FORCE
    }
}
#[doc = "Field `PRTTSTCTL` writer - Port Test Control"]
pub type PRTTSTCTL_W<'a> = crate::FieldWriter<'a, u32, HPRT_SPEC, u8, PRTTSTCTL_A, 4, 13>;
impl<'a> PRTTSTCTL_W<'a> {
    #[doc = "Test mode disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PRTTSTCTL_A::DISABLE)
    }
    #[doc = "Test_J mode."]
    #[inline(always)]
    pub fn j(self) -> &'a mut W {
        self.variant(PRTTSTCTL_A::J)
    }
    #[doc = "Test_K mode."]
    #[inline(always)]
    pub fn k(self) -> &'a mut W {
        self.variant(PRTTSTCTL_A::K)
    }
    #[doc = "Test_SE0_NAK mode."]
    #[inline(always)]
    pub fn se0nak(self) -> &'a mut W {
        self.variant(PRTTSTCTL_A::SE0NAK)
    }
    #[doc = "Test_Packet mode."]
    #[inline(always)]
    pub fn packet(self) -> &'a mut W {
        self.variant(PRTTSTCTL_A::PACKET)
    }
    #[doc = "Test_Force_Enable."]
    #[inline(always)]
    pub fn force(self) -> &'a mut W {
        self.variant(PRTTSTCTL_A::FORCE)
    }
}
#[doc = "Port Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRTSPD_A {
    #[doc = "0: High speed."]
    HS = 0,
    #[doc = "1: Full speed."]
    FS = 1,
    #[doc = "2: Low speed."]
    LS = 2,
}
impl From<PRTSPD_A> for u8 {
    #[inline(always)]
    fn from(variant: PRTSPD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRTSPD` reader - Port Speed"]
pub type PRTSPD_R = crate::FieldReader<u8, PRTSPD_A>;
impl PRTSPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRTSPD_A> {
        match self.bits {
            0 => Some(PRTSPD_A::HS),
            1 => Some(PRTSPD_A::FS),
            2 => Some(PRTSPD_A::LS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HS`"]
    #[inline(always)]
    pub fn is_hs(&self) -> bool {
        *self == PRTSPD_A::HS
    }
    #[doc = "Checks if the value of the field is `FS`"]
    #[inline(always)]
    pub fn is_fs(&self) -> bool {
        *self == PRTSPD_A::FS
    }
    #[doc = "Checks if the value of the field is `LS`"]
    #[inline(always)]
    pub fn is_ls(&self) -> bool {
        *self == PRTSPD_A::LS
    }
}
impl R {
    #[doc = "Bit 0 - Port Connect Status"]
    #[inline(always)]
    pub fn prtconnsts(&self) -> PRTCONNSTS_R {
        PRTCONNSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port Connect Detected"]
    #[inline(always)]
    pub fn prtconndet(&self) -> PRTCONNDET_R {
        PRTCONNDET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Enable"]
    #[inline(always)]
    pub fn prtena(&self) -> PRTENA_R {
        PRTENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port Enable/Disable Change"]
    #[inline(always)]
    pub fn prtenchng(&self) -> PRTENCHNG_R {
        PRTENCHNG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port Overcurrent Active"]
    #[inline(always)]
    pub fn prtovrcurract(&self) -> PRTOVRCURRACT_R {
        PRTOVRCURRACT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port Overcurrent Change"]
    #[inline(always)]
    pub fn prtovrcurrchng(&self) -> PRTOVRCURRCHNG_R {
        PRTOVRCURRCHNG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port Resume"]
    #[inline(always)]
    pub fn prtres(&self) -> PRTRES_R {
        PRTRES_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port Suspend"]
    #[inline(always)]
    pub fn prtsusp(&self) -> PRTSUSP_R {
        PRTSUSP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port Reset"]
    #[inline(always)]
    pub fn prtrst(&self) -> PRTRST_R {
        PRTRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Port Line Status"]
    #[inline(always)]
    pub fn prtlnsts(&self) -> PRTLNSTS_R {
        PRTLNSTS_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Port Power"]
    #[inline(always)]
    pub fn prtpwr(&self) -> PRTPWR_R {
        PRTPWR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:16 - Port Test Control"]
    #[inline(always)]
    pub fn prttstctl(&self) -> PRTTSTCTL_R {
        PRTTSTCTL_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:18 - Port Speed"]
    #[inline(always)]
    pub fn prtspd(&self) -> PRTSPD_R {
        PRTSPD_R::new(((self.bits >> 17) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Port Connect Detected"]
    #[inline(always)]
    pub fn prtconndet(&mut self) -> PRTCONNDET_W {
        PRTCONNDET_W::new(self)
    }
    #[doc = "Bit 2 - Port Enable"]
    #[inline(always)]
    pub fn prtena(&mut self) -> PRTENA_W {
        PRTENA_W::new(self)
    }
    #[doc = "Bit 3 - Port Enable/Disable Change"]
    #[inline(always)]
    pub fn prtenchng(&mut self) -> PRTENCHNG_W {
        PRTENCHNG_W::new(self)
    }
    #[doc = "Bit 5 - Port Overcurrent Change"]
    #[inline(always)]
    pub fn prtovrcurrchng(&mut self) -> PRTOVRCURRCHNG_W {
        PRTOVRCURRCHNG_W::new(self)
    }
    #[doc = "Bit 6 - Port Resume"]
    #[inline(always)]
    pub fn prtres(&mut self) -> PRTRES_W {
        PRTRES_W::new(self)
    }
    #[doc = "Bit 7 - Port Suspend"]
    #[inline(always)]
    pub fn prtsusp(&mut self) -> PRTSUSP_W {
        PRTSUSP_W::new(self)
    }
    #[doc = "Bit 8 - Port Reset"]
    #[inline(always)]
    pub fn prtrst(&mut self) -> PRTRST_W {
        PRTRST_W::new(self)
    }
    #[doc = "Bit 12 - Port Power"]
    #[inline(always)]
    pub fn prtpwr(&mut self) -> PRTPWR_W {
        PRTPWR_W::new(self)
    }
    #[doc = "Bits 13:16 - Port Test Control"]
    #[inline(always)]
    pub fn prttstctl(&mut self) -> PRTTSTCTL_W {
        PRTTSTCTL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Port Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hprt](index.html) module"]
pub struct HPRT_SPEC;
impl crate::RegisterSpec for HPRT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hprt::R](R) reader structure"]
impl crate::Readable for HPRT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hprt::W](W) writer structure"]
impl crate::Writable for HPRT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HPRT to value 0"]
impl crate::Resettable for HPRT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
