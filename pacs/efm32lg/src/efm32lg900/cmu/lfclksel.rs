#[doc = "Register `LFCLKSEL` reader"]
pub struct R(crate::R<LFCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LFCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LFCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LFCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LFCLKSEL` writer"]
pub struct W(crate::W<LFCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LFCLKSEL_SPEC>;
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
impl From<crate::W<LFCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LFCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LFA` reader - Clock Select for LFA"]
pub type LFA_R = crate::FieldReader<u8, LFA_A>;
#[doc = "Clock Select for LFA\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LFA_A {
    #[doc = "0: LFACLK is disabled"]
    DISABLED = 0,
    #[doc = "1: LFRCO selected as LFACLK"]
    LFRCO = 1,
    #[doc = "2: LFXO selected as LFACLK"]
    LFXO = 2,
    #[doc = "3: HFCORECLKLE divided by two or four is selected as LFACLK. The division factor is determined by CMU_CTRL_HFLE and CMU_HFCORECLKDIV_HFCORECLKLEDIV."]
    HFCORECLKLEDIV2 = 3,
}
impl From<LFA_A> for u8 {
    #[inline(always)]
    fn from(variant: LFA_A) -> Self {
        variant as _
    }
}
impl LFA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFA_A {
        match self.bits {
            0 => LFA_A::DISABLED,
            1 => LFA_A::LFRCO,
            2 => LFA_A::LFXO,
            3 => LFA_A::HFCORECLKLEDIV2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LFA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == LFA_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == LFA_A::LFXO
    }
    #[doc = "Checks if the value of the field is `HFCORECLKLEDIV2`"]
    #[inline(always)]
    pub fn is_hfcoreclklediv2(&self) -> bool {
        *self == LFA_A::HFCORECLKLEDIV2
    }
}
#[doc = "Field `LFA` writer - Clock Select for LFA"]
pub type LFA_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, LFCLKSEL_SPEC, u8, LFA_A, 2, O>;
impl<'a, const O: u8> LFA_W<'a, O> {
    #[doc = "LFACLK is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LFA_A::DISABLED)
    }
    #[doc = "LFRCO selected as LFACLK"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(LFA_A::LFRCO)
    }
    #[doc = "LFXO selected as LFACLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(LFA_A::LFXO)
    }
    #[doc = "HFCORECLKLE divided by two or four is selected as LFACLK. The division factor is determined by CMU_CTRL_HFLE and CMU_HFCORECLKDIV_HFCORECLKLEDIV."]
    #[inline(always)]
    pub fn hfcoreclklediv2(self) -> &'a mut W {
        self.variant(LFA_A::HFCORECLKLEDIV2)
    }
}
#[doc = "Field `LFB` reader - Clock Select for LFB"]
pub type LFB_R = crate::FieldReader<u8, LFB_A>;
#[doc = "Clock Select for LFB\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LFB_A {
    #[doc = "0: LFBCLK is disabled"]
    DISABLED = 0,
    #[doc = "1: LFRCO selected as LFBCLK"]
    LFRCO = 1,
    #[doc = "2: LFXO selected as LFBCLK"]
    LFXO = 2,
    #[doc = "3: HFCORECLKLE divided by two or four is selected as LFACLK. The division factor is determined by CMU_CTRL_HFLE and CMU_HFCORECLKDIV_HFCORECLKLEDIV."]
    HFCORECLKLEDIV2 = 3,
}
impl From<LFB_A> for u8 {
    #[inline(always)]
    fn from(variant: LFB_A) -> Self {
        variant as _
    }
}
impl LFB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFB_A {
        match self.bits {
            0 => LFB_A::DISABLED,
            1 => LFB_A::LFRCO,
            2 => LFB_A::LFXO,
            3 => LFB_A::HFCORECLKLEDIV2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LFB_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == LFB_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == LFB_A::LFXO
    }
    #[doc = "Checks if the value of the field is `HFCORECLKLEDIV2`"]
    #[inline(always)]
    pub fn is_hfcoreclklediv2(&self) -> bool {
        *self == LFB_A::HFCORECLKLEDIV2
    }
}
#[doc = "Field `LFB` writer - Clock Select for LFB"]
pub type LFB_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, LFCLKSEL_SPEC, u8, LFB_A, 2, O>;
impl<'a, const O: u8> LFB_W<'a, O> {
    #[doc = "LFBCLK is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LFB_A::DISABLED)
    }
    #[doc = "LFRCO selected as LFBCLK"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(LFB_A::LFRCO)
    }
    #[doc = "LFXO selected as LFBCLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(LFB_A::LFXO)
    }
    #[doc = "HFCORECLKLE divided by two or four is selected as LFACLK. The division factor is determined by CMU_CTRL_HFLE and CMU_HFCORECLKDIV_HFCORECLKLEDIV."]
    #[inline(always)]
    pub fn hfcoreclklediv2(self) -> &'a mut W {
        self.variant(LFB_A::HFCORECLKLEDIV2)
    }
}
#[doc = "Field `LFAE` reader - Clock Select for LFA Extended"]
pub type LFAE_R = crate::BitReader<bool>;
#[doc = "Field `LFAE` writer - Clock Select for LFA Extended"]
pub type LFAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LFCLKSEL_SPEC, bool, O>;
#[doc = "Field `LFBE` reader - Clock Select for LFB Extended"]
pub type LFBE_R = crate::BitReader<bool>;
#[doc = "Field `LFBE` writer - Clock Select for LFB Extended"]
pub type LFBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LFCLKSEL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Clock Select for LFA"]
    #[inline(always)]
    pub fn lfa(&self) -> LFA_R {
        LFA_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Clock Select for LFB"]
    #[inline(always)]
    pub fn lfb(&self) -> LFB_R {
        LFB_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 16 - Clock Select for LFA Extended"]
    #[inline(always)]
    pub fn lfae(&self) -> LFAE_R {
        LFAE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Clock Select for LFB Extended"]
    #[inline(always)]
    pub fn lfbe(&self) -> LFBE_R {
        LFBE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock Select for LFA"]
    #[inline(always)]
    #[must_use]
    pub fn lfa(&mut self) -> LFA_W<0> {
        LFA_W::new(self)
    }
    #[doc = "Bits 2:3 - Clock Select for LFB"]
    #[inline(always)]
    #[must_use]
    pub fn lfb(&mut self) -> LFB_W<2> {
        LFB_W::new(self)
    }
    #[doc = "Bit 16 - Clock Select for LFA Extended"]
    #[inline(always)]
    #[must_use]
    pub fn lfae(&mut self) -> LFAE_W<16> {
        LFAE_W::new(self)
    }
    #[doc = "Bit 20 - Clock Select for LFB Extended"]
    #[inline(always)]
    #[must_use]
    pub fn lfbe(&mut self) -> LFBE_W<20> {
        LFBE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Frequency Clock Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfclksel](index.html) module"]
pub struct LFCLKSEL_SPEC;
impl crate::RegisterSpec for LFCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lfclksel::R](R) reader structure"]
impl crate::Readable for LFCLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lfclksel::W](W) writer structure"]
impl crate::Writable for LFCLKSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LFCLKSEL to value 0x05"]
impl crate::Resettable for LFCLKSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
