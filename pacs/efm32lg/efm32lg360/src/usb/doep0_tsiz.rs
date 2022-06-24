#[doc = "Register `DOEP0_TSIZ` reader"]
pub struct R(crate::R<DOEP0_TSIZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEP0_TSIZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEP0_TSIZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEP0_TSIZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEP0_TSIZ` writer"]
pub struct W(crate::W<DOEP0_TSIZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEP0_TSIZ_SPEC>;
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
impl From<crate::W<DOEP0_TSIZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEP0_TSIZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFERSIZE` reader - Transfer Size"]
pub type XFERSIZE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `XFERSIZE` writer - Transfer Size"]
pub type XFERSIZE_W<'a> = crate::FieldWriter<'a, u32, DOEP0_TSIZ_SPEC, u32, u32, 19, 0>;
#[doc = "Field `PKTCNT` reader - Packet Count"]
pub type PKTCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PKTCNT` writer - Packet Count"]
pub type PKTCNT_W<'a> = crate::FieldWriter<'a, u32, DOEP0_TSIZ_SPEC, u16, u16, 10, 19>;
#[doc = "Receive Data PID / SETUP Packet Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXDPIDSUPCNT_A {
    #[doc = "0: DATA0 PID."]
    DATA0 = 0,
    #[doc = "1: DATA2 PID / 1 Packet."]
    DATA2 = 1,
    #[doc = "2: DATA1 PID / 2 Packets."]
    DATA1 = 2,
    #[doc = "3: MDATA PID / 3 Packets."]
    MDATA = 3,
}
impl From<RXDPIDSUPCNT_A> for u8 {
    #[inline(always)]
    fn from(variant: RXDPIDSUPCNT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RXDPIDSUPCNT` reader - Receive Data PID / SETUP Packet Count"]
pub type RXDPIDSUPCNT_R = crate::FieldReader<u8, RXDPIDSUPCNT_A>;
impl RXDPIDSUPCNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDPIDSUPCNT_A {
        match self.bits {
            0 => RXDPIDSUPCNT_A::DATA0,
            1 => RXDPIDSUPCNT_A::DATA2,
            2 => RXDPIDSUPCNT_A::DATA1,
            3 => RXDPIDSUPCNT_A::MDATA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DATA0`"]
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == RXDPIDSUPCNT_A::DATA0
    }
    #[doc = "Checks if the value of the field is `DATA2`"]
    #[inline(always)]
    pub fn is_data2(&self) -> bool {
        *self == RXDPIDSUPCNT_A::DATA2
    }
    #[doc = "Checks if the value of the field is `DATA1`"]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == RXDPIDSUPCNT_A::DATA1
    }
    #[doc = "Checks if the value of the field is `MDATA`"]
    #[inline(always)]
    pub fn is_mdata(&self) -> bool {
        *self == RXDPIDSUPCNT_A::MDATA
    }
}
impl R {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    pub fn xfersize(&self) -> XFERSIZE_R {
        XFERSIZE_R::new((self.bits & 0x0007_ffff) as u32)
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30 - Receive Data PID / SETUP Packet Count"]
    #[inline(always)]
    pub fn rxdpidsupcnt(&self) -> RXDPIDSUPCNT_R {
        RXDPIDSUPCNT_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    pub fn xfersize(&mut self) -> XFERSIZE_W {
        XFERSIZE_W::new(self)
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W {
        PKTCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device OUT Endpoint x+1 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep0_tsiz](index.html) module"]
pub struct DOEP0_TSIZ_SPEC;
impl crate::RegisterSpec for DOEP0_TSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doep0_tsiz::R](R) reader structure"]
impl crate::Readable for DOEP0_TSIZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doep0_tsiz::W](W) writer structure"]
impl crate::Writable for DOEP0_TSIZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOEP0_TSIZ to value 0"]
impl crate::Resettable for DOEP0_TSIZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
