#[doc = "Register `HC10_INT` reader"]
pub struct R(crate::R<HC10_INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HC10_INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HC10_INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HC10_INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HC10_INT` writer"]
pub struct W(crate::W<HC10_INT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HC10_INT_SPEC>;
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
impl From<crate::W<HC10_INT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HC10_INT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFERCOMPL` reader - Transfer Completed"]
pub type XFERCOMPL_R = crate::BitReader<bool>;
#[doc = "Field `XFERCOMPL` writer - Transfer Completed"]
pub type XFERCOMPL_W<'a> = crate::BitWriter<'a, u32, HC10_INT_SPEC, bool, 0>;
#[doc = "Field `CHHLTD` reader - Channel Halted"]
pub type CHHLTD_R = crate::BitReader<bool>;
#[doc = "Field `CHHLTD` writer - Channel Halted"]
pub type CHHLTD_W<'a> = crate::BitWriter<'a, u32, HC10_INT_SPEC, bool, 1>;
#[doc = "Field `AHBERR` reader - AHB Error"]
pub type AHBERR_R = crate::BitReader<bool>;
#[doc = "Field `AHBERR` writer - AHB Error"]
pub type AHBERR_W<'a> = crate::BitWriter<'a, u32, HC10_INT_SPEC, bool, 2>;
#[doc = "Field `STALL` reader - STALL Response Received Interrupt"]
pub type STALL_R = crate::BitReader<bool>;
#[doc = "Field `STALL` writer - STALL Response Received Interrupt"]
pub type STALL_W<'a> = crate::BitWriter<'a, u32, HC10_INT_SPEC, bool, 3>;
#[doc = "Field `NAK` reader - NAK Response Received Interrupt"]
pub type NAK_R = crate::BitReader<bool>;
#[doc = "Field `NAK` writer - NAK Response Received Interrupt"]
pub type NAK_W<'a> = crate::BitWriter<'a, u32, HC10_INT_SPEC, bool, 4>;
#[doc = "Field `ACK` reader - ACK Response Received/Transmitted Interrupt"]
pub type ACK_R = crate::BitReader<bool>;
#[doc = "Field `ACK` writer - ACK Response Received/Transmitted Interrupt"]
pub type ACK_W<'a> = crate::BitWriter<'a, u32, HC10_INT_SPEC, bool, 5>;
#[doc = "Field `XACTERR` reader - Transaction Error"]
pub type XACTERR_R = crate::BitReader<bool>;
#[doc = "Field `XACTERR` writer - Transaction Error"]
pub type XACTERR_W<'a> = crate::BitWriter<'a, u32, HC10_INT_SPEC, bool, 7>;
#[doc = "Field `BBLERR` reader - Babble Error"]
pub type BBLERR_R = crate::BitReader<bool>;
#[doc = "Field `BBLERR` writer - Babble Error"]
pub type BBLERR_W<'a> = crate::BitWriter<'a, u32, HC10_INT_SPEC, bool, 8>;
#[doc = "Field `FRMOVRUN` reader - Frame Overrun"]
pub type FRMOVRUN_R = crate::BitReader<bool>;
#[doc = "Field `FRMOVRUN` writer - Frame Overrun"]
pub type FRMOVRUN_W<'a> = crate::BitWriter<'a, u32, HC10_INT_SPEC, bool, 9>;
#[doc = "Field `DATATGLERR` reader - Data Toggle Error"]
pub type DATATGLERR_R = crate::BitReader<bool>;
#[doc = "Field `DATATGLERR` writer - Data Toggle Error"]
pub type DATATGLERR_W<'a> = crate::BitWriter<'a, u32, HC10_INT_SPEC, bool, 10>;
impl R {
    #[doc = "Bit 0 - Transfer Completed"]
    #[inline(always)]
    pub fn xfercompl(&self) -> XFERCOMPL_R {
        XFERCOMPL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Halted"]
    #[inline(always)]
    pub fn chhltd(&self) -> CHHLTD_R {
        CHHLTD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    pub fn ahberr(&self) -> AHBERR_R {
        AHBERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STALL Response Received Interrupt"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAK Response Received Interrupt"]
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACK Response Received/Transmitted Interrupt"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Transaction Error"]
    #[inline(always)]
    pub fn xacterr(&self) -> XACTERR_R {
        XACTERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Babble Error"]
    #[inline(always)]
    pub fn bblerr(&self) -> BBLERR_R {
        BBLERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Frame Overrun"]
    #[inline(always)]
    pub fn frmovrun(&self) -> FRMOVRUN_R {
        FRMOVRUN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data Toggle Error"]
    #[inline(always)]
    pub fn datatglerr(&self) -> DATATGLERR_R {
        DATATGLERR_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed"]
    #[inline(always)]
    pub fn xfercompl(&mut self) -> XFERCOMPL_W {
        XFERCOMPL_W::new(self)
    }
    #[doc = "Bit 1 - Channel Halted"]
    #[inline(always)]
    pub fn chhltd(&mut self) -> CHHLTD_W {
        CHHLTD_W::new(self)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    pub fn ahberr(&mut self) -> AHBERR_W {
        AHBERR_W::new(self)
    }
    #[doc = "Bit 3 - STALL Response Received Interrupt"]
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W {
        STALL_W::new(self)
    }
    #[doc = "Bit 4 - NAK Response Received Interrupt"]
    #[inline(always)]
    pub fn nak(&mut self) -> NAK_W {
        NAK_W::new(self)
    }
    #[doc = "Bit 5 - ACK Response Received/Transmitted Interrupt"]
    #[inline(always)]
    pub fn ack(&mut self) -> ACK_W {
        ACK_W::new(self)
    }
    #[doc = "Bit 7 - Transaction Error"]
    #[inline(always)]
    pub fn xacterr(&mut self) -> XACTERR_W {
        XACTERR_W::new(self)
    }
    #[doc = "Bit 8 - Babble Error"]
    #[inline(always)]
    pub fn bblerr(&mut self) -> BBLERR_W {
        BBLERR_W::new(self)
    }
    #[doc = "Bit 9 - Frame Overrun"]
    #[inline(always)]
    pub fn frmovrun(&mut self) -> FRMOVRUN_W {
        FRMOVRUN_W::new(self)
    }
    #[doc = "Bit 10 - Data Toggle Error"]
    #[inline(always)]
    pub fn datatglerr(&mut self) -> DATATGLERR_W {
        DATATGLERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Channel x Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc10_int](index.html) module"]
pub struct HC10_INT_SPEC;
impl crate::RegisterSpec for HC10_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hc10_int::R](R) reader structure"]
impl crate::Readable for HC10_INT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hc10_int::W](W) writer structure"]
impl crate::Writable for HC10_INT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HC10_INT to value 0"]
impl crate::Resettable for HC10_INT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
