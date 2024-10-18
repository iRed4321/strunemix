
use std::iter::FromIterator;
use proc_macro::TokenStream;
use syn::{
    DeriveInput, Ident, Type, Attribute, Fields, Meta, Path, PathArguments, PathSegment, punctuated::Punctuated
};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use heck::ToUpperCamelCase;

#[proc_macro_derive(Strunemix, attributes(strunemix, strunemix_derive_data, strunemix_derive_name, strunemix_derive, strunemix_default))]
pub fn field_type(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let (vis, ty, generics) = (&ast.vis, &ast.ident, &ast.generics);
    let enum_data = Ident::new(&(ty.to_string() + "AttrData"), Span::call_site());
    let enum_name = Ident::new(&(ty.to_string() + "AttrName"), Span::call_site());
    let derive_type = get_enum_derive(&ast.attrs, &["strunemix_derive_data", "strunemix_derive"], quote! {});
    let derive_name = get_enum_derive(&ast.attrs, &["strunemix_derive_name", "strunemix_derive"], 
    quote! {#[derive(Debug, PartialEq, Eq, Clone, Copy)] }
    );
    let fields = filter_fields(match ast.data {
        syn::Data::Struct(ref s) => &s.fields,
        _ => panic!("Strunemix can only derive structures")
    });

    let have_default = ast.attrs.iter()
        .find(|attr| attr.path().is_ident("strunemix_default"))
        .is_some();

    let haveskippedfields = match ast.data {
        syn::Data::Struct(ref s) => &s.fields,
        _ => panic!("Strunemix can only derive structures")
    }.len() != fields.len();

    if fields.is_empty() {
        panic!("Strunemix can only derive non-empty structures");
    }

    let field_name_constructs = fields.iter()
        .map(|(_, _, variant_ident)| quote! {
            #enum_name::#variant_ident
        });

    let field_type_to_variant = fields.iter()
        .map(|(_, _, variant_ident)| quote! {
            #enum_data::#variant_ident(_) => #enum_name::#variant_ident,
        });

    let from_field_name_constructs = field_name_constructs.clone();

    let field_name_variants = fields.iter()
        .map(|(_, _, variant_ident)| quote! {
            #variant_ident
        });

    let field_type_variants = fields.iter()
        .map(|(_, field_ty, variant_ident)| quote! {
            #variant_ident(#field_ty)
        });

    let field_type_constructs = fields.iter()
        .map(|(field_ident, _, variant_ident)| quote! {
            #enum_data::#variant_ident(#field_ident)
        });

    let from_field_type_constructs = field_type_constructs.clone();

    let fields_idents = fields.iter()
        .map(|(field_ident, _, _)| quote! {
            #field_ident
        });
    
    let field_name_by_strs = fields.iter()
        .map(|(field_ident, _, variant_ident)| {
            let field_name = field_ident.to_string();
            quote! {
                #field_name => Ok(#enum_name::#variant_ident)
            }
        });

    let field_name_to_strs = fields.iter()
        .map(|(field_ident, _, variant_ident)| {
            let field_name = field_ident.to_string();
            quote! {
                #enum_name::#variant_ident => #field_name
            }
        });

    let fields_idents_cpy = fields_idents.clone();

    let destructuring = quote! { #ty { #(#fields_idents_cpy,)* .. } };

    let fields_count = fields.len();

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let parts = fields.iter()
    .map(|(field_ident, _, variant_ident)| {
        quote! {
            #field_ident: match #field_ident {
                #enum_data::#variant_ident(value) => value,
                _ => return Err(StrunemixFromError::WrongOrder),
            }
        }
    });

    let fields_count_impl = if generics.params.is_empty() {
        quote! {
            impl #ty {
                #vis const FIELDS_COUNT: usize = #fields_count;
            }
        }
    } else {
        quote! {
            impl #impl_generics #ty #ty_generics
                #where_clause
            {
                pub const FIELDS_COUNT: usize = #fields_count;
            }
        }
    };

    let from_lifetime = quote! { 'field_name_from_lifetime__ };

    let mut impl_generics_tokens = TokenStream2::new();
    impl_generics.to_tokens(&mut impl_generics_tokens);
    if impl_generics_tokens.is_empty() {
        impl_generics_tokens = quote! { <#from_lifetime> };
    } else {
        let mut tokens: Vec<_> = quote! { #from_lifetime, }.into_iter().collect();
        let mut gen_iter = impl_generics_tokens.into_iter();
        if let Some(token) = gen_iter.next() {
            tokens.insert(0, token);
        }
        tokens.extend(gen_iter);
        impl_generics_tokens = TokenStream2::from_iter(tokens);
    }

    let converter = if generics.params.is_empty() {

        quote! {

            //Array
            impl From<#ty> for [#enum_data; #fields_count] {
                fn from(source: #ty) -> Self {
                    let #destructuring = source;
                    [#(#from_field_type_constructs),*]
                }
            }

            impl From<#enum_data> for #enum_name {
                fn from(source: #enum_data) -> Self {
                    match source {
                        #(#field_type_to_variant)*
                    }
                }
            }

        }
    } else {

        quote! {

            impl #impl_generics From<#ty #ty_generics> for [#enum_data #ty_generics; #fields_count]
                #where_clause
            {
                fn from(source: #ty #ty_generics) -> Self {
                    let #destructuring = source;
                    [#(#from_field_type_constructs),*]
                }
            }

            impl #impl_generics From<#enum_data #ty_generics> for #enum_name
                #where_clause
            {
                fn from(source: #enum_data #ty_generics) -> #enum_name {
                    match source {
                        #(#field_type_to_variant)*
                    }
                }
            }
        }
    };

    let tryfromarray = match (haveskippedfields, generics.params.is_empty(), !have_default) {
        (true, true, true) => quote! {},
        (true, true, false) => quote! {
            impl TryFrom<[#enum_data; #fields_count]> for #ty
            {
                type Error = StrunemixFromError;
                fn try_from(source: [#enum_data; #fields_count]) -> Result<Self, Self::Error> {

                    let [#(#fields_idents),*] = source;

                    Ok(#ty {
                        #(#parts,)*
                        ..Default::default()
                    })
                }
            }
        },
        (false, true, _) => quote! {

            impl TryFrom<[#enum_data; #fields_count]> for #ty
            {
                type Error = StrunemixFromError;
                fn try_from(source: [#enum_data; #fields_count]) -> Result<Self, Self::Error> {

                    let [#(#fields_idents),*] = source;

                    Ok(#ty {
                        #(#parts,)*
                    })
                }
            }
        },
        (true, false, true) => quote! {},
        (true, false, false) => quote! {
            impl #impl_generics TryFrom<[#enum_data #ty_generics; #fields_count]> for #ty #ty_generics
                #where_clause
            {
                type Error = StrunemixFromError;
                fn try_from(source: [#enum_data #ty_generics; #fields_count]) -> Result<Self, Self::Error> {
                    
                    let [#(#fields_idents),*] = source;

                    Ok(#ty {
                        #(#parts,)*
                        ..Default::default()
                    })
                }
            }
        },
        (false, false, _) => quote! {
            impl #impl_generics TryFrom<[#enum_data #ty_generics; #fields_count]> for #ty #ty_generics
                #where_clause
            {
                type Error = StrunemixFromError;
                fn try_from(source: [#enum_data #ty_generics; #fields_count]) -> Result<Self, Self::Error> {
                    
                    let [#(#fields_idents),*] = source;

                    Ok(#ty {
                        #(#parts,)*
                    })
                }
            }
        }

    };

    let enum_name_str = enum_name.to_string();
    let tokens = quote! {

        //name
        #derive_name
        #vis enum #enum_name {
            #(#field_name_variants),*
        }

        impl StrunemixName for #enum_name {
            fn get_str(&self) -> &'static str {
                match *self {
                    #(#field_name_to_strs),*
                }
            }
        }

        impl #impl_generics StrunemixData<#enum_name> for #enum_data #ty_generics #where_clause {}

        impl std::str::FromStr for #enum_name {
            type Err = StrunemixFromError;
            fn from_str(name: &str) -> Result<Self, Self::Err> {
                match name {
                    #(#field_name_by_strs),*,
                    _ => Err(StrunemixFromError::NotAnEnumName(name.to_string(), #enum_name_str.to_string())),
                }
            }
        }

        impl #impl_generics_tokens From<& #from_lifetime #ty #ty_generics> for [#enum_name; #fields_count] {
            fn from(_source: & #from_lifetime #ty #ty_generics) -> Self {
                [#(#from_field_name_constructs),*]
            }
        }


        // type
        #derive_type
        #vis enum #enum_data #generics
            #where_clause
        {
            #(#field_type_variants),*
        }

        #tryfromarray

        #converter

        #fields_count_impl

        impl #impl_generics StrunemixTrait<#enum_name, #enum_data #ty_generics, #fields_count> for #ty #ty_generics
            #where_clause
        {

            fn to_attr_data_array(self) -> [#enum_data #ty_generics; #fields_count] {
                let #destructuring = self;
                [#(#field_type_constructs),*]
            }
            
            fn as_attr_name_array() -> [#enum_name; #fields_count] {
                [#(#field_name_constructs),*]
            }
        }

    };
    tokens.into()
}

fn get_enum_derive(attrs: &[Attribute], derive_attr_names: &[&str], default: TokenStream2) -> TokenStream2 {

    attrs.iter()
    .find_map(|attr| 
        
        derive_attr_names.iter()
        .find_map(|attr_name| {
            if attr.path().is_ident(attr_name) {
                if let Meta::List(meta_list) = &attr.meta {
                    let mut meta_list = meta_list.clone();
                    meta_list.path = Path {
                        leading_colon: None,
                        segments: {
                            let mut segments = Punctuated::new();
                            segments.push(PathSegment {
                                ident: Ident::new("derive", Span::call_site()),
                                arguments: PathArguments::None,
                            });
                            segments
                        }
                    };
                    return Some(quote! { #[#meta_list] });
                }
            }
            None
        })

    )
    .unwrap_or(default)

}

fn filter_fields(fields: &Fields) -> Vec<(Ident, Type, Ident)> {
    fields.iter()
        .filter_map(|field| {
            if field.attrs.iter()
                .find(|attr| has_skip_attr(attr))
                .is_none() && field.ident.is_some()
            {
                let field_ty = field.ty.clone();
                let field_ident = field.ident.as_ref().unwrap().clone();
                let field_name = field.ident.as_ref().unwrap().to_string();
                let variant_ident = Ident::new(&field_name.to_upper_camel_case(), Span::call_site());
                Some((field_ident, field_ty, variant_ident))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

fn has_skip_attr(attr: &Attribute) -> bool {

    if !attr.path().is_ident("strunemix"){
        return false;
    }

    attr.parse_nested_meta(|meta| {
        if meta.path.is_ident("skip") {
            return Ok(());
        }

        Err(meta.error("Unknown attribute value, only `skip` allowed."))
    })
    .is_ok()

}