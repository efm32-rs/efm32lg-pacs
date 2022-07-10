#[doc = "Register `WRITECMD` writer"]
pub struct W(crate::W<WRITECMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRITECMD_SPEC>;
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
impl From<crate::W<WRITECMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRITECMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LADDRIM` writer - Load MSC_ADDRB into ADDR"]
pub type LADDRIM_W<'a> = crate::BitWriter<'a, u32, WRITECMD_SPEC, bool, 0>;
#[doc = "Field `ERASEPAGE` writer - Erase Page"]
pub type ERASEPAGE_W<'a> = crate::BitWriter<'a, u32, WRITECMD_SPEC, bool, 1>;
#[doc = "Field `WRITEEND` writer - End Write Mode"]
pub type WRITEEND_W<'a> = crate::BitWriter<'a, u32, WRITECMD_SPEC, bool, 2>;
#[doc = "Field `WRITEONCE` writer - Word Write-Once Trigger"]
pub type WRITEONCE_W<'a> = crate::BitWriter<'a, u32, WRITECMD_SPEC, bool, 3>;
#[doc = "Field `WRITETRIG` writer - Word Write Sequence Trigger"]
pub type WRITETRIG_W<'a> = crate::BitWriter<'a, u32, WRITECMD_SPEC, bool, 4>;
#[doc = "Field `ERASEABORT` writer - Abort erase sequence"]
pub type ERASEABORT_W<'a> = crate::BitWriter<'a, u32, WRITECMD_SPEC, bool, 5>;
#[doc = "Field `ERASEMAIN0` writer - Mass erase region 0"]
pub type ERASEMAIN0_W<'a> = crate::BitWriter<'a, u32, WRITECMD_SPEC, bool, 8>;
#[doc = "Field `CLEARWDATA` writer - Clear WDATA state"]
pub type CLEARWDATA_W<'a> = crate::BitWriter<'a, u32, WRITECMD_SPEC, bool, 12>;
impl W {
    #[doc = "Bit 0 - Load MSC_ADDRB into ADDR"]
    #[inline(always)]
    pub fn laddrim(&mut self) -> LADDRIM_W {
        LADDRIM_W::new(self)
    }
    #[doc = "Bit 1 - Erase Page"]
    #[inline(always)]
    pub fn erasepage(&mut self) -> ERASEPAGE_W {
        ERASEPAGE_W::new(self)
    }
    #[doc = "Bit 2 - End Write Mode"]
    #[inline(always)]
    pub fn writeend(&mut self) -> WRITEEND_W {
        WRITEEND_W::new(self)
    }
    #[doc = "Bit 3 - Word Write-Once Trigger"]
    #[inline(always)]
    pub fn writeonce(&mut self) -> WRITEONCE_W {
        WRITEONCE_W::new(self)
    }
    #[doc = "Bit 4 - Word Write Sequence Trigger"]
    #[inline(always)]
    pub fn writetrig(&mut self) -> WRITETRIG_W {
        WRITETRIG_W::new(self)
    }
    #[doc = "Bit 5 - Abort erase sequence"]
    #[inline(always)]
    pub fn eraseabort(&mut self) -> ERASEABORT_W {
        ERASEABORT_W::new(self)
    }
    #[doc = "Bit 8 - Mass erase region 0"]
    #[inline(always)]
    pub fn erasemain0(&mut self) -> ERASEMAIN0_W {
        ERASEMAIN0_W::new(self)
    }
    #[doc = "Bit 12 - Clear WDATA state"]
    #[inline(always)]
    pub fn clearwdata(&mut self) -> CLEARWDATA_W {
        CLEARWDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write Command Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [writecmd](index.html) module"]
pub struct WRITECMD_SPEC;
impl crate::RegisterSpec for WRITECMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [writecmd::W](W) writer structure"]
impl crate::Writable for WRITECMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WRITECMD to value 0"]
impl crate::Resettable for WRITECMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
