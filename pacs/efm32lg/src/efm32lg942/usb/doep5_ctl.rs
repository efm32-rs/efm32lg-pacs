#[doc = "Register `DOEP5_CTL` reader"]
pub struct R(crate::R<DOEP5_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEP5_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEP5_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEP5_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEP5_CTL` writer"]
pub struct W(crate::W<DOEP5_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEP5_CTL_SPEC>;
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
impl From<crate::W<DOEP5_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEP5_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPS` reader - Maximum Packet Size"]
pub type MPS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MPS` writer - Maximum Packet Size"]
pub type MPS_W<'a> = crate::FieldWriter<'a, u32, DOEP5_CTL_SPEC, u16, u16, 11, 0>;
#[doc = "Field `USBACTEP` reader - USB Active Endpoint"]
pub type USBACTEP_R = crate::BitReader<bool>;
#[doc = "Field `USBACTEP` writer - USB Active Endpoint"]
pub type USBACTEP_W<'a> = crate::BitWriter<'a, u32, DOEP5_CTL_SPEC, bool, 15>;
#[doc = "Field `DPIDEOF` reader - Endpoint Data PID / Even-odd Frame"]
pub type DPIDEOF_R = crate::BitReader<bool>;
#[doc = "Field `NAKSTS` reader - NAK Status"]
pub type NAKSTS_R = crate::BitReader<bool>;
#[doc = "Endpoint Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPTYPE_A {
    #[doc = "0: Control Endpoint."]
    CONTROL = 0,
    #[doc = "1: Isochronous Endpoint."]
    ISO = 1,
    #[doc = "2: Bulk Endpoint."]
    BULK = 2,
    #[doc = "3: Interrupt Endpoint."]
    INT = 3,
}
impl From<EPTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: EPTYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPTYPE` reader - Endpoint Type"]
pub type EPTYPE_R = crate::FieldReader<u8, EPTYPE_A>;
impl EPTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPTYPE_A {
        match self.bits {
            0 => EPTYPE_A::CONTROL,
            1 => EPTYPE_A::ISO,
            2 => EPTYPE_A::BULK,
            3 => EPTYPE_A::INT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONTROL`"]
    #[inline(always)]
    pub fn is_control(&self) -> bool {
        *self == EPTYPE_A::CONTROL
    }
    #[doc = "Checks if the value of the field is `ISO`"]
    #[inline(always)]
    pub fn is_iso(&self) -> bool {
        *self == EPTYPE_A::ISO
    }
    #[doc = "Checks if the value of the field is `BULK`"]
    #[inline(always)]
    pub fn is_bulk(&self) -> bool {
        *self == EPTYPE_A::BULK
    }
    #[doc = "Checks if the value of the field is `INT`"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == EPTYPE_A::INT
    }
}
#[doc = "Field `EPTYPE` writer - Endpoint Type"]
pub type EPTYPE_W<'a> = crate::FieldWriterSafe<'a, u32, DOEP5_CTL_SPEC, u8, EPTYPE_A, 2, 18>;
impl<'a> EPTYPE_W<'a> {
    #[doc = "Control Endpoint."]
    #[inline(always)]
    pub fn control(self) -> &'a mut W {
        self.variant(EPTYPE_A::CONTROL)
    }
    #[doc = "Isochronous Endpoint."]
    #[inline(always)]
    pub fn iso(self) -> &'a mut W {
        self.variant(EPTYPE_A::ISO)
    }
    #[doc = "Bulk Endpoint."]
    #[inline(always)]
    pub fn bulk(self) -> &'a mut W {
        self.variant(EPTYPE_A::BULK)
    }
    #[doc = "Interrupt Endpoint."]
    #[inline(always)]
    pub fn int(self) -> &'a mut W {
        self.variant(EPTYPE_A::INT)
    }
}
#[doc = "Field `SNP` reader - Snoop Mode"]
pub type SNP_R = crate::BitReader<bool>;
#[doc = "Field `SNP` writer - Snoop Mode"]
pub type SNP_W<'a> = crate::BitWriter<'a, u32, DOEP5_CTL_SPEC, bool, 20>;
#[doc = "Field `STALL` reader - STALL Handshake"]
pub type STALL_R = crate::BitReader<bool>;
#[doc = "Field `STALL` writer - STALL Handshake"]
pub type STALL_W<'a> = crate::BitWriter<'a, u32, DOEP5_CTL_SPEC, bool, 21>;
#[doc = "Field `CNAK` writer - Clear NAK"]
pub type CNAK_W<'a> = crate::BitWriter<'a, u32, DOEP5_CTL_SPEC, bool, 26>;
#[doc = "Field `SNAK` writer - Set NAK"]
pub type SNAK_W<'a> = crate::BitWriter<'a, u32, DOEP5_CTL_SPEC, bool, 27>;
#[doc = "Field `SETD0PIDEF` writer - Set DATA0 PID / Even Frame"]
pub type SETD0PIDEF_W<'a> = crate::BitWriter<'a, u32, DOEP5_CTL_SPEC, bool, 28>;
#[doc = "Field `SETD1PIDOF` writer - Set DATA1 PID / Odd Frame"]
pub type SETD1PIDOF_W<'a> = crate::BitWriter<'a, u32, DOEP5_CTL_SPEC, bool, 29>;
#[doc = "Field `EPDIS` reader - Endpoint Disable"]
pub type EPDIS_R = crate::BitReader<bool>;
#[doc = "Field `EPDIS` writer - Endpoint Disable"]
pub type EPDIS_W<'a> = crate::BitWriter<'a, u32, DOEP5_CTL_SPEC, bool, 30>;
#[doc = "Field `EPENA` reader - Endpoint Enable"]
pub type EPENA_R = crate::BitReader<bool>;
#[doc = "Field `EPENA` writer - Endpoint Enable"]
pub type EPENA_W<'a> = crate::BitWriter<'a, u32, DOEP5_CTL_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:10 - Maximum Packet Size"]
    #[inline(always)]
    pub fn mps(&self) -> MPS_R {
        MPS_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - USB Active Endpoint"]
    #[inline(always)]
    pub fn usbactep(&self) -> USBACTEP_R {
        USBACTEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Endpoint Data PID / Even-odd Frame"]
    #[inline(always)]
    pub fn dpideof(&self) -> DPIDEOF_R {
        DPIDEOF_R::new(((self.bits >> 16) & 1) != 0)
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
    #[doc = "Bit 21 - STALL Handshake"]
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
    #[doc = "Bits 0:10 - Maximum Packet Size"]
    #[inline(always)]
    pub fn mps(&mut self) -> MPS_W {
        MPS_W::new(self)
    }
    #[doc = "Bit 15 - USB Active Endpoint"]
    #[inline(always)]
    pub fn usbactep(&mut self) -> USBACTEP_W {
        USBACTEP_W::new(self)
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&mut self) -> EPTYPE_W {
        EPTYPE_W::new(self)
    }
    #[doc = "Bit 20 - Snoop Mode"]
    #[inline(always)]
    pub fn snp(&mut self) -> SNP_W {
        SNP_W::new(self)
    }
    #[doc = "Bit 21 - STALL Handshake"]
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W {
        STALL_W::new(self)
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
    #[doc = "Bit 28 - Set DATA0 PID / Even Frame"]
    #[inline(always)]
    pub fn setd0pidef(&mut self) -> SETD0PIDEF_W {
        SETD0PIDEF_W::new(self)
    }
    #[doc = "Bit 29 - Set DATA1 PID / Odd Frame"]
    #[inline(always)]
    pub fn setd1pidof(&mut self) -> SETD1PIDOF_W {
        SETD1PIDOF_W::new(self)
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
#[doc = "Device OUT Endpoint x+1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep5_ctl](index.html) module"]
pub struct DOEP5_CTL_SPEC;
impl crate::RegisterSpec for DOEP5_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doep5_ctl::R](R) reader structure"]
impl crate::Readable for DOEP5_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doep5_ctl::W](W) writer structure"]
impl crate::Writable for DOEP5_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOEP5_CTL to value 0"]
impl crate::Resettable for DOEP5_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
