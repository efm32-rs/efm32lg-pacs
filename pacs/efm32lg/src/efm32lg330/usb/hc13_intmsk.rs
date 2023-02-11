#[doc = "Register `HC13_INTMSK` reader"]
pub struct R(crate::R<HC13_INTMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HC13_INTMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HC13_INTMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HC13_INTMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HC13_INTMSK` writer"]
pub struct W(crate::W<HC13_INTMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HC13_INTMSK_SPEC>;
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
impl From<crate::W<HC13_INTMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HC13_INTMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFERCOMPLMSK` reader - Transfer Completed Mask"]
pub type XFERCOMPLMSK_R = crate::BitReader<bool>;
#[doc = "Field `XFERCOMPLMSK` writer - Transfer Completed Mask"]
pub type XFERCOMPLMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, HC13_INTMSK_SPEC, bool, O>;
#[doc = "Field `CHHLTDMSK` reader - Channel Halted Mask"]
pub type CHHLTDMSK_R = crate::BitReader<bool>;
#[doc = "Field `CHHLTDMSK` writer - Channel Halted Mask"]
pub type CHHLTDMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, HC13_INTMSK_SPEC, bool, O>;
#[doc = "Field `AHBERRMSK` reader - AHB Error Mask"]
pub type AHBERRMSK_R = crate::BitReader<bool>;
#[doc = "Field `AHBERRMSK` writer - AHB Error Mask"]
pub type AHBERRMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, HC13_INTMSK_SPEC, bool, O>;
#[doc = "Field `STALLMSK` reader - STALL Response Received Interrupt Mask"]
pub type STALLMSK_R = crate::BitReader<bool>;
#[doc = "Field `STALLMSK` writer - STALL Response Received Interrupt Mask"]
pub type STALLMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, HC13_INTMSK_SPEC, bool, O>;
#[doc = "Field `NAKMSK` reader - NAK Response Received Interrupt Mask"]
pub type NAKMSK_R = crate::BitReader<bool>;
#[doc = "Field `NAKMSK` writer - NAK Response Received Interrupt Mask"]
pub type NAKMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, HC13_INTMSK_SPEC, bool, O>;
#[doc = "Field `ACKMSK` reader - ACK Response Received/Transmitted Interrupt Mask"]
pub type ACKMSK_R = crate::BitReader<bool>;
#[doc = "Field `ACKMSK` writer - ACK Response Received/Transmitted Interrupt Mask"]
pub type ACKMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, HC13_INTMSK_SPEC, bool, O>;
#[doc = "Field `XACTERRMSK` reader - Transaction Error Mask"]
pub type XACTERRMSK_R = crate::BitReader<bool>;
#[doc = "Field `XACTERRMSK` writer - Transaction Error Mask"]
pub type XACTERRMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, HC13_INTMSK_SPEC, bool, O>;
#[doc = "Field `BBLERRMSK` reader - Babble Error Mask"]
pub type BBLERRMSK_R = crate::BitReader<bool>;
#[doc = "Field `BBLERRMSK` writer - Babble Error Mask"]
pub type BBLERRMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, HC13_INTMSK_SPEC, bool, O>;
#[doc = "Field `FRMOVRUNMSK` reader - Frame Overrun Mask"]
pub type FRMOVRUNMSK_R = crate::BitReader<bool>;
#[doc = "Field `FRMOVRUNMSK` writer - Frame Overrun Mask"]
pub type FRMOVRUNMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, HC13_INTMSK_SPEC, bool, O>;
#[doc = "Field `DATATGLERRMSK` reader - Data Toggle Error Mask"]
pub type DATATGLERRMSK_R = crate::BitReader<bool>;
#[doc = "Field `DATATGLERRMSK` writer - Data Toggle Error Mask"]
pub type DATATGLERRMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, HC13_INTMSK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transfer Completed Mask"]
    #[inline(always)]
    pub fn xfercomplmsk(&self) -> XFERCOMPLMSK_R {
        XFERCOMPLMSK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Halted Mask"]
    #[inline(always)]
    pub fn chhltdmsk(&self) -> CHHLTDMSK_R {
        CHHLTDMSK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB Error Mask"]
    #[inline(always)]
    pub fn ahberrmsk(&self) -> AHBERRMSK_R {
        AHBERRMSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STALL Response Received Interrupt Mask"]
    #[inline(always)]
    pub fn stallmsk(&self) -> STALLMSK_R {
        STALLMSK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAK Response Received Interrupt Mask"]
    #[inline(always)]
    pub fn nakmsk(&self) -> NAKMSK_R {
        NAKMSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACK Response Received/Transmitted Interrupt Mask"]
    #[inline(always)]
    pub fn ackmsk(&self) -> ACKMSK_R {
        ACKMSK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Transaction Error Mask"]
    #[inline(always)]
    pub fn xacterrmsk(&self) -> XACTERRMSK_R {
        XACTERRMSK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Babble Error Mask"]
    #[inline(always)]
    pub fn bblerrmsk(&self) -> BBLERRMSK_R {
        BBLERRMSK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Frame Overrun Mask"]
    #[inline(always)]
    pub fn frmovrunmsk(&self) -> FRMOVRUNMSK_R {
        FRMOVRUNMSK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data Toggle Error Mask"]
    #[inline(always)]
    pub fn datatglerrmsk(&self) -> DATATGLERRMSK_R {
        DATATGLERRMSK_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed Mask"]
    #[inline(always)]
    #[must_use]
    pub fn xfercomplmsk(&mut self) -> XFERCOMPLMSK_W<0> {
        XFERCOMPLMSK_W::new(self)
    }
    #[doc = "Bit 1 - Channel Halted Mask"]
    #[inline(always)]
    #[must_use]
    pub fn chhltdmsk(&mut self) -> CHHLTDMSK_W<1> {
        CHHLTDMSK_W::new(self)
    }
    #[doc = "Bit 2 - AHB Error Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ahberrmsk(&mut self) -> AHBERRMSK_W<2> {
        AHBERRMSK_W::new(self)
    }
    #[doc = "Bit 3 - STALL Response Received Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn stallmsk(&mut self) -> STALLMSK_W<3> {
        STALLMSK_W::new(self)
    }
    #[doc = "Bit 4 - NAK Response Received Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn nakmsk(&mut self) -> NAKMSK_W<4> {
        NAKMSK_W::new(self)
    }
    #[doc = "Bit 5 - ACK Response Received/Transmitted Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ackmsk(&mut self) -> ACKMSK_W<5> {
        ACKMSK_W::new(self)
    }
    #[doc = "Bit 7 - Transaction Error Mask"]
    #[inline(always)]
    #[must_use]
    pub fn xacterrmsk(&mut self) -> XACTERRMSK_W<7> {
        XACTERRMSK_W::new(self)
    }
    #[doc = "Bit 8 - Babble Error Mask"]
    #[inline(always)]
    #[must_use]
    pub fn bblerrmsk(&mut self) -> BBLERRMSK_W<8> {
        BBLERRMSK_W::new(self)
    }
    #[doc = "Bit 9 - Frame Overrun Mask"]
    #[inline(always)]
    #[must_use]
    pub fn frmovrunmsk(&mut self) -> FRMOVRUNMSK_W<9> {
        FRMOVRUNMSK_W::new(self)
    }
    #[doc = "Bit 10 - Data Toggle Error Mask"]
    #[inline(always)]
    #[must_use]
    pub fn datatglerrmsk(&mut self) -> DATATGLERRMSK_W<10> {
        DATATGLERRMSK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Channel x Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc13_intmsk](index.html) module"]
pub struct HC13_INTMSK_SPEC;
impl crate::RegisterSpec for HC13_INTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hc13_intmsk::R](R) reader structure"]
impl crate::Readable for HC13_INTMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hc13_intmsk::W](W) writer structure"]
impl crate::Writable for HC13_INTMSK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HC13_INTMSK to value 0"]
impl crate::Resettable for HC13_INTMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
