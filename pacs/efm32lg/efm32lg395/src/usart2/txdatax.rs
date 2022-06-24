#[doc = "Register `TXDATAX` writer"]
pub struct W(crate::W<TXDATAX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXDATAX_SPEC>;
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
impl From<crate::W<TXDATAX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXDATAX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDATAX` writer - TX Data"]
pub type TXDATAX_W<'a> = crate::FieldWriter<'a, u32, TXDATAX_SPEC, u16, u16, 9, 0>;
#[doc = "Field `UBRXAT` writer - Unblock RX After Transmission"]
pub type UBRXAT_W<'a> = crate::BitWriter<'a, u32, TXDATAX_SPEC, bool, 11>;
#[doc = "Field `TXTRIAT` writer - Set TXTRI After Transmission"]
pub type TXTRIAT_W<'a> = crate::BitWriter<'a, u32, TXDATAX_SPEC, bool, 12>;
#[doc = "Field `TXBREAK` writer - Transmit Data As Break"]
pub type TXBREAK_W<'a> = crate::BitWriter<'a, u32, TXDATAX_SPEC, bool, 13>;
#[doc = "Field `TXDISAT` writer - Clear TXEN After Transmission"]
pub type TXDISAT_W<'a> = crate::BitWriter<'a, u32, TXDATAX_SPEC, bool, 14>;
#[doc = "Field `RXENAT` writer - Enable RX After Transmission"]
pub type RXENAT_W<'a> = crate::BitWriter<'a, u32, TXDATAX_SPEC, bool, 15>;
impl W {
    #[doc = "Bits 0:8 - TX Data"]
    #[inline(always)]
    pub fn txdatax(&mut self) -> TXDATAX_W {
        TXDATAX_W::new(self)
    }
    #[doc = "Bit 11 - Unblock RX After Transmission"]
    #[inline(always)]
    pub fn ubrxat(&mut self) -> UBRXAT_W {
        UBRXAT_W::new(self)
    }
    #[doc = "Bit 12 - Set TXTRI After Transmission"]
    #[inline(always)]
    pub fn txtriat(&mut self) -> TXTRIAT_W {
        TXTRIAT_W::new(self)
    }
    #[doc = "Bit 13 - Transmit Data As Break"]
    #[inline(always)]
    pub fn txbreak(&mut self) -> TXBREAK_W {
        TXBREAK_W::new(self)
    }
    #[doc = "Bit 14 - Clear TXEN After Transmission"]
    #[inline(always)]
    pub fn txdisat(&mut self) -> TXDISAT_W {
        TXDISAT_W::new(self)
    }
    #[doc = "Bit 15 - Enable RX After Transmission"]
    #[inline(always)]
    pub fn rxenat(&mut self) -> RXENAT_W {
        RXENAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX Buffer Data Extended Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdatax](index.html) module"]
pub struct TXDATAX_SPEC;
impl crate::RegisterSpec for TXDATAX_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [txdatax::W](W) writer structure"]
impl crate::Writable for TXDATAX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXDATAX to value 0"]
impl crate::Resettable for TXDATAX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
