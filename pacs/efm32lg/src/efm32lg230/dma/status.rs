#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EN` reader - DMA Enable Status"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Control Current State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STATE_A {
    #[doc = "0: Idle"]
    IDLE = 0,
    #[doc = "1: Reading channel controller data"]
    RDCHCTRLDATA = 1,
    #[doc = "2: Reading source data end pointer"]
    RDSRCENDPTR = 2,
    #[doc = "3: Reading destination data end pointer"]
    RDDSTENDPTR = 3,
    #[doc = "4: Reading source data"]
    RDSRCDATA = 4,
    #[doc = "5: Writing destination data"]
    WRDSTDATA = 5,
    #[doc = "6: Waiting for DMA request to clear"]
    WAITREQCLR = 6,
    #[doc = "7: Writing channel controller data"]
    WRCHCTRLDATA = 7,
    #[doc = "8: Stalled"]
    STALLED = 8,
    #[doc = "9: Done"]
    DONE = 9,
    #[doc = "10: Peripheral scatter-gather transition"]
    PERSCATTRANS = 10,
}
impl From<STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STATE` reader - Control Current State"]
pub type STATE_R = crate::FieldReader<u8, STATE_A>;
impl STATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STATE_A> {
        match self.bits {
            0 => Some(STATE_A::IDLE),
            1 => Some(STATE_A::RDCHCTRLDATA),
            2 => Some(STATE_A::RDSRCENDPTR),
            3 => Some(STATE_A::RDDSTENDPTR),
            4 => Some(STATE_A::RDSRCDATA),
            5 => Some(STATE_A::WRDSTDATA),
            6 => Some(STATE_A::WAITREQCLR),
            7 => Some(STATE_A::WRCHCTRLDATA),
            8 => Some(STATE_A::STALLED),
            9 => Some(STATE_A::DONE),
            10 => Some(STATE_A::PERSCATTRANS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == STATE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `RDCHCTRLDATA`"]
    #[inline(always)]
    pub fn is_rdchctrldata(&self) -> bool {
        *self == STATE_A::RDCHCTRLDATA
    }
    #[doc = "Checks if the value of the field is `RDSRCENDPTR`"]
    #[inline(always)]
    pub fn is_rdsrcendptr(&self) -> bool {
        *self == STATE_A::RDSRCENDPTR
    }
    #[doc = "Checks if the value of the field is `RDDSTENDPTR`"]
    #[inline(always)]
    pub fn is_rddstendptr(&self) -> bool {
        *self == STATE_A::RDDSTENDPTR
    }
    #[doc = "Checks if the value of the field is `RDSRCDATA`"]
    #[inline(always)]
    pub fn is_rdsrcdata(&self) -> bool {
        *self == STATE_A::RDSRCDATA
    }
    #[doc = "Checks if the value of the field is `WRDSTDATA`"]
    #[inline(always)]
    pub fn is_wrdstdata(&self) -> bool {
        *self == STATE_A::WRDSTDATA
    }
    #[doc = "Checks if the value of the field is `WAITREQCLR`"]
    #[inline(always)]
    pub fn is_waitreqclr(&self) -> bool {
        *self == STATE_A::WAITREQCLR
    }
    #[doc = "Checks if the value of the field is `WRCHCTRLDATA`"]
    #[inline(always)]
    pub fn is_wrchctrldata(&self) -> bool {
        *self == STATE_A::WRCHCTRLDATA
    }
    #[doc = "Checks if the value of the field is `STALLED`"]
    #[inline(always)]
    pub fn is_stalled(&self) -> bool {
        *self == STATE_A::STALLED
    }
    #[doc = "Checks if the value of the field is `DONE`"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == STATE_A::DONE
    }
    #[doc = "Checks if the value of the field is `PERSCATTRANS`"]
    #[inline(always)]
    pub fn is_perscattrans(&self) -> bool {
        *self == STATE_A::PERSCATTRANS
    }
}
#[doc = "Field `CHNUM` reader - Channel Number"]
pub type CHNUM_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - DMA Enable Status"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - Control Current State"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - Channel Number"]
    #[inline(always)]
    pub fn chnum(&self) -> CHNUM_R {
        CHNUM_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
#[doc = "DMA Status Registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0x100b_0000"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x100b_0000
    }
}
