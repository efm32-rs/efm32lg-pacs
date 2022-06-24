#[doc = "Register `IFS` writer"]
pub struct W(crate::W<IFS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFS_SPEC>;
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
impl From<crate::W<IFS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP0` writer - Set Compare Match 0 Interrupt Flag"]
pub type COMP0_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 0>;
#[doc = "Field `COMP1` writer - Set Compare Match 1 Interrupt Flag"]
pub type COMP1_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 1>;
#[doc = "Field `UF` writer - Set Underflow Interrupt Flag"]
pub type UF_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 2>;
#[doc = "Field `REP0` writer - Set Repeat Counter 0 Interrupt Flag"]
pub type REP0_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 3>;
#[doc = "Field `REP1` writer - Set Repeat Counter 1 Interrupt Flag"]
pub type REP1_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 4>;
impl W {
    #[doc = "Bit 0 - Set Compare Match 0 Interrupt Flag"]
    #[inline(always)]
    pub fn comp0(&mut self) -> COMP0_W {
        COMP0_W::new(self)
    }
    #[doc = "Bit 1 - Set Compare Match 1 Interrupt Flag"]
    #[inline(always)]
    pub fn comp1(&mut self) -> COMP1_W {
        COMP1_W::new(self)
    }
    #[doc = "Bit 2 - Set Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn uf(&mut self) -> UF_W {
        UF_W::new(self)
    }
    #[doc = "Bit 3 - Set Repeat Counter 0 Interrupt Flag"]
    #[inline(always)]
    pub fn rep0(&mut self) -> REP0_W {
        REP0_W::new(self)
    }
    #[doc = "Bit 4 - Set Repeat Counter 1 Interrupt Flag"]
    #[inline(always)]
    pub fn rep1(&mut self) -> REP1_W {
        REP1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifs](index.html) module"]
pub struct IFS_SPEC;
impl crate::RegisterSpec for IFS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ifs::W](W) writer structure"]
impl crate::Writable for IFS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
