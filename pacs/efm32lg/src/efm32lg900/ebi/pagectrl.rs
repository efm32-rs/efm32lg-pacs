#[doc = "Register `PAGECTRL` reader"]
pub struct R(crate::R<PAGECTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAGECTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAGECTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAGECTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAGECTRL` writer"]
pub struct W(crate::W<PAGECTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAGECTRL_SPEC>;
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
impl From<crate::W<PAGECTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAGECTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Page Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAGELEN_A {
    #[doc = "0: 4 members in a page."]
    MEMBER4 = 0,
    #[doc = "1: 8 members in a page."]
    MEMBER8 = 1,
    #[doc = "2: 16 members in a page."]
    MEMBER16 = 2,
    #[doc = "3: 32 members in a page."]
    MEMBER32 = 3,
}
impl From<PAGELEN_A> for u8 {
    #[inline(always)]
    fn from(variant: PAGELEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAGELEN` reader - Page Length"]
pub type PAGELEN_R = crate::FieldReader<u8, PAGELEN_A>;
impl PAGELEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAGELEN_A {
        match self.bits {
            0 => PAGELEN_A::MEMBER4,
            1 => PAGELEN_A::MEMBER8,
            2 => PAGELEN_A::MEMBER16,
            3 => PAGELEN_A::MEMBER32,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MEMBER4`"]
    #[inline(always)]
    pub fn is_member4(&self) -> bool {
        *self == PAGELEN_A::MEMBER4
    }
    #[doc = "Checks if the value of the field is `MEMBER8`"]
    #[inline(always)]
    pub fn is_member8(&self) -> bool {
        *self == PAGELEN_A::MEMBER8
    }
    #[doc = "Checks if the value of the field is `MEMBER16`"]
    #[inline(always)]
    pub fn is_member16(&self) -> bool {
        *self == PAGELEN_A::MEMBER16
    }
    #[doc = "Checks if the value of the field is `MEMBER32`"]
    #[inline(always)]
    pub fn is_member32(&self) -> bool {
        *self == PAGELEN_A::MEMBER32
    }
}
#[doc = "Field `PAGELEN` writer - Page Length"]
pub type PAGELEN_W<'a> = crate::FieldWriterSafe<'a, u32, PAGECTRL_SPEC, u8, PAGELEN_A, 2, 0>;
impl<'a> PAGELEN_W<'a> {
    #[doc = "4 members in a page."]
    #[inline(always)]
    pub fn member4(self) -> &'a mut W {
        self.variant(PAGELEN_A::MEMBER4)
    }
    #[doc = "8 members in a page."]
    #[inline(always)]
    pub fn member8(self) -> &'a mut W {
        self.variant(PAGELEN_A::MEMBER8)
    }
    #[doc = "16 members in a page."]
    #[inline(always)]
    pub fn member16(self) -> &'a mut W {
        self.variant(PAGELEN_A::MEMBER16)
    }
    #[doc = "32 members in a page."]
    #[inline(always)]
    pub fn member32(self) -> &'a mut W {
        self.variant(PAGELEN_A::MEMBER32)
    }
}
#[doc = "Field `INCHIT` reader - Intrapage hit only on incremental addresses"]
pub type INCHIT_R = crate::BitReader<bool>;
#[doc = "Field `INCHIT` writer - Intrapage hit only on incremental addresses"]
pub type INCHIT_W<'a> = crate::BitWriter<'a, u32, PAGECTRL_SPEC, bool, 4>;
#[doc = "Field `RDPA` reader - Page Read Access Time"]
pub type RDPA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDPA` writer - Page Read Access Time"]
pub type RDPA_W<'a> = crate::FieldWriter<'a, u32, PAGECTRL_SPEC, u8, u8, 3, 8>;
#[doc = "Field `KEEPOPEN` reader - Maximum Page Open Time."]
pub type KEEPOPEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KEEPOPEN` writer - Maximum Page Open Time."]
pub type KEEPOPEN_W<'a> = crate::FieldWriter<'a, u32, PAGECTRL_SPEC, u8, u8, 7, 20>;
impl R {
    #[doc = "Bits 0:1 - Page Length"]
    #[inline(always)]
    pub fn pagelen(&self) -> PAGELEN_R {
        PAGELEN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Intrapage hit only on incremental addresses"]
    #[inline(always)]
    pub fn inchit(&self) -> INCHIT_R {
        INCHIT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Page Read Access Time"]
    #[inline(always)]
    pub fn rdpa(&self) -> RDPA_R {
        RDPA_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 20:26 - Maximum Page Open Time."]
    #[inline(always)]
    pub fn keepopen(&self) -> KEEPOPEN_R {
        KEEPOPEN_R::new(((self.bits >> 20) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Page Length"]
    #[inline(always)]
    pub fn pagelen(&mut self) -> PAGELEN_W {
        PAGELEN_W::new(self)
    }
    #[doc = "Bit 4 - Intrapage hit only on incremental addresses"]
    #[inline(always)]
    pub fn inchit(&mut self) -> INCHIT_W {
        INCHIT_W::new(self)
    }
    #[doc = "Bits 8:10 - Page Read Access Time"]
    #[inline(always)]
    pub fn rdpa(&mut self) -> RDPA_W {
        RDPA_W::new(self)
    }
    #[doc = "Bits 20:26 - Maximum Page Open Time."]
    #[inline(always)]
    pub fn keepopen(&mut self) -> KEEPOPEN_W {
        KEEPOPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Page Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pagectrl](index.html) module"]
pub struct PAGECTRL_SPEC;
impl crate::RegisterSpec for PAGECTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pagectrl::R](R) reader structure"]
impl crate::Readable for PAGECTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pagectrl::W](W) writer structure"]
impl crate::Writable for PAGECTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAGECTRL to value 0x0700"]
impl crate::Resettable for PAGECTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0700
    }
}
