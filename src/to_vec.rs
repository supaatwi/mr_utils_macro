use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_str, Attribute, Data, DeriveInput, Fields, Ident, Meta, Type};

#[derive(Default)]
struct FieldAttrs {
    default: Option<String>,
    format: Option<String>,
    deserialize_with: Option<Ident>, 
}

fn get_to_vec_attrs(attrs: &[Attribute]) -> FieldAttrs {
    let mut field_attrs = FieldAttrs::default();

    for attr in attrs {
        if attr.path().is_ident("to_vec") {
            if let Meta::List(list) = &attr.meta {
                let content = list.tokens.to_string();
                for part in content.split(',') {
                    let kv: Vec<&str> = part.split('=').map(|s| s.trim()).collect();
                    if kv.len() == 2 {
                        match kv[0] {
                            "default" => {
                                field_attrs.default = Some(kv[1].trim_matches('"').to_string());
                            }
                            "format" => {
                                field_attrs.format = Some(kv[1].trim_matches('"').to_string());
                            }
                            "deserialize_with" if !kv[1].trim().is_empty() => {
                                // Parse function name into Ident
                                if let Ok(ident) = parse_str::<Ident>(&kv[1].trim_matches('"')) {
                                    field_attrs.deserialize_with = Some(ident);
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    field_attrs
}

pub(crate) fn impl_to_vec(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields.named.iter().collect::<Vec<_>>(),
            _ => panic!("ToVec only works with named fields"),
        },
        _ => panic!("ToVec only works with structs"),
    };

    let field_details = fields.iter().map(|f| {
        let ident = f.ident.as_ref().unwrap();
        let field_name = ident.to_string();
        
        let is_option = if let Type::Path(type_path) = &f.ty {
            if let Some(segment) = type_path.path.segments.first() {
                segment.ident == "Option"
            } else {
                false
            }
        } else {
            false
        };

        let attrs = get_to_vec_attrs(&f.attrs);
        
        (ident, field_name, is_option, attrs)
    }).collect::<Vec<_>>();

    let field_conversions = field_details.iter().map(|(ident, name, is_option, attrs)| {
        let default_str = attrs.default.clone().unwrap_or_else(|| "".to_string());
        let format_str = attrs.format.clone();
        let deserialize_fn = attrs.deserialize_with.clone();

        let value_conversion = if let Some(func) = deserialize_fn {
            if *is_option {
                quote! {
                    self.#ident.as_ref().map_or_else(
                        || #default_str.to_string(),
                        |v| #func(&v.to_string())
                    )
                }
            } else {
                quote! {
                    #func(&self.#ident.to_string())
                }
            }
        } else if *is_option {
            if let Some(fmt) = format_str {
                quote! {
                    self.#ident.as_ref().map_or_else(|| #default_str.to_string(), |v| format!(#fmt, v))
                }
            } else {
                quote! {
                    self.#ident.as_ref().map_or_else(|| #default_str.to_string(), |v| v.to_string())
                }
            }
        } else {
            if let Some(fmt) = format_str {
                quote! {
                    format!(#fmt, self.#ident)
                }
            } else {
                quote! {
                    self.#ident.to_string()
                }
            }
        };

        quote! {
            (#name, #value_conversion)
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