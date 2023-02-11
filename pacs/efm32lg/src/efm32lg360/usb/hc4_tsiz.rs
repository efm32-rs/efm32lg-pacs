#[doc = "Register `HC4_TSIZ` reader"]
pub struct R(crate::R<HC4_TSIZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HC4_TSIZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HC4_TSIZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HC4_TSIZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HC4_TSIZ` writer"]
pub struct W(crate::W<HC4_TSIZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HC4_TSIZ_SPEC>;
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
impl From<crate::W<HC4_TSIZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HC4_TSIZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFERSIZE` reader - Transfer Size"]
pub type XFERSIZE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `XFERSIZE` writer - Transfer Size"]
pub type XFERSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HC4_TSIZ_SPEC, u32, u32, 19, O>;
#[doc = "Field `PKTCNT` reader - Packet Count"]
pub type PKTCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PKTCNT` writer - Packet Count"]
pub type PKTCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HC4_TSIZ_SPEC, u16, u16, 10, O>;
#[doc = "Field `PID` reader - Packet ID"]
pub type PID_R = crate::FieldReader<u8, PID_A>;
#[doc = "Packet ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PID_A {
    #[doc = "0: DATA0 PID."]
    DATA0 = 0,
    #[doc = "1: DATA2 PID."]
    DATA2 = 1,
    #[doc = "2: DATA1 PID."]
    DATA1 = 2,
    #[doc = "3: MDATA (non-control) / SETUP (control) PID."]
    MDATA = 3,
}
impl From<PID_A> for u8 {
    #[inline(always)]
    fn from(variant: PID_A) -> Self {
        variant as _
    }
}
impl PID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID_A {
        match self.bits {
            0 => PID_A::DATA0,
            1 => PID_A::DATA2,
            2 => PID_A::DATA1,
            3 => PID_A::MDATA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DATA0`"]
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == PID_A::DATA0
    }
    #[doc = "Checks if the value of the field is `DATA2`"]
    #[inline(always)]
    pub fn is_data2(&self) -> bool {
        *self == PID_A::DATA2
    }
    #[doc = "Checks if the value of the field is `DATA1`"]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == PID_A::DATA1
    }
    #[doc = "Checks if the value of the field is `MDATA`"]
    #[inline(always)]
    pub fn is_mdata(&self) -> bool {
        *self == PID_A::MDATA
    }
}
#[doc = "Field `PID` writer - Packet ID"]
pub type PID_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, HC4_TSIZ_SPEC, u8, PID_A, 2, O>;
impl<'a, const O: u8> PID_W<'a, O> {
    #[doc = "DATA0 PID."]
    #[inline(always)]
    pub fn data0(self) -> &'a mut W {
        self.variant(PID_A::DATA0)
    }
    #[doc = "DATA2 PID."]
    #[inline(always)]
    pub fn data2(self) -> &'a mut W {
        self.variant(PID_A::DATA2)
    }
    #[doc = "DATA1 PID."]
    #[inline(always)]
    pub fn data1(self) -> &'a mut W {
        self.variant(PID_A::DATA1)
    }
    #[doc = "MDATA (non-control) / SETUP (control) PID."]
    #[inline(always)]
    pub fn mdata(self) -> &'a mut W {
        self.variant(PID_A::MDATA)
    }
}
impl R {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    pub fn xfersize(&self) -> XFERSIZE_R {
        XFERSIZE_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30 - Packet ID"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    #[must_use]
    pub fn xfersize(&mut self) -> XFERSIZE_W<0> {
        XFERSIZE_W::new(self)
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt(&mut self) -> PKTCNT_W<19> {
        PKTCNT_W::new(self)
    }
    #[doc = "Bits 29:30 - Packet ID"]
    #[inline(always)]
    #[must_use]
    pub fn pid(&mut self) -> PID_W<29> {
        PID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Channel x Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc4_tsiz](index.html) module"]
pub struct HC4_TSIZ_SPEC;
impl crate::RegisterSpec for HC4_TSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hc4_tsiz::R](R) reader structure"]
impl crate::Readable for HC4_TSIZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hc4_tsiz::W](W) writer structure"]
impl crate::Writable for HC4_TSIZ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HC4_TSIZ to value 0"]
impl crate::Resettable for HC4_TSIZ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
