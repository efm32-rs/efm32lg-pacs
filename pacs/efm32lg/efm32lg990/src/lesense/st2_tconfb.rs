#[doc = "Register `ST2_TCONFB` reader"]
pub struct R(crate::R<ST2_TCONFB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ST2_TCONFB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ST2_TCONFB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ST2_TCONFB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ST2_TCONFB` writer"]
pub struct W(crate::W<ST2_TCONFB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ST2_TCONFB_SPEC>;
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
impl From<crate::W<ST2_TCONFB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ST2_TCONFB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP` reader - Sensor compare value"]
pub type COMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP` writer - Sensor compare value"]
pub type COMP_W<'a> = crate::FieldWriter<'a, u32, ST2_TCONFB_SPEC, u8, u8, 4, 0>;
#[doc = "Field `MASK` reader - Sensor mask"]
pub type MASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASK` writer - Sensor mask"]
pub type MASK_W<'a> = crate::FieldWriter<'a, u32, ST2_TCONFB_SPEC, u8, u8, 4, 4>;
#[doc = "Field `NEXTSTATE` reader - Next state index"]
pub type NEXTSTATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NEXTSTATE` writer - Next state index"]
pub type NEXTSTATE_W<'a> = crate::FieldWriter<'a, u32, ST2_TCONFB_SPEC, u8, u8, 4, 8>;
#[doc = "Field `PRSACT` reader - Configure transition action"]
pub type PRSACT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRSACT` writer - Configure transition action"]
pub type PRSACT_W<'a> = crate::FieldWriter<'a, u32, ST2_TCONFB_SPEC, u8, u8, 3, 12>;
#[doc = "Field `SETIF` reader - Set interrupt flag"]
pub type SETIF_R = crate::BitReader<bool>;
#[doc = "Field `SETIF` writer - Set interrupt flag"]
pub type SETIF_W<'a> = crate::BitWriter<'a, u32, ST2_TCONFB_SPEC, bool, 16>;
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
    #[doc = "Bit 16 - Set interrupt flag"]
    #[inline(always)]
    pub fn setif(&self) -> SETIF_R {
        SETIF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sensor compare value"]
    #[inline(always)]
    pub fn comp(&mut self) -> COMP_W {
        COMP_W::new(self)
    }
    #[doc = "Bits 4:7 - Sensor mask"]
    #[inline(always)]
    pub fn mask(&mut self) -> MASK_W {
        MASK_W::new(self)
    }
    #[doc = "Bits 8:11 - Next state index"]
    #[inline(always)]
    pub fn nextstate(&mut self) -> NEXTSTATE_W {
        NEXTSTATE_W::new(self)
    }
    #[doc = "Bits 12:14 - Configure transition action"]
    #[inline(always)]
    pub fn prsact(&mut self) -> PRSACT_W {
        PRSACT_W::new(self)
    }
    #[doc = "Bit 16 - Set interrupt flag"]
    #[inline(always)]
    pub fn setif(&mut self) -> SETIF_W {
        SETIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "State transition configuration B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2_tconfb](index.html) module"]
pub struct ST2_TCONFB_SPEC;
impl crate::RegisterSpec for ST2_TCONFB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [st2_tconfb::R](R) reader structure"]
impl crate::Readable for ST2_TCONFB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [st2_tconfb::W](W) writer structure"]
impl crate::Writable for ST2_TCONFB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ST2_TCONFB to value 0"]
impl crate::Resettable for ST2_TCONFB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
