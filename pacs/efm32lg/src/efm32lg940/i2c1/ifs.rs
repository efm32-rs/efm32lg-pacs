#[doc = "Register `IFS` writer"]
pub struct W(crate::W<IFS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFS_SPEC>;
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
impl From<crate::W<IFS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` writer - Set START Interrupt Flag"]
pub type START_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 0>;
#[doc = "Field `RSTART` writer - Set Repeated START Interrupt Flag"]
pub type RSTART_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 1>;
#[doc = "Field `ADDR` writer - Set Address Interrupt Flag"]
pub type ADDR_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 2>;
#[doc = "Field `TXC` writer - Set Transfer Completed Interrupt Flag"]
pub type TXC_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 3>;
#[doc = "Field `ACK` writer - Set Acknowledge Received Interrupt Flag"]
pub type ACK_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 6>;
#[doc = "Field `NACK` writer - Set Not Acknowledge Received Interrupt Flag"]
pub type NACK_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 7>;
#[doc = "Field `MSTOP` writer - Set MSTOP Interrupt Flag"]
pub type MSTOP_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 8>;
#[doc = "Field `ARBLOST` writer - Set Arbitration Lost Interrupt Flag"]
pub type ARBLOST_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 9>;
#[doc = "Field `BUSERR` writer - Set Bus Error Interrupt Flag"]
pub type BUSERR_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 10>;
#[doc = "Field `BUSHOLD` writer - Set Bus Held Interrupt Flag"]
pub type BUSHOLD_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 11>;
#[doc = "Field `TXOF` writer - Set Transmit Buffer Overflow Interrupt Flag"]
pub type TXOF_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 12>;
#[doc = "Field `RXUF` writer - Set Receive Buffer Underflow Interrupt Flag"]
pub type RXUF_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 13>;
#[doc = "Field `BITO` writer - Set Bus Idle Timeout Interrupt Flag"]
pub type BITO_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 14>;
#[doc = "Field `CLTO` writer - Set Clock Low Interrupt Flag"]
pub type CLTO_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 15>;
#[doc = "Field `SSTOP` writer - Set SSTOP Interrupt Flag"]
pub type SSTOP_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 16>;
impl W {
    #[doc = "Bit 0 - Set START Interrupt Flag"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W::new(self)
    }
    #[doc = "Bit 1 - Set Repeated START Interrupt Flag"]
    #[inline(always)]
    pub fn rstart(&mut self) -> RSTART_W {
        RSTART_W::new(self)
    }
    #[doc = "Bit 2 - Set Address Interrupt Flag"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W::new(self)
    }
    #[doc = "Bit 3 - Set Transfer Completed Interrupt Flag"]
    #[inline(always)]
    pub fn txc(&mut self) -> TXC_W {
        TXC_W::new(self)
    }
    #[doc = "Bit 6 - Set Acknowledge Received Interrupt Flag"]
    #[inline(always)]
    pub fn ack(&mut self) -> ACK_W {
        ACK_W::new(self)
    }
    #[doc = "Bit 7 - Set Not Acknowledge Received Interrupt Flag"]
    #[inline(always)]
    pub fn nack(&mut self) -> NACK_W {
        NACK_W::new(self)
    }
    #[doc = "Bit 8 - Set MSTOP Interrupt Flag"]
    #[inline(always)]
    pub fn mstop(&mut self) -> MSTOP_W {
        MSTOP_W::new(self)
    }
    #[doc = "Bit 9 - Set Arbitration Lost Interrupt Flag"]
    #[inline(always)]
    pub fn arblost(&mut self) -> ARBLOST_W {
        ARBLOST_W::new(self)
    }
    #[doc = "Bit 10 - Set Bus Error Interrupt Flag"]
    #[inline(always)]
    pub fn buserr(&mut self) -> BUSERR_W {
        BUSERR_W::new(self)
    }
    #[doc = "Bit 11 - Set Bus Held Interrupt Flag"]
    #[inline(always)]
    pub fn bushold(&mut self) -> BUSHOLD_W {
        BUSHOLD_W::new(self)
    }
    #[doc = "Bit 12 - Set Transmit Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn txof(&mut self) -> TXOF_W {
        TXOF_W::new(self)
    }
    #[doc = "Bit 13 - Set Receive Buffer Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxuf(&mut self) -> RXUF_W {
        RXUF_W::new(self)
    }
    #[doc = "Bit 14 - Set Bus Idle Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn bito(&mut self) -> BITO_W {
        BITO_W::new(self)
    }
    #[doc = "Bit 15 - Set Clock Low Interrupt Flag"]
    #[inline(always)]
    pub fn clto(&mut self) -> CLTO_W {
        CLTO_W::new(self)
    }
    #[doc = "Bit 16 - Set SSTOP Interrupt Flag"]
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
#[doc = "Interrupt Flag Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifs](index.html) module"]
pub struct IFS_SPEC;
impl crate::RegisterSpec for IFS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ifs::W](W) writer structure"]
impl crate::Writable for IFS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
