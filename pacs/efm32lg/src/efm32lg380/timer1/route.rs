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
#[doc = "Field `CC0PEN` reader - CC Channel 0 Pin Enable"]
pub type CC0PEN_R = crate::BitReader<bool>;
#[doc = "Field `CC0PEN` writer - CC Channel 0 Pin Enable"]
pub type CC0PEN_W<'a> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, 0>;
#[doc = "Field `CC1PEN` reader - CC Channel 1 Pin Enable"]
pub type CC1PEN_R = crate::BitReader<bool>;
#[doc = "Field `CC1PEN` writer - CC Channel 1 Pin Enable"]
pub type CC1PEN_W<'a> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, 1>;
#[doc = "Field `CC2PEN` reader - CC Channel 2 Pin Enable"]
pub type CC2PEN_R = crate::BitReader<bool>;
#[doc = "Field `CC2PEN` writer - CC Channel 2 Pin Enable"]
pub type CC2PEN_W<'a> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, 2>;
#[doc = "Field `CDTI0PEN` reader - CC Channel 0 Complementary Dead-Time Insertion Pin Enable"]
pub type CDTI0PEN_R = crate::BitReader<bool>;
#[doc = "Field `CDTI0PEN` writer - CC Channel 0 Complementary Dead-Time Insertion Pin Enable"]
pub type CDTI0PEN_W<'a> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, 8>;
#[doc = "Field `CDTI1PEN` reader - CC Channel 1 Complementary Dead-Time Insertion Pin Enable"]
pub type CDTI1PEN_R = crate::BitReader<bool>;
#[doc = "Field `CDTI1PEN` writer - CC Channel 1 Complementary Dead-Time Insertion Pin Enable"]
pub type CDTI1PEN_W<'a> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, 9>;
#[doc = "Field `CDTI2PEN` reader - CC Channel 2 Complementary Dead-Time Insertion Pin Enable"]
pub type CDTI2PEN_R = crate::BitReader<bool>;
#[doc = "Field `CDTI2PEN` writer - CC Channel 2 Complementary Dead-Time Insertion Pin Enable"]
pub type CDTI2PEN_W<'a> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, 10>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LOCATION_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
    #[doc = "4: Location 4"]
    LOC4 = 4,
    #[doc = "5: Location 5"]
    LOC5 = 5,
}
impl From<LOCATION_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCATION_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LOCATION` reader - I/O Location"]
pub type LOCATION_R = crate::FieldReader<u8, LOCATION_A>;
impl LOCATION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOCATION_A> {
        match self.bits {
            0 => Some(LOCATION_A::LOC0),
            1 => Some(LOCATION_A::LOC1),
            2 => Some(LOCATION_A::LOC2),
            3 => Some(LOCATION_A::LOC3),
            4 => Some(LOCATION_A::LOC4),
            5 => Some(LOCATION_A::LOC5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == LOCATION_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == LOCATION_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == LOCATION_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == LOCATION_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == LOCATION_A::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == LOCATION_A::LOC5
    }
}
#[doc = "Field `LOCATION` writer - I/O Location"]
pub type LOCATION_W<'a> = crate::FieldWriter<'a, u32, ROUTE_SPEC, u8, LOCATION_A, 3, 16>;
impl<'a> LOCATION_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(LOCATION_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(LOCATION_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(LOCATION_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(LOCATION_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(LOCATION_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W {
        self.variant(LOCATION_A::LOC5)
    }
}
impl R {
    #[doc = "Bit 0 - CC Channel 0 Pin Enable"]
    #[inline(always)]
    pub fn cc0pen(&self) -> CC0PEN_R {
        CC0PEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CC Channel 1 Pin Enable"]
    #[inline(always)]
    pub fn cc1pen(&self) -> CC1PEN_R {
        CC1PEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CC Channel 2 Pin Enable"]
    #[inline(always)]
    pub fn cc2pen(&self) -> CC2PEN_R {
        CC2PEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - CC Channel 0 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    pub fn cdti0pen(&self) -> CDTI0PEN_R {
        CDTI0PEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CC Channel 1 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    pub fn cdti1pen(&self) -> CDTI1PEN_R {
        CDTI1PEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CC Channel 2 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    pub fn cdti2pen(&self) -> CDTI2PEN_R {
        CDTI2PEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:18 - I/O Location"]
    #[inline(always)]
    pub fn location(&self) -> LOCATION_R {
        LOCATION_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CC Channel 0 Pin Enable"]
    #[inline(always)]
    pub fn cc0pen(&mut self) -> CC0PEN_W {
        CC0PEN_W::new(self)
    }
    #[doc = "Bit 1 - CC Channel 1 Pin Enable"]
    #[inline(always)]
    pub fn cc1pen(&mut self) -> CC1PEN_W {
        CC1PEN_W::new(self)
    }
    #[doc = "Bit 2 - CC Channel 2 Pin Enable"]
    #[inline(always)]
    pub fn cc2pen(&mut self) -> CC2PEN_W {
        CC2PEN_W::new(self)
    }
    #[doc = "Bit 8 - CC Channel 0 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    pub fn cdti0pen(&mut self) -> CDTI0PEN_W {
        CDTI0PEN_W::new(self)
    }
    #[doc = "Bit 9 - CC Channel 1 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    pub fn cdti1pen(&mut self) -> CDTI1PEN_W {
        CDTI1PEN_W::new(self)
    }
    #[doc = "Bit 10 - CC Channel 2 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    pub fn cdti2pen(&mut self) -> CDTI2PEN_W {
        CDTI2PEN_W::new(self)
    }
    #[doc = "Bits 16:18 - I/O Location"]
    #[inline(always)]
    pub fn location(&mut self) -> LOCATION_W {
        LOCATION_W::new(self)
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
}
#[doc = "`reset()` method sets ROUTE to value 0"]
impl crate::Resettable for ROUTE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
