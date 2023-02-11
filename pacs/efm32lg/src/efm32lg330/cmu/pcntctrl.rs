#[doc = "Register `PCNTCTRL` reader"]
pub struct R(crate::R<PCNTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCNTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCNTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCNTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCNTCTRL` writer"]
pub struct W(crate::W<PCNTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCNTCTRL_SPEC>;
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
impl From<crate::W<PCNTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCNTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCNT0CLKEN` reader - PCNT0 Clock Enable"]
pub type PCNT0CLKEN_R = crate::BitReader<bool>;
#[doc = "Field `PCNT0CLKEN` writer - PCNT0 Clock Enable"]
pub type PCNT0CLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTCTRL_SPEC, bool, O>;
#[doc = "Field `PCNT0CLKSEL` reader - PCNT0 Clock Select"]
pub type PCNT0CLKSEL_R = crate::BitReader<bool>;
#[doc = "Field `PCNT0CLKSEL` writer - PCNT0 Clock Select"]
pub type PCNT0CLKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTCTRL_SPEC, bool, O>;
#[doc = "Field `PCNT1CLKEN` reader - PCNT1 Clock Enable"]
pub type PCNT1CLKEN_R = crate::BitReader<bool>;
#[doc = "Field `PCNT1CLKEN` writer - PCNT1 Clock Enable"]
pub type PCNT1CLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTCTRL_SPEC, bool, O>;
#[doc = "Field `PCNT1CLKSEL` reader - PCNT1 Clock Select"]
pub type PCNT1CLKSEL_R = crate::BitReader<bool>;
#[doc = "Field `PCNT1CLKSEL` writer - PCNT1 Clock Select"]
pub type PCNT1CLKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTCTRL_SPEC, bool, O>;
#[doc = "Field `PCNT2CLKEN` reader - PCNT2 Clock Enable"]
pub type PCNT2CLKEN_R = crate::BitReader<bool>;
#[doc = "Field `PCNT2CLKEN` writer - PCNT2 Clock Enable"]
pub type PCNT2CLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTCTRL_SPEC, bool, O>;
#[doc = "Field `PCNT2CLKSEL` reader - PCNT2 Clock Select"]
pub type PCNT2CLKSEL_R = crate::BitReader<bool>;
#[doc = "Field `PCNT2CLKSEL` writer - PCNT2 Clock Select"]
pub type PCNT2CLKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PCNT0 Clock Enable"]
    #[inline(always)]
    pub fn pcnt0clken(&self) -> PCNT0CLKEN_R {
        PCNT0CLKEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PCNT0 Clock Select"]
    #[inline(always)]
    pub fn pcnt0clksel(&self) -> PCNT0CLKSEL_R {
        PCNT0CLKSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PCNT1 Clock Enable"]
    #[inline(always)]
    pub fn pcnt1clken(&self) -> PCNT1CLKEN_R {
        PCNT1CLKEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PCNT1 Clock Select"]
    #[inline(always)]
    pub fn pcnt1clksel(&self) -> PCNT1CLKSEL_R {
        PCNT1CLKSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PCNT2 Clock Enable"]
    #[inline(always)]
    pub fn pcnt2clken(&self) -> PCNT2CLKEN_R {
        PCNT2CLKEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PCNT2 Clock Select"]
    #[inline(always)]
    pub fn pcnt2clksel(&self) -> PCNT2CLKSEL_R {
        PCNT2CLKSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PCNT0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt0clken(&mut self) -> PCNT0CLKEN_W<0> {
        PCNT0CLKEN_W::new(self)
    }
    #[doc = "Bit 1 - PCNT0 Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt0clksel(&mut self) -> PCNT0CLKSEL_W<1> {
        PCNT0CLKSEL_W::new(self)
    }
    #[doc = "Bit 2 - PCNT1 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt1clken(&mut self) -> PCNT1CLKEN_W<2> {
        PCNT1CLKEN_W::new(self)
    }
    #[doc = "Bit 3 - PCNT1 Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt1clksel(&mut self) -> PCNT1CLKSEL_W<3> {
        PCNT1CLKSEL_W::new(self)
    }
    #[doc = "Bit 4 - PCNT2 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt2clken(&mut self) -> PCNT2CLKEN_W<4> {
        PCNT2CLKEN_W::new(self)
    }
    #[doc = "Bit 5 - PCNT2 Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt2clksel(&mut self) -> PCNT2CLKSEL_W<5> {
        PCNT2CLKSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PCNT Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcntctrl](index.html) module"]
pub struct PCNTCTRL_SPEC;
impl crate::RegisterSpec for PCNTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcntctrl::R](R) reader structure"]
impl crate::Readable for PCNTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcntctrl::W](W) writer structure"]
impl crate::Writable for PCNTCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCNTCTRL to value 0"]
impl crate::Resettable for PCNTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
