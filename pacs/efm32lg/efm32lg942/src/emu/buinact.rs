#[doc = "Register `BUINACT` reader"]
pub struct R(crate::R<BUINACT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUINACT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUINACT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUINACT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUINACT` writer"]
pub struct W(crate::W<BUINACT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUINACT_SPEC>;
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
impl From<crate::W<BUINACT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUINACT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUENTHRES` reader - "]
pub type BUENTHRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUENTHRES` writer - "]
pub type BUENTHRES_W<'a> = crate::FieldWriter<'a, u32, BUINACT_SPEC, u8, u8, 3, 0>;
#[doc = "Field `BUENRANGE` reader - "]
pub type BUENRANGE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUENRANGE` writer - "]
pub type BUENRANGE_W<'a> = crate::FieldWriter<'a, u32, BUINACT_SPEC, u8, u8, 2, 3>;
#[doc = "Power connection configuration when not in Backup mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `PWRCON` reader - Power connection configuration when not in Backup mode"]
pub type PWRCON_R = crate::FieldReader<u8, PWRCON_A>;
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
#[doc = "Field `PWRCON` writer - Power connection configuration when not in Backup mode"]
pub type PWRCON_W<'a> = crate::FieldWriterSafe<'a, u32, BUINACT_SPEC, u8, PWRCON_A, 2, 5>;
impl<'a> PWRCON_W<'a> {
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
    pub fn buenthres(&self) -> BUENTHRES_R {
        BUENTHRES_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn buenrange(&self) -> BUENRANGE_R {
        BUENRANGE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Power connection configuration when not in Backup mode"]
    #[inline(always)]
    pub fn pwrcon(&self) -> PWRCON_R {
        PWRCON_R::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn buenthres(&mut self) -> BUENTHRES_W {
        BUENTHRES_W::new(self)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn buenrange(&mut self) -> BUENRANGE_W {
        BUENRANGE_W::new(self)
    }
    #[doc = "Bits 5:6 - Power connection configuration when not in Backup mode"]
    #[inline(always)]
    pub fn pwrcon(&mut self) -> PWRCON_W {
        PWRCON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup mode inactive configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buinact](index.html) module"]
pub struct BUINACT_SPEC;
impl crate::RegisterSpec for BUINACT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buinact::R](R) reader structure"]
impl crate::Readable for BUINACT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buinact::W](W) writer structure"]
impl crate::Writable for BUINACT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BUINACT to value 0x0b"]
impl crate::Resettable for BUINACT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0b
    }
}
