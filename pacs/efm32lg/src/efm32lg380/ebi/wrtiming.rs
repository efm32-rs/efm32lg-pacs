#[doc = "Register `WRTIMING` reader"]
pub struct R(crate::R<WRTIMING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRTIMING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRTIMING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRTIMING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRTIMING` writer"]
pub struct W(crate::W<WRTIMING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRTIMING_SPEC>;
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
impl From<crate::W<WRTIMING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRTIMING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRSETUP` reader - Write Setup Time"]
pub type WRSETUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRSETUP` writer - Write Setup Time"]
pub type WRSETUP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WRTIMING_SPEC, u8, u8, 2, O>;
#[doc = "Field `WRSTRB` reader - Write Strobe Time"]
pub type WRSTRB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRSTRB` writer - Write Strobe Time"]
pub type WRSTRB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WRTIMING_SPEC, u8, u8, 6, O>;
#[doc = "Field `WRHOLD` reader - Write Hold Time"]
pub type WRHOLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRHOLD` writer - Write Hold Time"]
pub type WRHOLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WRTIMING_SPEC, u8, u8, 2, O>;
#[doc = "Field `HALFWE` reader - Half Cycle WEn Strobe Duration Enable"]
pub type HALFWE_R = crate::BitReader<bool>;
#[doc = "Field `HALFWE` writer - Half Cycle WEn Strobe Duration Enable"]
pub type HALFWE_W<'a, const O: u8> = crate::BitWriter<'a, u32, WRTIMING_SPEC, bool, O>;
#[doc = "Field `WBUFDIS` reader - Write Buffer Disable"]
pub type WBUFDIS_R = crate::BitReader<bool>;
#[doc = "Field `WBUFDIS` writer - Write Buffer Disable"]
pub type WBUFDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, WRTIMING_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Write Setup Time"]
    #[inline(always)]
    pub fn wrsetup(&self) -> WRSETUP_R {
        WRSETUP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:13 - Write Strobe Time"]
    #[inline(always)]
    pub fn wrstrb(&self) -> WRSTRB_R {
        WRSTRB_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:17 - Write Hold Time"]
    #[inline(always)]
    pub fn wrhold(&self) -> WRHOLD_R {
        WRHOLD_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 28 - Half Cycle WEn Strobe Duration Enable"]
    #[inline(always)]
    pub fn halfwe(&self) -> HALFWE_R {
        HALFWE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Write Buffer Disable"]
    #[inline(always)]
    pub fn wbufdis(&self) -> WBUFDIS_R {
        WBUFDIS_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Write Setup Time"]
    #[inline(always)]
    #[must_use]
    pub fn wrsetup(&mut self) -> WRSETUP_W<0> {
        WRSETUP_W::new(self)
    }
    #[doc = "Bits 8:13 - Write Strobe Time"]
    #[inline(always)]
    #[must_use]
    pub fn wrstrb(&mut self) -> WRSTRB_W<8> {
        WRSTRB_W::new(self)
    }
    #[doc = "Bits 16:17 - Write Hold Time"]
    #[inline(always)]
    #[must_use]
    pub fn wrhold(&mut self) -> WRHOLD_W<16> {
        WRHOLD_W::new(self)
    }
    #[doc = "Bit 28 - Half Cycle WEn Strobe Duration Enable"]
    #[inline(always)]
    #[must_use]
    pub fn halfwe(&mut self) -> HALFWE_W<28> {
        HALFWE_W::new(self)
    }
    #[doc = "Bit 29 - Write Buffer Disable"]
    #[inline(always)]
    #[must_use]
    pub fn wbufdis(&mut self) -> WBUFDIS_W<29> {
        WBUFDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrtiming](index.html) module"]
pub struct WRTIMING_SPEC;
impl crate::RegisterSpec for WRTIMING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wrtiming::R](R) reader structure"]
impl crate::Readable for WRTIMING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wrtiming::W](W) writer structure"]
impl crate::Writable for WRTIMING_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WRTIMING to value 0x0003_3f03"]
impl crate::Resettable for WRTIMING_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_3f03;
}
