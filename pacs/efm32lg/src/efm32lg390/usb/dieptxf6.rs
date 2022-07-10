#[doc = "Register `DIEPTXF6` reader"]
pub struct R(crate::R<DIEPTXF6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPTXF6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPTXF6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPTXF6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPTXF6` writer"]
pub struct W(crate::W<DIEPTXF6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPTXF6_SPEC>;
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
impl From<crate::W<DIEPTXF6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPTXF6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INEPNTXFSTADDR` reader - IN Endpoint FIFO 6 Transmit RAM Start Address"]
pub type INEPNTXFSTADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INEPNTXFSTADDR` writer - IN Endpoint FIFO 6 Transmit RAM Start Address"]
pub type INEPNTXFSTADDR_W<'a> = crate::FieldWriter<'a, u32, DIEPTXF6_SPEC, u16, u16, 12, 0>;
#[doc = "Field `INEPNTXFDEP` reader - IN Endpoint TxFIFO Depth"]
pub type INEPNTXFDEP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INEPNTXFDEP` writer - IN Endpoint TxFIFO Depth"]
pub type INEPNTXFDEP_W<'a> = crate::FieldWriter<'a, u32, DIEPTXF6_SPEC, u16, u16, 10, 16>;
impl R {
    #[doc = "Bits 0:11 - IN Endpoint FIFO 6 Transmit RAM Start Address"]
    #[inline(always)]
    pub fn inepntxfstaddr(&self) -> INEPNTXFSTADDR_R {
        INEPNTXFSTADDR_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:25 - IN Endpoint TxFIFO Depth"]
    #[inline(always)]
    pub fn inepntxfdep(&self) -> INEPNTXFDEP_R {
        INEPNTXFDEP_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - IN Endpoint FIFO 6 Transmit RAM Start Address"]
    #[inline(always)]
    pub fn inepntxfstaddr(&mut self) -> INEPNTXFSTADDR_W {
        INEPNTXFSTADDR_W::new(self)
    }
    #[doc = "Bits 16:25 - IN Endpoint TxFIFO Depth"]
    #[inline(always)]
    pub fn inepntxfdep(&mut self) -> INEPNTXFDEP_W {
        INEPNTXFDEP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device IN Endpoint Transmit FIFO 6 Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptxf6](index.html) module"]
pub struct DIEPTXF6_SPEC;
impl crate::RegisterSpec for DIEPTXF6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dieptxf6::R](R) reader structure"]
impl crate::Readable for DIEPTXF6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dieptxf6::W](W) writer structure"]
impl crate::Writable for DIEPTXF6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIEPTXF6 to value 0x0200_0e00"]
impl crate::Resettable for DIEPTXF6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_0e00
    }
}
