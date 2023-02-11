#[doc = "Register `READCTRL` reader"]
pub struct R(crate::R<READCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<READCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<READCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<READCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `READCTRL` writer"]
pub struct W(crate::W<READCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<READCTRL_SPEC>;
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
impl From<crate::W<READCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<READCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - Read Mode"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "Read Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Zero wait-states inserted in fetch or read transfers."]
    WS0 = 0,
    #[doc = "1: One wait-state inserted for each fetch or read transfer. This mode is required for a core frequency above 16 MHz."]
    WS1 = 1,
    #[doc = "2: Zero wait-states inserted with the Suppressed Conditional Branch Target Prefetch (SCBTP) function enabled. SCBTP saves energy by delaying the Cortex' conditional branch target prefetches until the conditional branch instruction is in the execute stage. When the instruction reaches this stage, the evaluation of the branch condition is completed and the core does not perform a speculative prefetch of both the branch target address and the next sequential address. With the SCBTP function enabled, one instruction fetch is saved for each branch not taken, with a negligible performance penalty."]
    WS0SCBTP = 2,
    #[doc = "3: One wait-state access with SCBTP enabled."]
    WS1SCBTP = 3,
    #[doc = "4: Two wait-states inserted for each fetch or read transfer. This mode is required for a core frequency above 32 MHz."]
    WS2 = 4,
    #[doc = "5: Two wait-state access with SCBTP enabled."]
    WS2SCBTP = 5,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::WS0),
            1 => Some(MODE_A::WS1),
            2 => Some(MODE_A::WS0SCBTP),
            3 => Some(MODE_A::WS1SCBTP),
            4 => Some(MODE_A::WS2),
            5 => Some(MODE_A::WS2SCBTP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `WS0`"]
    #[inline(always)]
    pub fn is_ws0(&self) -> bool {
        *self == MODE_A::WS0
    }
    #[doc = "Checks if the value of the field is `WS1`"]
    #[inline(always)]
    pub fn is_ws1(&self) -> bool {
        *self == MODE_A::WS1
    }
    #[doc = "Checks if the value of the field is `WS0SCBTP`"]
    #[inline(always)]
    pub fn is_ws0scbtp(&self) -> bool {
        *self == MODE_A::WS0SCBTP
    }
    #[doc = "Checks if the value of the field is `WS1SCBTP`"]
    #[inline(always)]
    pub fn is_ws1scbtp(&self) -> bool {
        *self == MODE_A::WS1SCBTP
    }
    #[doc = "Checks if the value of the field is `WS2`"]
    #[inline(always)]
    pub fn is_ws2(&self) -> bool {
        *self == MODE_A::WS2
    }
    #[doc = "Checks if the value of the field is `WS2SCBTP`"]
    #[inline(always)]
    pub fn is_ws2scbtp(&self) -> bool {
        *self == MODE_A::WS2SCBTP
    }
}
#[doc = "Field `MODE` writer - Read Mode"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, READCTRL_SPEC, u8, MODE_A, 3, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Zero wait-states inserted in fetch or read transfers."]
    #[inline(always)]
    pub fn ws0(self) -> &'a mut W {
        self.variant(MODE_A::WS0)
    }
    #[doc = "One wait-state inserted for each fetch or read transfer. This mode is required for a core frequency above 16 MHz."]
    #[inline(always)]
    pub fn ws1(self) -> &'a mut W {
        self.variant(MODE_A::WS1)
    }
    #[doc = "Zero wait-states inserted with the Suppressed Conditional Branch Target Prefetch (SCBTP) function enabled. SCBTP saves energy by delaying the Cortex' conditional branch target prefetches until the conditional branch instruction is in the execute stage. When the instruction reaches this stage, the evaluation of the branch condition is completed and the core does not perform a speculative prefetch of both the branch target address and the next sequential address. With the SCBTP function enabled, one instruction fetch is saved for each branch not taken, with a negligible performance penalty."]
    #[inline(always)]
    pub fn ws0scbtp(self) -> &'a mut W {
        self.variant(MODE_A::WS0SCBTP)
    }
    #[doc = "One wait-state access with SCBTP enabled."]
    #[inline(always)]
    pub fn ws1scbtp(self) -> &'a mut W {
        self.variant(MODE_A::WS1SCBTP)
    }
    #[doc = "Two wait-states inserted for each fetch or read transfer. This mode is required for a core frequency above 32 MHz."]
    #[inline(always)]
    pub fn ws2(self) -> &'a mut W {
        self.variant(MODE_A::WS2)
    }
    #[doc = "Two wait-state access with SCBTP enabled."]
    #[inline(always)]
    pub fn ws2scbtp(self) -> &'a mut W {
        self.variant(MODE_A::WS2SCBTP)
    }
}
#[doc = "Field `IFCDIS` reader - Internal Flash Cache Disable"]
pub type IFCDIS_R = crate::BitReader<bool>;
#[doc = "Field `IFCDIS` writer - Internal Flash Cache Disable"]
pub type IFCDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, READCTRL_SPEC, bool, O>;
#[doc = "Field `AIDIS` reader - Automatic Invalidate Disable"]
pub type AIDIS_R = crate::BitReader<bool>;
#[doc = "Field `AIDIS` writer - Automatic Invalidate Disable"]
pub type AIDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, READCTRL_SPEC, bool, O>;
#[doc = "Field `ICCDIS` reader - Interrupt Context Cache Disable"]
pub type ICCDIS_R = crate::BitReader<bool>;
#[doc = "Field `ICCDIS` writer - Interrupt Context Cache Disable"]
pub type ICCDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, READCTRL_SPEC, bool, O>;
#[doc = "Field `EBICDIS` reader - External Bus Interface Cache Disable"]
pub type EBICDIS_R = crate::BitReader<bool>;
#[doc = "Field `EBICDIS` writer - External Bus Interface Cache Disable"]
pub type EBICDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, READCTRL_SPEC, bool, O>;
#[doc = "Field `RAMCEN` reader - RAM Cache Enable"]
pub type RAMCEN_R = crate::BitReader<bool>;
#[doc = "Field `RAMCEN` writer - RAM Cache Enable"]
pub type RAMCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, READCTRL_SPEC, bool, O>;
#[doc = "Field `BUSSTRATEGY` reader - Strategy for bus matrix"]
pub type BUSSTRATEGY_R = crate::FieldReader<u8, BUSSTRATEGY_A>;
#[doc = "Strategy for bus matrix\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BUSSTRATEGY_A {
    #[doc = "0: `0`"]
    CPU = 0,
    #[doc = "1: `1`"]
    DMA = 1,
    #[doc = "2: `10`"]
    DMAEM1 = 2,
    #[doc = "3: `11`"]
    NONE = 3,
}
impl From<BUSSTRATEGY_A> for u8 {
    #[inline(always)]
    fn from(variant: BUSSTRATEGY_A) -> Self {
        variant as _
    }
}
impl BUSSTRATEGY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSSTRATEGY_A {
        match self.bits {
            0 => BUSSTRATEGY_A::CPU,
            1 => BUSSTRATEGY_A::DMA,
            2 => BUSSTRATEGY_A::DMAEM1,
            3 => BUSSTRATEGY_A::NONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CPU`"]
    #[inline(always)]
    pub fn is_cpu(&self) -> bool {
        *self == BUSSTRATEGY_A::CPU
    }
    #[doc = "Checks if the value of the field is `DMA`"]
    #[inline(always)]
    pub fn is_dma(&self) -> bool {
        *self == BUSSTRATEGY_A::DMA
    }
    #[doc = "Checks if the value of the field is `DMAEM1`"]
    #[inline(always)]
    pub fn is_dmaem1(&self) -> bool {
        *self == BUSSTRATEGY_A::DMAEM1
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == BUSSTRATEGY_A::NONE
    }
}
#[doc = "Field `BUSSTRATEGY` writer - Strategy for bus matrix"]
pub type BUSSTRATEGY_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, READCTRL_SPEC, u8, BUSSTRATEGY_A, 2, O>;
impl<'a, const O: u8> BUSSTRATEGY_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn cpu(self) -> &'a mut W {
        self.variant(BUSSTRATEGY_A::CPU)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn dma(self) -> &'a mut W {
        self.variant(BUSSTRATEGY_A::DMA)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn dmaem1(self) -> &'a mut W {
        self.variant(BUSSTRATEGY_A::DMAEM1)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(BUSSTRATEGY_A::NONE)
    }
}
impl R {
    #[doc = "Bits 0:2 - Read Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Internal Flash Cache Disable"]
    #[inline(always)]
    pub fn ifcdis(&self) -> IFCDIS_R {
        IFCDIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Automatic Invalidate Disable"]
    #[inline(always)]
    pub fn aidis(&self) -> AIDIS_R {
        AIDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Context Cache Disable"]
    #[inline(always)]
    pub fn iccdis(&self) -> ICCDIS_R {
        ICCDIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - External Bus Interface Cache Disable"]
    #[inline(always)]
    pub fn ebicdis(&self) -> EBICDIS_R {
        EBICDIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RAM Cache Enable"]
    #[inline(always)]
    pub fn ramcen(&self) -> RAMCEN_R {
        RAMCEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Strategy for bus matrix"]
    #[inline(always)]
    pub fn busstrategy(&self) -> BUSSTRATEGY_R {
        BUSSTRATEGY_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Read Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bit 3 - Internal Flash Cache Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ifcdis(&mut self) -> IFCDIS_W<3> {
        IFCDIS_W::new(self)
    }
    #[doc = "Bit 4 - Automatic Invalidate Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aidis(&mut self) -> AIDIS_W<4> {
        AIDIS_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt Context Cache Disable"]
    #[inline(always)]
    #[must_use]
    pub fn iccdis(&mut self) -> ICCDIS_W<5> {
        ICCDIS_W::new(self)
    }
    #[doc = "Bit 6 - External Bus Interface Cache Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ebicdis(&mut self) -> EBICDIS_W<6> {
        EBICDIS_W::new(self)
    }
    #[doc = "Bit 7 - RAM Cache Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ramcen(&mut self) -> RAMCEN_W<7> {
        RAMCEN_W::new(self)
    }
    #[doc = "Bits 16:17 - Strategy for bus matrix"]
    #[inline(always)]
    #[must_use]
    pub fn busstrategy(&mut self) -> BUSSTRATEGY_W<16> {
        BUSSTRATEGY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Read Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [readctrl](index.html) module"]
pub struct READCTRL_SPEC;
impl crate::RegisterSpec for READCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [readctrl::R](R) reader structure"]
impl crate::Readable for READCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [readctrl::W](W) writer structure"]
impl crate::Writable for READCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets READCTRL to value 0x01"]
impl crate::Resettable for READCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
