use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote, ToTokens};
use syn::{Attribute, Ident, ItemStruct, ItemType, Meta, Type, Visibility};

fn generate_alias_stmt(
    vis: &Visibility,
    alias_value: &impl ToTokens,
    alias_type: &impl ToTokens,
) -> ItemType {
    syn::parse2(quote! { #vis type #alias_value = #alias_type; }).unwrap()
}

/// Mark a struct as a resource for extraction from the `Peripherals` instance.
///
/// # Example
/// ```rust
/// use embassy_stm32::peripherals::*;
/// use embedded_resources::resource_group;
///
/// #[resource_group]
/// pub(crate) struct UsbResources { // `pub(crate)` enables resources to be used across a project hierarchy
///     dp: PA12, // type aliases are generated (`type Dp = PA12` in this case)
///     dm: PA11,
///     usb: USB,
/// }
///
/// #[resource_group(no_aliases)] // only custom aliases are generated
/// struct LedResources {
///     r: PA2,
///     g: PA3,
///     b: PA4,
///     #[alias = PWMTimer] // specify a custom alias for this resource
///     tim2: TIM2,
/// }
///
/// #[embassy_executor::task]
/// async fn usb_task(r: UsbResources) {
///     // use r.dp, r.dm, r.usb
/// }
///
/// async fn setup_leds<'a>(r: LedResources) -> SimplePWM<'a, PWMTimer> {
///     // setup three channel PWM (one for each color)       ^ alias used here
/// }
///
/// #[embassy_executor::task]
/// async fn led_task(rgb_pwm: SimplePWM<'a, PWMTimer>) {
///     // use rgb_pwm                       ^ alias used here
/// }
///
/// #[embassy_executor::main]
/// async fn main(spawner: embassy_executor::Spawner) {
///     let p = embassy_stm32::init(Default::default());
///
///     let rgb_pwm = setup_leds(led_resources!(p));
///
///     spawner.spawn(usb_task(usb_resources!(p))).unwrap();
///     spawner.spawn(led_task(rgb_pwm)).unwrap();
///
///     // can still use p.PA0, p.PA1, etc
/// }
/// ```
#[proc_macro_attribute]
pub fn resource_group(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut s: ItemStruct = syn::parse2(item.into()).expect("Resource item must be a struct.");

    let attr: Option<Ident> = syn::parse2(args.into()).unwrap();

    let generate_aliases = match attr {
        None => true,
        Some(ident) => {
            assert_eq!(
                ident.to_string(),
                "no_aliases",
                "Expected identifier \"no_aliases\"."
            );
            false
        }
    };

    let vis = s.vis.clone();

    // propagate visibility from struct to fields
    s.fields
        .iter_mut()
        .for_each(|field| field.vis = vis.clone());

    let mut aliases = Vec::new();

    // search for "alias" attribute and remove/record for rendering
    s.fields.iter_mut().for_each(|field| {
        let mut custom_alias = false;

        field.attrs = field
            .attrs
            .iter()
            .cloned()
            .filter(|attr| {
                if let Meta::NameValue(alias) = &attr.meta {
                    if let Some(ident) = alias.path.get_ident() {
                        if ident.to_string().eq("alias") {
                            aliases.push(generate_alias_stmt(&vis, &alias.value, &field.ty));
                            custom_alias = true;
                            return false;
                        }
                    }
                }

                true
            })
            .collect();

        if generate_aliases && !custom_alias {
            aliases.push(generate_alias_stmt(
                &vis,
                &format_ident!(
                    "{}",
                    inflector::cases::classcase::to_class_case(
                        field.ident.as_ref().unwrap().to_string().as_str()
                    )
                ),
                &field.ty,
            ));
        }
    });

    let use_macro_ident = Ident::new(
        inflector::cases::snakecase::to_snake_case(s.ident.to_string().as_str()).as_str(),
        Span::call_site(),
    );
    let macro_vis = if let Visibility::Restricted(_) = vis {
        Some(quote! { #vis use #use_macro_ident; })
    } else {
        None
    };

    let ident = &s.ident;
    let field_idents: Vec<Ident> = s
        .fields
        .iter()
        .cloned()
        .map(|field| field.ident.unwrap())
        .collect();
    let field_types: Vec<Type> = s
        .fields
        .iter()
        .cloned()
        .map(|field| {
            if let Type::Path(ty) = field.ty {
                let ident = &ty.path.segments.last().unwrap().ident;
                syn::parse2(quote! { #ident }).unwrap()
            } else {
                field.ty
            }
        })
        .collect();
    let field_attrs: Vec<Vec<Attribute>> =
        s.fields.iter().cloned().map(|field| field.attrs).collect();
    let doc = format!(
        "Extract `{}` from a `Peripherals` instance.",
        ident.to_string()
    );

    quote! {
        #(
            #aliases
        )*

        #s

        #[doc = #doc]
        macro_rules! #use_macro_ident {
            ( $P:ident ) => {
                #ident {
                    #(
                        #(
                            #field_attrs
                        )*
                        #field_idents: $P.#field_types
                    ),*
                }
            };
        }

        #macro_vis
    }
    .into()
}
