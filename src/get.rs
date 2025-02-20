// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{parse_macro_input, parse::Parse, parse::ParseStream, Token, Expr, Result, LitStr};


// struct GetArgs {
//     obj: Expr,
//     _comma1: Token![,],
//     path: LitStr,
//     _comma2: Token![,],
//     default: LitStr,
// }

// // implement Parse สำหรับ GetArgs
// pub(crate) impl Parse for GetArgs {
//     fn parse(input: ParseStream) -> Result<Self> {
//         Ok(GetArgs {
//             obj: input.parse()?,
//             _comma1: input.parse()?,
//             path: input.parse()?,
//             _comma2: input.parse()?,
//             default: input.parse()?
//         })
//     }
// }

// pub(crate) fn impl_get(input: &Expr) -> TokenStream {
    
//     let output = quote! {
//         {
//             let value = {
//                 let debug_str = format!("{:?}", #obj);
//                 let path_str = #path;
//                 let parts: Vec<&str> = path_str.split('.').collect();
                
//                 let cleaned_str = debug_str.replace('\n', "").replace(' ', "");
                
//                 let mut result = None;
//                 let mut current_path = String::new();
                
//                 for (i, part) in parts.iter().enumerate() {
//                     if i == 0 {
//                         current_path.push_str(part);
//                     } else {
//                         current_path.push_str(&format!(".{}", part));
//                     }
                    
//                     let pattern = format!("{}:", current_path);
//                     if let Some(start_idx) = cleaned_str.find(&pattern) {
//                         if i == parts.len() - 1 {
//                             let value_start = start_idx + pattern.len();
//                             let value_end = cleaned_str[value_start..].find(',')
//                                 .map(|i| value_start + i)
//                                 .unwrap_or_else(|| {
//                                     cleaned_str[value_start..].find('}')
//                                         .map(|i| value_start + i)
//                                         .unwrap_or(cleaned_str.len())
//                                 });
                            
//                             let value = cleaned_str[value_start..value_end].trim()
//                                 .trim_start_matches('"')
//                                 .trim_end_matches('"')
//                                 .to_string();
                            
//                             result = Some(value);
//                         }
//                     } else {
//                         result = None;
//                         break;
//                     }
//                 }
                
//                 result
//             };
            
//             value.unwrap_or_else(|| #default.to_string())
//         }
//     };
    
//     output.into()
// }