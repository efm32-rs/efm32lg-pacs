#[doc = "Register `ROUTE` reader"]
pub struct R(crate::R<ROUTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROUTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROUTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROUTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROUTE` writer"]
pub struct W(crate::W<ROUTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROUTE_SPEC>;
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
impl From<crate::W<ROUTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROUTE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWCLKPEN` reader - Serial Wire Clock Pin Enable"]
pub type SWCLKPEN_R = crate::BitReader<bool>;
#[doc = "Field `SWCLKPEN` writer - Serial Wire Clock Pin Enable"]
pub type SWCLKPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, O>;
#[doc = "Field `SWDIOPEN` reader - Serial Wire Data Pin Enable"]
pub type SWDIOPEN_R = crate::BitReader<bool>;
#[doc = "Field `SWDIOPEN` writer - Serial Wire Data Pin Enable"]
pub type SWDIOPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, O>;
#[doc = "Field `SWOPEN` reader - Serial Wire Viewer Output Pin Enable"]
pub type SWOPEN_R = crate::BitReader<bool>;
#[doc = "Field `SWOPEN` writer - Serial Wire Viewer Output Pin Enable"]
pub type SWOPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, O>;
#[doc = "Field `SWLOCATION` reader - I/O Location"]
pub type SWLOCATION_R = crate::FieldReader<u8, SWLOCATION_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SWLOCATION_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
}
impl From<SWLOCATION_A> for u8 {
    #[inline(always)]
    fn from(variant: SWLOCATION_A) -> Self {
        variant as _
    }
}
impl SWLOCATION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWLOCATION_A {
        match self.bits {
            0 => SWLOCATION_A::LOC0,
            1 => SWLOCATION_A::LOC1,
            2 => SWLOCATION_A::LOC2,
            3 => SWLOCATION_A::LOC3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == SWLOCATION_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == SWLOCATION_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == SWLOCATION_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == SWLOCATION_A::LOC3
    }
}
#[doc = "Field `SWLOCATION` writer - I/O Location"]
pub type SWLOCATION_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ROUTE_SPEC, u8, SWLOCATION_A, 2, O>;
impl<'a, const O: u8> SWLOCATION_W<'a, O> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(SWLOCATION_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(SWLOCATION_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(SWLOCATION_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(SWLOCATION_A::LOC3)
    }
}
#[doc = "Field `TCLKPEN` reader - ETM Trace Clock Pin Enable"]
pub type TCLKPEN_R = crate::BitReader<bool>;
#[doc = "Field `TCLKPEN` writer - ETM Trace Clock Pin Enable"]
pub type TCLKPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, O>;
#[doc = "Field `TD0PEN` reader - ETM Trace Data Pin Enable"]
pub type TD0PEN_R = crate::BitReader<bool>;
#[doc = "Field `TD0PEN` writer - ETM Trace Data Pin Enable"]
pub type TD0PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, O>;
#[doc = "Field `TD1PEN` reader - ETM Trace Data Pin Enable"]
pub type TD1PEN_R = crate::BitReader<bool>;
#[doc = "Field `TD1PEN` writer - ETM Trace Data Pin Enable"]
pub type TD1PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, O>;
#[doc = "Field `TD2PEN` reader - ETM Trace Data Pin Enable"]
pub type TD2PEN_R = crate::BitReader<bool>;
#[doc = "Field `TD2PEN` writer - ETM Trace Data Pin Enable"]
pub type TD2PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, O>;
#[doc = "Field `TD3PEN` reader - ETM Trace Data Pin Enable"]
pub type TD3PEN_R = crate::BitReader<bool>;
#[doc = "Field `TD3PEN` writer - ETM Trace Data Pin Enable"]
pub type TD3PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, O>;
#[doc = "Field `ETMLOCATION` reader - I/O Location"]
pub type ETMLOCATION_R = crate::FieldReader<u8, ETMLOCATION_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETMLOCATION_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
}
impl From<ETMLOCATION_A> for u8 {
    #[inline(always)]
    fn from(variant: ETMLOCATION_A) -> Self {
        variant as _
    }
}
impl ETMLOCATION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETMLOCATION_A {
        match self.bits {
            0 => ETMLOCATION_A::LOC0,
            1 => ETMLOCATION_A::LOC1,
            2 => ETMLOCATION_A::LOC2,
            3 => ETMLOCATION_A::LOC3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == ETMLOCATION_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == ETMLOCATION_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == ETMLOCATION_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == ETMLOCATION_A::LOC3
    }
}
#[doc = "Field `ETMLOCATION` writer - I/O Location"]
pub type ETMLOCATION_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ROUTE_SPEC, u8, ETMLOCATION_A, 2, O>;
impl<'a, const O: u8> ETMLOCATION_W<'a, O> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(ETMLOCATION_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(ETMLOCATION_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(ETMLOCATION_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(ETMLOCATION_A::LOC3)
    }
}
impl R {
    #[doc = "Bit 0 - Serial Wire Clock Pin Enable"]
    #[inline(always)]
    pub fn swclkpen(&self) -> SWCLKPEN_R {
        SWCLKPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Serial Wire Data Pin Enable"]
    #[inline(always)]
    pub fn swdiopen(&self) -> SWDIOPEN_R {
        SWDIOPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Serial Wire Viewer Output Pin Enable"]
    #[inline(always)]
    pub fn swopen(&self) -> SWOPEN_R {
        SWOPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:9 - I/O Location"]
    #[inline(always)]
    pub fn swlocation(&self) -> SWLOCATION_R {
        SWLOCATION_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - ETM Trace Clock Pin Enable"]
    #[inline(always)]
    pub fn tclkpen(&self) -> TCLKPEN_R {
        TCLKPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ETM Trace Data Pin Enable"]
    #[inline(always)]
    pub fn td0pen(&self) -> TD0PEN_R {
        TD0PEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ETM Trace Data Pin Enable"]
    #[inline(always)]
    pub fn td1pen(&self) -> TD1PEN_R {
        TD1PEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ETM Trace Data Pin Enable"]
    #[inline(always)]
    pub fn td2pen(&self) -> TD2PEN_R {
        TD2PEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ETM Trace Data Pin Enable"]
    #[inline(always)]
    pub fn td3pen(&self) -> TD3PEN_R {
        TD3PEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:25 - I/O Location"]
    #[inline(always)]
    pub fn etmlocation(&self) -> ETMLOCATION_R {
        ETMLOCATION_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Serial Wire Clock Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn swclkpen(&mut self) -> SWCLKPEN_W<0> {
        SWCLKPEN_W::new(self)
    }
    #[doc = "Bit 1 - Serial Wire Data Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn swdiopen(&mut self) -> SWDIOPEN_W<1> {
        SWDIOPEN_W::new(self)
    }
    #[doc = "Bit 2 - Serial Wire Viewer Output Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn swopen(&mut self) -> SWOPEN_W<2> {
        SWOPEN_W::new(self)
    }
    #[doc = "Bits 8:9 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn swlocation(&mut self) -> SWLOCATION_W<8> {
        SWLOCATION_W::new(self)
    }
    #[doc = "Bit 12 - ETM Trace Clock Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tclkpen(&mut self) -> TCLKPEN_W<12> {
        TCLKPEN_W::new(self)
    }
    #[doc = "Bit 13 - ETM Trace Data Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn td0pen(&mut self) -> TD0PEN_W<13> {
        TD0PEN_W::new(self)
    }
    #[doc = "Bit 14 - ETM Trace Data Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn td1pen(&mut self) -> TD1PEN_W<14> {
        TD1PEN_W::new(self)
    }
    #[doc = "Bit 15 - ETM Trace Data Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn td2pen(&mut self) -> TD2PEN_W<15> {
        TD2PEN_W::new(self)
    }
    #[doc = "Bit 16 - ETM Trace Data Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn td3pen(&mut self) -> TD3PEN_W<16> {
        TD3PEN_W::new(self)
    }
    #[doc = "Bits 24:25 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn etmlocation(&mut self) -> ETMLOCATION_W<24> {
        ETMLOCATION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O Routing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [route](index.html) module"]
pub struct ROUTE_SPEC;
impl crate::RegisterSpec for ROUTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [route::R](R) reader structure"]
impl crate::Readable for ROUTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [route::W](W) writer structure"]
impl crate::Writable for ROUTE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ROUTE to value 0x03"]
impl crate::Resettable for ROUTE_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
