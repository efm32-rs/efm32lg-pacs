#[doc = "Register `ETMAUTHSTATUS` reader"]
pub struct R(crate::R<ETMAUTHSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETMAUTHSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETMAUTHSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETMAUTHSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NONSECINVDBG` reader - Non-secure invasive Debug Status"]
pub type NONSECINVDBG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NONSECNONINVDBG` reader - Non-secure non-invasive Debug Status"]
pub type NONSECNONINVDBG_R = crate::FieldReader<u8, NONSECNONINVDBG_A>;
#[doc = "Non-secure non-invasive Debug Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NONSECNONINVDBG_A {
    #[doc = "2: Non-secure non-invasive debug disable"]
    DISABLE = 2,
    #[doc = "3: Non-secure non-invasive debug enable"]
    ENABLE = 3,
}
impl From<NONSECNONINVDBG_A> for u8 {
    #[inline(always)]
    fn from(variant: NONSECNONINVDBG_A) -> Self {
        variant as _
    }
}
impl NONSECNONINVDBG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NONSECNONINVDBG_A> {
        match self.bits {
            2 => Some(NONSECNONINVDBG_A::DISABLE),
            3 => Some(NONSECNONINVDBG_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == NONSECNONINVDBG_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == NONSECNONINVDBG_A::ENABLE
    }
}
#[doc = "Field `SECINVDBG` reader - Secure invasive Debug Status"]
pub type SECINVDBG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECNONINVDBG` reader - Secure non-invasive Debug Status"]
pub type SECNONINVDBG_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - Non-secure invasive Debug Status"]
    #[inline(always)]
    pub fn nonsecinvdbg(&self) -> NONSECINVDBG_R {
        NONSECINVDBG_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Non-secure non-invasive Debug Status"]
    #[inline(always)]
    pub fn nonsecnoninvdbg(&self) -> NONSECNONINVDBG_R {
        NONSECNONINVDBG_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Secure invasive Debug Status"]
    #[inline(always)]
    pub fn secinvdbg(&self) -> SECINVDBG_R {
        SECINVDBG_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Secure non-invasive Debug Status"]
    #[inline(always)]
    pub fn secnoninvdbg(&self) -> SECNONINVDBG_R {
        SECNONINVDBG_R::new(((self.bits >> 6) & 3) as u8)
    }
}
#[doc = "ETM Authentication Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmauthstatus](index.html) module"]
pub struct ETMAUTHSTATUS_SPEC;
impl crate::RegisterSpec for ETMAUTHSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etmauthstatus::R](R) reader structure"]
impl crate::Readable for ETMAUTHSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETMAUTHSTATUS to value 0xc0"]
impl crate::Resettable for ETMAUTHSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0xc0;
}
