#[doc = "Register `ST5_TCONFA` reader"]
pub struct R(crate::R<ST5_TCONFA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ST5_TCONFA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ST5_TCONFA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ST5_TCONFA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ST5_TCONFA` writer"]
pub struct W(crate::W<ST5_TCONFA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ST5_TCONFA_SPEC>;
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
impl From<crate::W<ST5_TCONFA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ST5_TCONFA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP` reader - Sensor compare value"]
pub type COMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP` writer - Sensor compare value"]
pub type COMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ST5_TCONFA_SPEC, u8, u8, 4, O>;
#[doc = "Field `MASK` reader - Sensor mask"]
pub type MASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASK` writer - Sensor mask"]
pub type MASK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ST5_TCONFA_SPEC, u8, u8, 4, O>;
#[doc = "Field `NEXTSTATE` reader - Next state index"]
pub type NEXTSTATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NEXTSTATE` writer - Next state index"]
pub type NEXTSTATE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ST5_TCONFA_SPEC, u8, u8, 4, O>;
#[doc = "Field `PRSACT` reader - Configure transition action"]
pub type PRSACT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRSACT` writer - Configure transition action"]
pub type PRSACT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ST5_TCONFA_SPEC, u8, u8, 3, O>;
#[doc = "Field `SETIF` reader - Set interrupt flag enable"]
pub type SETIF_R = crate::BitReader<bool>;
#[doc = "Field `SETIF` writer - Set interrupt flag enable"]
pub type SETIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ST5_TCONFA_SPEC, bool, O>;
#[doc = "Field `CHAIN` reader - Enable state descriptor chaining"]
pub type CHAIN_R = crate::BitReader<bool>;
#[doc = "Field `CHAIN` writer - Enable state descriptor chaining"]
pub type CHAIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ST5_TCONFA_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Sensor compare value"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Sensor mask"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Next state index"]
    #[inline(always)]
    pub fn nextstate(&self) -> NEXTSTATE_R {
        NEXTSTATE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Configure transition action"]
    #[inline(always)]
    pub fn prsact(&self) -> PRSACT_R {
        PRSACT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16 - Set interrupt flag enable"]
    #[inline(always)]
    pub fn setif(&self) -> SETIF_R {
        SETIF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable state descriptor chaining"]
    #[inline(always)]
    pub fn chain(&self) -> CHAIN_R {
        CHAIN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sensor compare value"]
    #[inline(always)]
    #[must_use]
    pub fn comp(&mut self) -> COMP_W<0> {
        COMP_W::new(self)
    }
    #[doc = "Bits 4:7 - Sensor mask"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MASK_W<4> {
        MASK_W::new(self)
    }
    #[doc = "Bits 8:11 - Next state index"]
    #[inline(always)]
    #[must_use]
    pub fn nextstate(&mut self) -> NEXTSTATE_W<8> {
        NEXTSTATE_W::new(self)
    }
    #[doc = "Bits 12:14 - Configure transition action"]
    #[inline(always)]
    #[must_use]
    pub fn prsact(&mut self) -> PRSACT_W<12> {
        PRSACT_W::new(self)
    }
    #[doc = "Bit 16 - Set interrupt flag enable"]
    #[inline(always)]
    #[must_use]
    pub fn setif(&mut self) -> SETIF_W<16> {
        SETIF_W::new(self)
    }
    #[doc = "Bit 18 - Enable state descriptor chaining"]
    #[inline(always)]
    #[must_use]
    pub fn chain(&mut self) -> CHAIN_W<18> {
        CHAIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "State transition configuration A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st5_tconfa](index.html) module"]
pub struct ST5_TCONFA_SPEC;
impl crate::RegisterSpec for ST5_TCONFA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [st5_tconfa::R](R) reader structure"]
impl crate::Readable for ST5_TCONFA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [st5_tconfa::W](W) writer structure"]
impl crate::Writable for ST5_TCONFA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ST5_TCONFA to value 0"]
impl crate::Resettable for ST5_TCONFA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
