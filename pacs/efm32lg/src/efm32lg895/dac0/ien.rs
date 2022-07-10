#[doc = "Register `IEN` reader"]
pub struct R(crate::R<IEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IEN` writer"]
pub struct W(crate::W<IEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IEN_SPEC>;
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
impl From<crate::W<IEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0` reader - Channel 0 Conversion Complete Interrupt Enable"]
pub type CH0_R = crate::BitReader<bool>;
#[doc = "Field `CH0` writer - Channel 0 Conversion Complete Interrupt Enable"]
pub type CH0_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 0>;
#[doc = "Field `CH1` reader - Channel 1 Conversion Complete Interrupt Enable"]
pub type CH1_R = crate::BitReader<bool>;
#[doc = "Field `CH1` writer - Channel 1 Conversion Complete Interrupt Enable"]
pub type CH1_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 1>;
#[doc = "Field `CH0UF` reader - Channel 0 Conversion Data Underflow Interrupt Enable"]
pub type CH0UF_R = crate::BitReader<bool>;
#[doc = "Field `CH0UF` writer - Channel 0 Conversion Data Underflow Interrupt Enable"]
pub type CH0UF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 4>;
#[doc = "Field `CH1UF` reader - Channel 1 Conversion Data Underflow Interrupt Enable"]
pub type CH1UF_R = crate::BitReader<bool>;
#[doc = "Field `CH1UF` writer - Channel 1 Conversion Data Underflow Interrupt Enable"]
pub type CH1UF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 5>;
impl R {
    #[doc = "Bit 0 - Channel 0 Conversion Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Conversion Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 0 Conversion Data Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn ch0uf(&self) -> CH0UF_R {
        CH0UF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 1 Conversion Data Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn ch1uf(&self) -> CH1UF_R {
        CH1UF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Conversion Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch0(&mut self) -> CH0_W {
        CH0_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Conversion Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch1(&mut self) -> CH1_W {
        CH1_W::new(self)
    }
    #[doc = "Bit 4 - Channel 0 Conversion Data Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn ch0uf(&mut self) -> CH0UF_W {
        CH0UF_W::new(self)
    }
    #[doc = "Bit 5 - Channel 1 Conversion Data Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn ch1uf(&mut self) -> CH1UF_W {
        CH1UF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](index.html) module"]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ien::R](R) reader structure"]
impl crate::Readable for IEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ien::W](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
