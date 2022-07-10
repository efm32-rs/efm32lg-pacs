#[doc = "Register `TIMEBASE` reader"]
pub struct R(crate::R<TIMEBASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMEBASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMEBASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMEBASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMEBASE` writer"]
pub struct W(crate::W<TIMEBASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMEBASE_SPEC>;
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
impl From<crate::W<TIMEBASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMEBASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASE` reader - Timebase used by MSC to time flash writes and erases"]
pub type BASE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BASE` writer - Timebase used by MSC to time flash writes and erases"]
pub type BASE_W<'a> = crate::FieldWriter<'a, u32, TIMEBASE_SPEC, u8, u8, 6, 0>;
#[doc = "Field `PERIOD` reader - Sets the timebase period"]
pub type PERIOD_R = crate::BitReader<bool>;
#[doc = "Field `PERIOD` writer - Sets the timebase period"]
pub type PERIOD_W<'a> = crate::BitWriter<'a, u32, TIMEBASE_SPEC, bool, 16>;
impl R {
    #[doc = "Bits 0:5 - Timebase used by MSC to time flash writes and erases"]
    #[inline(always)]
    pub fn base(&self) -> BASE_R {
        BASE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 16 - Sets the timebase period"]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Timebase used by MSC to time flash writes and erases"]
    #[inline(always)]
    pub fn base(&mut self) -> BASE_W {
        BASE_W::new(self)
    }
    #[doc = "Bit 16 - Sets the timebase period"]
    #[inline(always)]
    pub fn period(&mut self) -> PERIOD_W {
        PERIOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Write and Erase Timebase\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timebase](index.html) module"]
pub struct TIMEBASE_SPEC;
impl crate::RegisterSpec for TIMEBASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timebase::R](R) reader structure"]
impl crate::Readable for TIMEBASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timebase::W](W) writer structure"]
impl crate::Writable for TIMEBASE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMEBASE to value 0x10"]
impl crate::Resettable for TIMEBASE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
