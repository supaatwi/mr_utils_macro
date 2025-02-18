use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Data, DeriveInput, Fields, Meta, Type};

fn get_to_vec_attrs(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("to_vec") {
            if let Meta::List(list) = &attr.meta {
                let mut default = None;

                let content = list.tokens.to_string();
                for part in content.split(',') {
                    let kv: Vec<&str> = part.split('=').map(|s| s.trim()).collect();
                    if kv.len() == 2 {
                        match kv[0] {
                            "default" => {
                                default = Some(kv[1].trim_matches('"').to_string());
                            }
                            _ => {}
                        }
                    }
                }

                if let Some(default) = default {
                    return Some(default);
                }
            }
        }
    }
    None
}

pub(crate) fn impl_to_vec(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;

    // Get fields and their types from the struct
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields.named.iter().collect::<Vec<_>>(),
            _ => panic!("ToVec only works with named fields"),
        },
        _ => panic!("ToVec only works with structs"),
    };

    // Collect field information including type checks and default values
    let field_details = fields.iter().map(|f| {
        let ident = f.ident.as_ref().unwrap();
        let field_name = ident.to_string();
        
        // Check if field type is Option<T>
        let is_option = if let Type::Path(type_path) = &f.ty {
            if let Some(segment) = type_path.path.segments.first() {
                segment.ident == "Option"
            } else {
                false
            }
        } else {
            false
        };

        // Get default value from attribute
        let default_value = get_to_vec_attrs(&f.attrs).unwrap_or_else(|| "".to_string());

        (ident, field_name, is_option, default_value)
    }).collect::<Vec<_>>();

    // Generate field conversions based on type and default values
    let field_conversions = field_details.iter().map(|(ident, name, is_option, default)| {
        if *is_option {
            let default_str = default.clone();
            quote! {
                (#name, self.#ident.as_ref().map_or_else(|| #default_str.to_string(), |v| v.to_string()))
            }
        } else {
            quote! {
                (#name, self.#ident.to_string())
            }
        }
    });

    let gen = quote! {
        impl #name {
            pub fn to_vec(&self, fields: Option<&[&str]>) -> Vec<String> {
                let all_fields: &[(&str, String)] = &[
                    #(#field_conversions,)*
                ];

                match fields {
                    Some(field_names) => {
                        field_names
                            .iter()
                            .filter_map(|&name| {
                                all_fields
                                    .iter()
                                    .find(|(field, _)| field == &name)
                                    .map(|(_, value)| value.clone())
                            })
                            .collect()
                    }
                    None => all_fields.iter().map(|(_, value)| value.clone()).collect()
                }
            }
        }
    };

    gen.into()
}