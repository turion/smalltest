// extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;

use syn::*;

#[proc_macro_derive(Arbitrary)]
pub fn arbitrary_macro_derive(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = syn::parse(input).unwrap();
    // dbg!(&derive_input);
    let ident = derive_input.ident;

    match derive_input.data {
        Data::Struct(data_struct) => struct_arbitrary(ident, data_struct),
        Data::Enum(data_enum) => enum_arbitrary(ident, data_enum),
        _ => panic!("Unsupported derive_input.data: {:?}", derive_input.data),
    }
}

fn struct_arbitrary(ident: Ident, data_struct: DataStruct) -> TokenStream {
    match data_struct.fields {
        Fields::Named(FieldsNamed { named, .. }) => {
            let idents = named
                .iter()
                .map(|field| field.ident.as_ref().expect("Must have identifier"));

            let mut inner = quote! {
                smalltest::generator::single(
                    #ident {
                        #(#idents: #idents.clone()),*
                    }
                )
            };

            for field in named.iter().rev() {
                let field_type = &field.ty;
                let field_ident = field.ident.as_ref().expect("bla");
                inner = quote! {
                    #field_type::any().and_then(move |#field_ident| #inner)
                }
            }

            let code = quote! {
                impl Arbitrary for #ident {
                    type Gen = Box<dyn Generator<Item = #ident>>;

                    fn any() -> Self::Gen {
                        Box::new(
                            #inner
                        )
                    }
                }
            };
            // dbg!(&code);
            eprintln!("{}", &code.to_string());
            code.into()
        }
        _ => panic!("Only supports named fields"),
    }
}

fn enum_arbitrary(ident: Ident, data_enum: DataEnum) -> TokenStream {
    let variants = data_enum.variants.iter().map(|variant| {
        assert!(
            variant.fields.is_empty(),
            "Currently no variants with fields supported"
        );
        &variant.ident
    });

    let code = quote! {
        impl Arbitrary for #ident {
            type Gen = Box<dyn Generator<Item = #ident>>;

            fn any() -> Self::Gen {
                use smalltest::generator;
                Box::new(choose!(
                    #(single(#ident::#variants)),*
                ))
            }
        }
    };
    // dbg!(&code);
    eprintln!("{}", &code.to_string());

    code.into()
}
