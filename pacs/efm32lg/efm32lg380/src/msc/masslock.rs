#[doc = "Register `MASSLOCK` reader"]
pub struct R(crate::R<MASSLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MASSLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MASSLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MASSLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MASSLOCK` writer"]
pub struct W(crate::W<MASSLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASSLOCK_SPEC>;
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
impl From<crate::W<MASSLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MASSLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Mass Erase Lock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum LOCKKEY_A {
    #[doc = "0: Mass erase unlocked."]
    UNLOCKED = 0,
    #[doc = "1: Mass erase locked."]
    LOCKED = 1,
}
impl From<LOCKKEY_A> for u16 {
    #[inline(always)]
    fn from(variant: LOCKKEY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LOCKKEY` reader - Mass Erase Lock"]
pub type LOCKKEY_R = crate::FieldReader<u16, LOCKKEY_A>;
impl LOCKKEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOCKKEY_A> {
        match self.bits {
            0 => Some(LOCKKEY_A::UNLOCKED),
            1 => Some(LOCKKEY_A::LOCKED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCKKEY_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCKKEY_A::LOCKED
    }
}
#[doc = "Field `LOCKKEY` writer - Mass Erase Lock"]
pub type LOCKKEY_W<'a> = crate::FieldWriter<'a, u32, MASSLOCK_SPEC, u16, LOCKKEY_A, 16, 0>;
impl<'a> LOCKKEY_W<'a> {
    #[doc = "Mass erase unlocked."]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LOCKKEY_A::UNLOCKED)
    }
    #[doc = "Mass erase locked."]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LOCKKEY_A::LOCKED)
    }
}
impl R {
    #[doc = "Bits 0:15 - Mass Erase Lock"]
    #[inline(always)]
    pub fn lockkey(&self) -> LOCKKEY_R {
        LOCKKEY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Mass Erase Lock"]
    #[inline(always)]
    pub fn lockkey(&mut self) -> LOCKKEY_W {
        LOCKKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mass Erase Lock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [masslock](index.html) module"]
pub struct MASSLOCK_SPEC;
impl crate::RegisterSpec for MASSLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [masslock::R](R) reader structure"]
impl crate::Readable for MASSLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [masslock::W](W) writer structure"]
impl crate::Writable for MASSLOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MASSLOCK to value 0x01"]
impl crate::Resettable for MASSLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
