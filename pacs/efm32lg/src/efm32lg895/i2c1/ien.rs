#[doc = "Register `IEN` reader"]
pub struct R(crate::R<IEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IEN` writer"]
pub struct W(crate::W<IEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IEN_SPEC>;
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
impl From<crate::W<IEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - START Condition Interrupt Enable"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - START Condition Interrupt Enable"]
pub type START_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 0>;
#[doc = "Field `RSTART` reader - Repeated START condition Interrupt Enable"]
pub type RSTART_R = crate::BitReader<bool>;
#[doc = "Field `RSTART` writer - Repeated START condition Interrupt Enable"]
pub type RSTART_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 1>;
#[doc = "Field `ADDR` reader - Address Interrupt Enable"]
pub type ADDR_R = crate::BitReader<bool>;
#[doc = "Field `ADDR` writer - Address Interrupt Enable"]
pub type ADDR_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 2>;
#[doc = "Field `TXC` reader - Transfer Completed Interrupt Enable"]
pub type TXC_R = crate::BitReader<bool>;
#[doc = "Field `TXC` writer - Transfer Completed Interrupt Enable"]
pub type TXC_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 3>;
#[doc = "Field `TXBL` reader - Transmit Buffer level Interrupt Enable"]
pub type TXBL_R = crate::BitReader<bool>;
#[doc = "Field `TXBL` writer - Transmit Buffer level Interrupt Enable"]
pub type TXBL_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 4>;
#[doc = "Field `RXDATAV` reader - Receive Data Valid Interrupt Enable"]
pub type RXDATAV_R = crate::BitReader<bool>;
#[doc = "Field `RXDATAV` writer - Receive Data Valid Interrupt Enable"]
pub type RXDATAV_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 5>;
#[doc = "Field `ACK` reader - Acknowledge Received Interrupt Enable"]
pub type ACK_R = crate::BitReader<bool>;
#[doc = "Field `ACK` writer - Acknowledge Received Interrupt Enable"]
pub type ACK_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 6>;
#[doc = "Field `NACK` reader - Not Acknowledge Received Interrupt Enable"]
pub type NACK_R = crate::BitReader<bool>;
#[doc = "Field `NACK` writer - Not Acknowledge Received Interrupt Enable"]
pub type NACK_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 7>;
#[doc = "Field `MSTOP` reader - MSTOP Interrupt Enable"]
pub type MSTOP_R = crate::BitReader<bool>;
#[doc = "Field `MSTOP` writer - MSTOP Interrupt Enable"]
pub type MSTOP_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 8>;
#[doc = "Field `ARBLOST` reader - Arbitration Lost Interrupt Enable"]
pub type ARBLOST_R = crate::BitReader<bool>;
#[doc = "Field `ARBLOST` writer - Arbitration Lost Interrupt Enable"]
pub type ARBLOST_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 9>;
#[doc = "Field `BUSERR` reader - Bus Error Interrupt Enable"]
pub type BUSERR_R = crate::BitReader<bool>;
#[doc = "Field `BUSERR` writer - Bus Error Interrupt Enable"]
pub type BUSERR_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 10>;
#[doc = "Field `BUSHOLD` reader - Bus Held Interrupt Enable"]
pub type BUSHOLD_R = crate::BitReader<bool>;
#[doc = "Field `BUSHOLD` writer - Bus Held Interrupt Enable"]
pub type BUSHOLD_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 11>;
#[doc = "Field `TXOF` reader - Transmit Buffer Overflow Interrupt Enable"]
pub type TXOF_R = crate::BitReader<bool>;
#[doc = "Field `TXOF` writer - Transmit Buffer Overflow Interrupt Enable"]
pub type TXOF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 12>;
#[doc = "Field `RXUF` reader - Receive Buffer Underflow Interrupt Enable"]
pub type RXUF_R = crate::BitReader<bool>;
#[doc = "Field `RXUF` writer - Receive Buffer Underflow Interrupt Enable"]
pub type RXUF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 13>;
#[doc = "Field `BITO` reader - Bus Idle Timeout Interrupt Enable"]
pub type BITO_R = crate::BitReader<bool>;
#[doc = "Field `BITO` writer - Bus Idle Timeout Interrupt Enable"]
pub type BITO_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 14>;
#[doc = "Field `CLTO` reader - Clock Low Interrupt Enable"]
pub type CLTO_R = crate::BitReader<bool>;
#[doc = "Field `CLTO` writer - Clock Low Interrupt Enable"]
pub type CLTO_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 15>;
#[doc = "Field `SSTOP` reader - SSTOP Interrupt Enable"]
pub type SSTOP_R = crate::BitReader<bool>;
#[doc = "Field `SSTOP` writer - SSTOP Interrupt Enable"]
pub type SSTOP_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 16>;
impl R {
    #[doc = "Bit 0 - START Condition Interrupt Enable"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Repeated START condition Interrupt Enable"]
    #[inline(always)]
    pub fn rstart(&self) -> RSTART_R {
        RSTART_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Address Interrupt Enable"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transfer Completed Interrupt Enable"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit Buffer level Interrupt Enable"]
    #[inline(always)]
    pub fn txbl(&self) -> TXBL_R {
        TXBL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive Data Valid Interrupt Enable"]
    #[inline(always)]
    pub fn rxdatav(&self) -> RXDATAV_R {
        RXDATAV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Acknowledge Received Interrupt Enable"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Not Acknowledge Received Interrupt Enable"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MSTOP Interrupt Enable"]
    #[inline(always)]
    pub fn mstop(&self) -> MSTOP_R {
        MSTOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Arbitration Lost Interrupt Enable"]
    #[inline(always)]
    pub fn arblost(&self) -> ARBLOST_R {
        ARBLOST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bus Error Interrupt Enable"]
    #[inline(always)]
    pub fn buserr(&self) -> BUSERR_R {
        BUSERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bus Held Interrupt Enable"]
    #[inline(always)]
    pub fn bushold(&self) -> BUSHOLD_R {
        BUSHOLD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit Buffer Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn txof(&self) -> TXOF_R {
        TXOF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Receive Buffer Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn rxuf(&self) -> RXUF_R {
        RXUF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Bus Idle Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn bito(&self) -> BITO_R {
        BITO_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Clock Low Interrupt Enable"]
    #[inline(always)]
    pub fn clto(&self) -> CLTO_R {
        CLTO_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SSTOP Interrupt Enable"]
    #[inline(always)]
    pub fn sstop(&self) -> SSTOP_R {
        SSTOP_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - START Condition Interrupt Enable"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W::new(self)
    }
    #[doc = "Bit 1 - Repeated START condition Interrupt Enable"]
    #[inline(always)]
    pub fn rstart(&mut self) -> RSTART_W {
        RSTART_W::new(self)
    }
    #[doc = "Bit 2 - Address Interrupt Enable"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W::new(self)
    }
    #[doc = "Bit 3 - Transfer Completed Interrupt Enable"]
    #[inline(always)]
    pub fn txc(&mut self) -> TXC_W {
        TXC_W::new(self)
    }
    #[doc = "Bit 4 - Transmit Buffer level Interrupt Enable"]
    #[inline(always)]
    pub fn txbl(&mut self) -> TXBL_W {
        TXBL_W::new(self)
    }
    #[doc = "Bit 5 - Receive Data Valid Interrupt Enable"]
    #[inline(always)]
    pub fn rxdatav(&mut self) -> RXDATAV_W {
        RXDATAV_W::new(self)
    }
    #[doc = "Bit 6 - Acknowledge Received Interrupt Enable"]
    #[inline(always)]
    pub fn ack(&mut self) -> ACK_W {
        ACK_W::new(self)
    }
    #[doc = "Bit 7 - Not Acknowledge Received Interrupt Enable"]
    #[inline(always)]
    pub fn nack(&mut self) -> NACK_W {
        NACK_W::new(self)
    }
    #[doc = "Bit 8 - MSTOP Interrupt Enable"]
    #[inline(always)]
    pub fn mstop(&mut self) -> MSTOP_W {
        MSTOP_W::new(self)
    }
    #[doc = "Bit 9 - Arbitration Lost Interrupt Enable"]
    #[inline(always)]
    pub fn arblost(&mut self) -> ARBLOST_W {
        ARBLOST_W::new(self)
    }
    #[doc = "Bit 10 - Bus Error Interrupt Enable"]
    #[inline(always)]
    pub fn buserr(&mut self) -> BUSERR_W {
        BUSERR_W::new(self)
    }
    #[doc = "Bit 11 - Bus Held Interrupt Enable"]
    #[inline(always)]
    pub fn bushold(&mut self) -> BUSHOLD_W {
        BUSHOLD_W::new(self)
    }
    #[doc = "Bit 12 - Transmit Buffer Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn txof(&mut self) -> TXOF_W {
        TXOF_W::new(self)
    }
    #[doc = "Bit 13 - Receive Buffer Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn rxuf(&mut self) -> RXUF_W {
        RXUF_W::new(self)
    }
    #[doc = "Bit 14 - Bus Idle Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn bito(&mut self) -> BITO_W {
        BITO_W::new(self)
    }
    #[doc = "Bit 15 - Clock Low Interrupt Enable"]
    #[inline(always)]
    pub fn clto(&mut self) -> CLTO_W {
        CLTO_W::new(self)
    }
    #[doc = "Bit 16 - SSTOP Interrupt Enable"]
    #[inline(always)]
    pub fn sstop(&mut self) -> SSTOP_W {
        SSTOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](index.html) module"]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ien::R](R) reader structure"]
impl crate::Readable for IEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ien::W](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
