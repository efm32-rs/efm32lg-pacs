#[doc = "Register `GAHBCFG` reader"]
pub struct R(crate::R<GAHBCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GAHBCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GAHBCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GAHBCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GAHBCFG` writer"]
pub struct W(crate::W<GAHBCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GAHBCFG_SPEC>;
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
impl From<crate::W<GAHBCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GAHBCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GLBLINTRMSK` reader - Global Interrupt Mask host and device"]
pub type GLBLINTRMSK_R = crate::BitReader<bool>;
#[doc = "Field `GLBLINTRMSK` writer - Global Interrupt Mask host and device"]
pub type GLBLINTRMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GAHBCFG_SPEC, bool, O>;
#[doc = "Field `HBSTLEN` reader - Burst Length/Type host and device"]
pub type HBSTLEN_R = crate::FieldReader<u8, HBSTLEN_A>;
#[doc = "Burst Length/Type host and device\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HBSTLEN_A {
    #[doc = "0: Single transfer."]
    SINGLE = 0,
    #[doc = "1: Incrementing burst of unspecified length."]
    INCR = 1,
    #[doc = "3: 4-beat incrementing burst."]
    INCR4 = 3,
    #[doc = "5: 8-beat incrementing burst."]
    INCR8 = 5,
    #[doc = "7: 16-beat incrementing burst."]
    INCR16 = 7,
}
impl From<HBSTLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: HBSTLEN_A) -> Self {
        variant as _
    }
}
impl HBSTLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HBSTLEN_A> {
        match self.bits {
            0 => Some(HBSTLEN_A::SINGLE),
            1 => Some(HBSTLEN_A::INCR),
            3 => Some(HBSTLEN_A::INCR4),
            5 => Some(HBSTLEN_A::INCR8),
            7 => Some(HBSTLEN_A::INCR16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == HBSTLEN_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `INCR`"]
    #[inline(always)]
    pub fn is_incr(&self) -> bool {
        *self == HBSTLEN_A::INCR
    }
    #[doc = "Checks if the value of the field is `INCR4`"]
    #[inline(always)]
    pub fn is_incr4(&self) -> bool {
        *self == HBSTLEN_A::INCR4
    }
    #[doc = "Checks if the value of the field is `INCR8`"]
    #[inline(always)]
    pub fn is_incr8(&self) -> bool {
        *self == HBSTLEN_A::INCR8
    }
    #[doc = "Checks if the value of the field is `INCR16`"]
    #[inline(always)]
    pub fn is_incr16(&self) -> bool {
        *self == HBSTLEN_A::INCR16
    }
}
#[doc = "Field `HBSTLEN` writer - Burst Length/Type host and device"]
pub type HBSTLEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GAHBCFG_SPEC, u8, HBSTLEN_A, 4, O>;
impl<'a, const O: u8> HBSTLEN_W<'a, O> {
    #[doc = "Single transfer."]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(HBSTLEN_A::SINGLE)
    }
    #[doc = "Incrementing burst of unspecified length."]
    #[inline(always)]
    pub fn incr(self) -> &'a mut W {
        self.variant(HBSTLEN_A::INCR)
    }
    #[doc = "4-beat incrementing burst."]
    #[inline(always)]
    pub fn incr4(self) -> &'a mut W {
        self.variant(HBSTLEN_A::INCR4)
    }
    #[doc = "8-beat incrementing burst."]
    #[inline(always)]
    pub fn incr8(self) -> &'a mut W {
        self.variant(HBSTLEN_A::INCR8)
    }
    #[doc = "16-beat incrementing burst."]
    #[inline(always)]
    pub fn incr16(self) -> &'a mut W {
        self.variant(HBSTLEN_A::INCR16)
    }
}
#[doc = "Field `DMAEN` reader - DMA Enable host and device"]
pub type DMAEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAEN` writer - DMA Enable host and device"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GAHBCFG_SPEC, bool, O>;
#[doc = "Field `NPTXFEMPLVL` reader - Non-Periodic TxFIFO Empty Level host and device"]
pub type NPTXFEMPLVL_R = crate::BitReader<bool>;
#[doc = "Field `NPTXFEMPLVL` writer - Non-Periodic TxFIFO Empty Level host and device"]
pub type NPTXFEMPLVL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GAHBCFG_SPEC, bool, O>;
#[doc = "Field `PTXFEMPLVL` reader - Periodic TxFIFO Empty Level host only"]
pub type PTXFEMPLVL_R = crate::BitReader<bool>;
#[doc = "Field `PTXFEMPLVL` writer - Periodic TxFIFO Empty Level host only"]
pub type PTXFEMPLVL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GAHBCFG_SPEC, bool, O>;
#[doc = "Field `REMMEMSUPP` reader - Remote Memory Support"]
pub type REMMEMSUPP_R = crate::BitReader<bool>;
#[doc = "Field `REMMEMSUPP` writer - Remote Memory Support"]
pub type REMMEMSUPP_W<'a, const O: u8> = crate::BitWriter<'a, u32, GAHBCFG_SPEC, bool, O>;
#[doc = "Field `NOTIALLDMAWRIT` reader - Notify All DMA Writes"]
pub type NOTIALLDMAWRIT_R = crate::BitReader<bool>;
#[doc = "Field `NOTIALLDMAWRIT` writer - Notify All DMA Writes"]
pub type NOTIALLDMAWRIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GAHBCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Global Interrupt Mask host and device"]
    #[inline(always)]
    pub fn glblintrmsk(&self) -> GLBLINTRMSK_R {
        GLBLINTRMSK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Burst Length/Type host and device"]
    #[inline(always)]
    pub fn hbstlen(&self) -> HBSTLEN_R {
        HBSTLEN_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - DMA Enable host and device"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Non-Periodic TxFIFO Empty Level host and device"]
    #[inline(always)]
    pub fn nptxfemplvl(&self) -> NPTXFEMPLVL_R {
        NPTXFEMPLVL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Periodic TxFIFO Empty Level host only"]
    #[inline(always)]
    pub fn ptxfemplvl(&self) -> PTXFEMPLVL_R {
        PTXFEMPLVL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 21 - Remote Memory Support"]
    #[inline(always)]
    pub fn remmemsupp(&self) -> REMMEMSUPP_R {
        REMMEMSUPP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Notify All DMA Writes"]
    #[inline(always)]
    pub fn notialldmawrit(&self) -> NOTIALLDMAWRIT_R {
        NOTIALLDMAWRIT_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global Interrupt Mask host and device"]
    #[inline(always)]
    #[must_use]
    pub fn glblintrmsk(&mut self) -> GLBLINTRMSK_W<0> {
        GLBLINTRMSK_W::new(self)
    }
    #[doc = "Bits 1:4 - Burst Length/Type host and device"]
    #[inline(always)]
    #[must_use]
    pub fn hbstlen(&mut self) -> HBSTLEN_W<1> {
        HBSTLEN_W::new(self)
    }
    #[doc = "Bit 5 - DMA Enable host and device"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<5> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 7 - Non-Periodic TxFIFO Empty Level host and device"]
    #[inline(always)]
    #[must_use]
    pub fn nptxfemplvl(&mut self) -> NPTXFEMPLVL_W<7> {
        NPTXFEMPLVL_W::new(self)
    }
    #[doc = "Bit 8 - Periodic TxFIFO Empty Level host only"]
    #[inline(always)]
    #[must_use]
    pub fn ptxfemplvl(&mut self) -> PTXFEMPLVL_W<8> {
        PTXFEMPLVL_W::new(self)
    }
    #[doc = "Bit 21 - Remote Memory Support"]
    #[inline(always)]
    #[must_use]
    pub fn remmemsupp(&mut self) -> REMMEMSUPP_W<21> {
        REMMEMSUPP_W::new(self)
    }
    #[doc = "Bit 22 - Notify All DMA Writes"]
    #[inline(always)]
    #[must_use]
    pub fn notialldmawrit(&mut self) -> NOTIALLDMAWRIT_W<22> {
        NOTIALLDMAWRIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gahbcfg](index.html) module"]
pub struct GAHBCFG_SPEC;
impl crate::RegisterSpec for GAHBCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gahbcfg::R](R) reader structure"]
impl crate::Readable for GAHBCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gahbcfg::W](W) writer structure"]
impl crate::Writable for GAHBCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GAHBCFG to value 0"]
impl crate::Resettable for GAHBCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
