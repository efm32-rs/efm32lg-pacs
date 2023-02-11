#[doc = "Register `CH11_CTRL` reader"]
pub struct R(crate::R<CH11_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH11_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH11_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH11_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH11_CTRL` writer"]
pub struct W(crate::W<CH11_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH11_CTRL_SPEC>;
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
impl From<crate::W<CH11_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH11_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIGSEL` reader - Signal Select"]
pub type SIGSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SIGSEL` writer - Signal Select"]
pub type SIGSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH11_CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `SOURCESEL` reader - Source Select"]
pub type SOURCESEL_R = crate::FieldReader<u8, SOURCESEL_A>;
#[doc = "Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SOURCESEL_A {
    #[doc = "0: No source selected"]
    NONE = 0,
    #[doc = "1: Voltage Comparator"]
    VCMP = 1,
    #[doc = "2: Analog Comparator 0"]
    ACMP0 = 2,
    #[doc = "3: Analog Comparator 1"]
    ACMP1 = 3,
    #[doc = "6: Digital to Analog Converter 0"]
    DAC0 = 6,
    #[doc = "8: Analog to Digital Converter 0"]
    ADC0 = 8,
    #[doc = "16: Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    USART0 = 16,
    #[doc = "17: Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    USART1 = 17,
    #[doc = "18: Universal Synchronous/Asynchronous Receiver/Transmitter 2"]
    USART2 = 18,
    #[doc = "28: Timer 0"]
    TIMER0 = 28,
    #[doc = "29: Timer 1"]
    TIMER1 = 29,
    #[doc = "30: Timer 2"]
    TIMER2 = 30,
    #[doc = "31: Timer 3"]
    TIMER3 = 31,
    #[doc = "36: Universal Serial Bus Interface"]
    USB = 36,
    #[doc = "40: Real-Time Counter"]
    RTC = 40,
    #[doc = "41: Universal Asynchronous Receiver/Transmitter 0"]
    UART0 = 41,
    #[doc = "42: Universal Asynchronous Receiver/Transmitter 1"]
    UART1 = 42,
    #[doc = "48: General purpose Input/Output"]
    GPIOL = 48,
    #[doc = "49: General purpose Input/Output"]
    GPIOH = 49,
    #[doc = "52: Low Energy Timer 0"]
    LETIMER0 = 52,
    #[doc = "55: Backup RTC"]
    BURTC = 55,
    #[doc = "57: Low Energy Sensor Interface"]
    LESENSEL = 57,
    #[doc = "58: Low Energy Sensor Interface"]
    LESENSEH = 58,
    #[doc = "59: Low Energy Sensor Interface"]
    LESENSED = 59,
}
impl From<SOURCESEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SOURCESEL_A) -> Self {
        variant as _
    }
}
impl SOURCESEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SOURCESEL_A> {
        match self.bits {
            0 => Some(SOURCESEL_A::NONE),
            1 => Some(SOURCESEL_A::VCMP),
            2 => Some(SOURCESEL_A::ACMP0),
            3 => Some(SOURCESEL_A::ACMP1),
            6 => Some(SOURCESEL_A::DAC0),
            8 => Some(SOURCESEL_A::ADC0),
            16 => Some(SOURCESEL_A::USART0),
            17 => Some(SOURCESEL_A::USART1),
            18 => Some(SOURCESEL_A::USART2),
            28 => Some(SOURCESEL_A::TIMER0),
            29 => Some(SOURCESEL_A::TIMER1),
            30 => Some(SOURCESEL_A::TIMER2),
            31 => Some(SOURCESEL_A::TIMER3),
            36 => Some(SOURCESEL_A::USB),
            40 => Some(SOURCESEL_A::RTC),
            41 => Some(SOURCESEL_A::UART0),
            42 => Some(SOURCESEL_A::UART1),
            48 => Some(SOURCESEL_A::GPIOL),
            49 => Some(SOURCESEL_A::GPIOH),
            52 => Some(SOURCESEL_A::LETIMER0),
            55 => Some(SOURCESEL_A::BURTC),
            57 => Some(SOURCESEL_A::LESENSEL),
            58 => Some(SOURCESEL_A::LESENSEH),
            59 => Some(SOURCESEL_A::LESENSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SOURCESEL_A::NONE
    }
    #[doc = "Checks if the value of the field is `VCMP`"]
    #[inline(always)]
    pub fn is_vcmp(&self) -> bool {
        *self == SOURCESEL_A::VCMP
    }
    #[doc = "Checks if the value of the field is `ACMP0`"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == SOURCESEL_A::ACMP0
    }
    #[doc = "Checks if the value of the field is `ACMP1`"]
    #[inline(always)]
    pub fn is_acmp1(&self) -> bool {
        *self == SOURCESEL_A::ACMP1
    }
    #[doc = "Checks if the value of the field is `DAC0`"]
    #[inline(always)]
    pub fn is_dac0(&self) -> bool {
        *self == SOURCESEL_A::DAC0
    }
    #[doc = "Checks if the value of the field is `ADC0`"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == SOURCESEL_A::ADC0
    }
    #[doc = "Checks if the value of the field is `USART0`"]
    #[inline(always)]
    pub fn is_usart0(&self) -> bool {
        *self == SOURCESEL_A::USART0
    }
    #[doc = "Checks if the value of the field is `USART1`"]
    #[inline(always)]
    pub fn is_usart1(&self) -> bool {
        *self == SOURCESEL_A::USART1
    }
    #[doc = "Checks if the value of the field is `USART2`"]
    #[inline(always)]
    pub fn is_usart2(&self) -> bool {
        *self == SOURCESEL_A::USART2
    }
    #[doc = "Checks if the value of the field is `TIMER0`"]
    #[inline(always)]
    pub fn is_timer0(&self) -> bool {
        *self == SOURCESEL_A::TIMER0
    }
    #[doc = "Checks if the value of the field is `TIMER1`"]
    #[inline(always)]
    pub fn is_timer1(&self) -> bool {
        *self == SOURCESEL_A::TIMER1
    }
    #[doc = "Checks if the value of the field is `TIMER2`"]
    #[inline(always)]
    pub fn is_timer2(&self) -> bool {
        *self == SOURCESEL_A::TIMER2
    }
    #[doc = "Checks if the value of the field is `TIMER3`"]
    #[inline(always)]
    pub fn is_timer3(&self) -> bool {
        *self == SOURCESEL_A::TIMER3
    }
    #[doc = "Checks if the value of the field is `USB`"]
    #[inline(always)]
    pub fn is_usb(&self) -> bool {
        *self == SOURCESEL_A::USB
    }
    #[doc = "Checks if the value of the field is `RTC`"]
    #[inline(always)]
    pub fn is_rtc(&self) -> bool {
        *self == SOURCESEL_A::RTC
    }
    #[doc = "Checks if the value of the field is `UART0`"]
    #[inline(always)]
    pub fn is_uart0(&self) -> bool {
        *self == SOURCESEL_A::UART0
    }
    #[doc = "Checks if the value of the field is `UART1`"]
    #[inline(always)]
    pub fn is_uart1(&self) -> bool {
        *self == SOURCESEL_A::UART1
    }
    #[doc = "Checks if the value of the field is `GPIOL`"]
    #[inline(always)]
    pub fn is_gpiol(&self) -> bool {
        *self == SOURCESEL_A::GPIOL
    }
    #[doc = "Checks if the value of the field is `GPIOH`"]
    #[inline(always)]
    pub fn is_gpioh(&self) -> bool {
        *self == SOURCESEL_A::GPIOH
    }
    #[doc = "Checks if the value of the field is `LETIMER0`"]
    #[inline(always)]
    pub fn is_letimer0(&self) -> bool {
        *self == SOURCESEL_A::LETIMER0
    }
    #[doc = "Checks if the value of the field is `BURTC`"]
    #[inline(always)]
    pub fn is_burtc(&self) -> bool {
        *self == SOURCESEL_A::BURTC
    }
    #[doc = "Checks if the value of the field is `LESENSEL`"]
    #[inline(always)]
    pub fn is_lesensel(&self) -> bool {
        *self == SOURCESEL_A::LESENSEL
    }
    #[doc = "Checks if the value of the field is `LESENSEH`"]
    #[inline(always)]
    pub fn is_lesenseh(&self) -> bool {
        *self == SOURCESEL_A::LESENSEH
    }
    #[doc = "Checks if the value of the field is `LESENSED`"]
    #[inline(always)]
    pub fn is_lesensed(&self) -> bool {
        *self == SOURCESEL_A::LESENSED
    }
}
#[doc = "Field `SOURCESEL` writer - Source Select"]
pub type SOURCESEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CH11_CTRL_SPEC, u8, SOURCESEL_A, 6, O>;
impl<'a, const O: u8> SOURCESEL_W<'a, O> {
    #[doc = "No source selected"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SOURCESEL_A::NONE)
    }
    #[doc = "Voltage Comparator"]
    #[inline(always)]
    pub fn vcmp(self) -> &'a mut W {
        self.variant(SOURCESEL_A::VCMP)
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn acmp0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::ACMP0)
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn acmp1(self) -> &'a mut W {
        self.variant(SOURCESEL_A::ACMP1)
    }
    #[doc = "Digital to Analog Converter 0"]
    #[inline(always)]
    pub fn dac0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::DAC0)
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::ADC0)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    #[inline(always)]
    pub fn usart0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::USART0)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    #[inline(always)]
    pub fn usart1(self) -> &'a mut W {
        self.variant(SOURCESEL_A::USART1)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2"]
    #[inline(always)]
    pub fn usart2(self) -> &'a mut W {
        self.variant(SOURCESEL_A::USART2)
    }
    #[doc = "Timer 0"]
    #[inline(always)]
    pub fn timer0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::TIMER0)
    }
    #[doc = "Timer 1"]
    #[inline(always)]
    pub fn timer1(self) -> &'a mut W {
        self.variant(SOURCESEL_A::TIMER1)
    }
    #[doc = "Timer 2"]
    #[inline(always)]
    pub fn timer2(self) -> &'a mut W {
        self.variant(SOURCESEL_A::TIMER2)
    }
    #[doc = "Timer 3"]
    #[inline(always)]
    pub fn timer3(self) -> &'a mut W {
        self.variant(SOURCESEL_A::TIMER3)
    }
    #[doc = "Universal Serial Bus Interface"]
    #[inline(always)]
    pub fn usb(self) -> &'a mut W {
        self.variant(SOURCESEL_A::USB)
    }
    #[doc = "Real-Time Counter"]
    #[inline(always)]
    pub fn rtc(self) -> &'a mut W {
        self.variant(SOURCESEL_A::RTC)
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter 0"]
    #[inline(always)]
    pub fn uart0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::UART0)
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter 1"]
    #[inline(always)]
    pub fn uart1(self) -> &'a mut W {
        self.variant(SOURCESEL_A::UART1)
    }
    #[doc = "General purpose Input/Output"]
    #[inline(always)]
    pub fn gpiol(self) -> &'a mut W {
        self.variant(SOURCESEL_A::GPIOL)
    }
    #[doc = "General purpose Input/Output"]
    #[inline(always)]
    pub fn gpioh(self) -> &'a mut W {
        self.variant(SOURCESEL_A::GPIOH)
    }
    #[doc = "Low Energy Timer 0"]
    #[inline(always)]
    pub fn letimer0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::LETIMER0)
    }
    #[doc = "Backup RTC"]
    #[inline(always)]
    pub fn burtc(self) -> &'a mut W {
        self.variant(SOURCESEL_A::BURTC)
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline(always)]
    pub fn lesensel(self) -> &'a mut W {
        self.variant(SOURCESEL_A::LESENSEL)
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline(always)]
    pub fn lesenseh(self) -> &'a mut W {
        self.variant(SOURCESEL_A::LESENSEH)
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline(always)]
    pub fn lesensed(self) -> &'a mut W {
        self.variant(SOURCESEL_A::LESENSED)
    }
}
#[doc = "Field `EDSEL` reader - Edge Detect Select"]
pub type EDSEL_R = crate::FieldReader<u8, EDSEL_A>;
#[doc = "Edge Detect Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EDSEL_A {
    #[doc = "0: Signal is left as it is"]
    OFF = 0,
    #[doc = "1: A one HFPERCLK cycle pulse is generated for every positive edge of the incoming signal"]
    POSEDGE = 1,
    #[doc = "2: A one HFPERCLK clock cycle pulse is generated for every negative edge of the incoming signal"]
    NEGEDGE = 2,
    #[doc = "3: A one HFPERCLK clock cycle pulse is generated for every edge of the incoming signal"]
    BOTHEDGES = 3,
}
impl From<EDSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EDSEL_A) -> Self {
        variant as _
    }
}
impl EDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDSEL_A {
        match self.bits {
            0 => EDSEL_A::OFF,
            1 => EDSEL_A::POSEDGE,
            2 => EDSEL_A::NEGEDGE,
            3 => EDSEL_A::BOTHEDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == EDSEL_A::OFF
    }
    #[doc = "Checks if the value of the field is `POSEDGE`"]
    #[inline(always)]
    pub fn is_posedge(&self) -> bool {
        *self == EDSEL_A::POSEDGE
    }
    #[doc = "Checks if the value of the field is `NEGEDGE`"]
    #[inline(always)]
    pub fn is_negedge(&self) -> bool {
        *self == EDSEL_A::NEGEDGE
    }
    #[doc = "Checks if the value of the field is `BOTHEDGES`"]
    #[inline(always)]
    pub fn is_bothedges(&self) -> bool {
        *self == EDSEL_A::BOTHEDGES
    }
}
#[doc = "Field `EDSEL` writer - Edge Detect Select"]
pub type EDSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CH11_CTRL_SPEC, u8, EDSEL_A, 2, O>;
impl<'a, const O: u8> EDSEL_W<'a, O> {
    #[doc = "Signal is left as it is"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(EDSEL_A::OFF)
    }
    #[doc = "A one HFPERCLK cycle pulse is generated for every positive edge of the incoming signal"]
    #[inline(always)]
    pub fn posedge(self) -> &'a mut W {
        self.variant(EDSEL_A::POSEDGE)
    }
    #[doc = "A one HFPERCLK clock cycle pulse is generated for every negative edge of the incoming signal"]
    #[inline(always)]
    pub fn negedge(self) -> &'a mut W {
        self.variant(EDSEL_A::NEGEDGE)
    }
    #[doc = "A one HFPERCLK clock cycle pulse is generated for every edge of the incoming signal"]
    #[inline(always)]
    pub fn bothedges(self) -> &'a mut W {
        self.variant(EDSEL_A::BOTHEDGES)
    }
}
#[doc = "Field `ASYNC` reader - Asynchronous reflex"]
pub type ASYNC_R = crate::BitReader<bool>;
#[doc = "Field `ASYNC` writer - Asynchronous reflex"]
pub type ASYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH11_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Signal Select"]
    #[inline(always)]
    pub fn sigsel(&self) -> SIGSEL_R {
        SIGSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:21 - Source Select"]
    #[inline(always)]
    pub fn sourcesel(&self) -> SOURCESEL_R {
        SOURCESEL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:25 - Edge Detect Select"]
    #[inline(always)]
    pub fn edsel(&self) -> EDSEL_R {
        EDSEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28 - Asynchronous reflex"]
    #[inline(always)]
    pub fn async_(&self) -> ASYNC_R {
        ASYNC_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Signal Select"]
    #[inline(always)]
    #[must_use]
    pub fn sigsel(&mut self) -> SIGSEL_W<0> {
        SIGSEL_W::new(self)
    }
    #[doc = "Bits 16:21 - Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn sourcesel(&mut self) -> SOURCESEL_W<16> {
        SOURCESEL_W::new(self)
    }
    #[doc = "Bits 24:25 - Edge Detect Select"]
    #[inline(always)]
    #[must_use]
    pub fn edsel(&mut self) -> EDSEL_W<24> {
        EDSEL_W::new(self)
    }
    #[doc = "Bit 28 - Asynchronous reflex"]
    #[inline(always)]
    #[must_use]
    pub fn async_(&mut self) -> ASYNC_W<28> {
        ASYNC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch11_ctrl](index.html) module"]
pub struct CH11_CTRL_SPEC;
impl crate::RegisterSpec for CH11_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch11_ctrl::R](R) reader structure"]
impl crate::Readable for CH11_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch11_ctrl::W](W) writer structure"]
impl crate::Writable for CH11_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH11_CTRL to value 0"]
impl crate::Resettable for CH11_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
