#[doc = "Register `DTLOCK` reader"]
pub struct R(crate::R<DTLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTLOCK` writer"]
pub struct W(crate::W<DTLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTLOCK_SPEC>;
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
impl From<crate::W<DTLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCKKEY` reader - DTI Lock Key"]
pub type LOCKKEY_R = crate::FieldReader<u16, LOCKKEY_A>;
#[doc = "DTI Lock Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum LOCKKEY_A {
    #[doc = "0: `0`"]
    UNLOCKED = 0,
    #[doc = "1: `1`"]
    LOCKED = 1,
}
impl From<LOCKKEY_A> for u16 {
    #[inline(always)]
    fn from(variant: LOCKKEY_A) -> Self {
        variant as _
    }
}
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
#[doc = "Field `LOCKKEY` writer - DTI Lock Key"]
pub type LOCKKEY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DTLOCK_SPEC, u16, LOCKKEY_A, 16, O>;
impl<'a, const O: u8> LOCKKEY_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LOCKKEY_A::UNLOCKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LOCKKEY_A::LOCKED)
    }
}
impl R {
    #[doc = "Bits 0:15 - DTI Lock Key"]
    #[inline(always)]
    pub fn lockkey(&self) -> LOCKKEY_R {
        LOCKKEY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DTI Lock Key"]
    #[inline(always)]
    #[must_use]
    pub fn lockkey(&mut self) -> LOCKKEY_W<0> {
        LOCKKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DTI Configuration Lock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtlock](index.html) module"]
pub struct DTLOCK_SPEC;
impl crate::RegisterSpec for DTLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtlock::R](R) reader structure"]
impl crate::Readable for DTLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtlock::W](W) writer structure"]
impl crate::Writable for DTLOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTLOCK to value 0"]
impl crate::Resettable for DTLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
