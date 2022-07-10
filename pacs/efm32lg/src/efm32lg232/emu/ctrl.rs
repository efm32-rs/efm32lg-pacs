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
#[doc = "Field `EMVREG` reader - Energy Mode Voltage Regulator Control"]
pub type EMVREG_R = crate::BitReader<bool>;
#[doc = "Field `EMVREG` writer - Energy Mode Voltage Regulator Control"]
pub type EMVREG_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 0>;
#[doc = "Field `EM2BLOCK` reader - Energy Mode 2 Block"]
pub type EM2BLOCK_R = crate::BitReader<bool>;
#[doc = "Field `EM2BLOCK` writer - Energy Mode 2 Block"]
pub type EM2BLOCK_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 1>;
#[doc = "Field `EM4CTRL` reader - Energy Mode 4 Control"]
pub type EM4CTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EM4CTRL` writer - Energy Mode 4 Control"]
pub type EM4CTRL_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, 2>;
impl R {
    #[doc = "Bit 0 - Energy Mode Voltage Regulator Control"]
    #[inline(always)]
    pub fn emvreg(&self) -> EMVREG_R {
        EMVREG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Energy Mode 2 Block"]
    #[inline(always)]
    pub fn em2block(&self) -> EM2BLOCK_R {
        EM2BLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Energy Mode 4 Control"]
    #[inline(always)]
    pub fn em4ctrl(&self) -> EM4CTRL_R {
        EM4CTRL_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Energy Mode Voltage Regulator Control"]
    #[inline(always)]
    pub fn emvreg(&mut self) -> EMVREG_W {
        EMVREG_W::new(self)
    }
    #[doc = "Bit 1 - Energy Mode 2 Block"]
    #[inline(always)]
    pub fn em2block(&mut self) -> EM2BLOCK_W {
        EM2BLOCK_W::new(self)
    }
    #[doc = "Bits 2:3 - Energy Mode 4 Control"]
    #[inline(always)]
    pub fn em4ctrl(&mut self) -> EM4CTRL_W {
        EM4CTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
