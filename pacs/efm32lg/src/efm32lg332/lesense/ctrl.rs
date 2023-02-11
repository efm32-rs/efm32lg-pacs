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
#[doc = "Field `SCANMODE` reader - Configure scan mode"]
pub type SCANMODE_R = crate::FieldReader<u8, SCANMODE_A>;
#[doc = "Configure scan mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCANMODE_A {
    #[doc = "0: A new scan is started each time the period counter overflows"]
    PERIODIC = 0,
    #[doc = "1: A single scan is performed when START in CMD is set"]
    ONESHOT = 1,
    #[doc = "2: Pulse on PRS channel"]
    PRS = 2,
}
impl From<SCANMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: SCANMODE_A) -> Self {
        variant as _
    }
}
impl SCANMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SCANMODE_A> {
        match self.bits {
            0 => Some(SCANMODE_A::PERIODIC),
            1 => Some(SCANMODE_A::ONESHOT),
            2 => Some(SCANMODE_A::PRS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PERIODIC`"]
    #[inline(always)]
    pub fn is_periodic(&self) -> bool {
        *self == SCANMODE_A::PERIODIC
    }
    #[doc = "Checks if the value of the field is `ONESHOT`"]
    #[inline(always)]
    pub fn is_oneshot(&self) -> bool {
        *self == SCANMODE_A::ONESHOT
    }
    #[doc = "Checks if the value of the field is `PRS`"]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == SCANMODE_A::PRS
    }
}
#[doc = "Field `SCANMODE` writer - Configure scan mode"]
pub type SCANMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, SCANMODE_A, 2, O>;
impl<'a, const O: u8> SCANMODE_W<'a, O> {
    #[doc = "A new scan is started each time the period counter overflows"]
    #[inline(always)]
    pub fn periodic(self) -> &'a mut W {
        self.variant(SCANMODE_A::PERIODIC)
    }
    #[doc = "A single scan is performed when START in CMD is set"]
    #[inline(always)]
    pub fn oneshot(self) -> &'a mut W {
        self.variant(SCANMODE_A::ONESHOT)
    }
    #[doc = "Pulse on PRS channel"]
    #[inline(always)]
    pub fn prs(self) -> &'a mut W {
        self.variant(SCANMODE_A::PRS)
    }
}
#[doc = "Field `PRSSEL` reader - Scan start PRS select"]
pub type PRSSEL_R = crate::FieldReader<u8, PRSSEL_A>;
#[doc = "Scan start PRS select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSEL_A {
    #[doc = "0: PRS Channel 0 selected as input"]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected as input"]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected as input"]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected as input"]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected as input"]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected as input"]
    PRSCH5 = 5,
    #[doc = "6: PRS Channel 6 selected as input"]
    PRSCH6 = 6,
    #[doc = "7: PRS Channel 7 selected as input"]
    PRSCH7 = 7,
    #[doc = "8: PRS Channel 8 selected as input"]
    PRSCH8 = 8,
    #[doc = "9: PRS Channel 9 selected as input"]
    PRSCH9 = 9,
    #[doc = "10: PRS Channel 10 selected as input"]
    PRSCH10 = 10,
    #[doc = "11: PRS Channel 11 selected as input"]
    PRSCH11 = 11,
}
impl From<PRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSEL_A) -> Self {
        variant as _
    }
}
impl PRSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRSSEL_A> {
        match self.bits {
            0 => Some(PRSSEL_A::PRSCH0),
            1 => Some(PRSSEL_A::PRSCH1),
            2 => Some(PRSSEL_A::PRSCH2),
            3 => Some(PRSSEL_A::PRSCH3),
            4 => Some(PRSSEL_A::PRSCH4),
            5 => Some(PRSSEL_A::PRSCH5),
            6 => Some(PRSSEL_A::PRSCH6),
            7 => Some(PRSSEL_A::PRSCH7),
            8 => Some(PRSSEL_A::PRSCH8),
            9 => Some(PRSSEL_A::PRSCH9),
            10 => Some(PRSSEL_A::PRSCH10),
            11 => Some(PRSSEL_A::PRSCH11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL_A::PRSCH11
    }
}
#[doc = "Field `PRSSEL` writer - Scan start PRS select"]
pub type PRSSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, PRSSEL_A, 4, O>;
impl<'a, const O: u8> PRSSEL_W<'a, O> {
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH11)
    }
}
#[doc = "Field `SCANCONF` reader - Select scan configuration"]
pub type SCANCONF_R = crate::FieldReader<u8, SCANCONF_A>;
#[doc = "Select scan configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCANCONF_A {
    #[doc = "0: The channel configuration register registers used are directly mapped to the channel number."]
    DIRMAP = 0,
    #[doc = "1: The channel configuration register registers used are CHX+8_CONF for channels 0-7 and CHX-8_CONF for channels 8-15."]
    INVMAP = 1,
    #[doc = "2: The channel configuration register registers used toggles between CHX_CONF and CHX+8_CONF when channel x triggers"]
    TOGGLE = 2,
    #[doc = "3: The decoder state defines the CONF registers to be used."]
    DECDEF = 3,
}
impl From<SCANCONF_A> for u8 {
    #[inline(always)]
    fn from(variant: SCANCONF_A) -> Self {
        variant as _
    }
}
impl SCANCONF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCANCONF_A {
        match self.bits {
            0 => SCANCONF_A::DIRMAP,
            1 => SCANCONF_A::INVMAP,
            2 => SCANCONF_A::TOGGLE,
            3 => SCANCONF_A::DECDEF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIRMAP`"]
    #[inline(always)]
    pub fn is_dirmap(&self) -> bool {
        *self == SCANCONF_A::DIRMAP
    }
    #[doc = "Checks if the value of the field is `INVMAP`"]
    #[inline(always)]
    pub fn is_invmap(&self) -> bool {
        *self == SCANCONF_A::INVMAP
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == SCANCONF_A::TOGGLE
    }
    #[doc = "Checks if the value of the field is `DECDEF`"]
    #[inline(always)]
    pub fn is_decdef(&self) -> bool {
        *self == SCANCONF_A::DECDEF
    }
}
#[doc = "Field `SCANCONF` writer - Select scan configuration"]
pub type SCANCONF_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, SCANCONF_A, 2, O>;
impl<'a, const O: u8> SCANCONF_W<'a, O> {
    #[doc = "The channel configuration register registers used are directly mapped to the channel number."]
    #[inline(always)]
    pub fn dirmap(self) -> &'a mut W {
        self.variant(SCANCONF_A::DIRMAP)
    }
    #[doc = "The channel configuration register registers used are CHX+8_CONF for channels 0-7 and CHX-8_CONF for channels 8-15."]
    #[inline(always)]
    pub fn invmap(self) -> &'a mut W {
        self.variant(SCANCONF_A::INVMAP)
    }
    #[doc = "The channel configuration register registers used toggles between CHX_CONF and CHX+8_CONF when channel x triggers"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(SCANCONF_A::TOGGLE)
    }
    #[doc = "The decoder state defines the CONF registers to be used."]
    #[inline(always)]
    pub fn decdef(self) -> &'a mut W {
        self.variant(SCANCONF_A::DECDEF)
    }
}
#[doc = "Field `ACMP0INV` reader - Invert analog comparator 0 output"]
pub type ACMP0INV_R = crate::BitReader<bool>;
#[doc = "Field `ACMP0INV` writer - Invert analog comparator 0 output"]
pub type ACMP0INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ACMP1INV` reader - Invert analog comparator 1 output"]
pub type ACMP1INV_R = crate::BitReader<bool>;
#[doc = "Field `ACMP1INV` writer - Invert analog comparator 1 output"]
pub type ACMP1INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ALTEXMAP` reader - Alternative excitation map"]
pub type ALTEXMAP_R = crate::BitReader<bool>;
#[doc = "Field `ALTEXMAP` writer - Alternative excitation map"]
pub type ALTEXMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `DUALSAMPLE` reader - Enable dual sample mode"]
pub type DUALSAMPLE_R = crate::BitReader<bool>;
#[doc = "Field `DUALSAMPLE` writer - Enable dual sample mode"]
pub type DUALSAMPLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `BUFOW` reader - Result buffer overwrite"]
pub type BUFOW_R = crate::BitReader<bool>;
#[doc = "Field `BUFOW` writer - Result buffer overwrite"]
pub type BUFOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `STRSCANRES` reader - Enable storing of SCANRES"]
pub type STRSCANRES_R = crate::BitReader<bool>;
#[doc = "Field `STRSCANRES` writer - Enable storing of SCANRES"]
pub type STRSCANRES_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `BUFIDL` reader - Result buffer interrupt and DMA trigger level"]
pub type BUFIDL_R = crate::BitReader<bool>;
#[doc = "Field `BUFIDL` writer - Result buffer interrupt and DMA trigger level"]
pub type BUFIDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `DMAWU` reader - DMA wake-up from EM2"]
pub type DMAWU_R = crate::FieldReader<u8, DMAWU_A>;
#[doc = "DMA wake-up from EM2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMAWU_A {
    #[doc = "0: No DMA wake-up from EM2"]
    DISABLE = 0,
    #[doc = "1: DMA wake-up from EM2 when data is valid in the result buffer"]
    BUFDATAV = 1,
    #[doc = "2: DMA wake-up from EM2 when the result buffer is full/half-full depending on BUFIDL configuration"]
    BUFLEVEL = 2,
}
impl From<DMAWU_A> for u8 {
    #[inline(always)]
    fn from(variant: DMAWU_A) -> Self {
        variant as _
    }
}
impl DMAWU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DMAWU_A> {
        match self.bits {
            0 => Some(DMAWU_A::DISABLE),
            1 => Some(DMAWU_A::BUFDATAV),
            2 => Some(DMAWU_A::BUFLEVEL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DMAWU_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `BUFDATAV`"]
    #[inline(always)]
    pub fn is_bufdatav(&self) -> bool {
        *self == DMAWU_A::BUFDATAV
    }
    #[doc = "Checks if the value of the field is `BUFLEVEL`"]
    #[inline(always)]
    pub fn is_buflevel(&self) -> bool {
        *self == DMAWU_A::BUFLEVEL
    }
}
#[doc = "Field `DMAWU` writer - DMA wake-up from EM2"]
pub type DMAWU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, DMAWU_A, 2, O>;
impl<'a, const O: u8> DMAWU_W<'a, O> {
    #[doc = "No DMA wake-up from EM2"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMAWU_A::DISABLE)
    }
    #[doc = "DMA wake-up from EM2 when data is valid in the result buffer"]
    #[inline(always)]
    pub fn bufdatav(self) -> &'a mut W {
        self.variant(DMAWU_A::BUFDATAV)
    }
    #[doc = "DMA wake-up from EM2 when the result buffer is full/half-full depending on BUFIDL configuration"]
    #[inline(always)]
    pub fn buflevel(self) -> &'a mut W {
        self.variant(DMAWU_A::BUFLEVEL)
    }
}
#[doc = "Field `DEBUGRUN` reader - Debug Mode Run Enable"]
pub type DEBUGRUN_R = crate::BitReader<bool>;
#[doc = "Field `DEBUGRUN` writer - Debug Mode Run Enable"]
pub type DEBUGRUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Configure scan mode"]
    #[inline(always)]
    pub fn scanmode(&self) -> SCANMODE_R {
        SCANMODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:5 - Scan start PRS select"]
    #[inline(always)]
    pub fn prssel(&self) -> PRSSEL_R {
        PRSSEL_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - Select scan configuration"]
    #[inline(always)]
    pub fn scanconf(&self) -> SCANCONF_R {
        SCANCONF_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 9 - Invert analog comparator 0 output"]
    #[inline(always)]
    pub fn acmp0inv(&self) -> ACMP0INV_R {
        ACMP0INV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Invert analog comparator 1 output"]
    #[inline(always)]
    pub fn acmp1inv(&self) -> ACMP1INV_R {
        ACMP1INV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Alternative excitation map"]
    #[inline(always)]
    pub fn altexmap(&self) -> ALTEXMAP_R {
        ALTEXMAP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable dual sample mode"]
    #[inline(always)]
    pub fn dualsample(&self) -> DUALSAMPLE_R {
        DUALSAMPLE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Result buffer overwrite"]
    #[inline(always)]
    pub fn bufow(&self) -> BUFOW_R {
        BUFOW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable storing of SCANRES"]
    #[inline(always)]
    pub fn strscanres(&self) -> STRSCANRES_R {
        STRSCANRES_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Result buffer interrupt and DMA trigger level"]
    #[inline(always)]
    pub fn bufidl(&self) -> BUFIDL_R {
        BUFIDL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:21 - DMA wake-up from EM2"]
    #[inline(always)]
    pub fn dmawu(&self) -> DMAWU_R {
        DMAWU_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&self) -> DEBUGRUN_R {
        DEBUGRUN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Configure scan mode"]
    #[inline(always)]
    #[must_use]
    pub fn scanmode(&mut self) -> SCANMODE_W<0> {
        SCANMODE_W::new(self)
    }
    #[doc = "Bits 2:5 - Scan start PRS select"]
    #[inline(always)]
    #[must_use]
    pub fn prssel(&mut self) -> PRSSEL_W<2> {
        PRSSEL_W::new(self)
    }
    #[doc = "Bits 6:7 - Select scan configuration"]
    #[inline(always)]
    #[must_use]
    pub fn scanconf(&mut self) -> SCANCONF_W<6> {
        SCANCONF_W::new(self)
    }
    #[doc = "Bit 9 - Invert analog comparator 0 output"]
    #[inline(always)]
    #[must_use]
    pub fn acmp0inv(&mut self) -> ACMP0INV_W<9> {
        ACMP0INV_W::new(self)
    }
    #[doc = "Bit 10 - Invert analog comparator 1 output"]
    #[inline(always)]
    #[must_use]
    pub fn acmp1inv(&mut self) -> ACMP1INV_W<10> {
        ACMP1INV_W::new(self)
    }
    #[doc = "Bit 11 - Alternative excitation map"]
    #[inline(always)]
    #[must_use]
    pub fn altexmap(&mut self) -> ALTEXMAP_W<11> {
        ALTEXMAP_W::new(self)
    }
    #[doc = "Bit 13 - Enable dual sample mode"]
    #[inline(always)]
    #[must_use]
    pub fn dualsample(&mut self) -> DUALSAMPLE_W<13> {
        DUALSAMPLE_W::new(self)
    }
    #[doc = "Bit 16 - Result buffer overwrite"]
    #[inline(always)]
    #[must_use]
    pub fn bufow(&mut self) -> BUFOW_W<16> {
        BUFOW_W::new(self)
    }
    #[doc = "Bit 17 - Enable storing of SCANRES"]
    #[inline(always)]
    #[must_use]
    pub fn strscanres(&mut self) -> STRSCANRES_W<17> {
        STRSCANRES_W::new(self)
    }
    #[doc = "Bit 18 - Result buffer interrupt and DMA trigger level"]
    #[inline(always)]
    #[must_use]
    pub fn bufidl(&mut self) -> BUFIDL_W<18> {
        BUFIDL_W::new(self)
    }
    #[doc = "Bits 20:21 - DMA wake-up from EM2"]
    #[inline(always)]
    #[must_use]
    pub fn dmawu(&mut self) -> DMAWU_W<20> {
        DMAWU_W::new(self)
    }
    #[doc = "Bit 22 - Debug Mode Run Enable"]
    #[inline(always)]
    #[must_use]
    pub fn debugrun(&mut self) -> DEBUGRUN_W<22> {
        DEBUGRUN_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
