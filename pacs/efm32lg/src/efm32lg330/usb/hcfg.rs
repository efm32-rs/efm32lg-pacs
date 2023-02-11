#[doc = "Register `HCFG` reader"]
pub struct R(crate::R<HCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCFG` writer"]
pub struct W(crate::W<HCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCFG_SPEC>;
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
impl From<crate::W<HCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSLSPCLKSEL` reader - FS/LS PHY Clock Select"]
pub type FSLSPCLKSEL_R = crate::FieldReader<u8, FSLSPCLKSEL_A>;
#[doc = "FS/LS PHY Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSLSPCLKSEL_A {
    #[doc = "1: Internal PHY clock is running at 48 MHz (undivided)."]
    DIV1 = 1,
    #[doc = "2: Internal PHY clock is running at 6 MHz (48 MHz divided by 8)."]
    DIV8 = 2,
}
impl From<FSLSPCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FSLSPCLKSEL_A) -> Self {
        variant as _
    }
}
impl FSLSPCLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FSLSPCLKSEL_A> {
        match self.bits {
            1 => Some(FSLSPCLKSEL_A::DIV1),
            2 => Some(FSLSPCLKSEL_A::DIV8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == FSLSPCLKSEL_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == FSLSPCLKSEL_A::DIV8
    }
}
#[doc = "Field `FSLSPCLKSEL` writer - FS/LS PHY Clock Select"]
pub type FSLSPCLKSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HCFG_SPEC, u8, FSLSPCLKSEL_A, 2, O>;
impl<'a, const O: u8> FSLSPCLKSEL_W<'a, O> {
    #[doc = "Internal PHY clock is running at 48 MHz (undivided)."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(FSLSPCLKSEL_A::DIV1)
    }
    #[doc = "Internal PHY clock is running at 6 MHz (48 MHz divided by 8)."]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(FSLSPCLKSEL_A::DIV8)
    }
}
#[doc = "Field `FSLSSUPP` reader - FS- and LS-Only Support"]
pub type FSLSSUPP_R = crate::BitReader<bool>;
#[doc = "Field `FSLSSUPP` writer - FS- and LS-Only Support"]
pub type FSLSSUPP_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCFG_SPEC, bool, O>;
#[doc = "Field `ENA32KHZS` reader - Enable 32 KHz Suspend mode"]
pub type ENA32KHZS_R = crate::BitReader<bool>;
#[doc = "Field `ENA32KHZS` writer - Enable 32 KHz Suspend mode"]
pub type ENA32KHZS_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCFG_SPEC, bool, O>;
#[doc = "Field `RESVALID` reader - Resume Validation Period"]
pub type RESVALID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESVALID` writer - Resume Validation Period"]
pub type RESVALID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCFG_SPEC, u8, u8, 8, O>;
#[doc = "Field `MODECHTIMEN` reader - Mode Change Time"]
pub type MODECHTIMEN_R = crate::BitReader<bool>;
#[doc = "Field `MODECHTIMEN` writer - Mode Change Time"]
pub type MODECHTIMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - FS/LS PHY Clock Select"]
    #[inline(always)]
    pub fn fslspclksel(&self) -> FSLSPCLKSEL_R {
        FSLSPCLKSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - FS- and LS-Only Support"]
    #[inline(always)]
    pub fn fslssupp(&self) -> FSLSSUPP_R {
        FSLSSUPP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable 32 KHz Suspend mode"]
    #[inline(always)]
    pub fn ena32khzs(&self) -> ENA32KHZS_R {
        ENA32KHZS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Resume Validation Period"]
    #[inline(always)]
    pub fn resvalid(&self) -> RESVALID_R {
        RESVALID_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Mode Change Time"]
    #[inline(always)]
    pub fn modechtimen(&self) -> MODECHTIMEN_R {
        MODECHTIMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FS/LS PHY Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn fslspclksel(&mut self) -> FSLSPCLKSEL_W<0> {
        FSLSPCLKSEL_W::new(self)
    }
    #[doc = "Bit 2 - FS- and LS-Only Support"]
    #[inline(always)]
    #[must_use]
    pub fn fslssupp(&mut self) -> FSLSSUPP_W<2> {
        FSLSSUPP_W::new(self)
    }
    #[doc = "Bit 7 - Enable 32 KHz Suspend mode"]
    #[inline(always)]
    #[must_use]
    pub fn ena32khzs(&mut self) -> ENA32KHZS_W<7> {
        ENA32KHZS_W::new(self)
    }
    #[doc = "Bits 8:15 - Resume Validation Period"]
    #[inline(always)]
    #[must_use]
    pub fn resvalid(&mut self) -> RESVALID_W<8> {
        RESVALID_W::new(self)
    }
    #[doc = "Bit 31 - Mode Change Time"]
    #[inline(always)]
    #[must_use]
    pub fn modechtimen(&mut self) -> MODECHTIMEN_W<31> {
        MODECHTIMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcfg](index.html) module"]
pub struct HCFG_SPEC;
impl crate::RegisterSpec for HCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcfg::R](R) reader structure"]
impl crate::Readable for HCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcfg::W](W) writer structure"]
impl crate::Writable for HCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCFG to value 0x0020_0000"]
impl crate::Resettable for HCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0020_0000;
}
