use proc_macro::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, DeriveInput, Token};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let derive = syn::parse_macro_input!(input as DeriveInput);
    match parse_derive(&derive) {
        Ok(token_steam) => token_steam.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
fn parse_derive(derive: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_data = parse_struct_data(derive)?;
    let struct_data_ident: Vec<_> = struct_data.iter().map(|f| &f.ident).collect();
    let struct_data_ty: Vec<_> = struct_data.iter().map(|f| &f.ty).collect();
    let struct_ident = &derive.ident;
    let struct_ident_name = derive.ident.to_string();
    let build_struct_ident_name = format!("{}Builder", struct_ident_name);
    let build_struct_ident = syn::Ident::new(&build_struct_ident_name, derive.span());
    let mut build_struct_data = proc_macro2::TokenStream::new();
    let mut build_struct_data_init = proc_macro2::TokenStream::new();
    let mut build_struct_data_is_none = proc_macro2::TokenStream::new();
    let mut build_struct_init = proc_macro2::TokenStream::new();
    for (ident, ty) in struct_data_ident.iter().zip(struct_data_ty.iter()) {
        if let Some(ty1) = parse_struct_data_ty(ty) {
            build_struct_data.extend(quote! {
                #ident:std::option::Option<#ty1>,
            });
            build_struct_data_init.extend(quote! {
                    pub fn #ident(&mut self,#ident:#ty1)->&mut Self{
                        self.#ident = Some(#ident);
                    self
                }
            });
            build_struct_init.extend(quote! {
                #ident:self.#ident.clone(),
            });
        } else {
            build_struct_data.extend(quote! {
                #ident:std::option::Option<#ty>,
            });
            build_struct_data_init.extend(quote! {
                pub fn #ident(&mut self,#ident:#ty)->&mut Self{
                    self.#ident = Some(#ident);
                    self
                }
            });
            build_struct_init.extend(quote! {
                #ident:self.#ident.clone().unwrap(),
            });
            build_struct_data_is_none.extend(quote! {
                if self.#ident.is_none(){
                    let err=format!("{} is none",stringify!(self.#ident));
                    return std::result::Result::Err(err.into())
                }
            });
        }
    }
    Ok(quote! {
        pub struct #build_struct_ident{
            #build_struct_data
        }
        impl #struct_ident{
            pub fn builder()->#build_struct_ident{
                #build_struct_ident{
                    #(#struct_data_ident:std::option::Option::None),*
                }
            }
        }
        impl #build_struct_ident{
            #build_struct_data_init
            pub fn build(&mut self)->std::result::Result<#struct_ident,std::boxed::Box<dyn std::error::Error>>{
                #build_struct_data_is_none
                std::result::Result::Ok(#struct_ident{
                    #build_struct_init
                })
            }
        }
    })
}
fn parse_struct_data(
    derive: &DeriveInput,
) -> syn::Result<&syn::punctuated::Punctuated<syn::Field, Token![,]>> {
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = derive.data
    {
        return Ok(named);
    }
    Err(syn::Error::new_spanned(derive, "parse error "))
}
fn parse_struct_data_ty(ty: &syn::Type) -> Option<&syn::Type> {
    if let syn::Type::Path(syn::TypePath { ref path, .. }) = ty {
        if let Some(segment) = path.segments.last() {
            if segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                    ref args,
                    ..
                }) = segment.arguments
                {
                    if let Some(syn::GenericArgument::Type(ref Type)) = args.first() {
                        return Some(Type);
                    }
                }
            }
        }
    }
    None
}
