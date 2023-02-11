#[doc = "Register `ETMTECR1` reader"]
pub struct R(crate::R<ETMTECR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETMTECR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETMTECR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETMTECR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETMTECR1` writer"]
pub struct W(crate::W<ETMTECR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETMTECR1_SPEC>;
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
impl From<crate::W<ETMTECR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETMTECR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADRCMP` reader - Address Comparator"]
pub type ADRCMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADRCMP` writer - Address Comparator"]
pub type ADRCMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETMTECR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `MEMMAP` reader - Memmap"]
pub type MEMMAP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MEMMAP` writer - Memmap"]
pub type MEMMAP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETMTECR1_SPEC, u16, u16, 16, O>;
#[doc = "Field `INCEXCTL` reader - Trace Include/Exclude Flag"]
pub type INCEXCTL_R = crate::BitReader<bool>;
#[doc = "Field `INCEXCTL` writer - Trace Include/Exclude Flag"]
pub type INCEXCTL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETMTECR1_SPEC, bool, O>;
#[doc = "Field `TCE` reader - Trace Control Enable"]
pub type TCE_R = crate::BitReader<bool>;
#[doc = "Field `TCE` writer - Trace Control Enable"]
pub type TCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETMTECR1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Address Comparator"]
    #[inline(always)]
    pub fn adrcmp(&self) -> ADRCMP_R {
        ADRCMP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:23 - Memmap"]
    #[inline(always)]
    pub fn memmap(&self) -> MEMMAP_R {
        MEMMAP_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bit 24 - Trace Include/Exclude Flag"]
    #[inline(always)]
    pub fn incexctl(&self) -> INCEXCTL_R {
        INCEXCTL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Trace Control Enable"]
    #[inline(always)]
    pub fn tce(&self) -> TCE_R {
        TCE_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Address Comparator"]
    #[inline(always)]
    #[must_use]
    pub fn adrcmp(&mut self) -> ADRCMP_W<0> {
        ADRCMP_W::new(self)
    }
    #[doc = "Bits 8:23 - Memmap"]
    #[inline(always)]
    #[must_use]
    pub fn memmap(&mut self) -> MEMMAP_W<8> {
        MEMMAP_W::new(self)
    }
    #[doc = "Bit 24 - Trace Include/Exclude Flag"]
    #[inline(always)]
    #[must_use]
    pub fn incexctl(&mut self) -> INCEXCTL_W<24> {
        INCEXCTL_W::new(self)
    }
    #[doc = "Bit 25 - Trace Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tce(&mut self) -> TCE_W<25> {
        TCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ETM Trace control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmtecr1](index.html) module"]
pub struct ETMTECR1_SPEC;
impl crate::RegisterSpec for ETMTECR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etmtecr1::R](R) reader structure"]
impl crate::Readable for ETMTECR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etmtecr1::W](W) writer structure"]
impl crate::Writable for ETMTECR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETMTECR1 to value 0"]
impl crate::Resettable for ETMTECR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
