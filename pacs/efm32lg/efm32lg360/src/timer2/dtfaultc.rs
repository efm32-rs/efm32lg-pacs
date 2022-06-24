#[doc = "Register `DTFAULTC` writer"]
pub struct W(crate::W<DTFAULTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTFAULTC_SPEC>;
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
impl From<crate::W<DTFAULTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTFAULTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTPRS0FC` writer - DTI PRS0 Fault Clear"]
pub type DTPRS0FC_W<'a> = crate::BitWriter<'a, u32, DTFAULTC_SPEC, bool, 0>;
#[doc = "Field `DTPRS1FC` writer - DTI PRS1 Fault Clear"]
pub type DTPRS1FC_W<'a> = crate::BitWriter<'a, u32, DTFAULTC_SPEC, bool, 1>;
#[doc = "Field `DTDBGFC` writer - DTI Debugger Fault Clear"]
pub type DTDBGFC_W<'a> = crate::BitWriter<'a, u32, DTFAULTC_SPEC, bool, 2>;
#[doc = "Field `TLOCKUPFC` writer - DTI Lockup Fault Clear"]
pub type TLOCKUPFC_W<'a> = crate::BitWriter<'a, u32, DTFAULTC_SPEC, bool, 3>;
impl W {
    #[doc = "Bit 0 - DTI PRS0 Fault Clear"]
    #[inline(always)]
    pub fn dtprs0fc(&mut self) -> DTPRS0FC_W {
        DTPRS0FC_W::new(self)
    }
    #[doc = "Bit 1 - DTI PRS1 Fault Clear"]
    #[inline(always)]
    pub fn dtprs1fc(&mut self) -> DTPRS1FC_W {
        DTPRS1FC_W::new(self)
    }
    #[doc = "Bit 2 - DTI Debugger Fault Clear"]
    #[inline(always)]
    pub fn dtdbgfc(&mut self) -> DTDBGFC_W {
        DTDBGFC_W::new(self)
    }
    #[doc = "Bit 3 - DTI Lockup Fault Clear"]
    #[inline(always)]
    pub fn tlockupfc(&mut self) -> TLOCKUPFC_W {
        TLOCKUPFC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DTI Fault Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtfaultc](index.html) module"]
pub struct DTFAULTC_SPEC;
impl crate::RegisterSpec for DTFAULTC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dtfaultc::W](W) writer structure"]
impl crate::Writable for DTFAULTC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DTFAULTC to value 0"]
impl crate::Resettable for DTFAULTC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
