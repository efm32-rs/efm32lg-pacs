#[doc = "Register `TXDOUBLEX` writer"]
pub struct W(crate::W<TXDOUBLEX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXDOUBLEX_SPEC>;
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
impl From<crate::W<TXDOUBLEX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXDOUBLEX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDATA0` writer - TX Data"]
pub type TXDATA0_W<'a> = crate::FieldWriter<'a, u32, TXDOUBLEX_SPEC, u16, u16, 9, 0>;
#[doc = "Field `UBRXAT0` writer - Unblock RX After Transmission"]
pub type UBRXAT0_W<'a> = crate::BitWriter<'a, u32, TXDOUBLEX_SPEC, bool, 11>;
#[doc = "Field `TXTRIAT0` writer - Set TXTRI After Transmission"]
pub type TXTRIAT0_W<'a> = crate::BitWriter<'a, u32, TXDOUBLEX_SPEC, bool, 12>;
#[doc = "Field `TXBREAK0` writer - Transmit Data As Break"]
pub type TXBREAK0_W<'a> = crate::BitWriter<'a, u32, TXDOUBLEX_SPEC, bool, 13>;
#[doc = "Field `TXDISAT0` writer - Clear TXEN After Transmission"]
pub type TXDISAT0_W<'a> = crate::BitWriter<'a, u32, TXDOUBLEX_SPEC, bool, 14>;
#[doc = "Field `RXENAT0` writer - Enable RX After Transmission"]
pub type RXENAT0_W<'a> = crate::BitWriter<'a, u32, TXDOUBLEX_SPEC, bool, 15>;
#[doc = "Field `TXDATA1` writer - TX Data"]
pub type TXDATA1_W<'a> = crate::FieldWriter<'a, u32, TXDOUBLEX_SPEC, u16, u16, 9, 16>;
#[doc = "Field `UBRXAT1` writer - Unblock RX After Transmission"]
pub type UBRXAT1_W<'a> = crate::BitWriter<'a, u32, TXDOUBLEX_SPEC, bool, 27>;
#[doc = "Field `TXTRIAT1` writer - Set TXTRI After Transmission"]
pub type TXTRIAT1_W<'a> = crate::BitWriter<'a, u32, TXDOUBLEX_SPEC, bool, 28>;
#[doc = "Field `TXBREAK1` writer - Transmit Data As Break"]
pub type TXBREAK1_W<'a> = crate::BitWriter<'a, u32, TXDOUBLEX_SPEC, bool, 29>;
#[doc = "Field `TXDISAT1` writer - Clear TXEN After Transmission"]
pub type TXDISAT1_W<'a> = crate::BitWriter<'a, u32, TXDOUBLEX_SPEC, bool, 30>;
#[doc = "Field `RXENAT1` writer - Enable RX After Transmission"]
pub type RXENAT1_W<'a> = crate::BitWriter<'a, u32, TXDOUBLEX_SPEC, bool, 31>;
impl W {
    #[doc = "Bits 0:8 - TX Data"]
    #[inline(always)]
    pub fn txdata0(&mut self) -> TXDATA0_W {
        TXDATA0_W::new(self)
    }
    #[doc = "Bit 11 - Unblock RX After Transmission"]
    #[inline(always)]
    pub fn ubrxat0(&mut self) -> UBRXAT0_W {
        UBRXAT0_W::new(self)
    }
    #[doc = "Bit 12 - Set TXTRI After Transmission"]
    #[inline(always)]
    pub fn txtriat0(&mut self) -> TXTRIAT0_W {
        TXTRIAT0_W::new(self)
    }
    #[doc = "Bit 13 - Transmit Data As Break"]
    #[inline(always)]
    pub fn txbreak0(&mut self) -> TXBREAK0_W {
        TXBREAK0_W::new(self)
    }
    #[doc = "Bit 14 - Clear TXEN After Transmission"]
    #[inline(always)]
    pub fn txdisat0(&mut self) -> TXDISAT0_W {
        TXDISAT0_W::new(self)
    }
    #[doc = "Bit 15 - Enable RX After Transmission"]
    #[inline(always)]
    pub fn rxenat0(&mut self) -> RXENAT0_W {
        RXENAT0_W::new(self)
    }
    #[doc = "Bits 16:24 - TX Data"]
    #[inline(always)]
    pub fn txdata1(&mut self) -> TXDATA1_W {
        TXDATA1_W::new(self)
    }
    #[doc = "Bit 27 - Unblock RX After Transmission"]
    #[inline(always)]
    pub fn ubrxat1(&mut self) -> UBRXAT1_W {
        UBRXAT1_W::new(self)
    }
    #[doc = "Bit 28 - Set TXTRI After Transmission"]
    #[inline(always)]
    pub fn txtriat1(&mut self) -> TXTRIAT1_W {
        TXTRIAT1_W::new(self)
    }
    #[doc = "Bit 29 - Transmit Data As Break"]
    #[inline(always)]
    pub fn txbreak1(&mut self) -> TXBREAK1_W {
        TXBREAK1_W::new(self)
    }
    #[doc = "Bit 30 - Clear TXEN After Transmission"]
    #[inline(always)]
    pub fn txdisat1(&mut self) -> TXDISAT1_W {
        TXDISAT1_W::new(self)
    }
    #[doc = "Bit 31 - Enable RX After Transmission"]
    #[inline(always)]
    pub fn rxenat1(&mut self) -> RXENAT1_W {
        RXENAT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX Buffer Double Data Extended Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdoublex](index.html) module"]
pub struct TXDOUBLEX_SPEC;
impl crate::RegisterSpec for TXDOUBLEX_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [txdoublex::W](W) writer structure"]
impl crate::Writable for TXDOUBLEX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXDOUBLEX to value 0"]
impl crate::Resettable for TXDOUBLEX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
