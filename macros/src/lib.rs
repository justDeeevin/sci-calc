use convert_case::{Case, Casing};
use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{
    Expr, FnArg, GenericArgument, Ident, ItemEnum, ItemFn, Path, PathArguments, Type,
    parse_macro_input, parse_quote,
};

#[proc_macro_attribute]
pub fn constructors(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let def = parse_macro_input!(item as ItemEnum);
    let name = &def.ident;

    let functions = def.variants.iter().map(|variant| -> ItemFn {
        let ident = &variant.ident;

        let fn_name = Ident::new(&ident.to_string().to_case(Case::Snake), ident.span());

        let mut boxed = Vec::with_capacity(variant.fields.len());

        let args = variant
            .fields
            .iter()
            .enumerate()
            .map(|(i, field)| -> FnArg {
                let ty = {
                    let mut is_box = false;
                    let mut out = &field.ty;
                    if let Type::Path(path) = out {
                        if let Some(segment) = path.path.segments.last() {
                            if segment.ident == "Box" {
                                is_box = true;
                                let PathArguments::AngleBracketed(args) = &segment.arguments else {
                                    panic!("Box must have a generic argument")
                                };
                                let Some(GenericArgument::Type(ty)) = args.args.first() else {
                                    panic!("Box must have a generic argument")
                                };
                                out = ty;
                            }
                        }
                    }
                    boxed.push(is_box);
                    out
                };
                let name = Ident::new(&format!("_{i}"), Span::call_site().into());
                parse_quote!(#name: #ty)
            })
            .collect::<Vec<_>>();

        let values = boxed
            .iter()
            .enumerate()
            .map(|(i, boxed)| {
                let name = Ident::new(&format!("_{i}"), Span::call_site().into());
                if *boxed {
                    parse_quote!(Box::new(#name))
                } else {
                    parse_quote!(#name)
                }
            })
            .collect::<Vec<Expr>>();

        parse_quote! {
            pub fn #fn_name(#(#args),*) -> #name {
                <#name>::#ident(#(#values),*)
            }
        }
    });

    quote! {
        #def
        pub mod expr {
            use super::*;
            #(#functions)*
        }
        use expr::*;
    }
    .into()
}

#[proc_macro_attribute]
pub fn constants(attr: TokenStream, item: TokenStream) -> TokenStream {
    let def = parse_macro_input!(item as ItemEnum);
    let expr = parse_macro_input!(attr as Path);
    let expr_type = expr.segments.first().unwrap();
    let names = def
        .variants
        .iter()
        .map(|variant| {
            let ident = &variant.ident;
            Ident::new(&ident.to_string().to_case(Case::UpperSnake), ident.span())
        })
        .collect::<Vec<_>>();
    let ident = &def.ident;
    let variants = def.variants.iter().map(|variant| &variant.ident);

    quote! {
        #def
        pub mod consts {
            use super::*;
            #(pub const #names: #expr_type = #expr(<#ident>::#variants);)*
        }
        use consts::*;
    }
    .into()
}
