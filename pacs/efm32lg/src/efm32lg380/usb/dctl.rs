#[doc = "Register `DCTL` reader"]
pub struct R(crate::R<DCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCTL` writer"]
pub struct W(crate::W<DCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCTL_SPEC>;
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
impl From<crate::W<DCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RMTWKUPSIG` reader - Remote Wakeup Signaling"]
pub type RMTWKUPSIG_R = crate::BitReader<bool>;
#[doc = "Field `RMTWKUPSIG` writer - Remote Wakeup Signaling"]
pub type RMTWKUPSIG_W<'a> = crate::BitWriter<'a, u32, DCTL_SPEC, bool, 0>;
#[doc = "Field `SFTDISCON` reader - Soft Disconnect"]
pub type SFTDISCON_R = crate::BitReader<bool>;
#[doc = "Field `SFTDISCON` writer - Soft Disconnect"]
pub type SFTDISCON_W<'a> = crate::BitWriter<'a, u32, DCTL_SPEC, bool, 1>;
#[doc = "Field `GNPINNAKSTS` reader - Global Non-periodic IN NAK Status"]
pub type GNPINNAKSTS_R = crate::BitReader<bool>;
#[doc = "Field `GOUTNAKSTS` reader - Global OUT NAK Status"]
pub type GOUTNAKSTS_R = crate::BitReader<bool>;
#[doc = "Test Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSTCTL_A {
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
impl From<TSTCTL_A> for u8 {
    #[inline(always)]
    fn from(variant: TSTCTL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TSTCTL` reader - Test Control"]
pub type TSTCTL_R = crate::FieldReader<u8, TSTCTL_A>;
impl TSTCTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSTCTL_A> {
        match self.bits {
            0 => Some(TSTCTL_A::DISABLE),
            1 => Some(TSTCTL_A::J),
            2 => Some(TSTCTL_A::K),
            3 => Some(TSTCTL_A::SE0NAK),
            4 => Some(TSTCTL_A::PACKET),
            5 => Some(TSTCTL_A::FORCE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TSTCTL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `J`"]
    #[inline(always)]
    pub fn is_j(&self) -> bool {
        *self == TSTCTL_A::J
    }
    #[doc = "Checks if the value of the field is `K`"]
    #[inline(always)]
    pub fn is_k(&self) -> bool {
        *self == TSTCTL_A::K
    }
    #[doc = "Checks if the value of the field is `SE0NAK`"]
    #[inline(always)]
    pub fn is_se0nak(&self) -> bool {
        *self == TSTCTL_A::SE0NAK
    }
    #[doc = "Checks if the value of the field is `PACKET`"]
    #[inline(always)]
    pub fn is_packet(&self) -> bool {
        *self == TSTCTL_A::PACKET
    }
    #[doc = "Checks if the value of the field is `FORCE`"]
    #[inline(always)]
    pub fn is_force(&self) -> bool {
        *self == TSTCTL_A::FORCE
    }
}
#[doc = "Field `TSTCTL` writer - Test Control"]
pub type TSTCTL_W<'a> = crate::FieldWriter<'a, u32, DCTL_SPEC, u8, TSTCTL_A, 3, 4>;
impl<'a> TSTCTL_W<'a> {
    #[doc = "Test mode disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TSTCTL_A::DISABLE)
    }
    #[doc = "Test_J mode."]
    #[inline(always)]
    pub fn j(self) -> &'a mut W {
        self.variant(TSTCTL_A::J)
    }
    #[doc = "Test_K mode."]
    #[inline(always)]
    pub fn k(self) -> &'a mut W {
        self.variant(TSTCTL_A::K)
    }
    #[doc = "Test_SE0_NAK mode."]
    #[inline(always)]
    pub fn se0nak(self) -> &'a mut W {
        self.variant(TSTCTL_A::SE0NAK)
    }
    #[doc = "Test_Packet mode."]
    #[inline(always)]
    pub fn packet(self) -> &'a mut W {
        self.variant(TSTCTL_A::PACKET)
    }
    #[doc = "Test_Force_Enable."]
    #[inline(always)]
    pub fn force(self) -> &'a mut W {
        self.variant(TSTCTL_A::FORCE)
    }
}
#[doc = "Field `SGNPINNAK` writer - Set Global Non-periodic IN NAK"]
pub type SGNPINNAK_W<'a> = crate::BitWriter<'a, u32, DCTL_SPEC, bool, 7>;
#[doc = "Field `CGNPINNAK` writer - Clear Global Non-periodic IN NAK"]
pub type CGNPINNAK_W<'a> = crate::BitWriter<'a, u32, DCTL_SPEC, bool, 8>;
#[doc = "Field `SGOUTNAK` writer - Set Global OUT NAK"]
pub type SGOUTNAK_W<'a> = crate::BitWriter<'a, u32, DCTL_SPEC, bool, 9>;
#[doc = "Field `CGOUTNAK` writer - Clear Global OUT NAK"]
pub type CGOUTNAK_W<'a> = crate::BitWriter<'a, u32, DCTL_SPEC, bool, 10>;
#[doc = "Field `PWRONPRGDONE` reader - Power-On Programming Done"]
pub type PWRONPRGDONE_R = crate::BitReader<bool>;
#[doc = "Field `PWRONPRGDONE` writer - Power-On Programming Done"]
pub type PWRONPRGDONE_W<'a> = crate::BitWriter<'a, u32, DCTL_SPEC, bool, 11>;
#[doc = "Field `IGNRFRMNUM` reader - Ignore Frame number For Isochronous End points"]
pub type IGNRFRMNUM_R = crate::BitReader<bool>;
#[doc = "Field `IGNRFRMNUM` writer - Ignore Frame number For Isochronous End points"]
pub type IGNRFRMNUM_W<'a> = crate::BitWriter<'a, u32, DCTL_SPEC, bool, 15>;
#[doc = "Field `NAKONBBLE` reader - NAK on Babble Error"]
pub type NAKONBBLE_R = crate::BitReader<bool>;
#[doc = "Field `NAKONBBLE` writer - NAK on Babble Error"]
pub type NAKONBBLE_W<'a> = crate::BitWriter<'a, u32, DCTL_SPEC, bool, 16>;
impl R {
    #[doc = "Bit 0 - Remote Wakeup Signaling"]
    #[inline(always)]
    pub fn rmtwkupsig(&self) -> RMTWKUPSIG_R {
        RMTWKUPSIG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Soft Disconnect"]
    #[inline(always)]
    pub fn sftdiscon(&self) -> SFTDISCON_R {
        SFTDISCON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Global Non-periodic IN NAK Status"]
    #[inline(always)]
    pub fn gnpinnaksts(&self) -> GNPINNAKSTS_R {
        GNPINNAKSTS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Global OUT NAK Status"]
    #[inline(always)]
    pub fn goutnaksts(&self) -> GOUTNAKSTS_R {
        GOUTNAKSTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Test Control"]
    #[inline(always)]
    pub fn tstctl(&self) -> TSTCTL_R {
        TSTCTL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 11 - Power-On Programming Done"]
    #[inline(always)]
    pub fn pwronprgdone(&self) -> PWRONPRGDONE_R {
        PWRONPRGDONE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Ignore Frame number For Isochronous End points"]
    #[inline(always)]
    pub fn ignrfrmnum(&self) -> IGNRFRMNUM_R {
        IGNRFRMNUM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - NAK on Babble Error"]
    #[inline(always)]
    pub fn nakonbble(&self) -> NAKONBBLE_R {
        NAKONBBLE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Remote Wakeup Signaling"]
    #[inline(always)]
    pub fn rmtwkupsig(&mut self) -> RMTWKUPSIG_W {
        RMTWKUPSIG_W::new(self)
    }
    #[doc = "Bit 1 - Soft Disconnect"]
    #[inline(always)]
    pub fn sftdiscon(&mut self) -> SFTDISCON_W {
        SFTDISCON_W::new(self)
    }
    #[doc = "Bits 4:6 - Test Control"]
    #[inline(always)]
    pub fn tstctl(&mut self) -> TSTCTL_W {
        TSTCTL_W::new(self)
    }
    #[doc = "Bit 7 - Set Global Non-periodic IN NAK"]
    #[inline(always)]
    pub fn sgnpinnak(&mut self) -> SGNPINNAK_W {
        SGNPINNAK_W::new(self)
    }
    #[doc = "Bit 8 - Clear Global Non-periodic IN NAK"]
    #[inline(always)]
    pub fn cgnpinnak(&mut self) -> CGNPINNAK_W {
        CGNPINNAK_W::new(self)
    }
    #[doc = "Bit 9 - Set Global OUT NAK"]
    #[inline(always)]
    pub fn sgoutnak(&mut self) -> SGOUTNAK_W {
        SGOUTNAK_W::new(self)
    }
    #[doc = "Bit 10 - Clear Global OUT NAK"]
    #[inline(always)]
    pub fn cgoutnak(&mut self) -> CGOUTNAK_W {
        CGOUTNAK_W::new(self)
    }
    #[doc = "Bit 11 - Power-On Programming Done"]
    #[inline(always)]
    pub fn pwronprgdone(&mut self) -> PWRONPRGDONE_W {
        PWRONPRGDONE_W::new(self)
    }
    #[doc = "Bit 15 - Ignore Frame number For Isochronous End points"]
    #[inline(always)]
    pub fn ignrfrmnum(&mut self) -> IGNRFRMNUM_W {
        IGNRFRMNUM_W::new(self)
    }
    #[doc = "Bit 16 - NAK on Babble Error"]
    #[inline(always)]
    pub fn nakonbble(&mut self) -> NAKONBBLE_W {
        NAKONBBLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dctl](index.html) module"]
pub struct DCTL_SPEC;
impl crate::RegisterSpec for DCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dctl::R](R) reader structure"]
impl crate::Readable for DCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dctl::W](W) writer structure"]
impl crate::Writable for DCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCTL to value 0"]
impl crate::Resettable for DCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
