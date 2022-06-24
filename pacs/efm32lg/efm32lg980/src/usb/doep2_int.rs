#[doc = "Register `DOEP2_INT` reader"]
pub struct R(crate::R<DOEP2_INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEP2_INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEP2_INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEP2_INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEP2_INT` writer"]
pub struct W(crate::W<DOEP2_INT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEP2_INT_SPEC>;
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
impl From<crate::W<DOEP2_INT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEP2_INT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFERCOMPL` reader - Transfer Completed Interrupt"]
pub type XFERCOMPL_R = crate::BitReader<bool>;
#[doc = "Field `XFERCOMPL` writer - Transfer Completed Interrupt"]
pub type XFERCOMPL_W<'a> = crate::BitWriter<'a, u32, DOEP2_INT_SPEC, bool, 0>;
#[doc = "Field `EPDISBLD` reader - Endpoint Disabled Interrupt"]
pub type EPDISBLD_R = crate::BitReader<bool>;
#[doc = "Field `EPDISBLD` writer - Endpoint Disabled Interrupt"]
pub type EPDISBLD_W<'a> = crate::BitWriter<'a, u32, DOEP2_INT_SPEC, bool, 1>;
#[doc = "Field `AHBERR` reader - AHB Error"]
pub type AHBERR_R = crate::BitReader<bool>;
#[doc = "Field `AHBERR` writer - AHB Error"]
pub type AHBERR_W<'a> = crate::BitWriter<'a, u32, DOEP2_INT_SPEC, bool, 2>;
#[doc = "Field `SETUP` reader - Setup Phase Done"]
pub type SETUP_R = crate::BitReader<bool>;
#[doc = "Field `SETUP` writer - Setup Phase Done"]
pub type SETUP_W<'a> = crate::BitWriter<'a, u32, DOEP2_INT_SPEC, bool, 3>;
#[doc = "Field `OUTTKNEPDIS` reader - OUT Token Received When Endpoint Disabled"]
pub type OUTTKNEPDIS_R = crate::BitReader<bool>;
#[doc = "Field `OUTTKNEPDIS` writer - OUT Token Received When Endpoint Disabled"]
pub type OUTTKNEPDIS_W<'a> = crate::BitWriter<'a, u32, DOEP2_INT_SPEC, bool, 4>;
#[doc = "Field `BACK2BACKSETUP` reader - Back-to-Back SETUP Packets Received"]
pub type BACK2BACKSETUP_R = crate::BitReader<bool>;
#[doc = "Field `BACK2BACKSETUP` writer - Back-to-Back SETUP Packets Received"]
pub type BACK2BACKSETUP_W<'a> = crate::BitWriter<'a, u32, DOEP2_INT_SPEC, bool, 6>;
#[doc = "Field `PKTDRPSTS` reader - Packet Drop Status"]
pub type PKTDRPSTS_R = crate::BitReader<bool>;
#[doc = "Field `PKTDRPSTS` writer - Packet Drop Status"]
pub type PKTDRPSTS_W<'a> = crate::BitWriter<'a, u32, DOEP2_INT_SPEC, bool, 11>;
#[doc = "Field `BBLEERR` reader - Babble Error"]
pub type BBLEERR_R = crate::BitReader<bool>;
#[doc = "Field `BBLEERR` writer - Babble Error"]
pub type BBLEERR_W<'a> = crate::BitWriter<'a, u32, DOEP2_INT_SPEC, bool, 12>;
#[doc = "Field `NAKINTRPT` reader - NAK Interrupt"]
pub type NAKINTRPT_R = crate::BitReader<bool>;
#[doc = "Field `NAKINTRPT` writer - NAK Interrupt"]
pub type NAKINTRPT_W<'a> = crate::BitWriter<'a, u32, DOEP2_INT_SPEC, bool, 13>;
impl R {
    #[doc = "Bit 0 - Transfer Completed Interrupt"]
    #[inline(always)]
    pub fn xfercompl(&self) -> XFERCOMPL_R {
        XFERCOMPL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt"]
    #[inline(always)]
    pub fn epdisbld(&self) -> EPDISBLD_R {
        EPDISBLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    pub fn ahberr(&self) -> AHBERR_R {
        AHBERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Setup Phase Done"]
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OUT Token Received When Endpoint Disabled"]
    #[inline(always)]
    pub fn outtknepdis(&self) -> OUTTKNEPDIS_R {
        OUTTKNEPDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Back-to-Back SETUP Packets Received"]
    #[inline(always)]
    pub fn back2backsetup(&self) -> BACK2BACKSETUP_R {
        BACK2BACKSETUP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 11 - Packet Drop Status"]
    #[inline(always)]
    pub fn pktdrpsts(&self) -> PKTDRPSTS_R {
        PKTDRPSTS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Babble Error"]
    #[inline(always)]
    pub fn bbleerr(&self) -> BBLEERR_R {
        BBLEERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NAK Interrupt"]
    #[inline(always)]
    pub fn nakintrpt(&self) -> NAKINTRPT_R {
        NAKINTRPT_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed Interrupt"]
    #[inline(always)]
    pub fn xfercompl(&mut self) -> XFERCOMPL_W {
        XFERCOMPL_W::new(self)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt"]
    #[inline(always)]
    pub fn epdisbld(&mut self) -> EPDISBLD_W {
        EPDISBLD_W::new(self)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    pub fn ahberr(&mut self) -> AHBERR_W {
        AHBERR_W::new(self)
    }
    #[doc = "Bit 3 - Setup Phase Done"]
    #[inline(always)]
    pub fn setup(&mut self) -> SETUP_W {
        SETUP_W::new(self)
    }
    #[doc = "Bit 4 - OUT Token Received When Endpoint Disabled"]
    #[inline(always)]
    pub fn outtknepdis(&mut self) -> OUTTKNEPDIS_W {
        OUTTKNEPDIS_W::new(self)
    }
    #[doc = "Bit 6 - Back-to-Back SETUP Packets Received"]
    #[inline(always)]
    pub fn back2backsetup(&mut self) -> BACK2BACKSETUP_W {
        BACK2BACKSETUP_W::new(self)
    }
    #[doc = "Bit 11 - Packet Drop Status"]
    #[inline(always)]
    pub fn pktdrpsts(&mut self) -> PKTDRPSTS_W {
        PKTDRPSTS_W::new(self)
    }
    #[doc = "Bit 12 - Babble Error"]
    #[inline(always)]
    pub fn bbleerr(&mut self) -> BBLEERR_W {
        BBLEERR_W::new(self)
    }
    #[doc = "Bit 13 - NAK Interrupt"]
    #[inline(always)]
    pub fn nakintrpt(&mut self) -> NAKINTRPT_W {
        NAKINTRPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device OUT Endpoint x+1 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep2_int](index.html) module"]
pub struct DOEP2_INT_SPEC;
impl crate::RegisterSpec for DOEP2_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doep2_int::R](R) reader structure"]
impl crate::Readable for DOEP2_INT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doep2_int::W](W) writer structure"]
impl crate::Writable for DOEP2_INT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOEP2_INT to value 0"]
impl crate::Resettable for DOEP2_INT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
