#[doc = "Register `GOTGCTL` reader"]
pub struct R(crate::R<GOTGCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GOTGCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GOTGCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GOTGCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GOTGCTL` writer"]
pub struct W(crate::W<GOTGCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GOTGCTL_SPEC>;
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
impl From<crate::W<GOTGCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GOTGCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SESREQSCS` reader - Session Request Success device only"]
pub type SESREQSCS_R = crate::BitReader<bool>;
#[doc = "Field `SESREQ` reader - Session Request device only"]
pub type SESREQ_R = crate::BitReader<bool>;
#[doc = "Field `SESREQ` writer - Session Request device only"]
pub type SESREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
#[doc = "Field `VBVALIDOVEN` reader - VBUS-Valid Override Enable"]
pub type VBVALIDOVEN_R = crate::BitReader<bool>;
#[doc = "Field `VBVALIDOVEN` writer - VBUS-Valid Override Enable"]
pub type VBVALIDOVEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
#[doc = "Field `VBVALIDOVVAL` reader - VBUS Valid Override Value"]
pub type VBVALIDOVVAL_R = crate::BitReader<bool>;
#[doc = "Field `VBVALIDOVVAL` writer - VBUS Valid Override Value"]
pub type VBVALIDOVVAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
#[doc = "Field `BVALIDOVEN` reader - BValid Override Enable"]
pub type BVALIDOVEN_R = crate::BitReader<bool>;
#[doc = "Field `BVALIDOVEN` writer - BValid Override Enable"]
pub type BVALIDOVEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
#[doc = "Field `BVALIDOVVAL` reader - Bvalid Override Value"]
pub type BVALIDOVVAL_R = crate::BitReader<bool>;
#[doc = "Field `BVALIDOVVAL` writer - Bvalid Override Value"]
pub type BVALIDOVVAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
#[doc = "Field `AVALIDOVEN` reader - AValid Override Enable"]
pub type AVALIDOVEN_R = crate::BitReader<bool>;
#[doc = "Field `AVALIDOVEN` writer - AValid Override Enable"]
pub type AVALIDOVEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
#[doc = "Field `AVALIDOVVAL` reader - Avalid Override Value"]
pub type AVALIDOVVAL_R = crate::BitReader<bool>;
#[doc = "Field `AVALIDOVVAL` writer - Avalid Override Value"]
pub type AVALIDOVVAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
#[doc = "Field `HSTNEGSCS` reader - Host Negotiation Success device only"]
pub type HSTNEGSCS_R = crate::BitReader<bool>;
#[doc = "Field `HNPREQ` reader - HNP Request device only"]
pub type HNPREQ_R = crate::BitReader<bool>;
#[doc = "Field `HNPREQ` writer - HNP Request device only"]
pub type HNPREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
#[doc = "Field `HSTSETHNPEN` reader - Host Set HNP Enable host only"]
pub type HSTSETHNPEN_R = crate::BitReader<bool>;
#[doc = "Field `HSTSETHNPEN` writer - Host Set HNP Enable host only"]
pub type HSTSETHNPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
#[doc = "Field `DEVHNPEN` reader - Device HNP Enabled device only"]
pub type DEVHNPEN_R = crate::BitReader<bool>;
#[doc = "Field `DEVHNPEN` writer - Device HNP Enabled device only"]
pub type DEVHNPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
#[doc = "Field `CONIDSTS` reader - Connector ID Status host and device"]
pub type CONIDSTS_R = crate::BitReader<bool>;
#[doc = "Field `DBNCTIME` reader - Long/Short Debounce Time host only"]
pub type DBNCTIME_R = crate::BitReader<bool>;
#[doc = "Field `ASESVLD` reader - A-Session Valid host only"]
pub type ASESVLD_R = crate::BitReader<bool>;
#[doc = "Field `BSESVLD` reader - B-Session Valid device only"]
pub type BSESVLD_R = crate::BitReader<bool>;
#[doc = "Field `OTGVER` reader - OTG Version"]
pub type OTGVER_R = crate::BitReader<bool>;
#[doc = "Field `OTGVER` writer - OTG Version"]
pub type OTGVER_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Session Request Success device only"]
    #[inline(always)]
    pub fn sesreqscs(&self) -> SESREQSCS_R {
        SESREQSCS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Session Request device only"]
    #[inline(always)]
    pub fn sesreq(&self) -> SESREQ_R {
        SESREQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VBUS-Valid Override Enable"]
    #[inline(always)]
    pub fn vbvalidoven(&self) -> VBVALIDOVEN_R {
        VBVALIDOVEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VBUS Valid Override Value"]
    #[inline(always)]
    pub fn vbvalidovval(&self) -> VBVALIDOVVAL_R {
        VBVALIDOVVAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BValid Override Enable"]
    #[inline(always)]
    pub fn bvalidoven(&self) -> BVALIDOVEN_R {
        BVALIDOVEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bvalid Override Value"]
    #[inline(always)]
    pub fn bvalidovval(&self) -> BVALIDOVVAL_R {
        BVALIDOVVAL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AValid Override Enable"]
    #[inline(always)]
    pub fn avalidoven(&self) -> AVALIDOVEN_R {
        AVALIDOVEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Avalid Override Value"]
    #[inline(always)]
    pub fn avalidovval(&self) -> AVALIDOVVAL_R {
        AVALIDOVVAL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Host Negotiation Success device only"]
    #[inline(always)]
    pub fn hstnegscs(&self) -> HSTNEGSCS_R {
        HSTNEGSCS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HNP Request device only"]
    #[inline(always)]
    pub fn hnpreq(&self) -> HNPREQ_R {
        HNPREQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Host Set HNP Enable host only"]
    #[inline(always)]
    pub fn hstsethnpen(&self) -> HSTSETHNPEN_R {
        HSTSETHNPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Device HNP Enabled device only"]
    #[inline(always)]
    pub fn devhnpen(&self) -> DEVHNPEN_R {
        DEVHNPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Connector ID Status host and device"]
    #[inline(always)]
    pub fn conidsts(&self) -> CONIDSTS_R {
        CONIDSTS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Long/Short Debounce Time host only"]
    #[inline(always)]
    pub fn dbnctime(&self) -> DBNCTIME_R {
        DBNCTIME_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - A-Session Valid host only"]
    #[inline(always)]
    pub fn asesvld(&self) -> ASESVLD_R {
        ASESVLD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - B-Session Valid device only"]
    #[inline(always)]
    pub fn bsesvld(&self) -> BSESVLD_R {
        BSESVLD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - OTG Version"]
    #[inline(always)]
    pub fn otgver(&self) -> OTGVER_R {
        OTGVER_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Session Request device only"]
    #[inline(always)]
    #[must_use]
    pub fn sesreq(&mut self) -> SESREQ_W<1> {
        SESREQ_W::new(self)
    }
    #[doc = "Bit 2 - VBUS-Valid Override Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbvalidoven(&mut self) -> VBVALIDOVEN_W<2> {
        VBVALIDOVEN_W::new(self)
    }
    #[doc = "Bit 3 - VBUS Valid Override Value"]
    #[inline(always)]
    #[must_use]
    pub fn vbvalidovval(&mut self) -> VBVALIDOVVAL_W<3> {
        VBVALIDOVVAL_W::new(self)
    }
    #[doc = "Bit 4 - BValid Override Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bvalidoven(&mut self) -> BVALIDOVEN_W<4> {
        BVALIDOVEN_W::new(self)
    }
    #[doc = "Bit 5 - Bvalid Override Value"]
    #[inline(always)]
    #[must_use]
    pub fn bvalidovval(&mut self) -> BVALIDOVVAL_W<5> {
        BVALIDOVVAL_W::new(self)
    }
    #[doc = "Bit 6 - AValid Override Enable"]
    #[inline(always)]
    #[must_use]
    pub fn avalidoven(&mut self) -> AVALIDOVEN_W<6> {
        AVALIDOVEN_W::new(self)
    }
    #[doc = "Bit 7 - Avalid Override Value"]
    #[inline(always)]
    #[must_use]
    pub fn avalidovval(&mut self) -> AVALIDOVVAL_W<7> {
        AVALIDOVVAL_W::new(self)
    }
    #[doc = "Bit 9 - HNP Request device only"]
    #[inline(always)]
    #[must_use]
    pub fn hnpreq(&mut self) -> HNPREQ_W<9> {
        HNPREQ_W::new(self)
    }
    #[doc = "Bit 10 - Host Set HNP Enable host only"]
    #[inline(always)]
    #[must_use]
    pub fn hstsethnpen(&mut self) -> HSTSETHNPEN_W<10> {
        HSTSETHNPEN_W::new(self)
    }
    #[doc = "Bit 11 - Device HNP Enabled device only"]
    #[inline(always)]
    #[must_use]
    pub fn devhnpen(&mut self) -> DEVHNPEN_W<11> {
        DEVHNPEN_W::new(self)
    }
    #[doc = "Bit 20 - OTG Version"]
    #[inline(always)]
    #[must_use]
    pub fn otgver(&mut self) -> OTGVER_W<20> {
        OTGVER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gotgctl](index.html) module"]
pub struct GOTGCTL_SPEC;
impl crate::RegisterSpec for GOTGCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gotgctl::R](R) reader structure"]
impl crate::Readable for GOTGCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gotgctl::W](W) writer structure"]
impl crate::Writable for GOTGCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GOTGCTL to value 0x0001_0000"]
impl crate::Resettable for GOTGCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0000;
}
