#[doc = "Register `CH14_INTERACT` reader"]
pub struct R(crate::R<CH14_INTERACT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH14_INTERACT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH14_INTERACT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH14_INTERACT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH14_INTERACT` writer"]
pub struct W(crate::W<CH14_INTERACT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH14_INTERACT_SPEC>;
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
impl From<crate::W<CH14_INTERACT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH14_INTERACT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACMPTHRES` reader - Set ACMP threshold"]
pub type ACMPTHRES_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ACMPTHRES` writer - Set ACMP threshold"]
pub type ACMPTHRES_W<'a> = crate::FieldWriter<'a, u32, CH14_INTERACT_SPEC, u16, u16, 12, 0>;
#[doc = "Field `SAMPLE` reader - Select sample mode"]
pub type SAMPLE_R = crate::BitReader<bool>;
#[doc = "Field `SAMPLE` writer - Select sample mode"]
pub type SAMPLE_W<'a> = crate::BitWriter<'a, u32, CH14_INTERACT_SPEC, bool, 12>;
#[doc = "Enable interrupt generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SETIF_A {
    #[doc = "0: No interrupt is generated"]
    NONE = 0,
    #[doc = "1: Set interrupt flag if the sensor triggers."]
    LEVEL = 1,
    #[doc = "2: Set interrupt flag on positive edge on the sensor state"]
    POSEDGE = 2,
    #[doc = "3: Set interrupt flag on negative edge on the sensor state"]
    NEGEDGE = 3,
}
impl From<SETIF_A> for u8 {
    #[inline(always)]
    fn from(variant: SETIF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SETIF` reader - Enable interrupt generation"]
pub type SETIF_R = crate::FieldReader<u8, SETIF_A>;
impl SETIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SETIF_A {
        match self.bits {
            0 => SETIF_A::NONE,
            1 => SETIF_A::LEVEL,
            2 => SETIF_A::POSEDGE,
            3 => SETIF_A::NEGEDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SETIF_A::NONE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == SETIF_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `POSEDGE`"]
    #[inline(always)]
    pub fn is_posedge(&self) -> bool {
        *self == SETIF_A::POSEDGE
    }
    #[doc = "Checks if the value of the field is `NEGEDGE`"]
    #[inline(always)]
    pub fn is_negedge(&self) -> bool {
        *self == SETIF_A::NEGEDGE
    }
}
#[doc = "Field `SETIF` writer - Enable interrupt generation"]
pub type SETIF_W<'a> = crate::FieldWriterSafe<'a, u32, CH14_INTERACT_SPEC, u8, SETIF_A, 2, 13>;
impl<'a> SETIF_W<'a> {
    #[doc = "No interrupt is generated"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SETIF_A::NONE)
    }
    #[doc = "Set interrupt flag if the sensor triggers."]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(SETIF_A::LEVEL)
    }
    #[doc = "Set interrupt flag on positive edge on the sensor state"]
    #[inline(always)]
    pub fn posedge(self) -> &'a mut W {
        self.variant(SETIF_A::POSEDGE)
    }
    #[doc = "Set interrupt flag on negative edge on the sensor state"]
    #[inline(always)]
    pub fn negedge(self) -> &'a mut W {
        self.variant(SETIF_A::NEGEDGE)
    }
}
#[doc = "Set GPIO mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXMODE_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Push Pull, GPIO is driven high"]
    HIGH = 1,
    #[doc = "2: Push Pull, GPIO is driven low"]
    LOW = 2,
    #[doc = "3: DAC output"]
    DACOUT = 3,
}
impl From<EXMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: EXMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXMODE` reader - Set GPIO mode"]
pub type EXMODE_R = crate::FieldReader<u8, EXMODE_A>;
impl EXMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXMODE_A {
        match self.bits {
            0 => EXMODE_A::DISABLE,
            1 => EXMODE_A::HIGH,
            2 => EXMODE_A::LOW,
            3 => EXMODE_A::DACOUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EXMODE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == EXMODE_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == EXMODE_A::LOW
    }
    #[doc = "Checks if the value of the field is `DACOUT`"]
    #[inline(always)]
    pub fn is_dacout(&self) -> bool {
        *self == EXMODE_A::DACOUT
    }
}
#[doc = "Field `EXMODE` writer - Set GPIO mode"]
pub type EXMODE_W<'a> = crate::FieldWriterSafe<'a, u32, CH14_INTERACT_SPEC, u8, EXMODE_A, 2, 15>;
impl<'a> EXMODE_W<'a> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EXMODE_A::DISABLE)
    }
    #[doc = "Push Pull, GPIO is driven high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(EXMODE_A::HIGH)
    }
    #[doc = "Push Pull, GPIO is driven low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(EXMODE_A::LOW)
    }
    #[doc = "DAC output"]
    #[inline(always)]
    pub fn dacout(self) -> &'a mut W {
        self.variant(EXMODE_A::DACOUT)
    }
}
#[doc = "Field `EXCLK` reader - Select clock used for excitation timing"]
pub type EXCLK_R = crate::BitReader<bool>;
#[doc = "Field `EXCLK` writer - Select clock used for excitation timing"]
pub type EXCLK_W<'a> = crate::BitWriter<'a, u32, CH14_INTERACT_SPEC, bool, 17>;
#[doc = "Field `SAMPLECLK` reader - Select clock used for timing of sample delay"]
pub type SAMPLECLK_R = crate::BitReader<bool>;
#[doc = "Field `SAMPLECLK` writer - Select clock used for timing of sample delay"]
pub type SAMPLECLK_W<'a> = crate::BitWriter<'a, u32, CH14_INTERACT_SPEC, bool, 18>;
#[doc = "Field `ALTEX` reader - Use alternative excite pin"]
pub type ALTEX_R = crate::BitReader<bool>;
#[doc = "Field `ALTEX` writer - Use alternative excite pin"]
pub type ALTEX_W<'a> = crate::BitWriter<'a, u32, CH14_INTERACT_SPEC, bool, 19>;
impl R {
    #[doc = "Bits 0:11 - Set ACMP threshold"]
    #[inline(always)]
    pub fn acmpthres(&self) -> ACMPTHRES_R {
        ACMPTHRES_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - Select sample mode"]
    #[inline(always)]
    pub fn sample(&self) -> SAMPLE_R {
        SAMPLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Enable interrupt generation"]
    #[inline(always)]
    pub fn setif(&self) -> SETIF_R {
        SETIF_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 15:16 - Set GPIO mode"]
    #[inline(always)]
    pub fn exmode(&self) -> EXMODE_R {
        EXMODE_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17 - Select clock used for excitation timing"]
    #[inline(always)]
    pub fn exclk(&self) -> EXCLK_R {
        EXCLK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Select clock used for timing of sample delay"]
    #[inline(always)]
    pub fn sampleclk(&self) -> SAMPLECLK_R {
        SAMPLECLK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Use alternative excite pin"]
    #[inline(always)]
    pub fn altex(&self) -> ALTEX_R {
        ALTEX_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Set ACMP threshold"]
    #[inline(always)]
    pub fn acmpthres(&mut self) -> ACMPTHRES_W {
        ACMPTHRES_W::new(self)
    }
    #[doc = "Bit 12 - Select sample mode"]
    #[inline(always)]
    pub fn sample(&mut self) -> SAMPLE_W {
        SAMPLE_W::new(self)
    }
    #[doc = "Bits 13:14 - Enable interrupt generation"]
    #[inline(always)]
    pub fn setif(&mut self) -> SETIF_W {
        SETIF_W::new(self)
    }
    #[doc = "Bits 15:16 - Set GPIO mode"]
    #[inline(always)]
    pub fn exmode(&mut self) -> EXMODE_W {
        EXMODE_W::new(self)
    }
    #[doc = "Bit 17 - Select clock used for excitation timing"]
    #[inline(always)]
    pub fn exclk(&mut self) -> EXCLK_W {
        EXCLK_W::new(self)
    }
    #[doc = "Bit 18 - Select clock used for timing of sample delay"]
    #[inline(always)]
    pub fn sampleclk(&mut self) -> SAMPLECLK_W {
        SAMPLECLK_W::new(self)
    }
    #[doc = "Bit 19 - Use alternative excite pin"]
    #[inline(always)]
    pub fn altex(&mut self) -> ALTEX_W {
        ALTEX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scan configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch14_interact](index.html) module"]
pub struct CH14_INTERACT_SPEC;
impl crate::RegisterSpec for CH14_INTERACT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch14_interact::R](R) reader structure"]
impl crate::Readable for CH14_INTERACT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch14_interact::W](W) writer structure"]
impl crate::Writable for CH14_INTERACT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH14_INTERACT to value 0"]
impl crate::Resettable for CH14_INTERACT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
