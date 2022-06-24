#[doc = "Register `EM4CONF` reader"]
pub struct R(crate::R<EM4CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EM4CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EM4CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EM4CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EM4CONF` writer"]
pub struct W(crate::W<EM4CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EM4CONF_SPEC>;
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
impl From<crate::W<EM4CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EM4CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VREGEN` reader - EM4 voltage regulator enable"]
pub type VREGEN_R = crate::BitReader<bool>;
#[doc = "Field `VREGEN` writer - EM4 voltage regulator enable"]
pub type VREGEN_W<'a> = crate::BitWriter<'a, u32, EM4CONF_SPEC, bool, 0>;
#[doc = "Field `BURTCWU` reader - Backup RTC EM4 wakeup enable"]
pub type BURTCWU_R = crate::BitReader<bool>;
#[doc = "Field `BURTCWU` writer - Backup RTC EM4 wakeup enable"]
pub type BURTCWU_W<'a> = crate::BitWriter<'a, u32, EM4CONF_SPEC, bool, 1>;
#[doc = "Select EM4 duty oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OSC_A {
    #[doc = "0: ULFRCO is available."]
    ULFRCO = 0,
    #[doc = "1: LFRCO is available. Can only be set if LFRCO is running before EM4/backup entry."]
    LFRCO = 1,
    #[doc = "2: LFXO is available. Can only be set if LFXO is available before EM4/backup entry."]
    LFXO = 2,
}
impl From<OSC_A> for u8 {
    #[inline(always)]
    fn from(variant: OSC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OSC` reader - Select EM4 duty oscillator"]
pub type OSC_R = crate::FieldReader<u8, OSC_A>;
impl OSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OSC_A> {
        match self.bits {
            0 => Some(OSC_A::ULFRCO),
            1 => Some(OSC_A::LFRCO),
            2 => Some(OSC_A::LFXO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == OSC_A::ULFRCO
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == OSC_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == OSC_A::LFXO
    }
}
#[doc = "Field `OSC` writer - Select EM4 duty oscillator"]
pub type OSC_W<'a> = crate::FieldWriter<'a, u32, EM4CONF_SPEC, u8, OSC_A, 2, 2>;
impl<'a> OSC_W<'a> {
    #[doc = "ULFRCO is available."]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(OSC_A::ULFRCO)
    }
    #[doc = "LFRCO is available. Can only be set if LFRCO is running before EM4/backup entry."]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(OSC_A::LFRCO)
    }
    #[doc = "LFXO is available. Can only be set if LFXO is available before EM4/backup entry."]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(OSC_A::LFXO)
    }
}
#[doc = "Field `BUBODRSTDIS` reader - Disable reset from Backup BOD in EM4"]
pub type BUBODRSTDIS_R = crate::BitReader<bool>;
#[doc = "Field `BUBODRSTDIS` writer - Disable reset from Backup BOD in EM4"]
pub type BUBODRSTDIS_W<'a> = crate::BitWriter<'a, u32, EM4CONF_SPEC, bool, 4>;
#[doc = "Field `LOCKCONF` reader - EM4 configuration lock enable"]
pub type LOCKCONF_R = crate::BitReader<bool>;
#[doc = "Field `LOCKCONF` writer - EM4 configuration lock enable"]
pub type LOCKCONF_W<'a> = crate::BitWriter<'a, u32, EM4CONF_SPEC, bool, 16>;
impl R {
    #[doc = "Bit 0 - EM4 voltage regulator enable"]
    #[inline(always)]
    pub fn vregen(&self) -> VREGEN_R {
        VREGEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Backup RTC EM4 wakeup enable"]
    #[inline(always)]
    pub fn burtcwu(&self) -> BURTCWU_R {
        BURTCWU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Select EM4 duty oscillator"]
    #[inline(always)]
    pub fn osc(&self) -> OSC_R {
        OSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Disable reset from Backup BOD in EM4"]
    #[inline(always)]
    pub fn bubodrstdis(&self) -> BUBODRSTDIS_R {
        BUBODRSTDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - EM4 configuration lock enable"]
    #[inline(always)]
    pub fn lockconf(&self) -> LOCKCONF_R {
        LOCKCONF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EM4 voltage regulator enable"]
    #[inline(always)]
    pub fn vregen(&mut self) -> VREGEN_W {
        VREGEN_W::new(self)
    }
    #[doc = "Bit 1 - Backup RTC EM4 wakeup enable"]
    #[inline(always)]
    pub fn burtcwu(&mut self) -> BURTCWU_W {
        BURTCWU_W::new(self)
    }
    #[doc = "Bits 2:3 - Select EM4 duty oscillator"]
    #[inline(always)]
    pub fn osc(&mut self) -> OSC_W {
        OSC_W::new(self)
    }
    #[doc = "Bit 4 - Disable reset from Backup BOD in EM4"]
    #[inline(always)]
    pub fn bubodrstdis(&mut self) -> BUBODRSTDIS_W {
        BUBODRSTDIS_W::new(self)
    }
    #[doc = "Bit 16 - EM4 configuration lock enable"]
    #[inline(always)]
    pub fn lockconf(&mut self) -> LOCKCONF_W {
        LOCKCONF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Energy mode 4 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [em4conf](index.html) module"]
pub struct EM4CONF_SPEC;
impl crate::RegisterSpec for EM4CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [em4conf::R](R) reader structure"]
impl crate::Readable for EM4CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [em4conf::W](W) writer structure"]
impl crate::Writable for EM4CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EM4CONF to value 0"]
impl crate::Resettable for EM4CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
