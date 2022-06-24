#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUTOTRI` reader - Automatic Transmitter Tristate"]
pub type AUTOTRI_R = crate::BitReader<bool>;
#[doc = "Field `AUTOTRI` writer - Automatic Transmitter Tristate"]
pub type AUTOTRI_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 0>;
#[doc = "Field `DATABITS` reader - Data-Bit Mode"]
pub type DATABITS_R = crate::BitReader<bool>;
#[doc = "Field `DATABITS` writer - Data-Bit Mode"]
pub type DATABITS_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 1>;
#[doc = "Parity-Bit Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PARITY_A {
    #[doc = "0: Parity bits are not used"]
    NONE = 0,
    #[doc = "2: Even parity are used. Parity bits are automatically generated and checked by hardware."]
    EVEN = 2,
    #[doc = "3: Odd parity is used. Parity bits are automatically generated and checked by hardware."]
    ODD = 3,
}
impl From<PARITY_A> for u8 {
    #[inline(always)]
    fn from(variant: PARITY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PARITY` reader - Parity-Bit Mode"]
pub type PARITY_R = crate::FieldReader<u8, PARITY_A>;
impl PARITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PARITY_A> {
        match self.bits {
            0 => Some(PARITY_A::NONE),
            2 => Some(PARITY_A::EVEN),
            3 => Some(PARITY_A::ODD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PARITY_A::NONE
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == PARITY_A::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == PARITY_A::ODD
    }
}
#[doc = "Field `PARITY` writer - Parity-Bit Mode"]
pub type PARITY_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, PARITY_A, 2, 2>;
impl<'a> PARITY_W<'a> {
    #[doc = "Parity bits are not used"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(PARITY_A::NONE)
    }
    #[doc = "Even parity are used. Parity bits are automatically generated and checked by hardware."]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(PARITY_A::EVEN)
    }
    #[doc = "Odd parity is used. Parity bits are automatically generated and checked by hardware."]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(PARITY_A::ODD)
    }
}
#[doc = "Field `STOPBITS` reader - Stop-Bit Mode"]
pub type STOPBITS_R = crate::BitReader<bool>;
#[doc = "Field `STOPBITS` writer - Stop-Bit Mode"]
pub type STOPBITS_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 4>;
#[doc = "Field `INV` reader - Invert Input And Output"]
pub type INV_R = crate::BitReader<bool>;
#[doc = "Field `INV` writer - Invert Input And Output"]
pub type INV_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 5>;
#[doc = "Field `ERRSDMA` reader - Clear RX DMA On Error"]
pub type ERRSDMA_R = crate::BitReader<bool>;
#[doc = "Field `ERRSDMA` writer - Clear RX DMA On Error"]
pub type ERRSDMA_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 6>;
#[doc = "Field `LOOPBK` reader - Loopback Enable"]
pub type LOOPBK_R = crate::BitReader<bool>;
#[doc = "Field `LOOPBK` writer - Loopback Enable"]
pub type LOOPBK_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 7>;
#[doc = "Field `SFUBRX` reader - Start-Frame UnBlock RX"]
pub type SFUBRX_R = crate::BitReader<bool>;
#[doc = "Field `SFUBRX` writer - Start-Frame UnBlock RX"]
pub type SFUBRX_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 8>;
#[doc = "Field `MPM` reader - Multi-Processor Mode"]
pub type MPM_R = crate::BitReader<bool>;
#[doc = "Field `MPM` writer - Multi-Processor Mode"]
pub type MPM_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 9>;
#[doc = "Field `MPAB` reader - Multi-Processor Address-Bit"]
pub type MPAB_R = crate::BitReader<bool>;
#[doc = "Field `MPAB` writer - Multi-Processor Address-Bit"]
pub type MPAB_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 10>;
#[doc = "Field `BIT8DV` reader - Bit 8 Default Value"]
pub type BIT8DV_R = crate::BitReader<bool>;
#[doc = "Field `BIT8DV` writer - Bit 8 Default Value"]
pub type BIT8DV_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 11>;
#[doc = "Field `RXDMAWU` reader - RX DMA Wakeup"]
pub type RXDMAWU_R = crate::BitReader<bool>;
#[doc = "Field `RXDMAWU` writer - RX DMA Wakeup"]
pub type RXDMAWU_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 12>;
#[doc = "Field `TXDMAWU` reader - TX DMA Wakeup"]
pub type TXDMAWU_R = crate::BitReader<bool>;
#[doc = "Field `TXDMAWU` writer - TX DMA Wakeup"]
pub type TXDMAWU_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 13>;
#[doc = "TX Delay Transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXDELAY_A {
    #[doc = "0: Frames are transmitted immediately"]
    NONE = 0,
    #[doc = "1: Transmission of new frames are delayed by a single baud period"]
    SINGLE = 1,
    #[doc = "2: Transmission of new frames are delayed by two baud periods"]
    DOUBLE = 2,
    #[doc = "3: Transmission of new frames are delayed by three baud periods"]
    TRIPLE = 3,
}
impl From<TXDELAY_A> for u8 {
    #[inline(always)]
    fn from(variant: TXDELAY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TXDELAY` reader - TX Delay Transmission"]
pub type TXDELAY_R = crate::FieldReader<u8, TXDELAY_A>;
impl TXDELAY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDELAY_A {
        match self.bits {
            0 => TXDELAY_A::NONE,
            1 => TXDELAY_A::SINGLE,
            2 => TXDELAY_A::DOUBLE,
            3 => TXDELAY_A::TRIPLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == TXDELAY_A::NONE
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == TXDELAY_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `DOUBLE`"]
    #[inline(always)]
    pub fn is_double(&self) -> bool {
        *self == TXDELAY_A::DOUBLE
    }
    #[doc = "Checks if the value of the field is `TRIPLE`"]
    #[inline(always)]
    pub fn is_triple(&self) -> bool {
        *self == TXDELAY_A::TRIPLE
    }
}
#[doc = "Field `TXDELAY` writer - TX Delay Transmission"]
pub type TXDELAY_W<'a> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, TXDELAY_A, 2, 14>;
impl<'a> TXDELAY_W<'a> {
    #[doc = "Frames are transmitted immediately"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(TXDELAY_A::NONE)
    }
    #[doc = "Transmission of new frames are delayed by a single baud period"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(TXDELAY_A::SINGLE)
    }
    #[doc = "Transmission of new frames are delayed by two baud periods"]
    #[inline(always)]
    pub fn double(self) -> &'a mut W {
        self.variant(TXDELAY_A::DOUBLE)
    }
    #[doc = "Transmission of new frames are delayed by three baud periods"]
    #[inline(always)]
    pub fn triple(self) -> &'a mut W {
        self.variant(TXDELAY_A::TRIPLE)
    }
}
impl R {
    #[doc = "Bit 0 - Automatic Transmitter Tristate"]
    #[inline(always)]
    pub fn autotri(&self) -> AUTOTRI_R {
        AUTOTRI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data-Bit Mode"]
    #[inline(always)]
    pub fn databits(&self) -> DATABITS_R {
        DATABITS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Parity-Bit Mode"]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Stop-Bit Mode"]
    #[inline(always)]
    pub fn stopbits(&self) -> STOPBITS_R {
        STOPBITS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Invert Input And Output"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Clear RX DMA On Error"]
    #[inline(always)]
    pub fn errsdma(&self) -> ERRSDMA_R {
        ERRSDMA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Loopback Enable"]
    #[inline(always)]
    pub fn loopbk(&self) -> LOOPBK_R {
        LOOPBK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Start-Frame UnBlock RX"]
    #[inline(always)]
    pub fn sfubrx(&self) -> SFUBRX_R {
        SFUBRX_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Multi-Processor Mode"]
    #[inline(always)]
    pub fn mpm(&self) -> MPM_R {
        MPM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Multi-Processor Address-Bit"]
    #[inline(always)]
    pub fn mpab(&self) -> MPAB_R {
        MPAB_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bit 8 Default Value"]
    #[inline(always)]
    pub fn bit8dv(&self) -> BIT8DV_R {
        BIT8DV_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RX DMA Wakeup"]
    #[inline(always)]
    pub fn rxdmawu(&self) -> RXDMAWU_R {
        RXDMAWU_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TX DMA Wakeup"]
    #[inline(always)]
    pub fn txdmawu(&self) -> TXDMAWU_R {
        TXDMAWU_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - TX Delay Transmission"]
    #[inline(always)]
    pub fn txdelay(&self) -> TXDELAY_R {
        TXDELAY_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Automatic Transmitter Tristate"]
    #[inline(always)]
    pub fn autotri(&mut self) -> AUTOTRI_W {
        AUTOTRI_W::new(self)
    }
    #[doc = "Bit 1 - Data-Bit Mode"]
    #[inline(always)]
    pub fn databits(&mut self) -> DATABITS_W {
        DATABITS_W::new(self)
    }
    #[doc = "Bits 2:3 - Parity-Bit Mode"]
    #[inline(always)]
    pub fn parity(&mut self) -> PARITY_W {
        PARITY_W::new(self)
    }
    #[doc = "Bit 4 - Stop-Bit Mode"]
    #[inline(always)]
    pub fn stopbits(&mut self) -> STOPBITS_W {
        STOPBITS_W::new(self)
    }
    #[doc = "Bit 5 - Invert Input And Output"]
    #[inline(always)]
    pub fn inv(&mut self) -> INV_W {
        INV_W::new(self)
    }
    #[doc = "Bit 6 - Clear RX DMA On Error"]
    #[inline(always)]
    pub fn errsdma(&mut self) -> ERRSDMA_W {
        ERRSDMA_W::new(self)
    }
    #[doc = "Bit 7 - Loopback Enable"]
    #[inline(always)]
    pub fn loopbk(&mut self) -> LOOPBK_W {
        LOOPBK_W::new(self)
    }
    #[doc = "Bit 8 - Start-Frame UnBlock RX"]
    #[inline(always)]
    pub fn sfubrx(&mut self) -> SFUBRX_W {
        SFUBRX_W::new(self)
    }
    #[doc = "Bit 9 - Multi-Processor Mode"]
    #[inline(always)]
    pub fn mpm(&mut self) -> MPM_W {
        MPM_W::new(self)
    }
    #[doc = "Bit 10 - Multi-Processor Address-Bit"]
    #[inline(always)]
    pub fn mpab(&mut self) -> MPAB_W {
        MPAB_W::new(self)
    }
    #[doc = "Bit 11 - Bit 8 Default Value"]
    #[inline(always)]
    pub fn bit8dv(&mut self) -> BIT8DV_W {
        BIT8DV_W::new(self)
    }
    #[doc = "Bit 12 - RX DMA Wakeup"]
    #[inline(always)]
    pub fn rxdmawu(&mut self) -> RXDMAWU_W {
        RXDMAWU_W::new(self)
    }
    #[doc = "Bit 13 - TX DMA Wakeup"]
    #[inline(always)]
    pub fn txdmawu(&mut self) -> TXDMAWU_W {
        TXDMAWU_W::new(self)
    }
    #[doc = "Bits 14:15 - TX Delay Transmission"]
    #[inline(always)]
    pub fn txdelay(&mut self) -> TXDELAY_W {
        TXDELAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
