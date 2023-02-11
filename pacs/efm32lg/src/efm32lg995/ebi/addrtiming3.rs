#[doc = "Register `ADDRTIMING3` reader"]
pub struct R(crate::R<ADDRTIMING3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDRTIMING3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDRTIMING3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDRTIMING3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDRTIMING3` writer"]
pub struct W(crate::W<ADDRTIMING3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDRTIMING3_SPEC>;
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
impl From<crate::W<ADDRTIMING3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDRTIMING3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRSETUP` reader - Address Setup Time"]
pub type ADDRSETUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDRSETUP` writer - Address Setup Time"]
pub type ADDRSETUP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDRTIMING3_SPEC, u8, u8, 2, O>;
#[doc = "Field `ADDRHOLD` reader - Address Hold Time"]
pub type ADDRHOLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDRHOLD` writer - Address Hold Time"]
pub type ADDRHOLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDRTIMING3_SPEC, u8, u8, 2, O>;
#[doc = "Field `HALFALE` reader - Half Cycle ALE Strobe Duration Enable"]
pub type HALFALE_R = crate::BitReader<bool>;
#[doc = "Field `HALFALE` writer - Half Cycle ALE Strobe Duration Enable"]
pub type HALFALE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADDRTIMING3_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Address Setup Time"]
    #[inline(always)]
    pub fn addrsetup(&self) -> ADDRSETUP_R {
        ADDRSETUP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Address Hold Time"]
    #[inline(always)]
    pub fn addrhold(&self) -> ADDRHOLD_R {
        ADDRHOLD_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 28 - Half Cycle ALE Strobe Duration Enable"]
    #[inline(always)]
    pub fn halfale(&self) -> HALFALE_R {
        HALFALE_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Address Setup Time"]
    #[inline(always)]
    #[must_use]
    pub fn addrsetup(&mut self) -> ADDRSETUP_W<0> {
        ADDRSETUP_W::new(self)
    }
    #[doc = "Bits 8:9 - Address Hold Time"]
    #[inline(always)]
    #[must_use]
    pub fn addrhold(&mut self) -> ADDRHOLD_W<8> {
        ADDRHOLD_W::new(self)
    }
    #[doc = "Bit 28 - Half Cycle ALE Strobe Duration Enable"]
    #[inline(always)]
    #[must_use]
    pub fn halfale(&mut self) -> HALFALE_W<28> {
        HALFALE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Address Timing Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addrtiming3](index.html) module"]
pub struct ADDRTIMING3_SPEC;
impl crate::RegisterSpec for ADDRTIMING3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addrtiming3::R](R) reader structure"]
impl crate::Readable for ADDRTIMING3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addrtiming3::W](W) writer structure"]
impl crate::Writable for ADDRTIMING3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDRTIMING3 to value 0x0303"]
impl crate::Resettable for ADDRTIMING3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0303;
}
