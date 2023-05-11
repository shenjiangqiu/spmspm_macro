use proc_macro2::{Ident, Span};
use syn::{parse::Parse, punctuated::Punctuated, token::Comma, Expr, Lit, Token};

use crate::common;

struct StructList(syn::Ident, Vec<syn::Type>);
impl Parse for StructList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let struct_name = Ident::parse(input)?;
        let _: Token![;] = input.parse().unwrap();
        let types = Punctuated::<syn::Type, Comma>::parse_terminated(input)?;
        Ok(StructList(struct_name, types.into_iter().collect()))
    }
}

pub(crate) fn jump_cycles_inner(item: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    // the input is a list of structs seperated by commas
    // we need to parse the input into a list of structs
    let structs: StructList = syn::parse2(item).unwrap();
    let mut output = proc_macro2::TokenStream::new();
    let mut jump_type_enum_names = vec![];
    let struct_items = structs.1;
    let struct_name = structs.0;
    let jump_type_variable_names: Vec<_> = struct_items
        .iter()
        .map(|item| match item {
            syn::Type::Path(path) => {
                let path = path.path.clone();
                let segs = path.segments;
                let s = segs.into_iter().last().unwrap();
                let args = s.arguments;
                let ident = s.ident;

                let mut type_args = vec![];
                match args {
                    syn::PathArguments::None => {}
                    syn::PathArguments::AngleBracketed(angle) => {
                        let args = angle.args;
                        args.into_iter().for_each(|a| match a {
                            syn::GenericArgument::Const(c) => match c {
                                Expr::Lit(lit) => match lit.lit {
                                    Lit::Int(int_lit) => {
                                        let int_number = int_lit.base10_digits();
                                        type_args.push(int_number.to_string());
                                    }

                                    _ => panic!("the lit should be int"),
                                },

                                _ => panic!("only support literal generics"),
                            },

                            _ => panic!("only support const generacs"),
                        });
                    }
                    syn::PathArguments::Parenthesized(_) => {
                        panic!("not support parenchesized genrics")
                    }
                }
                jump_type_enum_names.push(format!("{}{}", ident.to_string(), type_args.join("G")));
                let snake_case = heck::AsSnakeCase(ident.to_string());
                let under_scored_consts = type_args.join("_");
                if type_args.is_empty() {
                    format!("{}", snake_case,)
                } else {
                    format!("{}_{}", snake_case, under_scored_consts)
                }
            }
            _ => panic!("only support normal types"),
        })
        .collect();

    let jump_type_variable_idents = jump_type_variable_names
        .iter()
        .map(|name| Ident::new(&name, Span::call_site()));

    // first create the struct that conains all the fiends
    let struct_token_stream =
        super::common::generate_struct_impl(&struct_name, jump_type_variable_idents.clone());

    let jump_type_enum_idents = jump_type_enum_names
        .iter()
        .map(|name| Ident::new(&name, Span::call_site()));
    let type_iter_stream = super::common::generate_type_enum(&struct_name, jump_type_enum_idents);
    output.extend(common::generate_basic_type_definitions());
    output.extend(common::generate_trait_definitions(&struct_name));
    output.extend(common::generate_struct_definition(
        &struct_name,
        jump_type_variable_idents,
        struct_items.into_iter(),
    ));
    output.extend(struct_token_stream);
    output.extend(type_iter_stream);
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn test() {
        let tokens = quote!(MyTypes; Arrow,BigCackGoods,CackBoughtByMe<233,445>);
        let output = jump_cycles_inner(tokens);
        println!("{}", output);
    }
}
