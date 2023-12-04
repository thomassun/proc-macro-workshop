use proc_macro::TokenStream;
use quote::quote;
use syn::Data::Struct;
use syn::{DeriveInput, Fields, Ident};
use Fields::Named;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    
    let name = &ast.ident;
    let builder_str_name = format!("{}Builder", name);
    let builder_ident = Ident::new(&builder_str_name, name.span());
    let fields = if let Struct(syn::DataStruct {
        fields: Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        todo!()
    };
    let optionalized = fields.iter().map(|field| {
        let name = &field.ident;
        let ty = &field.ty;
        quote! {#name: Option<#ty>}
    });
    let default_assigments = fields.iter().map(|f| {
        let name = &f.ident;
        quote! {#name: None}
    });
    let builer_implementation_methods = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! {
            fn #name(&mut self, #name: #ty) -> &mut Self {
                self.#name = Some(#name);
                self
            }
        }
    });
    let check_n_extract_values = fields.iter().map(|f| {
        
        let name = &f.ident;
        quote! {
            #name:self.#name.clone().ok_or(Box::<dyn std::error::Error>::from(concat!(stringify!(#name)," is not provided")))?
        }
    });
    // fields;
    let expanded = quote! {
        impl #name {
            pub fn builder() -> #builder_ident{
                #builder_ident{
                    #(#default_assigments),*
                }
            }
        }

        struct #builder_ident {
            #(#optionalized),*
        }

        impl  #builder_ident {
            #(#builer_implementation_methods)*
            pub fn build(&mut self ) ->Result<#name,Box<dyn std::error::Error>>{
                // #(#check_n_extract_values)*
                std::result::Result::Ok(#name { #(#check_n_extract_values),*})
            }
        }

    };
    expanded.into()
}
