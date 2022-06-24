#[doc = "Register `RDS` reader"]
pub struct R(crate::R<RDS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RDS` writer"]
pub struct W(crate::W<RDS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RDS_SPEC>;
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
impl From<crate::W<RDS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RDS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDSCH0` reader - Retain Descriptor State"]
pub type RDSCH0_R = crate::BitReader<bool>;
#[doc = "Field `RDSCH0` writer - Retain Descriptor State"]
pub type RDSCH0_W<'a> = crate::BitWriter<'a, u32, RDS_SPEC, bool, 0>;
#[doc = "Field `RDSCH1` reader - Retain Descriptor State"]
pub type RDSCH1_R = crate::BitReader<bool>;
#[doc = "Field `RDSCH1` writer - Retain Descriptor State"]
pub type RDSCH1_W<'a> = crate::BitWriter<'a, u32, RDS_SPEC, bool, 1>;
#[doc = "Field `RDSCH2` reader - Retain Descriptor State"]
pub type RDSCH2_R = crate::BitReader<bool>;
#[doc = "Field `RDSCH2` writer - Retain Descriptor State"]
pub type RDSCH2_W<'a> = crate::BitWriter<'a, u32, RDS_SPEC, bool, 2>;
#[doc = "Field `RDSCH3` reader - Retain Descriptor State"]
pub type RDSCH3_R = crate::BitReader<bool>;
#[doc = "Field `RDSCH3` writer - Retain Descriptor State"]
pub type RDSCH3_W<'a> = crate::BitWriter<'a, u32, RDS_SPEC, bool, 3>;
#[doc = "Field `RDSCH4` reader - Retain Descriptor State"]
pub type RDSCH4_R = crate::BitReader<bool>;
#[doc = "Field `RDSCH4` writer - Retain Descriptor State"]
pub type RDSCH4_W<'a> = crate::BitWriter<'a, u32, RDS_SPEC, bool, 4>;
#[doc = "Field `RDSCH5` reader - Retain Descriptor State"]
pub type RDSCH5_R = crate::BitReader<bool>;
#[doc = "Field `RDSCH5` writer - Retain Descriptor State"]
pub type RDSCH5_W<'a> = crate::BitWriter<'a, u32, RDS_SPEC, bool, 5>;
#[doc = "Field `RDSCH6` reader - Retain Descriptor State"]
pub type RDSCH6_R = crate::BitReader<bool>;
#[doc = "Field `RDSCH6` writer - Retain Descriptor State"]
pub type RDSCH6_W<'a> = crate::BitWriter<'a, u32, RDS_SPEC, bool, 6>;
#[doc = "Field `RDSCH7` reader - Retain Descriptor State"]
pub type RDSCH7_R = crate::BitReader<bool>;
#[doc = "Field `RDSCH7` writer - Retain Descriptor State"]
pub type RDSCH7_W<'a> = crate::BitWriter<'a, u32, RDS_SPEC, bool, 7>;
#[doc = "Field `RDSCH8` reader - Retain Descriptor State"]
pub type RDSCH8_R = crate::BitReader<bool>;
#[doc = "Field `RDSCH8` writer - Retain Descriptor State"]
pub type RDSCH8_W<'a> = crate::BitWriter<'a, u32, RDS_SPEC, bool, 8>;
#[doc = "Field `RDSCH9` reader - Retain Descriptor State"]
pub type RDSCH9_R = crate::BitReader<bool>;
#[doc = "Field `RDSCH9` writer - Retain Descriptor State"]
pub type RDSCH9_W<'a> = crate::BitWriter<'a, u32, RDS_SPEC, bool, 9>;
#[doc = "Field `RDSCH10` reader - Retain Descriptor State"]
pub type RDSCH10_R = crate::BitReader<bool>;
#[doc = "Field `RDSCH10` writer - Retain Descriptor State"]
pub type RDSCH10_W<'a> = crate::BitWriter<'a, u32, RDS_SPEC, bool, 10>;
#[doc = "Field `RDSCH11` reader - Retain Descriptor State"]
pub type RDSCH11_R = crate::BitReader<bool>;
#[doc = "Field `RDSCH11` writer - Retain Descriptor State"]
pub type RDSCH11_W<'a> = crate::BitWriter<'a, u32, RDS_SPEC, bool, 11>;
impl R {
    #[doc = "Bit 0 - Retain Descriptor State"]
    #[inline(always)]
    pub fn rdsch0(&self) -> RDSCH0_R {
        RDSCH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Retain Descriptor State"]
    #[inline(always)]
    pub fn rdsch1(&self) -> RDSCH1_R {
        RDSCH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Retain Descriptor State"]
    #[inline(always)]
    pub fn rdsch2(&self) -> RDSCH2_R {
        RDSCH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Retain Descriptor State"]
    #[inline(always)]
    pub fn rdsch3(&self) -> RDSCH3_R {
        RDSCH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Retain Descriptor State"]
    #[inline(always)]
    pub fn rdsch4(&self) -> RDSCH4_R {
        RDSCH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Retain Descriptor State"]
    #[inline(always)]
    pub fn rdsch5(&self) -> RDSCH5_R {
        RDSCH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Retain Descriptor State"]
    #[inline(always)]
    pub fn rdsch6(&self) -> RDSCH6_R {
        RDSCH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Retain Descriptor State"]
    #[inline(always)]
    pub fn rdsch7(&self) -> RDSCH7_R {
        RDSCH7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Retain Descriptor State"]
    #[inline(always)]
    pub fn rdsch8(&self) -> RDSCH8_R {
        RDSCH8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Retain Descriptor State"]
    #[inline(always)]
    pub fn rdsch9(&self) -> RDSCH9_R {
        RDSCH9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Retain Descriptor State"]
    #[inline(always)]
    pub fn rdsch10(&self) -> RDSCH10_R {
        RDSCH10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Retain Descriptor State"]
    #[inline(always)]
    pub fn rdsch11(&self) -> RDSCH11_R {
        RDSCH11_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Retain Descriptor State"]
    #[inline(always)]
    pub fn rdsch0(&mut self) -> RDSCH0_W {
        RDSCH0_W::new(self)
    }
    #[doc = "Bit 1 - Retain Descriptor State"]
    #[inline(always)]
    pub fn rdsch1(&mut self) -> RDSCH1_W {
        RDSCH1_W::new(self)
    }
    #[doc = "Bit 2 - Retain Descriptor State"]
    #[inline(always)]
    pub fn rdsch2(&mut self) -> RDSCH2_W {
        RDSCH2_W::new(self)
    }
    #[doc = "Bit 3 - Retain Descriptor State"]
    #[inline(always)]
    pub fn rdsch3(&mut self) -> RDSCH3_W {
        RDSCH3_W::new(self)
    }
    #[doc = "Bit 4 - Retain Descriptor State"]
    #[inline(always)]
    pub fn rdsch4(&mut self) -> RDSCH4_W {
        RDSCH4_W::new(self)
    }
    #[doc = "Bit 5 - Retain Descriptor State"]
    #[inline(always)]
    pub fn rdsch5(&mut self) -> RDSCH5_W {
        RDSCH5_W::new(self)
    }
    #[doc = "Bit 6 - Retain Descriptor State"]
    #[inline(always)]
    pub fn rdsch6(&mut self) -> RDSCH6_W {
        RDSCH6_W::new(self)
    }
    #[doc = "Bit 7 - Retain Descriptor State"]
    #[inline(always)]
    pub fn rdsch7(&mut self) -> RDSCH7_W {
        RDSCH7_W::new(self)
    }
    #[doc = "Bit 8 - Retain Descriptor State"]
    #[inline(always)]
    pub fn rdsch8(&mut self) -> RDSCH8_W {
        RDSCH8_W::new(self)
    }
    #[doc = "Bit 9 - Retain Descriptor State"]
    #[inline(always)]
    pub fn rdsch9(&mut self) -> RDSCH9_W {
        RDSCH9_W::new(self)
    }
    #[doc = "Bit 10 - Retain Descriptor State"]
    #[inline(always)]
    pub fn rdsch10(&mut self) -> RDSCH10_W {
        RDSCH10_W::new(self)
    }
    #[doc = "Bit 11 - Retain Descriptor State"]
    #[inline(always)]
    pub fn rdsch11(&mut self) -> RDSCH11_W {
        RDSCH11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Retain Descriptor State\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rds](index.html) module"]
pub struct RDS_SPEC;
impl crate::RegisterSpec for RDS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rds::R](R) reader structure"]
impl crate::Readable for RDS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rds::W](W) writer structure"]
impl crate::Writable for RDS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RDS to value 0"]
impl crate::Resettable for RDS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
