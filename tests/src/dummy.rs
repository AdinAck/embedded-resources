#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::marker::PhantomData;

pub struct Peri<'a, P> {
    #[expect(unused)]
    p: P,
    _lifetime: PhantomData<&'a mut P>,
}

pub(crate) struct PA2;
pub(crate) struct PA3;
pub(crate) struct PA4;
pub(crate) struct PA11;
pub(crate) struct PA12;
pub(crate) struct TIM2;
pub(crate) struct USB_OTG_FS;

pub(crate) struct Peripherals {
    pub(crate) PA2: Peri<'static, PA2>,
    pub(crate) PA3: Peri<'static, PA3>,
    pub(crate) PA4: Peri<'static, PA4>,
    pub(crate) PA11: Peri<'static, PA11>,
    pub(crate) PA12: Peri<'static, PA12>,
    pub(crate) TIM2: Peri<'static, TIM2>,
    pub(crate) USB_OTG_FS: Peri<'static, USB_OTG_FS>,
}

impl Peripherals {
    pub(crate) const fn new() -> Self {
        Self {
            PA2: Peri::<'static> {
                p: PA2,
                _lifetime: PhantomData,
            },
            PA3: Peri::<'static> {
                p: PA3,
                _lifetime: PhantomData,
            },
            PA4: Peri::<'static> {
                p: PA4,
                _lifetime: PhantomData,
            },
            PA11: Peri::<'static> {
                p: PA11,
                _lifetime: PhantomData,
            },
            PA12: Peri::<'static> {
                p: PA12,
                _lifetime: PhantomData,
            },
            TIM2: Peri::<'static> {
                p: TIM2,
                _lifetime: PhantomData,
            },
            USB_OTG_FS: Peri::<'static> {
                p: USB_OTG_FS,
                _lifetime: PhantomData,
            },
        }
    }
}
