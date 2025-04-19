#[cfg(test)]
mod dummy;

#[cfg(test)]
mod tests {
    use std::any::{Any, TypeId};

    use crate::dummy::{Peri, Peripherals};

    use super::dummy::{self as peripherals, PA12, PA2, PA3, PA4, TIM2, USB_OTG_FS};
    use embedded_resources::resource_group;

    #[resource_group]
    #[allow(non_snake_case)] // outer attribute
    pub(crate) struct UsbResources {
        dp: PA12,
        dm: peripherals::PA11, // user-provided type is flexible
        usb: USB_OTG_FS,
    }

    #[resource_group(no_aliases)]
    struct LedResources {
        r: PA2,
        g: PA3,
        b: PA4,
        #[cfg(not(bogus_flag))] // inner attribute (with alias as well)
        #[alias = PWMTimer] // optional attribute to specify a type alias
        tim2: TIM2,
    }

    /// tests basic usage, type resolution, aliases, and attribute persistence
    #[test]
    fn basic() {
        let p = Peripherals::new();
        let leds = led_resources!(p);
        let usb = usb_resources!(p);

        assert_eq!(leds.r.type_id(), TypeId::of::<Peri<PA2>>());
        assert_eq!(leds.g.type_id(), TypeId::of::<Peri<PA3>>());
        assert_eq!(leds.b.type_id(), TypeId::of::<Peri<PA4>>());
        assert_eq!(leds.tim2.type_id(), TypeId::of::<Peri<TIM2>>());
        assert_eq!(leds.tim2.type_id(), TypeId::of::<Peri<PWMTimer>>()); // verify type alias

        assert_eq!(usb.dp.type_id(), TypeId::of::<Peri<Dp>>());
        assert_eq!(usb.dm.type_id(), TypeId::of::<Peri<Dm>>());
        assert_eq!(usb.usb.type_id(), TypeId::of::<Peri<Usb>>());
    }
}
