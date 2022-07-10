#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "BURTC Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: The BURTC is disabled."]
    DISABLE = 0,
    #[doc = "1: The BURTC is in normal operating mode, operating in EM0-EM2. Oscillators must be enabled in CMU for use."]
    EM2EN = 1,
    #[doc = "2: The BURTC is enabled in EM0-EM3. Will prevent CMU from disabling used oscillators all the way down to EM3."]
    EM3EN = 2,
    #[doc = "3: The BURTC is enabled in EM0-EM4. Will prevent CMU from disabling used oscillators all the way down to EM4."]
    EM4EN = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - BURTC Enable"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::DISABLE,
            1 => MODE_A::EM2EN,
            2 => MODE_A::EM3EN,
            3 => MODE_A::EM4EN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MODE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `EM2EN`"]
    #[inline(always)]
    pub fn is_em2en(&self) -> bool {
        *self == MODE_A::EM2EN
    }
    #[doc = "Checks if the value of the field is `EM3EN`"]
    #[inline(always)]
    pub fn is_em3en(&self) -> bool {
        *self == MODE_A::EM3EN
    }
    #[doc = "Checks if the value of the field is `EM4EN`"]
    #[inline(always)]
    pub fn is_em4en(&self) -> bool {
        *self == MODE_A::EM4EN
    }
}
#[doc = "Field `MODE` writer - BURTC Enable"]
pub type MODE_W<'a> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, MODE_A, 2, 0>;
impl<'a> MODE_W<'a> {
    #[doc = "The BURTC is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MODE_A::DISABLE)
    }
    #[doc = "The BURTC is in normal operating mode, operating in EM0-EM2. Oscillators must be enabled in CMU for use."]
    #[inline(always)]
    pub fn em2en(self) -> &'a mut W {
        self.variant(MODE_A::EM2EN)
    }
    #[doc = "The BURTC is enabled in EM0-EM3. Will prevent CMU from disabling used oscillators all the way down to EM3."]
    #[inline(always)]
    pub fn em3en(self) -> &'a mut W {
        self.variant(MODE_A::EM3EN)
    }
    #[doc = "The BURTC is enabled in EM0-EM4. Will prevent CMU from disabling used oscillators all the way down to EM4."]
    #[inline(always)]
    pub fn em4en(self) -> &'a mut W {
        self.variant(MODE_A::EM4EN)
    }
}
#[doc = "Field `DEBUGRUN` reader - Debug Mode Run Enable"]
pub type DEBUGRUN_R = crate::BitReader<bool>;
#[doc = "Field `DEBUGRUN` writer - Debug Mode Run Enable"]
pub type DEBUGRUN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 2>;
#[doc = "Field `RSTEN` reader - Enable BURTC reset"]
pub type RSTEN_R = crate::BitReader<bool>;
#[doc = "Field `RSTEN` writer - Enable BURTC reset"]
pub type RSTEN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 3>;
#[doc = "Field `COMP0TOP` reader - Compare clear enable"]
pub type COMP0TOP_R = crate::BitReader<bool>;
#[doc = "Field `COMP0TOP` writer - Compare clear enable"]
pub type COMP0TOP_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 4>;
#[doc = "Low power mode compare configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPCOMP_A {
    #[doc = "0: Do not ignore any bits for compare match evaluation."]
    IGN0LSB = 0,
    #[doc = "1: The LSB of the counter is ignored for compare match evaluation."]
    IGN1LSB = 1,
    #[doc = "2: The two LSBs of the counter are ignored for compare match evaluation."]
    IGN2LSB = 2,
    #[doc = "3: The three LSBs of the counter are ignored for compare match evaluation."]
    IGN3LSB = 3,
    #[doc = "4: The four LSBs of the counter are ignored for compare match evaluation."]
    IGN4LSB = 4,
    #[doc = "5: The five LSBs of the counter are ignored for compare match evaluation."]
    IGN5LSB = 5,
    #[doc = "6: The six LSBs of the counter are ignored for compare match evaluation."]
    IGN6LSB = 6,
    #[doc = "7: The seven LSBs of the counter are ignored for compare match evaluation."]
    IGN7LSB = 7,
}
impl From<LPCOMP_A> for u8 {
    #[inline(always)]
    fn from(variant: LPCOMP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LPCOMP` reader - Low power mode compare configuration"]
pub type LPCOMP_R = crate::FieldReader<u8, LPCOMP_A>;
impl LPCOMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPCOMP_A {
        match self.bits {
            0 => LPCOMP_A::IGN0LSB,
            1 => LPCOMP_A::IGN1LSB,
            2 => LPCOMP_A::IGN2LSB,
            3 => LPCOMP_A::IGN3LSB,
            4 => LPCOMP_A::IGN4LSB,
            5 => LPCOMP_A::IGN5LSB,
            6 => LPCOMP_A::IGN6LSB,
            7 => LPCOMP_A::IGN7LSB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IGN0LSB`"]
    #[inline(always)]
    pub fn is_ign0lsb(&self) -> bool {
        *self == LPCOMP_A::IGN0LSB
    }
    #[doc = "Checks if the value of the field is `IGN1LSB`"]
    #[inline(always)]
    pub fn is_ign1lsb(&self) -> bool {
        *self == LPCOMP_A::IGN1LSB
    }
    #[doc = "Checks if the value of the field is `IGN2LSB`"]
    #[inline(always)]
    pub fn is_ign2lsb(&self) -> bool {
        *self == LPCOMP_A::IGN2LSB
    }
    #[doc = "Checks if the value of the field is `IGN3LSB`"]
    #[inline(always)]
    pub fn is_ign3lsb(&self) -> bool {
        *self == LPCOMP_A::IGN3LSB
    }
    #[doc = "Checks if the value of the field is `IGN4LSB`"]
    #[inline(always)]
    pub fn is_ign4lsb(&self) -> bool {
        *self == LPCOMP_A::IGN4LSB
    }
    #[doc = "Checks if the value of the field is `IGN5LSB`"]
    #[inline(always)]
    pub fn is_ign5lsb(&self) -> bool {
        *self == LPCOMP_A::IGN5LSB
    }
    #[doc = "Checks if the value of the field is `IGN6LSB`"]
    #[inline(always)]
    pub fn is_ign6lsb(&self) -> bool {
        *self == LPCOMP_A::IGN6LSB
    }
    #[doc = "Checks if the value of the field is `IGN7LSB`"]
    #[inline(always)]
    pub fn is_ign7lsb(&self) -> bool {
        *self == LPCOMP_A::IGN7LSB
    }
}
#[doc = "Field `LPCOMP` writer - Low power mode compare configuration"]
pub type LPCOMP_W<'a> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, LPCOMP_A, 3, 5>;
impl<'a> LPCOMP_W<'a> {
    #[doc = "Do not ignore any bits for compare match evaluation."]
    #[inline(always)]
    pub fn ign0lsb(self) -> &'a mut W {
        self.variant(LPCOMP_A::IGN0LSB)
    }
    #[doc = "The LSB of the counter is ignored for compare match evaluation."]
    #[inline(always)]
    pub fn ign1lsb(self) -> &'a mut W {
        self.variant(LPCOMP_A::IGN1LSB)
    }
    #[doc = "The two LSBs of the counter are ignored for compare match evaluation."]
    #[inline(always)]
    pub fn ign2lsb(self) -> &'a mut W {
        self.variant(LPCOMP_A::IGN2LSB)
    }
    #[doc = "The three LSBs of the counter are ignored for compare match evaluation."]
    #[inline(always)]
    pub fn ign3lsb(self) -> &'a mut W {
        self.variant(LPCOMP_A::IGN3LSB)
    }
    #[doc = "The four LSBs of the counter are ignored for compare match evaluation."]
    #[inline(always)]
    pub fn ign4lsb(self) -> &'a mut W {
        self.variant(LPCOMP_A::IGN4LSB)
    }
    #[doc = "The five LSBs of the counter are ignored for compare match evaluation."]
    #[inline(always)]
    pub fn ign5lsb(self) -> &'a mut W {
        self.variant(LPCOMP_A::IGN5LSB)
    }
    #[doc = "The six LSBs of the counter are ignored for compare match evaluation."]
    #[inline(always)]
    pub fn ign6lsb(self) -> &'a mut W {
        self.variant(LPCOMP_A::IGN6LSB)
    }
    #[doc = "The seven LSBs of the counter are ignored for compare match evaluation."]
    #[inline(always)]
    pub fn ign7lsb(self) -> &'a mut W {
        self.variant(LPCOMP_A::IGN7LSB)
    }
}
#[doc = "Select BURTC prescaler factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESC_A {
    #[doc = "0: No prescaling."]
    DIV1 = 0,
    #[doc = "1: Prescaling factor of 2"]
    DIV2 = 1,
    #[doc = "2: Prescaling factor of 4"]
    DIV4 = 2,
    #[doc = "3: Prescaling factor of 8"]
    DIV8 = 3,
    #[doc = "4: Prescaling factor of 16"]
    DIV16 = 4,
    #[doc = "5: Prescaling factor of 32"]
    DIV32 = 5,
    #[doc = "6: Prescaling factor of 64"]
    DIV64 = 6,
    #[doc = "7: Prescaling factor of 128"]
    DIV128 = 7,
}
impl From<PRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRESC` reader - Select BURTC prescaler factor"]
pub type PRESC_R = crate::FieldReader<u8, PRESC_A>;
impl PRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRESC_A {
        match self.bits {
            0 => PRESC_A::DIV1,
            1 => PRESC_A::DIV2,
            2 => PRESC_A::DIV4,
            3 => PRESC_A::DIV8,
            4 => PRESC_A::DIV16,
            5 => PRESC_A::DIV32,
            6 => PRESC_A::DIV64,
            7 => PRESC_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PRESC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESC_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESC_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESC_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESC_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESC_A::DIV128
    }
}
#[doc = "Field `PRESC` writer - Select BURTC prescaler factor"]
pub type PRESC_W<'a> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, PRESC_A, 3, 8>;
impl<'a> PRESC_W<'a> {
    #[doc = "No prescaling."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PRESC_A::DIV1)
    }
    #[doc = "Prescaling factor of 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESC_A::DIV2)
    }
    #[doc = "Prescaling factor of 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESC_A::DIV4)
    }
    #[doc = "Prescaling factor of 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESC_A::DIV8)
    }
    #[doc = "Prescaling factor of 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESC_A::DIV16)
    }
    #[doc = "Prescaling factor of 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PRESC_A::DIV32)
    }
    #[doc = "Prescaling factor of 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESC_A::DIV64)
    }
    #[doc = "Prescaling factor of 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(PRESC_A::DIV128)
    }
}
#[doc = "Select BURTC clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: No clock source selected for BURTC."]
    NONE = 0,
    #[doc = "1: LFRCO selected as BURTC clock source."]
    LFRCO = 1,
    #[doc = "2: LFXO selected as BURTC clock source."]
    LFXO = 2,
    #[doc = "3: ULFRCO selected as BURTC clock source."]
    ULFRCO = 3,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLKSEL` reader - Select BURTC clock source"]
pub type CLKSEL_R = crate::FieldReader<u8, CLKSEL_A>;
impl CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKSEL_A {
        match self.bits {
            0 => CLKSEL_A::NONE,
            1 => CLKSEL_A::LFRCO,
            2 => CLKSEL_A::LFXO,
            3 => CLKSEL_A::ULFRCO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CLKSEL_A::NONE
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == CLKSEL_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == CLKSEL_A::LFXO
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == CLKSEL_A::ULFRCO
    }
}
#[doc = "Field `CLKSEL` writer - Select BURTC clock source"]
pub type CLKSEL_W<'a> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, CLKSEL_A, 2, 12>;
impl<'a> CLKSEL_W<'a> {
    #[doc = "No clock source selected for BURTC."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(CLKSEL_A::NONE)
    }
    #[doc = "LFRCO selected as BURTC clock source."]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(CLKSEL_A::LFRCO)
    }
    #[doc = "LFXO selected as BURTC clock source."]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(CLKSEL_A::LFXO)
    }
    #[doc = "ULFRCO selected as BURTC clock source."]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(CLKSEL_A::ULFRCO)
    }
}
#[doc = "Field `BUMODETSEN` reader - Backup mode timestamp enable"]
pub type BUMODETSEN_R = crate::BitReader<bool>;
#[doc = "Field `BUMODETSEN` writer - Backup mode timestamp enable"]
pub type BUMODETSEN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 14>;
impl R {
    #[doc = "Bits 0:1 - BURTC Enable"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&self) -> DEBUGRUN_R {
        DEBUGRUN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable BURTC reset"]
    #[inline(always)]
    pub fn rsten(&self) -> RSTEN_R {
        RSTEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Compare clear enable"]
    #[inline(always)]
    pub fn comp0top(&self) -> COMP0TOP_R {
        COMP0TOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Low power mode compare configuration"]
    #[inline(always)]
    pub fn lpcomp(&self) -> LPCOMP_R {
        LPCOMP_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Select BURTC prescaler factor"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:13 - Select BURTC clock source"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Backup mode timestamp enable"]
    #[inline(always)]
    pub fn bumodetsen(&self) -> BUMODETSEN_R {
        BUMODETSEN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - BURTC Enable"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W::new(self)
    }
    #[doc = "Bit 2 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&mut self) -> DEBUGRUN_W {
        DEBUGRUN_W::new(self)
    }
    #[doc = "Bit 3 - Enable BURTC reset"]
    #[inline(always)]
    pub fn rsten(&mut self) -> RSTEN_W {
        RSTEN_W::new(self)
    }
    #[doc = "Bit 4 - Compare clear enable"]
    #[inline(always)]
    pub fn comp0top(&mut self) -> COMP0TOP_W {
        COMP0TOP_W::new(self)
    }
    #[doc = "Bits 5:7 - Low power mode compare configuration"]
    #[inline(always)]
    pub fn lpcomp(&mut self) -> LPCOMP_W {
        LPCOMP_W::new(self)
    }
    #[doc = "Bits 8:10 - Select BURTC prescaler factor"]
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W {
        PRESC_W::new(self)
    }
    #[doc = "Bits 12:13 - Select BURTC clock source"]
    #[inline(always)]
    pub fn clksel(&mut self) -> CLKSEL_W {
        CLKSEL_W::new(self)
    }
    #[doc = "Bit 14 - Backup mode timestamp enable"]
    #[inline(always)]
    pub fn bumodetsen(&mut self) -> BUMODETSEN_W {
        BUMODETSEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0x08"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
