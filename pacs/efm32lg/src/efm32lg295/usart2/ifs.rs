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
#[doc = "Field `TXC` writer - Set TX Complete Interrupt Flag"]
pub type TXC_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 0>;
#[doc = "Field `RXFULL` writer - Set RX Buffer Full Interrupt Flag"]
pub type RXFULL_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 3>;
#[doc = "Field `RXOF` writer - Set RX Overflow Interrupt Flag"]
pub type RXOF_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 4>;
#[doc = "Field `RXUF` writer - Set RX Underflow Interrupt Flag"]
pub type RXUF_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 5>;
#[doc = "Field `TXOF` writer - Set TX Overflow Interrupt Flag"]
pub type TXOF_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 6>;
#[doc = "Field `TXUF` writer - Set TX Underflow Interrupt Flag"]
pub type TXUF_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 7>;
#[doc = "Field `PERR` writer - Set Parity Error Interrupt Flag"]
pub type PERR_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 8>;
#[doc = "Field `FERR` writer - Set Framing Error Interrupt Flag"]
pub type FERR_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 9>;
#[doc = "Field `MPAF` writer - Set Multi-Processor Address Frame Interrupt Flag"]
pub type MPAF_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 10>;
#[doc = "Field `SSM` writer - Set Slave-Select in Master mode Interrupt Flag"]
pub type SSM_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 11>;
#[doc = "Field `CCF` writer - Set Collision Check Fail Interrupt Flag"]
pub type CCF_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 12>;
impl W {
    #[doc = "Bit 0 - Set TX Complete Interrupt Flag"]
    #[inline(always)]
    pub fn txc(&mut self) -> TXC_W {
        TXC_W::new(self)
    }
    #[doc = "Bit 3 - Set RX Buffer Full Interrupt Flag"]
    #[inline(always)]
    pub fn rxfull(&mut self) -> RXFULL_W {
        RXFULL_W::new(self)
    }
    #[doc = "Bit 4 - Set RX Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxof(&mut self) -> RXOF_W {
        RXOF_W::new(self)
    }
    #[doc = "Bit 5 - Set RX Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxuf(&mut self) -> RXUF_W {
        RXUF_W::new(self)
    }
    #[doc = "Bit 6 - Set TX Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn txof(&mut self) -> TXOF_W {
        TXOF_W::new(self)
    }
    #[doc = "Bit 7 - Set TX Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn txuf(&mut self) -> TXUF_W {
        TXUF_W::new(self)
    }
    #[doc = "Bit 8 - Set Parity Error Interrupt Flag"]
    #[inline(always)]
    pub fn perr(&mut self) -> PERR_W {
        PERR_W::new(self)
    }
    #[doc = "Bit 9 - Set Framing Error Interrupt Flag"]
    #[inline(always)]
    pub fn ferr(&mut self) -> FERR_W {
        FERR_W::new(self)
    }
    #[doc = "Bit 10 - Set Multi-Processor Address Frame Interrupt Flag"]
    #[inline(always)]
    pub fn mpaf(&mut self) -> MPAF_W {
        MPAF_W::new(self)
    }
    #[doc = "Bit 11 - Set Slave-Select in Master mode Interrupt Flag"]
    #[inline(always)]
    pub fn ssm(&mut self) -> SSM_W {
        SSM_W::new(self)
    }
    #[doc = "Bit 12 - Set Collision Check Fail Interrupt Flag"]
    #[inline(always)]
    pub fn ccf(&mut self) -> CCF_W {
        CCF_W::new(self)
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
