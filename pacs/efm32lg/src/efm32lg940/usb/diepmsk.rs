#[doc = "Register `DIEPMSK` reader"]
pub struct R(crate::R<DIEPMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPMSK` writer"]
pub struct W(crate::W<DIEPMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPMSK_SPEC>;
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
impl From<crate::W<DIEPMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFERCOMPLMSK` reader - Transfer Completed Interrupt Mask"]
pub type XFERCOMPLMSK_R = crate::BitReader<bool>;
#[doc = "Field `XFERCOMPLMSK` writer - Transfer Completed Interrupt Mask"]
pub type XFERCOMPLMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPMSK_SPEC, bool, O>;
#[doc = "Field `EPDISBLDMSK` reader - Endpoint Disabled Interrupt Mask"]
pub type EPDISBLDMSK_R = crate::BitReader<bool>;
#[doc = "Field `EPDISBLDMSK` writer - Endpoint Disabled Interrupt Mask"]
pub type EPDISBLDMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPMSK_SPEC, bool, O>;
#[doc = "Field `AHBERRMSK` reader - AHB Error Mask"]
pub type AHBERRMSK_R = crate::BitReader<bool>;
#[doc = "Field `AHBERRMSK` writer - AHB Error Mask"]
pub type AHBERRMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPMSK_SPEC, bool, O>;
#[doc = "Field `TIMEOUTMSK` reader - Timeout Condition Mask"]
pub type TIMEOUTMSK_R = crate::BitReader<bool>;
#[doc = "Field `TIMEOUTMSK` writer - Timeout Condition Mask"]
pub type TIMEOUTMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPMSK_SPEC, bool, O>;
#[doc = "Field `INTKNTXFEMPMSK` reader - IN Token Received When TxFIFO Empty Mask"]
pub type INTKNTXFEMPMSK_R = crate::BitReader<bool>;
#[doc = "Field `INTKNTXFEMPMSK` writer - IN Token Received When TxFIFO Empty Mask"]
pub type INTKNTXFEMPMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPMSK_SPEC, bool, O>;
#[doc = "Field `INEPNAKEFFMSK` reader - IN Endpoint NAK Effective Mask"]
pub type INEPNAKEFFMSK_R = crate::BitReader<bool>;
#[doc = "Field `INEPNAKEFFMSK` writer - IN Endpoint NAK Effective Mask"]
pub type INEPNAKEFFMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPMSK_SPEC, bool, O>;
#[doc = "Field `TXFIFOUNDRNMSK` reader - Fifo Underrun Mask"]
pub type TXFIFOUNDRNMSK_R = crate::BitReader<bool>;
#[doc = "Field `TXFIFOUNDRNMSK` writer - Fifo Underrun Mask"]
pub type TXFIFOUNDRNMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPMSK_SPEC, bool, O>;
#[doc = "Field `NAKMSK` reader - NAK interrupt Mask"]
pub type NAKMSK_R = crate::BitReader<bool>;
#[doc = "Field `NAKMSK` writer - NAK interrupt Mask"]
pub type NAKMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPMSK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transfer Completed Interrupt Mask"]
    #[inline(always)]
    pub fn xfercomplmsk(&self) -> XFERCOMPLMSK_R {
        XFERCOMPLMSK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt Mask"]
    #[inline(always)]
    pub fn epdisbldmsk(&self) -> EPDISBLDMSK_R {
        EPDISBLDMSK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB Error Mask"]
    #[inline(always)]
    pub fn ahberrmsk(&self) -> AHBERRMSK_R {
        AHBERRMSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timeout Condition Mask"]
    #[inline(always)]
    pub fn timeoutmsk(&self) -> TIMEOUTMSK_R {
        TIMEOUTMSK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IN Token Received When TxFIFO Empty Mask"]
    #[inline(always)]
    pub fn intkntxfempmsk(&self) -> INTKNTXFEMPMSK_R {
        INTKNTXFEMPMSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - IN Endpoint NAK Effective Mask"]
    #[inline(always)]
    pub fn inepnakeffmsk(&self) -> INEPNAKEFFMSK_R {
        INEPNAKEFFMSK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Fifo Underrun Mask"]
    #[inline(always)]
    pub fn txfifoundrnmsk(&self) -> TXFIFOUNDRNMSK_R {
        TXFIFOUNDRNMSK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 13 - NAK interrupt Mask"]
    #[inline(always)]
    pub fn nakmsk(&self) -> NAKMSK_R {
        NAKMSK_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn xfercomplmsk(&mut self) -> XFERCOMPLMSK_W<0> {
        XFERCOMPLMSK_W::new(self)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn epdisbldmsk(&mut self) -> EPDISBLDMSK_W<1> {
        EPDISBLDMSK_W::new(self)
    }
    #[doc = "Bit 2 - AHB Error Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ahberrmsk(&mut self) -> AHBERRMSK_W<2> {
        AHBERRMSK_W::new(self)
    }
    #[doc = "Bit 3 - Timeout Condition Mask"]
    #[inline(always)]
    #[must_use]
    pub fn timeoutmsk(&mut self) -> TIMEOUTMSK_W<3> {
        TIMEOUTMSK_W::new(self)
    }
    #[doc = "Bit 4 - IN Token Received When TxFIFO Empty Mask"]
    #[inline(always)]
    #[must_use]
    pub fn intkntxfempmsk(&mut self) -> INTKNTXFEMPMSK_W<4> {
        INTKNTXFEMPMSK_W::new(self)
    }
    #[doc = "Bit 6 - IN Endpoint NAK Effective Mask"]
    #[inline(always)]
    #[must_use]
    pub fn inepnakeffmsk(&mut self) -> INEPNAKEFFMSK_W<6> {
        INEPNAKEFFMSK_W::new(self)
    }
    #[doc = "Bit 8 - Fifo Underrun Mask"]
    #[inline(always)]
    #[must_use]
    pub fn txfifoundrnmsk(&mut self) -> TXFIFOUNDRNMSK_W<8> {
        TXFIFOUNDRNMSK_W::new(self)
    }
    #[doc = "Bit 13 - NAK interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn nakmsk(&mut self) -> NAKMSK_W<13> {
        NAKMSK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device IN Endpoint Common Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepmsk](index.html) module"]
pub struct DIEPMSK_SPEC;
impl crate::RegisterSpec for DIEPMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diepmsk::R](R) reader structure"]
impl crate::Readable for DIEPMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diepmsk::W](W) writer structure"]
impl crate::Writable for DIEPMSK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPMSK to value 0"]
impl crate::Resettable for DIEPMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
