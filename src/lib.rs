use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse::Parse, punctuated::Punctuated, token::Comma, Expr, Lit};

#[proc_macro]
pub fn jump_cycles(_item: TokenStream) -> TokenStream {
    let item = proc_macro2::TokenStream::from(_item);
    let output = jump_cycles_inner(item);
    output.into()
}
struct StructList(Vec<syn::Type>);
impl Parse for StructList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let result = Punctuated::<syn::Type, Comma>::parse_terminated(input)?;
        Ok(StructList(result.into_iter().collect()))
    }
}

fn jump_cycles_inner(item: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    // the input is a list of structs seperated by commas
    // we need to parse the input into a list of structs
    let structs: StructList = syn::parse2(item).unwrap();
    let mut output = proc_macro2::TokenStream::new();
    let mut jump_type_enum_names = vec![];
    let mut jump_type_variable_names = vec![];
    let struct_items = structs.0;
    struct_items.iter().for_each(|item| match item {
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
                syn::PathArguments::Parenthesized(_) => panic!("not support parenchesized genrics"),
            }
            jump_type_enum_names.push(format!("{}{}", ident.to_string(), type_args.join("G")));
            let snake_case = heck::AsSnakeCase(ident.to_string());
            let under_scored_consts = type_args.join("_");
            if type_args.is_empty() {
                jump_type_variable_names.push(format!("{}", snake_case,));
            } else {
                jump_type_variable_names.push(format!("{}_{}", snake_case, under_scored_consts));
            }
        }
        _ => panic!("only support normal types"),
    });

    let jump_type_variable_idents = jump_type_variable_names
        .iter()
        .map(|name| Ident::new(&name, Span::call_site()));
    let jump_type_variable_idents_apply = jump_type_variable_idents.clone();
    let jump_type_variable_idents_apply_mut = jump_type_variable_idents.clone();
    let jump_type_variable_idents_apply_mut_1 = jump_type_variable_idents.clone();
    let jump_type_variable_idents_apply_mut_2 = jump_type_variable_idents.clone();
    let jump_type_variable_idents_apply_reduce_1 = jump_type_variable_idents.clone();
    let jump_type_variable_idents_apply_reduce_2 = jump_type_variable_idents.clone();

    let jump_type_enum_idents = jump_type_enum_names
        .iter()
        .map(|name| Ident::new(&name, Span::call_site()));
    // first create the struct that conains all the fiends
    let struct_token_stream = quote!(
        macro_rules! generate_id{
            ($t:ty;$($name:ident),+ $(,)?) => {
                $(
                    /// a wrapper for the id
                    #[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq,Hash )]
                    #[repr(transparent)]
                    pub struct $name(pub $t);
                    impl $name {
                        pub fn new(id: $t) -> Self {
                            Self(id)
                        }
                    }
                    impl std::fmt::Debug for $name {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            self.0.fmt(f)
                        }
                    }
                    impl std::fmt::Display for $name {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            self.0.fmt(f)
                        }
                    }
                    impl std::ops::Deref for $name {
                        type Target = $t;
                        fn deref(&self) -> &Self::Target {
                            &self.0
                        }
                    }
                    impl std::ops::DerefMut for $name {
                        fn deref_mut(&mut self) -> &mut Self::Target {
                            &mut self.0
                        }
                    }
                    impl std::convert::From<$t> for $name {
                        fn from(id: $t) -> Self {
                            Self(id)
                        }
                    }
                    impl std::convert::From<$name> for $t {
                        fn from(id: $name) -> Self {
                            id.0
                        }
                    }
                    impl Default for $name {
                        fn default() -> Self {
                            Self(0)
                        }
                    }

                )+
            };
        }

        generate_id!(
            usize;
            LogicRowId,
            LogicColId,
            PhysicRowId,
            PhysicColId,
            SubarrayId,
            RingId,
            RingBufferId,
            TsvId,
            WordId,
        );

        impl PhysicColId {
            pub fn word_id(&self) -> WordId {
                WordId(self.0 / 4)
            }
        }
        generate_id!(u8;RingPort);

        #[derive(Debug, Default, Clone)]
        pub struct RowIdWordId {
            pub row_id: PhysicRowId,
            pub word_id: WordId,
        }
        impl RowIdWordId{
            pub fn new(row_id:PhysicRowId,word_id:WordId)->Self{
                Self{
                    row_id,
                    word_id,
                }
            }
        }
        #[derive(Debug, Default, Clone)]
        pub struct RowLocation {
            pub subarray_id: SubarrayId,
            pub row_id_world_id: RowIdWordId,
        }
        impl RowLocation{
            pub fn new(subarray_id:SubarrayId,row_id_world_id:RowIdWordId)->Self{
                Self{
                    subarray_id,
                    row_id_world_id,
                }
            }
        }
        #[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone, Copy)]
        pub struct JumpCycles{
            #(
                pub #jump_type_variable_idents :#struct_items
            ),*
        }
        pub trait JumpCycle {
            fn total(&self) -> usize;
            fn get_one_jump(&self) -> usize;
            fn get_multi_jump(&self) -> usize;
            fn get_one_jump_mut(&mut self) -> &mut usize;
            fn get_multi_jump_mut(&mut self) -> &mut usize;
        }

        pub trait AddableJumpCycle: JumpCycle {
            fn add(&mut self, jump_cycle: &Self);
        }
        pub trait UpdatableJumpCycle {
            fn update(
                &mut self,
                row_status: &RowIdWordId,
                loc: &RowLocation,
                size: WordId,
                remap_cycle: usize,
            );
        }
        pub trait RowCycleAction {
            fn apply<T: JumpCycle + UpdatableJumpCycle + AddableJumpCycle>(&mut self, item: &T);
        }
        pub trait RowCycleActionMut {
            fn apply_mut<T: JumpCycle + UpdatableJumpCycle + AddableJumpCycle>(&mut self, item: &mut T);
        }
        pub trait RowCycleActionPairMut {
            fn apply_pair_mut<T: JumpCycle + UpdatableJumpCycle + AddableJumpCycle>(
                &mut self,
                source: &T,
                target: &mut T,
            );
        }
        pub trait RowCycleArrayReduce{
            fn apply_reduce<T:JumpCycle + UpdatableJumpCycle + AddableJumpCycle>(&mut self,source:&[JumpCycles],target:&mut T, mapper:impl FnMut(&JumpCycles)->&T);
        }
        impl JumpCycles {
            pub fn apply(&self, action: &mut impl RowCycleAction) {
                #(action.apply(&self.#jump_type_variable_idents_apply);)*
            }
            pub fn apply_mut(&mut self, action: &mut impl RowCycleActionMut) {
                #(action.apply_mut(&mut self.#jump_type_variable_idents_apply_mut);)*
            }
            pub fn apply_pair_mut(
                &self,
                target: &mut Self,
                action: &mut impl RowCycleActionPairMut,
            ) {
                #(action.apply_pair_mut(&self.#jump_type_variable_idents_apply_mut_1, &mut target.#jump_type_variable_idents_apply_mut_2);)*
            }
            pub fn apply_reduce(input_array:&[Self],target:&mut Self,action:&mut impl RowCycleArrayReduce){
                #(action.apply_reduce(input_array,&mut target.#jump_type_variable_idents_apply_reduce_1,|item| &item.#jump_type_variable_idents_apply_reduce_2);)*
            }
        }

    );
    let first_jump_type_ident = Ident::new(&jump_type_enum_names[0], Span::call_site());
    let jump_type_enum_idents_clone = jump_type_enum_idents.clone();

    let type_defines_stream = quote!(
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub enum JumpCyclesTypes{
            #(
                #jump_type_enum_idents_clone,
            )*
            End,
        }
        impl Default for JumpCyclesTypes{
            fn default()->Self{
                Self::#first_jump_type_ident
            }
        }
    );
    let jump_type_enum_next_idents = jump_type_enum_names
        .iter()
        .map(|x| x.as_str())
        .skip(1)
        .chain(["End"])
        .map(|name| Ident::new(name, Span::call_site()));
    let jump_type_enum_idents_clone = jump_type_enum_idents.clone();
    let type_iter_stream = quote!(
        impl JumpCyclesTypes{
            fn move_to_next(&mut self){
                *self = match self{
                    #(
                        JumpCyclesTypes::#jump_type_enum_idents_clone=>JumpCyclesTypes::#jump_type_enum_next_idents,
                    )*
                    JumpCyclesTypes::End=>JumpCyclesTypes::End,
                }
            }
        }
        impl Iterator for JumpCyclesTypes {
            type Item = JumpCyclesTypes;

            fn next(&mut self) -> Option<Self::Item> {
                let current = self.clone();
                if current == JumpCyclesTypes::End {
                    return None
                }
                self.move_to_next();
                return Some(current)
            }
        }
    );

    output.extend(struct_token_stream);
    output.extend(type_defines_stream);
    output.extend(type_iter_stream);
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    #[test]
    fn test() {
        let tokens = quote!(Arrow,BigCackGoods,CackBoughtByMe<233,445>);
        let output = jump_cycles_inner(tokens);
        println!("{}", output);
    }
}
