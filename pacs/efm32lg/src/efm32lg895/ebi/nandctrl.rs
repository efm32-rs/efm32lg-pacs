#[doc = "Register `NANDCTRL` reader"]
pub struct R(crate::R<NANDCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NANDCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NANDCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NANDCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NANDCTRL` writer"]
pub struct W(crate::W<NANDCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NANDCTRL_SPEC>;
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
impl From<crate::W<NANDCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NANDCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - NAND Flash control enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - NAND Flash control enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, NANDCTRL_SPEC, bool, O>;
#[doc = "Field `BANKSEL` reader - NAND Flash Bank"]
pub type BANKSEL_R = crate::FieldReader<u8, BANKSEL_A>;
#[doc = "NAND Flash Bank\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BANKSEL_A {
    #[doc = "0: Memory bank 0 is connected to a NAND Flash device."]
    BANK0 = 0,
    #[doc = "1: Memory bank 1 is connected to a NAND Flash device."]
    BANK1 = 1,
    #[doc = "2: Memory bank 2 is connected to a NAND Flash device."]
    BANK2 = 2,
    #[doc = "3: Memory bank 3 is connected to a NAND Flash device."]
    BANK3 = 3,
}
impl From<BANKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: BANKSEL_A) -> Self {
        variant as _
    }
}
impl BANKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BANKSEL_A {
        match self.bits {
            0 => BANKSEL_A::BANK0,
            1 => BANKSEL_A::BANK1,
            2 => BANKSEL_A::BANK2,
            3 => BANKSEL_A::BANK3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BANK0`"]
    #[inline(always)]
    pub fn is_bank0(&self) -> bool {
        *self == BANKSEL_A::BANK0
    }
    #[doc = "Checks if the value of the field is `BANK1`"]
    #[inline(always)]
    pub fn is_bank1(&self) -> bool {
        *self == BANKSEL_A::BANK1
    }
    #[doc = "Checks if the value of the field is `BANK2`"]
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        *self == BANKSEL_A::BANK2
    }
    #[doc = "Checks if the value of the field is `BANK3`"]
    #[inline(always)]
    pub fn is_bank3(&self) -> bool {
        *self == BANKSEL_A::BANK3
    }
}
#[doc = "Field `BANKSEL` writer - NAND Flash Bank"]
pub type BANKSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, NANDCTRL_SPEC, u8, BANKSEL_A, 2, O>;
impl<'a, const O: u8> BANKSEL_W<'a, O> {
    #[doc = "Memory bank 0 is connected to a NAND Flash device."]
    #[inline(always)]
    pub fn bank0(self) -> &'a mut W {
        self.variant(BANKSEL_A::BANK0)
    }
    #[doc = "Memory bank 1 is connected to a NAND Flash device."]
    #[inline(always)]
    pub fn bank1(self) -> &'a mut W {
        self.variant(BANKSEL_A::BANK1)
    }
    #[doc = "Memory bank 2 is connected to a NAND Flash device."]
    #[inline(always)]
    pub fn bank2(self) -> &'a mut W {
        self.variant(BANKSEL_A::BANK2)
    }
    #[doc = "Memory bank 3 is connected to a NAND Flash device."]
    #[inline(always)]
    pub fn bank3(self) -> &'a mut W {
        self.variant(BANKSEL_A::BANK3)
    }
}
impl R {
    #[doc = "Bit 0 - NAND Flash control enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - NAND Flash Bank"]
    #[inline(always)]
    pub fn banksel(&self) -> BANKSEL_R {
        BANKSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - NAND Flash control enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bits 4:5 - NAND Flash Bank"]
    #[inline(always)]
    #[must_use]
    pub fn banksel(&mut self) -> BANKSEL_W<4> {
        BANKSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NAND Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nandctrl](index.html) module"]
pub struct NANDCTRL_SPEC;
impl crate::RegisterSpec for NANDCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nandctrl::R](R) reader structure"]
impl crate::Readable for NANDCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nandctrl::W](W) writer structure"]
impl crate::Writable for NANDCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NANDCTRL to value 0"]
impl crate::Resettable for NANDCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
