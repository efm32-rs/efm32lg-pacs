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
#[doc = "Field `CH0` reader - "]
pub type CH0_R = crate::BitReader<bool>;
#[doc = "Field `CH0` writer - "]
pub type CH0_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 0>;
#[doc = "Field `CH1` reader - "]
pub type CH1_R = crate::BitReader<bool>;
#[doc = "Field `CH1` writer - "]
pub type CH1_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 1>;
#[doc = "Field `CH2` reader - "]
pub type CH2_R = crate::BitReader<bool>;
#[doc = "Field `CH2` writer - "]
pub type CH2_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 2>;
#[doc = "Field `CH3` reader - "]
pub type CH3_R = crate::BitReader<bool>;
#[doc = "Field `CH3` writer - "]
pub type CH3_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 3>;
#[doc = "Field `CH4` reader - "]
pub type CH4_R = crate::BitReader<bool>;
#[doc = "Field `CH4` writer - "]
pub type CH4_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 4>;
#[doc = "Field `CH5` reader - "]
pub type CH5_R = crate::BitReader<bool>;
#[doc = "Field `CH5` writer - "]
pub type CH5_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 5>;
#[doc = "Field `CH6` reader - "]
pub type CH6_R = crate::BitReader<bool>;
#[doc = "Field `CH6` writer - "]
pub type CH6_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 6>;
#[doc = "Field `CH7` reader - "]
pub type CH7_R = crate::BitReader<bool>;
#[doc = "Field `CH7` writer - "]
pub type CH7_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 7>;
#[doc = "Field `CH8` reader - "]
pub type CH8_R = crate::BitReader<bool>;
#[doc = "Field `CH8` writer - "]
pub type CH8_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 8>;
#[doc = "Field `CH9` reader - "]
pub type CH9_R = crate::BitReader<bool>;
#[doc = "Field `CH9` writer - "]
pub type CH9_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 9>;
#[doc = "Field `CH10` reader - "]
pub type CH10_R = crate::BitReader<bool>;
#[doc = "Field `CH10` writer - "]
pub type CH10_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 10>;
#[doc = "Field `CH11` reader - "]
pub type CH11_R = crate::BitReader<bool>;
#[doc = "Field `CH11` writer - "]
pub type CH11_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 11>;
#[doc = "Field `CH12` reader - "]
pub type CH12_R = crate::BitReader<bool>;
#[doc = "Field `CH12` writer - "]
pub type CH12_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 12>;
#[doc = "Field `CH13` reader - "]
pub type CH13_R = crate::BitReader<bool>;
#[doc = "Field `CH13` writer - "]
pub type CH13_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 13>;
#[doc = "Field `CH14` reader - "]
pub type CH14_R = crate::BitReader<bool>;
#[doc = "Field `CH14` writer - "]
pub type CH14_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 14>;
#[doc = "Field `CH15` reader - "]
pub type CH15_R = crate::BitReader<bool>;
#[doc = "Field `CH15` writer - "]
pub type CH15_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 15>;
#[doc = "Field `SCANCOMPLETE` reader - "]
pub type SCANCOMPLETE_R = crate::BitReader<bool>;
#[doc = "Field `SCANCOMPLETE` writer - "]
pub type SCANCOMPLETE_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 16>;
#[doc = "Field `DEC` reader - "]
pub type DEC_R = crate::BitReader<bool>;
#[doc = "Field `DEC` writer - "]
pub type DEC_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 17>;
#[doc = "Field `DECERR` reader - "]
pub type DECERR_R = crate::BitReader<bool>;
#[doc = "Field `DECERR` writer - "]
pub type DECERR_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 18>;
#[doc = "Field `BUFDATAV` reader - "]
pub type BUFDATAV_R = crate::BitReader<bool>;
#[doc = "Field `BUFDATAV` writer - "]
pub type BUFDATAV_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 19>;
#[doc = "Field `BUFLEVEL` reader - "]
pub type BUFLEVEL_R = crate::BitReader<bool>;
#[doc = "Field `BUFLEVEL` writer - "]
pub type BUFLEVEL_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 20>;
#[doc = "Field `BUFOF` reader - "]
pub type BUFOF_R = crate::BitReader<bool>;
#[doc = "Field `BUFOF` writer - "]
pub type BUFOF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 21>;
#[doc = "Field `CNTOF` reader - "]
pub type CNTOF_R = crate::BitReader<bool>;
#[doc = "Field `CNTOF` writer - "]
pub type CNTOF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 22>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ch4(&self) -> CH4_R {
        CH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ch5(&self) -> CH5_R {
        CH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ch6(&self) -> CH6_R {
        CH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ch7(&self) -> CH7_R {
        CH7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ch8(&self) -> CH8_R {
        CH8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ch9(&self) -> CH9_R {
        CH9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ch10(&self) -> CH10_R {
        CH10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ch11(&self) -> CH11_R {
        CH11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ch12(&self) -> CH12_R {
        CH12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ch13(&self) -> CH13_R {
        CH13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ch14(&self) -> CH14_R {
        CH14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ch15(&self) -> CH15_R {
        CH15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn scancomplete(&self) -> SCANCOMPLETE_R {
        SCANCOMPLETE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn dec(&self) -> DEC_R {
        DEC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn decerr(&self) -> DECERR_R {
        DECERR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn bufdatav(&self) -> BUFDATAV_R {
        BUFDATAV_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn buflevel(&self) -> BUFLEVEL_R {
        BUFLEVEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn bufof(&self) -> BUFOF_R {
        BUFOF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn cntof(&self) -> CNTOF_R {
        CNTOF_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ch0(&mut self) -> CH0_W {
        CH0_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ch1(&mut self) -> CH1_W {
        CH1_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ch2(&mut self) -> CH2_W {
        CH2_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ch3(&mut self) -> CH3_W {
        CH3_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ch4(&mut self) -> CH4_W {
        CH4_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ch5(&mut self) -> CH5_W {
        CH5_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ch6(&mut self) -> CH6_W {
        CH6_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ch7(&mut self) -> CH7_W {
        CH7_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ch8(&mut self) -> CH8_W {
        CH8_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ch9(&mut self) -> CH9_W {
        CH9_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ch10(&mut self) -> CH10_W {
        CH10_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ch11(&mut self) -> CH11_W {
        CH11_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ch12(&mut self) -> CH12_W {
        CH12_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ch13(&mut self) -> CH13_W {
        CH13_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ch14(&mut self) -> CH14_W {
        CH14_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ch15(&mut self) -> CH15_W {
        CH15_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn scancomplete(&mut self) -> SCANCOMPLETE_W {
        SCANCOMPLETE_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn dec(&mut self) -> DEC_W {
        DEC_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn decerr(&mut self) -> DECERR_W {
        DECERR_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn bufdatav(&mut self) -> BUFDATAV_W {
        BUFDATAV_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn buflevel(&mut self) -> BUFLEVEL_W {
        BUFLEVEL_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn bufof(&mut self) -> BUFOF_W {
        BUFOF_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn cntof(&mut self) -> CNTOF_W {
        CNTOF_W::new(self)
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
