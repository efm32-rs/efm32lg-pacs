#[doc = "Register `GUSBCFG` reader"]
pub struct R(crate::R<GUSBCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GUSBCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GUSBCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GUSBCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GUSBCFG` writer"]
pub struct W(crate::W<GUSBCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GUSBCFG_SPEC>;
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
impl From<crate::W<GUSBCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GUSBCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUTCAL` reader - Timeout Calibration host and device"]
pub type TOUTCAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TOUTCAL` writer - Timeout Calibration host and device"]
pub type TOUTCAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GUSBCFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `FSINTF` reader - Full-Speed Serial Interface Select host and device"]
pub type FSINTF_R = crate::BitReader<bool>;
#[doc = "Field `FSINTF` writer - Full-Speed Serial Interface Select host and device"]
pub type FSINTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, O>;
#[doc = "Field `SRPCAP` reader - SRP-Capable host and device"]
pub type SRPCAP_R = crate::BitReader<bool>;
#[doc = "Field `SRPCAP` writer - SRP-Capable host and device"]
pub type SRPCAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, O>;
#[doc = "Field `HNPCAP` reader - HNP-Capable host and device"]
pub type HNPCAP_R = crate::BitReader<bool>;
#[doc = "Field `HNPCAP` writer - HNP-Capable host and device"]
pub type HNPCAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, O>;
#[doc = "Field `USBTRDTIM` reader - USB Turnaround Time device only"]
pub type USBTRDTIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USBTRDTIM` writer - USB Turnaround Time device only"]
pub type USBTRDTIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GUSBCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `TERMSELDLPULSE` reader - TermSel DLine Pulsing Selection device only"]
pub type TERMSELDLPULSE_R = crate::BitReader<bool>;
#[doc = "Field `TERMSELDLPULSE` writer - TermSel DLine Pulsing Selection device only"]
pub type TERMSELDLPULSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, O>;
#[doc = "Field `TXENDDELAY` reader - Tx End Delay device only"]
pub type TXENDDELAY_R = crate::BitReader<bool>;
#[doc = "Field `TXENDDELAY` writer - Tx End Delay device only"]
pub type TXENDDELAY_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, O>;
#[doc = "Field `FORCEHSTMODE` reader - Force Host Mode host and device"]
pub type FORCEHSTMODE_R = crate::BitReader<bool>;
#[doc = "Field `FORCEHSTMODE` writer - Force Host Mode host and device"]
pub type FORCEHSTMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, O>;
#[doc = "Field `FORCEDEVMODE` reader - Force Device Mode host and device"]
pub type FORCEDEVMODE_R = crate::BitReader<bool>;
#[doc = "Field `FORCEDEVMODE` writer - Force Device Mode host and device"]
pub type FORCEDEVMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, O>;
#[doc = "Field `CORRUPTTXPKT` writer - Corrupt Tx packet host and device"]
pub type CORRUPTTXPKT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Timeout Calibration host and device"]
    #[inline(always)]
    pub fn toutcal(&self) -> TOUTCAL_R {
        TOUTCAL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 5 - Full-Speed Serial Interface Select host and device"]
    #[inline(always)]
    pub fn fsintf(&self) -> FSINTF_R {
        FSINTF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - SRP-Capable host and device"]
    #[inline(always)]
    pub fn srpcap(&self) -> SRPCAP_R {
        SRPCAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HNP-Capable host and device"]
    #[inline(always)]
    pub fn hnpcap(&self) -> HNPCAP_R {
        HNPCAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - USB Turnaround Time device only"]
    #[inline(always)]
    pub fn usbtrdtim(&self) -> USBTRDTIM_R {
        USBTRDTIM_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - TermSel DLine Pulsing Selection device only"]
    #[inline(always)]
    pub fn termseldlpulse(&self) -> TERMSELDLPULSE_R {
        TERMSELDLPULSE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 28 - Tx End Delay device only"]
    #[inline(always)]
    pub fn txenddelay(&self) -> TXENDDELAY_R {
        TXENDDELAY_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Force Host Mode host and device"]
    #[inline(always)]
    pub fn forcehstmode(&self) -> FORCEHSTMODE_R {
        FORCEHSTMODE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Force Device Mode host and device"]
    #[inline(always)]
    pub fn forcedevmode(&self) -> FORCEDEVMODE_R {
        FORCEDEVMODE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Timeout Calibration host and device"]
    #[inline(always)]
    #[must_use]
    pub fn toutcal(&mut self) -> TOUTCAL_W<0> {
        TOUTCAL_W::new(self)
    }
    #[doc = "Bit 5 - Full-Speed Serial Interface Select host and device"]
    #[inline(always)]
    #[must_use]
    pub fn fsintf(&mut self) -> FSINTF_W<5> {
        FSINTF_W::new(self)
    }
    #[doc = "Bit 8 - SRP-Capable host and device"]
    #[inline(always)]
    #[must_use]
    pub fn srpcap(&mut self) -> SRPCAP_W<8> {
        SRPCAP_W::new(self)
    }
    #[doc = "Bit 9 - HNP-Capable host and device"]
    #[inline(always)]
    #[must_use]
    pub fn hnpcap(&mut self) -> HNPCAP_W<9> {
        HNPCAP_W::new(self)
    }
    #[doc = "Bits 10:13 - USB Turnaround Time device only"]
    #[inline(always)]
    #[must_use]
    pub fn usbtrdtim(&mut self) -> USBTRDTIM_W<10> {
        USBTRDTIM_W::new(self)
    }
    #[doc = "Bit 22 - TermSel DLine Pulsing Selection device only"]
    #[inline(always)]
    #[must_use]
    pub fn termseldlpulse(&mut self) -> TERMSELDLPULSE_W<22> {
        TERMSELDLPULSE_W::new(self)
    }
    #[doc = "Bit 28 - Tx End Delay device only"]
    #[inline(always)]
    #[must_use]
    pub fn txenddelay(&mut self) -> TXENDDELAY_W<28> {
        TXENDDELAY_W::new(self)
    }
    #[doc = "Bit 29 - Force Host Mode host and device"]
    #[inline(always)]
    #[must_use]
    pub fn forcehstmode(&mut self) -> FORCEHSTMODE_W<29> {
        FORCEHSTMODE_W::new(self)
    }
    #[doc = "Bit 30 - Force Device Mode host and device"]
    #[inline(always)]
    #[must_use]
    pub fn forcedevmode(&mut self) -> FORCEDEVMODE_W<30> {
        FORCEDEVMODE_W::new(self)
    }
    #[doc = "Bit 31 - Corrupt Tx packet host and device"]
    #[inline(always)]
    #[must_use]
    pub fn corrupttxpkt(&mut self) -> CORRUPTTXPKT_W<31> {
        CORRUPTTXPKT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gusbcfg](index.html) module"]
pub struct GUSBCFG_SPEC;
impl crate::RegisterSpec for GUSBCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gusbcfg::R](R) reader structure"]
impl crate::Readable for GUSBCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gusbcfg::W](W) writer structure"]
impl crate::Writable for GUSBCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GUSBCFG to value 0x1440"]
impl crate::Resettable for GUSBCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x1440;
}
