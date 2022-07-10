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
#[doc = "Field `PHYPEN` reader - USB PHY Pin Enable"]
pub type PHYPEN_R = crate::BitReader<bool>;
#[doc = "Field `PHYPEN` writer - USB PHY Pin Enable"]
pub type PHYPEN_W<'a> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, 0>;
#[doc = "Field `VBUSENPEN` reader - VBUSEN Pin Enable"]
pub type VBUSENPEN_R = crate::BitReader<bool>;
#[doc = "Field `VBUSENPEN` writer - VBUSEN Pin Enable"]
pub type VBUSENPEN_W<'a> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, 1>;
#[doc = "Field `DMPUPEN` reader - DMPU Pin Enable"]
pub type DMPUPEN_R = crate::BitReader<bool>;
#[doc = "Field `DMPUPEN` writer - DMPU Pin Enable"]
pub type DMPUPEN_W<'a> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, 2>;
impl R {
    #[doc = "Bit 0 - USB PHY Pin Enable"]
    #[inline(always)]
    pub fn phypen(&self) -> PHYPEN_R {
        PHYPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VBUSEN Pin Enable"]
    #[inline(always)]
    pub fn vbusenpen(&self) -> VBUSENPEN_R {
        VBUSENPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMPU Pin Enable"]
    #[inline(always)]
    pub fn dmpupen(&self) -> DMPUPEN_R {
        DMPUPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB PHY Pin Enable"]
    #[inline(always)]
    pub fn phypen(&mut self) -> PHYPEN_W {
        PHYPEN_W::new(self)
    }
    #[doc = "Bit 1 - VBUSEN Pin Enable"]
    #[inline(always)]
    pub fn vbusenpen(&mut self) -> VBUSENPEN_W {
        VBUSENPEN_W::new(self)
    }
    #[doc = "Bit 2 - DMPU Pin Enable"]
    #[inline(always)]
    pub fn dmpupen(&mut self) -> DMPUPEN_W {
        DMPUPEN_W::new(self)
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
