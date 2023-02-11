#[doc = "Register `ETMCR` reader"]
pub struct R(crate::R<ETMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETMCR` writer"]
pub struct W(crate::W<ETMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETMCR_SPEC>;
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
impl From<crate::W<ETMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POWERDWN` reader - ETM Control in low power mode"]
pub type POWERDWN_R = crate::BitReader<bool>;
#[doc = "Field `POWERDWN` writer - ETM Control in low power mode"]
pub type POWERDWN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETMCR_SPEC, bool, O>;
#[doc = "Field `PORTSIZE` reader - ETM Port Size"]
pub type PORTSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PORTSIZE` writer - ETM Port Size"]
pub type PORTSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETMCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `STALL` reader - Stall Processor"]
pub type STALL_R = crate::BitReader<bool>;
#[doc = "Field `STALL` writer - Stall Processor"]
pub type STALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETMCR_SPEC, bool, O>;
#[doc = "Field `BRANCHOUTPUT` reader - Branch Output"]
pub type BRANCHOUTPUT_R = crate::BitReader<bool>;
#[doc = "Field `BRANCHOUTPUT` writer - Branch Output"]
pub type BRANCHOUTPUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETMCR_SPEC, bool, O>;
#[doc = "Field `DBGREQCTRL` reader - Debug Request Control"]
pub type DBGREQCTRL_R = crate::BitReader<bool>;
#[doc = "Field `DBGREQCTRL` writer - Debug Request Control"]
pub type DBGREQCTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETMCR_SPEC, bool, O>;
#[doc = "Field `ETMPROG` reader - ETM Programming"]
pub type ETMPROG_R = crate::BitReader<bool>;
#[doc = "Field `ETMPROG` writer - ETM Programming"]
pub type ETMPROG_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETMCR_SPEC, bool, O>;
#[doc = "Field `ETMPORTSEL` reader - ETM Port Selection"]
pub type ETMPORTSEL_R = crate::BitReader<bool>;
#[doc = "Field `ETMPORTSEL` writer - ETM Port Selection"]
pub type ETMPORTSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETMCR_SPEC, bool, O>;
#[doc = "Field `PORTMODE2` reader - Port Mode\\[2\\]"]
pub type PORTMODE2_R = crate::BitReader<bool>;
#[doc = "Field `PORTMODE2` writer - Port Mode\\[2\\]"]
pub type PORTMODE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETMCR_SPEC, bool, O>;
#[doc = "Field `PORTMODE` reader - Port Mode Control"]
pub type PORTMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PORTMODE` writer - Port Mode Control"]
pub type PORTMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETMCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `EPORTSIZE` reader - Port Size\\[3\\]"]
pub type EPORTSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPORTSIZE` writer - Port Size\\[3\\]"]
pub type EPORTSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETMCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TSTAMPEN` reader - Time Stamp Enable"]
pub type TSTAMPEN_R = crate::BitReader<bool>;
#[doc = "Field `TSTAMPEN` writer - Time Stamp Enable"]
pub type TSTAMPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETMCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ETM Control in low power mode"]
    #[inline(always)]
    pub fn powerdwn(&self) -> POWERDWN_R {
        POWERDWN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - ETM Port Size"]
    #[inline(always)]
    pub fn portsize(&self) -> PORTSIZE_R {
        PORTSIZE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Stall Processor"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Branch Output"]
    #[inline(always)]
    pub fn branchoutput(&self) -> BRANCHOUTPUT_R {
        BRANCHOUTPUT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Debug Request Control"]
    #[inline(always)]
    pub fn dbgreqctrl(&self) -> DBGREQCTRL_R {
        DBGREQCTRL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ETM Programming"]
    #[inline(always)]
    pub fn etmprog(&self) -> ETMPROG_R {
        ETMPROG_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ETM Port Selection"]
    #[inline(always)]
    pub fn etmportsel(&self) -> ETMPORTSEL_R {
        ETMPORTSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Port Mode\\[2\\]"]
    #[inline(always)]
    pub fn portmode2(&self) -> PORTMODE2_R {
        PORTMODE2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Port Mode Control"]
    #[inline(always)]
    pub fn portmode(&self) -> PORTMODE_R {
        PORTMODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 21:22 - Port Size\\[3\\]"]
    #[inline(always)]
    pub fn eportsize(&self) -> EPORTSIZE_R {
        EPORTSIZE_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 28 - Time Stamp Enable"]
    #[inline(always)]
    pub fn tstampen(&self) -> TSTAMPEN_R {
        TSTAMPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ETM Control in low power mode"]
    #[inline(always)]
    #[must_use]
    pub fn powerdwn(&mut self) -> POWERDWN_W<0> {
        POWERDWN_W::new(self)
    }
    #[doc = "Bits 4:6 - ETM Port Size"]
    #[inline(always)]
    #[must_use]
    pub fn portsize(&mut self) -> PORTSIZE_W<4> {
        PORTSIZE_W::new(self)
    }
    #[doc = "Bit 7 - Stall Processor"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<7> {
        STALL_W::new(self)
    }
    #[doc = "Bit 8 - Branch Output"]
    #[inline(always)]
    #[must_use]
    pub fn branchoutput(&mut self) -> BRANCHOUTPUT_W<8> {
        BRANCHOUTPUT_W::new(self)
    }
    #[doc = "Bit 9 - Debug Request Control"]
    #[inline(always)]
    #[must_use]
    pub fn dbgreqctrl(&mut self) -> DBGREQCTRL_W<9> {
        DBGREQCTRL_W::new(self)
    }
    #[doc = "Bit 10 - ETM Programming"]
    #[inline(always)]
    #[must_use]
    pub fn etmprog(&mut self) -> ETMPROG_W<10> {
        ETMPROG_W::new(self)
    }
    #[doc = "Bit 11 - ETM Port Selection"]
    #[inline(always)]
    #[must_use]
    pub fn etmportsel(&mut self) -> ETMPORTSEL_W<11> {
        ETMPORTSEL_W::new(self)
    }
    #[doc = "Bit 13 - Port Mode\\[2\\]"]
    #[inline(always)]
    #[must_use]
    pub fn portmode2(&mut self) -> PORTMODE2_W<13> {
        PORTMODE2_W::new(self)
    }
    #[doc = "Bits 16:17 - Port Mode Control"]
    #[inline(always)]
    #[must_use]
    pub fn portmode(&mut self) -> PORTMODE_W<16> {
        PORTMODE_W::new(self)
    }
    #[doc = "Bits 21:22 - Port Size\\[3\\]"]
    #[inline(always)]
    #[must_use]
    pub fn eportsize(&mut self) -> EPORTSIZE_W<21> {
        EPORTSIZE_W::new(self)
    }
    #[doc = "Bit 28 - Time Stamp Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tstampen(&mut self) -> TSTAMPEN_W<28> {
        TSTAMPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Main Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmcr](index.html) module"]
pub struct ETMCR_SPEC;
impl crate::RegisterSpec for ETMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etmcr::R](R) reader structure"]
impl crate::Readable for ETMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etmcr::W](W) writer structure"]
impl crate::Writable for ETMCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETMCR to value 0x0411"]
impl crate::Resettable for ETMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0411;
}
