#[doc = "Register `DOEP0CTL` reader"]
pub struct R(crate::R<DOEP0CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEP0CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEP0CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEP0CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEP0CTL` writer"]
pub struct W(crate::W<DOEP0CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEP0CTL_SPEC>;
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
impl From<crate::W<DOEP0CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEP0CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPS` reader - Maximum Packet Size"]
pub type MPS_R = crate::FieldReader<u8, MPS_A>;
#[doc = "Maximum Packet Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
#[doc = "Field `USBACTEP` reader - USB Active Endpoint"]
pub type USBACTEP_R = crate::BitReader<bool>;
#[doc = "Field `NAKSTS` reader - NAK Status"]
pub type NAKSTS_R = crate::BitReader<bool>;
#[doc = "Field `EPTYPE` reader - Endpoint Type"]
pub type EPTYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SNP` reader - Snoop Mode"]
pub type SNP_R = crate::BitReader<bool>;
#[doc = "Field `SNP` writer - Snoop Mode"]
pub type SNP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEP0CTL_SPEC, bool, O>;
#[doc = "Field `STALL` reader - Handshake"]
pub type STALL_R = crate::BitReader<bool>;
#[doc = "Field `STALL` writer - Handshake"]
pub type STALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEP0CTL_SPEC, bool, O>;
#[doc = "Field `CNAK` writer - Clear NAK"]
pub type CNAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEP0CTL_SPEC, bool, O>;
#[doc = "Field `SNAK` writer - Set NAK"]
pub type SNAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEP0CTL_SPEC, bool, O>;
#[doc = "Field `EPDIS` reader - Endpoint Disable"]
pub type EPDIS_R = crate::BitReader<bool>;
#[doc = "Field `EPENA` reader - Endpoint Enable"]
pub type EPENA_R = crate::BitReader<bool>;
#[doc = "Field `EPENA` writer - Endpoint Enable"]
pub type EPENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEP0CTL_SPEC, bool, O>;
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
    #[doc = "Bit 20 - Snoop Mode"]
    #[inline(always)]
    pub fn snp(&self) -> SNP_R {
        SNP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Handshake"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 1) != 0)
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
    #[doc = "Bit 20 - Snoop Mode"]
    #[inline(always)]
    #[must_use]
    pub fn snp(&mut self) -> SNP_W<20> {
        SNP_W::new(self)
    }
    #[doc = "Bit 21 - Handshake"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<21> {
        STALL_W::new(self)
    }
    #[doc = "Bit 26 - Clear NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cnak(&mut self) -> CNAK_W<26> {
        CNAK_W::new(self)
    }
    #[doc = "Bit 27 - Set NAK"]
    #[inline(always)]
    #[must_use]
    pub fn snak(&mut self) -> SNAK_W<27> {
        SNAK_W::new(self)
    }
    #[doc = "Bit 31 - Endpoint Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epena(&mut self) -> EPENA_W<31> {
        EPENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device OUT Endpoint 0 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep0ctl](index.html) module"]
pub struct DOEP0CTL_SPEC;
impl crate::RegisterSpec for DOEP0CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doep0ctl::R](R) reader structure"]
impl crate::Readable for DOEP0CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doep0ctl::W](W) writer structure"]
impl crate::Writable for DOEP0CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEP0CTL to value 0x8000"]
impl crate::Resettable for DOEP0CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
