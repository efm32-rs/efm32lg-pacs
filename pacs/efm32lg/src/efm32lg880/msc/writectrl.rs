#[doc = "Register `WRITECTRL` reader"]
pub struct R(crate::R<WRITECTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRITECTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRITECTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRITECTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRITECTRL` writer"]
pub struct W(crate::W<WRITECTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRITECTRL_SPEC>;
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
impl From<crate::W<WRITECTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRITECTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WREN` reader - Enable Write/Erase Controller"]
pub type WREN_R = crate::BitReader<bool>;
#[doc = "Field `WREN` writer - Enable Write/Erase Controller"]
pub type WREN_W<'a> = crate::BitWriter<'a, u32, WRITECTRL_SPEC, bool, 0>;
#[doc = "Field `IRQERASEABORT` reader - Abort Page Erase on Interrupt"]
pub type IRQERASEABORT_R = crate::BitReader<bool>;
#[doc = "Field `IRQERASEABORT` writer - Abort Page Erase on Interrupt"]
pub type IRQERASEABORT_W<'a> = crate::BitWriter<'a, u32, WRITECTRL_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - Enable Write/Erase Controller"]
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Abort Page Erase on Interrupt"]
    #[inline(always)]
    pub fn irqeraseabort(&self) -> IRQERASEABORT_R {
        IRQERASEABORT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write/Erase Controller"]
    #[inline(always)]
    pub fn wren(&mut self) -> WREN_W {
        WREN_W::new(self)
    }
    #[doc = "Bit 1 - Abort Page Erase on Interrupt"]
    #[inline(always)]
    pub fn irqeraseabort(&mut self) -> IRQERASEABORT_W {
        IRQERASEABORT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [writectrl](index.html) module"]
pub struct WRITECTRL_SPEC;
impl crate::RegisterSpec for WRITECTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [writectrl::R](R) reader structure"]
impl crate::Readable for WRITECTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [writectrl::W](W) writer structure"]
impl crate::Writable for WRITECTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WRITECTRL to value 0"]
impl crate::Resettable for WRITECTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
