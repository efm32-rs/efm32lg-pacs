#[doc = "Register `GRSTCTL` reader"]
pub struct R(crate::R<GRSTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GRSTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GRSTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GRSTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GRSTCTL` writer"]
pub struct W(crate::W<GRSTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GRSTCTL_SPEC>;
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
impl From<crate::W<GRSTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GRSTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSFTRST` reader - Core Soft Reset host and device"]
pub type CSFTRST_R = crate::BitReader<bool>;
#[doc = "Field `CSFTRST` writer - Core Soft Reset host and device"]
pub type CSFTRST_W<'a> = crate::BitWriter<'a, u32, GRSTCTL_SPEC, bool, 0>;
#[doc = "Field `FRMCNTRRST` reader - Host Frame Counter Reset host only"]
pub type FRMCNTRRST_R = crate::BitReader<bool>;
#[doc = "Field `FRMCNTRRST` writer - Host Frame Counter Reset host only"]
pub type FRMCNTRRST_W<'a> = crate::BitWriter<'a, u32, GRSTCTL_SPEC, bool, 2>;
#[doc = "Field `RXFFLSH` reader - RxFIFO Flush host and device"]
pub type RXFFLSH_R = crate::BitReader<bool>;
#[doc = "Field `RXFFLSH` writer - RxFIFO Flush host and device"]
pub type RXFFLSH_W<'a> = crate::BitWriter<'a, u32, GRSTCTL_SPEC, bool, 4>;
#[doc = "Field `TXFFLSH` reader - TxFIFO Flush host and device"]
pub type TXFFLSH_R = crate::BitReader<bool>;
#[doc = "Field `TXFFLSH` writer - TxFIFO Flush host and device"]
pub type TXFFLSH_W<'a> = crate::BitWriter<'a, u32, GRSTCTL_SPEC, bool, 5>;
#[doc = "TxFIFO Number host and device\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXFNUM_A {
    #[doc = "0: Host mode: Non-periodic TxFIFO flush. Device: Tx FIFO 0 flush"]
    F0 = 0,
    #[doc = "1: Host mode: Periodic TxFIFO flush. Device: TXFIFO 1 flush."]
    F1 = 1,
    #[doc = "2: Device mode: TXFIFO 2 flush."]
    F2 = 2,
    #[doc = "3: Device mode: TXFIFO 3 flush."]
    F3 = 3,
    #[doc = "4: Device mode: TXFIFO 4 flush."]
    F4 = 4,
    #[doc = "5: Device mode: TXFIFO 5 flush."]
    F5 = 5,
    #[doc = "6: Device mode: TXFIFO 6 flush."]
    F6 = 6,
    #[doc = "16: Flush all the transmit FIFOs in device or host mode."]
    FALL = 16,
}
impl From<TXFNUM_A> for u8 {
    #[inline(always)]
    fn from(variant: TXFNUM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TXFNUM` reader - TxFIFO Number host and device"]
pub type TXFNUM_R = crate::FieldReader<u8, TXFNUM_A>;
impl TXFNUM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TXFNUM_A> {
        match self.bits {
            0 => Some(TXFNUM_A::F0),
            1 => Some(TXFNUM_A::F1),
            2 => Some(TXFNUM_A::F2),
            3 => Some(TXFNUM_A::F3),
            4 => Some(TXFNUM_A::F4),
            5 => Some(TXFNUM_A::F5),
            6 => Some(TXFNUM_A::F6),
            16 => Some(TXFNUM_A::FALL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `F0`"]
    #[inline(always)]
    pub fn is_f0(&self) -> bool {
        *self == TXFNUM_A::F0
    }
    #[doc = "Checks if the value of the field is `F1`"]
    #[inline(always)]
    pub fn is_f1(&self) -> bool {
        *self == TXFNUM_A::F1
    }
    #[doc = "Checks if the value of the field is `F2`"]
    #[inline(always)]
    pub fn is_f2(&self) -> bool {
        *self == TXFNUM_A::F2
    }
    #[doc = "Checks if the value of the field is `F3`"]
    #[inline(always)]
    pub fn is_f3(&self) -> bool {
        *self == TXFNUM_A::F3
    }
    #[doc = "Checks if the value of the field is `F4`"]
    #[inline(always)]
    pub fn is_f4(&self) -> bool {
        *self == TXFNUM_A::F4
    }
    #[doc = "Checks if the value of the field is `F5`"]
    #[inline(always)]
    pub fn is_f5(&self) -> bool {
        *self == TXFNUM_A::F5
    }
    #[doc = "Checks if the value of the field is `F6`"]
    #[inline(always)]
    pub fn is_f6(&self) -> bool {
        *self == TXFNUM_A::F6
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == TXFNUM_A::FALL
    }
}
#[doc = "Field `TXFNUM` writer - TxFIFO Number host and device"]
pub type TXFNUM_W<'a> = crate::FieldWriter<'a, u32, GRSTCTL_SPEC, u8, TXFNUM_A, 5, 6>;
impl<'a> TXFNUM_W<'a> {
    #[doc = "Host mode: Non-periodic TxFIFO flush. Device: Tx FIFO 0 flush"]
    #[inline(always)]
    pub fn f0(self) -> &'a mut W {
        self.variant(TXFNUM_A::F0)
    }
    #[doc = "Host mode: Periodic TxFIFO flush. Device: TXFIFO 1 flush."]
    #[inline(always)]
    pub fn f1(self) -> &'a mut W {
        self.variant(TXFNUM_A::F1)
    }
    #[doc = "Device mode: TXFIFO 2 flush."]
    #[inline(always)]
    pub fn f2(self) -> &'a mut W {
        self.variant(TXFNUM_A::F2)
    }
    #[doc = "Device mode: TXFIFO 3 flush."]
    #[inline(always)]
    pub fn f3(self) -> &'a mut W {
        self.variant(TXFNUM_A::F3)
    }
    #[doc = "Device mode: TXFIFO 4 flush."]
    #[inline(always)]
    pub fn f4(self) -> &'a mut W {
        self.variant(TXFNUM_A::F4)
    }
    #[doc = "Device mode: TXFIFO 5 flush."]
    #[inline(always)]
    pub fn f5(self) -> &'a mut W {
        self.variant(TXFNUM_A::F5)
    }
    #[doc = "Device mode: TXFIFO 6 flush."]
    #[inline(always)]
    pub fn f6(self) -> &'a mut W {
        self.variant(TXFNUM_A::F6)
    }
    #[doc = "Flush all the transmit FIFOs in device or host mode."]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(TXFNUM_A::FALL)
    }
}
#[doc = "Field `DMAREQ` reader - DMA Request Signal host and device"]
pub type DMAREQ_R = crate::BitReader<bool>;
#[doc = "Field `AHBIDLE` reader - AHB Master Idle host and device"]
pub type AHBIDLE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Core Soft Reset host and device"]
    #[inline(always)]
    pub fn csftrst(&self) -> CSFTRST_R {
        CSFTRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Host Frame Counter Reset host only"]
    #[inline(always)]
    pub fn frmcntrrst(&self) -> FRMCNTRRST_R {
        FRMCNTRRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - RxFIFO Flush host and device"]
    #[inline(always)]
    pub fn rxfflsh(&self) -> RXFFLSH_R {
        RXFFLSH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TxFIFO Flush host and device"]
    #[inline(always)]
    pub fn txfflsh(&self) -> TXFFLSH_R {
        TXFFLSH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - TxFIFO Number host and device"]
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - DMA Request Signal host and device"]
    #[inline(always)]
    pub fn dmareq(&self) -> DMAREQ_R {
        DMAREQ_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - AHB Master Idle host and device"]
    #[inline(always)]
    pub fn ahbidle(&self) -> AHBIDLE_R {
        AHBIDLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Core Soft Reset host and device"]
    #[inline(always)]
    pub fn csftrst(&mut self) -> CSFTRST_W {
        CSFTRST_W::new(self)
    }
    #[doc = "Bit 2 - Host Frame Counter Reset host only"]
    #[inline(always)]
    pub fn frmcntrrst(&mut self) -> FRMCNTRRST_W {
        FRMCNTRRST_W::new(self)
    }
    #[doc = "Bit 4 - RxFIFO Flush host and device"]
    #[inline(always)]
    pub fn rxfflsh(&mut self) -> RXFFLSH_W {
        RXFFLSH_W::new(self)
    }
    #[doc = "Bit 5 - TxFIFO Flush host and device"]
    #[inline(always)]
    pub fn txfflsh(&mut self) -> TXFFLSH_W {
        TXFFLSH_W::new(self)
    }
    #[doc = "Bits 6:10 - TxFIFO Number host and device"]
    #[inline(always)]
    pub fn txfnum(&mut self) -> TXFNUM_W {
        TXFNUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grstctl](index.html) module"]
pub struct GRSTCTL_SPEC;
impl crate::RegisterSpec for GRSTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [grstctl::R](R) reader structure"]
impl crate::Readable for GRSTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [grstctl::W](W) writer structure"]
impl crate::Writable for GRSTCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GRSTCTL to value 0x8000_0000"]
impl crate::Resettable for GRSTCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
