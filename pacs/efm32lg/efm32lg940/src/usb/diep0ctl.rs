#[doc = "Register `DIEP0CTL` reader"]
pub struct R(crate::R<DIEP0CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEP0CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEP0CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEP0CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEP0CTL` writer"]
pub struct W(crate::W<DIEP0CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEP0CTL_SPEC>;
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
impl From<crate::W<DIEP0CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEP0CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Maximum Packet Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MPS_A {
    #[doc = "0: 64 bytes."]
    _64B = 0,
    #[doc = "1: 32 bytes."]
    _32B = 1,
    #[doc = "2: 16 bytes."]
    _16B = 2,
    #[doc = "3: 8 bytes."]
    _8B = 3,
}
impl From<MPS_A> for u8 {
    #[inline(always)]
    fn from(variant: MPS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MPS` reader - Maximum Packet Size"]
pub type MPS_R = crate::FieldReader<u8, MPS_A>;
impl MPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPS_A {
        match self.bits {
            0 => MPS_A::_64B,
            1 => MPS_A::_32B,
            2 => MPS_A::_16B,
            3 => MPS_A::_8B,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_64B`"]
    #[inline(always)]
    pub fn is_64b(&self) -> bool {
        *self == MPS_A::_64B
    }
    #[doc = "Checks if the value of the field is `_32B`"]
    #[inline(always)]
    pub fn is_32b(&self) -> bool {
        *self == MPS_A::_32B
    }
    #[doc = "Checks if the value of the field is `_16B`"]
    #[inline(always)]
    pub fn is_16b(&self) -> bool {
        *self == MPS_A::_16B
    }
    #[doc = "Checks if the value of the field is `_8B`"]
    #[inline(always)]
    pub fn is_8b(&self) -> bool {
        *self == MPS_A::_8B
    }
}
#[doc = "Field `MPS` writer - Maximum Packet Size"]
pub type MPS_W<'a> = crate::FieldWriterSafe<'a, u32, DIEP0CTL_SPEC, u8, MPS_A, 2, 0>;
impl<'a> MPS_W<'a> {
    #[doc = "64 bytes."]
    #[inline(always)]
    pub fn _64b(self) -> &'a mut W {
        self.variant(MPS_A::_64B)
    }
    #[doc = "32 bytes."]
    #[inline(always)]
    pub fn _32b(self) -> &'a mut W {
        self.variant(MPS_A::_32B)
    }
    #[doc = "16 bytes."]
    #[inline(always)]
    pub fn _16b(self) -> &'a mut W {
        self.variant(MPS_A::_16B)
    }
    #[doc = "8 bytes."]
    #[inline(always)]
    pub fn _8b(self) -> &'a mut W {
        self.variant(MPS_A::_8B)
    }
}
#[doc = "Field `USBACTEP` reader - USB Active Endpoint"]
pub type USBACTEP_R = crate::BitReader<bool>;
#[doc = "Field `NAKSTS` reader - NAK Status"]
pub type NAKSTS_R = crate::BitReader<bool>;
#[doc = "Field `EPTYPE` reader - Endpoint Type"]
pub type EPTYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STALL` reader - Handshake"]
pub type STALL_R = crate::BitReader<bool>;
#[doc = "Field `STALL` writer - Handshake"]
pub type STALL_W<'a> = crate::BitWriter<'a, u32, DIEP0CTL_SPEC, bool, 21>;
#[doc = "Field `TXFNUM` reader - TxFIFO Number"]
pub type TXFNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXFNUM` writer - TxFIFO Number"]
pub type TXFNUM_W<'a> = crate::FieldWriter<'a, u32, DIEP0CTL_SPEC, u8, u8, 4, 22>;
#[doc = "Field `CNAK` writer - Clear NAK"]
pub type CNAK_W<'a> = crate::BitWriter<'a, u32, DIEP0CTL_SPEC, bool, 26>;
#[doc = "Field `SNAK` writer - Set NAK"]
pub type SNAK_W<'a> = crate::BitWriter<'a, u32, DIEP0CTL_SPEC, bool, 27>;
#[doc = "Field `EPDIS` reader - Endpoint Disable"]
pub type EPDIS_R = crate::BitReader<bool>;
#[doc = "Field `EPDIS` writer - Endpoint Disable"]
pub type EPDIS_W<'a> = crate::BitWriter<'a, u32, DIEP0CTL_SPEC, bool, 30>;
#[doc = "Field `EPENA` reader - Endpoint Enable"]
pub type EPENA_R = crate::BitReader<bool>;
#[doc = "Field `EPENA` writer - Endpoint Enable"]
pub type EPENA_W<'a> = crate::BitWriter<'a, u32, DIEP0CTL_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:1 - Maximum Packet Size"]
    #[inline(always)]
    pub fn mps(&self) -> MPS_R {
        MPS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 15 - USB Active Endpoint"]
    #[inline(always)]
    pub fn usbactep(&self) -> USBACTEP_R {
        USBACTEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - NAK Status"]
    #[inline(always)]
    pub fn naksts(&self) -> NAKSTS_R {
        NAKSTS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&self) -> EPTYPE_R {
        EPTYPE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21 - Handshake"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:25 - TxFIFO Number"]
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Endpoint Disable"]
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Endpoint Enable"]
    #[inline(always)]
    pub fn epena(&self) -> EPENA_R {
        EPENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Maximum Packet Size"]
    #[inline(always)]
    pub fn mps(&mut self) -> MPS_W {
        MPS_W::new(self)
    }
    #[doc = "Bit 21 - Handshake"]
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W {
        STALL_W::new(self)
    }
    #[doc = "Bits 22:25 - TxFIFO Number"]
    #[inline(always)]
    pub fn txfnum(&mut self) -> TXFNUM_W {
        TXFNUM_W::new(self)
    }
    #[doc = "Bit 26 - Clear NAK"]
    #[inline(always)]
    pub fn cnak(&mut self) -> CNAK_W {
        CNAK_W::new(self)
    }
    #[doc = "Bit 27 - Set NAK"]
    #[inline(always)]
    pub fn snak(&mut self) -> SNAK_W {
        SNAK_W::new(self)
    }
    #[doc = "Bit 30 - Endpoint Disable"]
    #[inline(always)]
    pub fn epdis(&mut self) -> EPDIS_W {
        EPDIS_W::new(self)
    }
    #[doc = "Bit 31 - Endpoint Enable"]
    #[inline(always)]
    pub fn epena(&mut self) -> EPENA_W {
        EPENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device IN Endpoint 0 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep0ctl](index.html) module"]
pub struct DIEP0CTL_SPEC;
impl crate::RegisterSpec for DIEP0CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diep0ctl::R](R) reader structure"]
impl crate::Readable for DIEP0CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diep0ctl::W](W) writer structure"]
impl crate::Writable for DIEP0CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIEP0CTL to value 0x8000"]
impl crate::Resettable for DIEP0CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000
    }
}
