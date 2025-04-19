#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub struct Peri<P> {
    #[expect(unused)]
    p: P,
}

pub(crate) struct PA2;
pub(crate) struct PA3;
pub(crate) struct PA4;
pub(crate) struct PA11;
pub(crate) struct PA12;
pub(crate) struct TIM2;
pub(crate) struct USB_OTG_FS;

pub(crate) struct Peripherals {
    pub(crate) PA2: Peri<PA2>,
    pub(crate) PA3: Peri<PA3>,
    pub(crate) PA4: Peri<PA4>,
    pub(crate) PA11: Peri<PA11>,
    pub(crate) PA12: Peri<PA12>,
    pub(crate) TIM2: Peri<TIM2>,
    pub(crate) USB_OTG_FS: Peri<USB_OTG_FS>,
}

impl Peripherals {
    pub(crate) const fn new() -> Self {
        Self {
            PA2: Peri { p: PA2 },
            PA3: Peri { p: PA3 },
            PA4: Peri { p: PA4 },
            PA11: Peri { p: PA11 },
            PA12: Peri { p: PA12 },
            TIM2: Peri { p: TIM2 },
            USB_OTG_FS: Peri { p: USB_OTG_FS },
        }
    }
}
