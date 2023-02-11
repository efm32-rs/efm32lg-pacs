#[doc = "Register `BUACT` reader"]
pub struct R(crate::R<BUACT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUACT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUACT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUACT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUACT` writer"]
pub struct W(crate::W<BUACT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUACT_SPEC>;
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
impl From<crate::W<BUACT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUACT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUEXTHRES` reader - "]
pub type BUEXTHRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUEXTHRES` writer - "]
pub type BUEXTHRES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUACT_SPEC, u8, u8, 3, O>;
#[doc = "Field `BUEXRANGE` reader - "]
pub type BUEXRANGE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUEXRANGE` writer - "]
pub type BUEXRANGE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUACT_SPEC, u8, u8, 2, O>;
#[doc = "Field `PWRCON` reader - Power connection configuration when in Backup mode"]
pub type PWRCON_R = crate::FieldReader<u8, PWRCON_A>;
#[doc = "Power connection configuration when in Backup mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWRCON_A {
    #[doc = "0: No connection."]
    NONE = 0,
    #[doc = "1: Main power and backup power are connected through a diode, allowing current to flow from backup power source to main power source, but not the other way."]
    BUMAIN = 1,
    #[doc = "2: Main power and backup power are connected through a diode, allowing current to flow from main power source to backup power source, but not the other way."]
    MAINBU = 2,
    #[doc = "3: Main power and backup power are connected without diode."]
    NODIODE = 3,
}
impl From<PWRCON_A> for u8 {
    #[inline(always)]
    fn from(variant: PWRCON_A) -> Self {
        variant as _
    }
}
impl PWRCON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRCON_A {
        match self.bits {
            0 => PWRCON_A::NONE,
            1 => PWRCON_A::BUMAIN,
            2 => PWRCON_A::MAINBU,
            3 => PWRCON_A::NODIODE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PWRCON_A::NONE
    }
    #[doc = "Checks if the value of the field is `BUMAIN`"]
    #[inline(always)]
    pub fn is_bumain(&self) -> bool {
        *self == PWRCON_A::BUMAIN
    }
    #[doc = "Checks if the value of the field is `MAINBU`"]
    #[inline(always)]
    pub fn is_mainbu(&self) -> bool {
        *self == PWRCON_A::MAINBU
    }
    #[doc = "Checks if the value of the field is `NODIODE`"]
    #[inline(always)]
    pub fn is_nodiode(&self) -> bool {
        *self == PWRCON_A::NODIODE
    }
}
#[doc = "Field `PWRCON` writer - Power connection configuration when in Backup mode"]
pub type PWRCON_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, BUACT_SPEC, u8, PWRCON_A, 2, O>;
impl<'a, const O: u8> PWRCON_W<'a, O> {
    #[doc = "No connection."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(PWRCON_A::NONE)
    }
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from backup power source to main power source, but not the other way."]
    #[inline(always)]
    pub fn bumain(self) -> &'a mut W {
        self.variant(PWRCON_A::BUMAIN)
    }
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from main power source to backup power source, but not the other way."]
    #[inline(always)]
    pub fn mainbu(self) -> &'a mut W {
        self.variant(PWRCON_A::MAINBU)
    }
    #[doc = "Main power and backup power are connected without diode."]
    #[inline(always)]
    pub fn nodiode(self) -> &'a mut W {
        self.variant(PWRCON_A::NODIODE)
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn buexthres(&self) -> BUEXTHRES_R {
        BUEXTHRES_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn buexrange(&self) -> BUEXRANGE_R {
        BUEXRANGE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Power connection configuration when in Backup mode"]
    #[inline(always)]
    pub fn pwrcon(&self) -> PWRCON_R {
        PWRCON_R::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn buexthres(&mut self) -> BUEXTHRES_W<0> {
        BUEXTHRES_W::new(self)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    #[must_use]
    pub fn buexrange(&mut self) -> BUEXRANGE_W<3> {
        BUEXRANGE_W::new(self)
    }
    #[doc = "Bits 5:6 - Power connection configuration when in Backup mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwrcon(&mut self) -> PWRCON_W<5> {
        PWRCON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup mode active configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buact](index.html) module"]
pub struct BUACT_SPEC;
impl crate::RegisterSpec for BUACT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buact::R](R) reader structure"]
impl crate::Readable for BUACT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buact::W](W) writer structure"]
impl crate::Writable for BUACT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUACT to value 0x0b"]
impl crate::Resettable for BUACT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0b;
}
