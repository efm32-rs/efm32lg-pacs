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
#[doc = "Field `UF` reader - Underflow Interrupt Enable"]
pub type UF_R = crate::BitReader<bool>;
#[doc = "Field `UF` writer - Underflow Interrupt Enable"]
pub type UF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 0>;
#[doc = "Field `OF` reader - Overflow Interrupt Enable"]
pub type OF_R = crate::BitReader<bool>;
#[doc = "Field `OF` writer - Overflow Interrupt Enable"]
pub type OF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 1>;
#[doc = "Field `DIRCNG` reader - Direction Change Detect Interrupt Enable"]
pub type DIRCNG_R = crate::BitReader<bool>;
#[doc = "Field `DIRCNG` writer - Direction Change Detect Interrupt Enable"]
pub type DIRCNG_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 2>;
#[doc = "Field `AUXOF` reader - Auxiliary Overflow Interrupt Enable"]
pub type AUXOF_R = crate::BitReader<bool>;
#[doc = "Field `AUXOF` writer - Auxiliary Overflow Interrupt Enable"]
pub type AUXOF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn uf(&self) -> UF_R {
        UF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn of(&self) -> OF_R {
        OF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Direction Change Detect Interrupt Enable"]
    #[inline(always)]
    pub fn dircng(&self) -> DIRCNG_R {
        DIRCNG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Auxiliary Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn auxof(&self) -> AUXOF_R {
        AUXOF_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn uf(&mut self) -> UF_W {
        UF_W::new(self)
    }
    #[doc = "Bit 1 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn of(&mut self) -> OF_W {
        OF_W::new(self)
    }
    #[doc = "Bit 2 - Direction Change Detect Interrupt Enable"]
    #[inline(always)]
    pub fn dircng(&mut self) -> DIRCNG_W {
        DIRCNG_W::new(self)
    }
    #[doc = "Bit 3 - Auxiliary Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn auxof(&mut self) -> AUXOF_W {
        AUXOF_W::new(self)
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
