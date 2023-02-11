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
#[doc = "Field `EN` reader - Watchdog Timer Enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Watchdog Timer Enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `DEBUGRUN` reader - Debug Mode Run Enable"]
pub type DEBUGRUN_R = crate::BitReader<bool>;
#[doc = "Field `DEBUGRUN` writer - Debug Mode Run Enable"]
pub type DEBUGRUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `EM2RUN` reader - Energy Mode 2 Run Enable"]
pub type EM2RUN_R = crate::BitReader<bool>;
#[doc = "Field `EM2RUN` writer - Energy Mode 2 Run Enable"]
pub type EM2RUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `EM3RUN` reader - Energy Mode 3 Run Enable"]
pub type EM3RUN_R = crate::BitReader<bool>;
#[doc = "Field `EM3RUN` writer - Energy Mode 3 Run Enable"]
pub type EM3RUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `LOCK` reader - Configuration lock"]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` writer - Configuration lock"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `EM4BLOCK` reader - Energy Mode 4 Block"]
pub type EM4BLOCK_R = crate::BitReader<bool>;
#[doc = "Field `EM4BLOCK` writer - Energy Mode 4 Block"]
pub type EM4BLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SWOSCBLOCK` reader - Software Oscillator Disable Block"]
pub type SWOSCBLOCK_R = crate::BitReader<bool>;
#[doc = "Field `SWOSCBLOCK` writer - Software Oscillator Disable Block"]
pub type SWOSCBLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `PERSEL` reader - Watchdog Timeout Period Select"]
pub type PERSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PERSEL` writer - Watchdog Timeout Period Select"]
pub type PERSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `CLKSEL` reader - Watchdog Clock Select"]
pub type CLKSEL_R = crate::FieldReader<u8, CLKSEL_A>;
#[doc = "Watchdog Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: ULFRCO"]
    ULFRCO = 0,
    #[doc = "1: LFRCO"]
    LFRCO = 1,
    #[doc = "2: LFXO"]
    LFXO = 2,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
impl CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKSEL_A> {
        match self.bits {
            0 => Some(CLKSEL_A::ULFRCO),
            1 => Some(CLKSEL_A::LFRCO),
            2 => Some(CLKSEL_A::LFXO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == CLKSEL_A::ULFRCO
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == CLKSEL_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == CLKSEL_A::LFXO
    }
}
#[doc = "Field `CLKSEL` writer - Watchdog Clock Select"]
pub type CLKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, CLKSEL_A, 2, O>;
impl<'a, const O: u8> CLKSEL_W<'a, O> {
    #[doc = "ULFRCO"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(CLKSEL_A::ULFRCO)
    }
    #[doc = "LFRCO"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(CLKSEL_A::LFRCO)
    }
    #[doc = "LFXO"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(CLKSEL_A::LFXO)
    }
}
impl R {
    #[doc = "Bit 0 - Watchdog Timer Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&self) -> DEBUGRUN_R {
        DEBUGRUN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Energy Mode 2 Run Enable"]
    #[inline(always)]
    pub fn em2run(&self) -> EM2RUN_R {
        EM2RUN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Energy Mode 3 Run Enable"]
    #[inline(always)]
    pub fn em3run(&self) -> EM3RUN_R {
        EM3RUN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configuration lock"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Energy Mode 4 Block"]
    #[inline(always)]
    pub fn em4block(&self) -> EM4BLOCK_R {
        EM4BLOCK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Software Oscillator Disable Block"]
    #[inline(always)]
    pub fn swoscblock(&self) -> SWOSCBLOCK_R {
        SWOSCBLOCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Watchdog Timeout Period Select"]
    #[inline(always)]
    pub fn persel(&self) -> PERSEL_R {
        PERSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - Watchdog Clock Select"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Timer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Debug Mode Run Enable"]
    #[inline(always)]
    #[must_use]
    pub fn debugrun(&mut self) -> DEBUGRUN_W<1> {
        DEBUGRUN_W::new(self)
    }
    #[doc = "Bit 2 - Energy Mode 2 Run Enable"]
    #[inline(always)]
    #[must_use]
    pub fn em2run(&mut self) -> EM2RUN_W<2> {
        EM2RUN_W::new(self)
    }
    #[doc = "Bit 3 - Energy Mode 3 Run Enable"]
    #[inline(always)]
    #[must_use]
    pub fn em3run(&mut self) -> EM3RUN_W<3> {
        EM3RUN_W::new(self)
    }
    #[doc = "Bit 4 - Configuration lock"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<4> {
        LOCK_W::new(self)
    }
    #[doc = "Bit 5 - Energy Mode 4 Block"]
    #[inline(always)]
    #[must_use]
    pub fn em4block(&mut self) -> EM4BLOCK_W<5> {
        EM4BLOCK_W::new(self)
    }
    #[doc = "Bit 6 - Software Oscillator Disable Block"]
    #[inline(always)]
    #[must_use]
    pub fn swoscblock(&mut self) -> SWOSCBLOCK_W<6> {
        SWOSCBLOCK_W::new(self)
    }
    #[doc = "Bits 8:11 - Watchdog Timeout Period Select"]
    #[inline(always)]
    #[must_use]
    pub fn persel(&mut self) -> PERSEL_W<8> {
        PERSEL_W::new(self)
    }
    #[doc = "Bits 12:13 - Watchdog Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<12> {
        CLKSEL_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x0f00"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f00;
}
