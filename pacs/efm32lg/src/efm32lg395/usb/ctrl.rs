#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VBUSENAP` reader - VBUSEN Active Polarity"]
pub type VBUSENAP_R = crate::BitReader<bool>;
#[doc = "Field `VBUSENAP` writer - VBUSEN Active Polarity"]
pub type VBUSENAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `DMPUAP` reader - DMPU Active Polarity"]
pub type DMPUAP_R = crate::BitReader<bool>;
#[doc = "Field `DMPUAP` writer - DMPU Active Polarity"]
pub type DMPUAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `VREGDIS` reader - Voltage Regulator Disable"]
pub type VREGDIS_R = crate::BitReader<bool>;
#[doc = "Field `VREGDIS` writer - Voltage Regulator Disable"]
pub type VREGDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `VREGOSEN` reader - VREGO Sense Enable"]
pub type VREGOSEN_R = crate::BitReader<bool>;
#[doc = "Field `VREGOSEN` writer - VREGO Sense Enable"]
pub type VREGOSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `BIASPROGEM01` reader - Regulator Bias Programming Value in EM0/1"]
pub type BIASPROGEM01_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BIASPROGEM01` writer - Regulator Bias Programming Value in EM0/1"]
pub type BIASPROGEM01_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `BIASPROGEM23` reader - Regulator Bias Programming Value in EM2/3"]
pub type BIASPROGEM23_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BIASPROGEM23` writer - Regulator Bias Programming Value in EM2/3"]
pub type BIASPROGEM23_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - VBUSEN Active Polarity"]
    #[inline(always)]
    pub fn vbusenap(&self) -> VBUSENAP_R {
        VBUSENAP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMPU Active Polarity"]
    #[inline(always)]
    pub fn dmpuap(&self) -> DMPUAP_R {
        DMPUAP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - Voltage Regulator Disable"]
    #[inline(always)]
    pub fn vregdis(&self) -> VREGDIS_R {
        VREGDIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - VREGO Sense Enable"]
    #[inline(always)]
    pub fn vregosen(&self) -> VREGOSEN_R {
        VREGOSEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Regulator Bias Programming Value in EM0/1"]
    #[inline(always)]
    pub fn biasprogem01(&self) -> BIASPROGEM01_R {
        BIASPROGEM01_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Regulator Bias Programming Value in EM2/3"]
    #[inline(always)]
    pub fn biasprogem23(&self) -> BIASPROGEM23_R {
        BIASPROGEM23_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - VBUSEN Active Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn vbusenap(&mut self) -> VBUSENAP_W<0> {
        VBUSENAP_W::new(self)
    }
    #[doc = "Bit 1 - DMPU Active Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn dmpuap(&mut self) -> DMPUAP_W<1> {
        DMPUAP_W::new(self)
    }
    #[doc = "Bit 16 - Voltage Regulator Disable"]
    #[inline(always)]
    #[must_use]
    pub fn vregdis(&mut self) -> VREGDIS_W<16> {
        VREGDIS_W::new(self)
    }
    #[doc = "Bit 17 - VREGO Sense Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vregosen(&mut self) -> VREGOSEN_W<17> {
        VREGOSEN_W::new(self)
    }
    #[doc = "Bits 20:21 - Regulator Bias Programming Value in EM0/1"]
    #[inline(always)]
    #[must_use]
    pub fn biasprogem01(&mut self) -> BIASPROGEM01_W<20> {
        BIASPROGEM01_W::new(self)
    }
    #[doc = "Bits 24:25 - Regulator Bias Programming Value in EM2/3"]
    #[inline(always)]
    #[must_use]
    pub fn biasprogem23(&mut self) -> BIASPROGEM23_W<24> {
        BIASPROGEM23_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
