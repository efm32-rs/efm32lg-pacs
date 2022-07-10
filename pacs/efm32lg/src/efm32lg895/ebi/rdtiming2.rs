#[doc = "Register `RDTIMING2` reader"]
pub struct R(crate::R<RDTIMING2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDTIMING2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDTIMING2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDTIMING2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RDTIMING2` writer"]
pub struct W(crate::W<RDTIMING2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RDTIMING2_SPEC>;
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
impl From<crate::W<RDTIMING2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RDTIMING2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDSETUP` reader - Read Setup Time"]
pub type RDSETUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDSETUP` writer - Read Setup Time"]
pub type RDSETUP_W<'a> = crate::FieldWriter<'a, u32, RDTIMING2_SPEC, u8, u8, 2, 0>;
#[doc = "Field `RDSTRB` reader - Read Strobe Time"]
pub type RDSTRB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDSTRB` writer - Read Strobe Time"]
pub type RDSTRB_W<'a> = crate::FieldWriter<'a, u32, RDTIMING2_SPEC, u8, u8, 6, 8>;
#[doc = "Field `RDHOLD` reader - Read Hold Time"]
pub type RDHOLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDHOLD` writer - Read Hold Time"]
pub type RDHOLD_W<'a> = crate::FieldWriter<'a, u32, RDTIMING2_SPEC, u8, u8, 2, 16>;
#[doc = "Field `HALFRE` reader - Half Cycle REn Strobe Duration Enable"]
pub type HALFRE_R = crate::BitReader<bool>;
#[doc = "Field `HALFRE` writer - Half Cycle REn Strobe Duration Enable"]
pub type HALFRE_W<'a> = crate::BitWriter<'a, u32, RDTIMING2_SPEC, bool, 28>;
#[doc = "Field `PREFETCH` reader - Prefetch Enable"]
pub type PREFETCH_R = crate::BitReader<bool>;
#[doc = "Field `PREFETCH` writer - Prefetch Enable"]
pub type PREFETCH_W<'a> = crate::BitWriter<'a, u32, RDTIMING2_SPEC, bool, 29>;
#[doc = "Field `PAGEMODE` reader - Page Mode Access Enable"]
pub type PAGEMODE_R = crate::BitReader<bool>;
#[doc = "Field `PAGEMODE` writer - Page Mode Access Enable"]
pub type PAGEMODE_W<'a> = crate::BitWriter<'a, u32, RDTIMING2_SPEC, bool, 30>;
impl R {
    #[doc = "Bits 0:1 - Read Setup Time"]
    #[inline(always)]
    pub fn rdsetup(&self) -> RDSETUP_R {
        RDSETUP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:13 - Read Strobe Time"]
    #[inline(always)]
    pub fn rdstrb(&self) -> RDSTRB_R {
        RDSTRB_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:17 - Read Hold Time"]
    #[inline(always)]
    pub fn rdhold(&self) -> RDHOLD_R {
        RDHOLD_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 28 - Half Cycle REn Strobe Duration Enable"]
    #[inline(always)]
    pub fn halfre(&self) -> HALFRE_R {
        HALFRE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Prefetch Enable"]
    #[inline(always)]
    pub fn prefetch(&self) -> PREFETCH_R {
        PREFETCH_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Page Mode Access Enable"]
    #[inline(always)]
    pub fn pagemode(&self) -> PAGEMODE_R {
        PAGEMODE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Read Setup Time"]
    #[inline(always)]
    pub fn rdsetup(&mut self) -> RDSETUP_W {
        RDSETUP_W::new(self)
    }
    #[doc = "Bits 8:13 - Read Strobe Time"]
    #[inline(always)]
    pub fn rdstrb(&mut self) -> RDSTRB_W {
        RDSTRB_W::new(self)
    }
    #[doc = "Bits 16:17 - Read Hold Time"]
    #[inline(always)]
    pub fn rdhold(&mut self) -> RDHOLD_W {
        RDHOLD_W::new(self)
    }
    #[doc = "Bit 28 - Half Cycle REn Strobe Duration Enable"]
    #[inline(always)]
    pub fn halfre(&mut self) -> HALFRE_W {
        HALFRE_W::new(self)
    }
    #[doc = "Bit 29 - Prefetch Enable"]
    #[inline(always)]
    pub fn prefetch(&mut self) -> PREFETCH_W {
        PREFETCH_W::new(self)
    }
    #[doc = "Bit 30 - Page Mode Access Enable"]
    #[inline(always)]
    pub fn pagemode(&mut self) -> PAGEMODE_W {
        PAGEMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Read Timing Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdtiming2](index.html) module"]
pub struct RDTIMING2_SPEC;
impl crate::RegisterSpec for RDTIMING2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rdtiming2::R](R) reader structure"]
impl crate::Readable for RDTIMING2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rdtiming2::W](W) writer structure"]
impl crate::Writable for RDTIMING2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RDTIMING2 to value 0x0003_3f03"]
impl crate::Resettable for RDTIMING2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0003_3f03
    }
}
