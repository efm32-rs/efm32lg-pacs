#[doc = "Register `HC8_CHAR` reader"]
pub struct R(crate::R<HC8_CHAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HC8_CHAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HC8_CHAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HC8_CHAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HC8_CHAR` writer"]
pub struct W(crate::W<HC8_CHAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HC8_CHAR_SPEC>;
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
impl From<crate::W<HC8_CHAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HC8_CHAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPS` reader - Maximum Packet Size"]
pub type MPS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MPS` writer - Maximum Packet Size"]
pub type MPS_W<'a> = crate::FieldWriter<'a, u32, HC8_CHAR_SPEC, u16, u16, 11, 0>;
#[doc = "Field `EPNUM` reader - Endpoint Number"]
pub type EPNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPNUM` writer - Endpoint Number"]
pub type EPNUM_W<'a> = crate::FieldWriter<'a, u32, HC8_CHAR_SPEC, u8, u8, 4, 11>;
#[doc = "Field `EPDIR` reader - Endpoint Direction"]
pub type EPDIR_R = crate::BitReader<bool>;
#[doc = "Field `EPDIR` writer - Endpoint Direction"]
pub type EPDIR_W<'a> = crate::BitWriter<'a, u32, HC8_CHAR_SPEC, bool, 15>;
#[doc = "Field `LSPDDEV` reader - Low-Speed Device"]
pub type LSPDDEV_R = crate::BitReader<bool>;
#[doc = "Field `LSPDDEV` writer - Low-Speed Device"]
pub type LSPDDEV_W<'a> = crate::BitWriter<'a, u32, HC8_CHAR_SPEC, bool, 17>;
#[doc = "Endpoint Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPTYPE_A {
    #[doc = "0: Control endpoint."]
    CONTROL = 0,
    #[doc = "1: Isochronous endpoint."]
    ISO = 1,
    #[doc = "2: Bulk endpoint."]
    BULK = 2,
    #[doc = "3: Interrupt endpoint."]
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
pub type EPTYPE_W<'a> = crate::FieldWriterSafe<'a, u32, HC8_CHAR_SPEC, u8, EPTYPE_A, 2, 18>;
impl<'a> EPTYPE_W<'a> {
    #[doc = "Control endpoint."]
    #[inline(always)]
    pub fn control(self) -> &'a mut W {
        self.variant(EPTYPE_A::CONTROL)
    }
    #[doc = "Isochronous endpoint."]
    #[inline(always)]
    pub fn iso(self) -> &'a mut W {
        self.variant(EPTYPE_A::ISO)
    }
    #[doc = "Bulk endpoint."]
    #[inline(always)]
    pub fn bulk(self) -> &'a mut W {
        self.variant(EPTYPE_A::BULK)
    }
    #[doc = "Interrupt endpoint."]
    #[inline(always)]
    pub fn int(self) -> &'a mut W {
        self.variant(EPTYPE_A::INT)
    }
}
#[doc = "Field `MC` reader - Multi Count"]
pub type MC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MC` writer - Multi Count"]
pub type MC_W<'a> = crate::FieldWriter<'a, u32, HC8_CHAR_SPEC, u8, u8, 2, 20>;
#[doc = "Field `DEVADDR` reader - Device Address"]
pub type DEVADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEVADDR` writer - Device Address"]
pub type DEVADDR_W<'a> = crate::FieldWriter<'a, u32, HC8_CHAR_SPEC, u8, u8, 7, 22>;
#[doc = "Field `ODDFRM` reader - Odd Frame"]
pub type ODDFRM_R = crate::BitReader<bool>;
#[doc = "Field `ODDFRM` writer - Odd Frame"]
pub type ODDFRM_W<'a> = crate::BitWriter<'a, u32, HC8_CHAR_SPEC, bool, 29>;
#[doc = "Field `CHDIS` reader - Channel Disable"]
pub type CHDIS_R = crate::BitReader<bool>;
#[doc = "Field `CHDIS` writer - Channel Disable"]
pub type CHDIS_W<'a> = crate::BitWriter<'a, u32, HC8_CHAR_SPEC, bool, 30>;
#[doc = "Field `CHENA` reader - Channel Enable"]
pub type CHENA_R = crate::BitReader<bool>;
#[doc = "Field `CHENA` writer - Channel Enable"]
pub type CHENA_W<'a> = crate::BitWriter<'a, u32, HC8_CHAR_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:10 - Maximum Packet Size"]
    #[inline(always)]
    pub fn mps(&self) -> MPS_R {
        MPS_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14 - Endpoint Number"]
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Endpoint Direction"]
    #[inline(always)]
    pub fn epdir(&self) -> EPDIR_R {
        EPDIR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Low-Speed Device"]
    #[inline(always)]
    pub fn lspddev(&self) -> LSPDDEV_R {
        LSPDDEV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&self) -> EPTYPE_R {
        EPTYPE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Multi Count"]
    #[inline(always)]
    pub fn mc(&self) -> MC_R {
        MC_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:28 - Device Address"]
    #[inline(always)]
    pub fn devaddr(&self) -> DEVADDR_R {
        DEVADDR_R::new(((self.bits >> 22) & 0x7f) as u8)
    }
    #[doc = "Bit 29 - Odd Frame"]
    #[inline(always)]
    pub fn oddfrm(&self) -> ODDFRM_R {
        ODDFRM_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Channel Disable"]
    #[inline(always)]
    pub fn chdis(&self) -> CHDIS_R {
        CHDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Channel Enable"]
    #[inline(always)]
    pub fn chena(&self) -> CHENA_R {
        CHENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum Packet Size"]
    #[inline(always)]
    pub fn mps(&mut self) -> MPS_W {
        MPS_W::new(self)
    }
    #[doc = "Bits 11:14 - Endpoint Number"]
    #[inline(always)]
    pub fn epnum(&mut self) -> EPNUM_W {
        EPNUM_W::new(self)
    }
    #[doc = "Bit 15 - Endpoint Direction"]
    #[inline(always)]
    pub fn epdir(&mut self) -> EPDIR_W {
        EPDIR_W::new(self)
    }
    #[doc = "Bit 17 - Low-Speed Device"]
    #[inline(always)]
    pub fn lspddev(&mut self) -> LSPDDEV_W {
        LSPDDEV_W::new(self)
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&mut self) -> EPTYPE_W {
        EPTYPE_W::new(self)
    }
    #[doc = "Bits 20:21 - Multi Count"]
    #[inline(always)]
    pub fn mc(&mut self) -> MC_W {
        MC_W::new(self)
    }
    #[doc = "Bits 22:28 - Device Address"]
    #[inline(always)]
    pub fn devaddr(&mut self) -> DEVADDR_W {
        DEVADDR_W::new(self)
    }
    #[doc = "Bit 29 - Odd Frame"]
    #[inline(always)]
    pub fn oddfrm(&mut self) -> ODDFRM_W {
        ODDFRM_W::new(self)
    }
    #[doc = "Bit 30 - Channel Disable"]
    #[inline(always)]
    pub fn chdis(&mut self) -> CHDIS_W {
        CHDIS_W::new(self)
    }
    #[doc = "Bit 31 - Channel Enable"]
    #[inline(always)]
    pub fn chena(&mut self) -> CHENA_W {
        CHENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Channel x Characteristics Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc8_char](index.html) module"]
pub struct HC8_CHAR_SPEC;
impl crate::RegisterSpec for HC8_CHAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hc8_char::R](R) reader structure"]
impl crate::Readable for HC8_CHAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hc8_char::W](W) writer structure"]
impl crate::Writable for HC8_CHAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HC8_CHAR to value 0"]
impl crate::Resettable for HC8_CHAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
