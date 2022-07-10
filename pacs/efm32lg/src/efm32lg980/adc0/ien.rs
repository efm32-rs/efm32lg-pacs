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
#[doc = "Field `SINGLE` reader - Single Conversion Complete Interrupt Enable"]
pub type SINGLE_R = crate::BitReader<bool>;
#[doc = "Field `SINGLE` writer - Single Conversion Complete Interrupt Enable"]
pub type SINGLE_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 0>;
#[doc = "Field `SCAN` reader - Scan Conversion Complete Interrupt Enable"]
pub type SCAN_R = crate::BitReader<bool>;
#[doc = "Field `SCAN` writer - Scan Conversion Complete Interrupt Enable"]
pub type SCAN_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 1>;
#[doc = "Field `SINGLEOF` reader - Single Result Overflow Interrupt Enable"]
pub type SINGLEOF_R = crate::BitReader<bool>;
#[doc = "Field `SINGLEOF` writer - Single Result Overflow Interrupt Enable"]
pub type SINGLEOF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 8>;
#[doc = "Field `SCANOF` reader - Scan Result Overflow Interrupt Enable"]
pub type SCANOF_R = crate::BitReader<bool>;
#[doc = "Field `SCANOF` writer - Scan Result Overflow Interrupt Enable"]
pub type SCANOF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 9>;
impl R {
    #[doc = "Bit 0 - Single Conversion Complete Interrupt Enable"]
    #[inline(always)]
    pub fn single(&self) -> SINGLE_R {
        SINGLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scan Conversion Complete Interrupt Enable"]
    #[inline(always)]
    pub fn scan(&self) -> SCAN_R {
        SCAN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Single Result Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn singleof(&self) -> SINGLEOF_R {
        SINGLEOF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Scan Result Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn scanof(&self) -> SCANOF_R {
        SCANOF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Single Conversion Complete Interrupt Enable"]
    #[inline(always)]
    pub fn single(&mut self) -> SINGLE_W {
        SINGLE_W::new(self)
    }
    #[doc = "Bit 1 - Scan Conversion Complete Interrupt Enable"]
    #[inline(always)]
    pub fn scan(&mut self) -> SCAN_W {
        SCAN_W::new(self)
    }
    #[doc = "Bit 8 - Single Result Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn singleof(&mut self) -> SINGLEOF_W {
        SINGLEOF_W::new(self)
    }
    #[doc = "Bit 9 - Scan Result Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn scanof(&mut self) -> SCANOF_W {
        SCANOF_W::new(self)
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
