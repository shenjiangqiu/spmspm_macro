use syn::{Data, DataStruct, FieldsNamed, Generics};

use crate::common;

pub(crate) fn jump_cycles_struct_inner(input: syn::DeriveInput) -> proc_macro2::TokenStream {
    let syn::DeriveInput {
        ident,
        generics,
        data,
        ..
    } = input;
    let Generics { params, .. } = generics;
    if !params.is_empty() {
        panic!("JumpCyclesStruct does not support generics")
    }
    if let Data::Struct(DataStruct {
        fields: syn::Fields::Named(FieldsNamed { named, .. }),
        ..
    }) = data
    {
        let indents = named.iter().map(|f| f.ident.clone().unwrap());

        // let types = named.iter().map(|f| f.ty.clone());
        let mut token_stream = proc_macro2::TokenStream::new();
        let jump_type_enum_idents = indents
            .clone()
            .into_iter()
            .map(|id| syn::Ident::new(&heck::AsPascalCase(id.to_string()).to_string(), id.span()));

        let struct_stream = common::generate_struct_impl(&ident, indents.clone());
        let enum_stream = common::generate_type_enum(&ident, jump_type_enum_idents);

        token_stream.extend(common::generate_basic_type_definitions());
        token_stream.extend(common::generate_trait_definitions(&ident));

        token_stream.extend(struct_stream);
        token_stream.extend(enum_stream);
        token_stream
    } else {
        panic!("JumpCyclesStruct only supports structs")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let tokens = syn::parse_quote! {
            struct MyDuduAA{
                aaa:AAA,
                bbb:BBB,
                bb33:BB<44>,
                cc:CC<44,55>,
            }
        };
        let output = jump_cycles_struct_inner(tokens);
        println!("{}", output);
    }
}
