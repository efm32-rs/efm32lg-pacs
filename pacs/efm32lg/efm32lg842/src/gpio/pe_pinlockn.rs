#[doc = "Register `PE_PINLOCKN` reader"]
pub struct R(crate::R<PE_PINLOCKN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PE_PINLOCKN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PE_PINLOCKN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PE_PINLOCKN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PE_PINLOCKN` writer"]
pub struct W(crate::W<PE_PINLOCKN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PE_PINLOCKN_SPEC>;
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
impl From<crate::W<PE_PINLOCKN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PE_PINLOCKN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PINLOCKN` reader - Unlocked Pins"]
pub type PINLOCKN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PINLOCKN` writer - Unlocked Pins"]
pub type PINLOCKN_W<'a> = crate::FieldWriter<'a, u32, PE_PINLOCKN_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Unlocked Pins"]
    #[inline(always)]
    pub fn pinlockn(&self) -> PINLOCKN_R {
        PINLOCKN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Unlocked Pins"]
    #[inline(always)]
    pub fn pinlockn(&mut self) -> PINLOCKN_W {
        PINLOCKN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Unlocked Pins Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pe_pinlockn](index.html) module"]
pub struct PE_PINLOCKN_SPEC;
impl crate::RegisterSpec for PE_PINLOCKN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pe_pinlockn::R](R) reader structure"]
impl crate::Readable for PE_PINLOCKN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pe_pinlockn::W](W) writer structure"]
impl crate::Writable for PE_PINLOCKN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PE_PINLOCKN to value 0xffff"]
impl crate::Resettable for PE_PINLOCKN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
