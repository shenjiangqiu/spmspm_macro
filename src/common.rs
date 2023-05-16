//! ## rust module
//! ## Author: Jiangqiu Shen
//! ## Date: 2023-05-11
//! Description: all common mothods used in the crate.

use proc_macro2::Span;

/// ## rust function
/// ## Author: Jiangqiu Shen
/// ## Date: 2023-05-11
/// Description: generate a enum which represent all fiends in the struct. and a End is added to the end of the enum.
pub fn generate_type_enum(
    struct_ident: &syn::Ident,
    jump_type_enum_idents: impl Iterator<Item = syn::Ident> + Clone,
) -> proc_macro2::TokenStream {
    let jump_type_enum_idents_clone1 = jump_type_enum_idents.clone();
    let jump_type_enum_idents_clone2 = jump_type_enum_idents.clone();
    let first_jump_type_ident = jump_type_enum_idents.clone().next().unwrap();
    let next_jump_type_idents = jump_type_enum_idents
        .clone()
        .skip(1)
        .chain(std::iter::once(syn::Ident::new("End", Span::call_site())));
    let enum_ident = syn::Ident::new(&format!("{}Types", struct_ident), struct_ident.span());
    let type_iter_stream = quote:: quote!(

        /// ## rust function
        /// ## Author: Jiangqiu Shen
        /// ## Date: 2023-05-11
        /// Description: a enum contains all the fiends in the struct. and a End is added to the end of the enum.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub enum #enum_ident{
            #(
                #jump_type_enum_idents_clone1,
            )*
            End,
        }
        impl Default for #enum_ident{
            fn default()->Self{
                Self::#first_jump_type_ident
            }
        }
        impl #enum_ident{
            /// ## rust function
            /// ## Author: Jiangqiu Shen
            /// ## Date: 2023-05-11
            /// Description: move the enum to the next field. the last filed will move to End.
            fn move_to_next(&mut self){
                *self = match self{
                    #(
                        #enum_ident::#jump_type_enum_idents_clone2=>#enum_ident::#next_jump_type_idents,
                    )*
                    #enum_ident::End=>#enum_ident::End,
                }
            }
        }
        impl Iterator for #enum_ident {
            type Item = #enum_ident;

            fn next(&mut self) -> Option<Self::Item> {
                let current = self.clone();
                if current == #enum_ident::End {
                    return None
                }
                self.move_to_next();
                return Some(current)
            }
        }
        pub const TOTAL_TYPES_COUNT:usize = #enum_ident::End as usize;
    );
    type_iter_stream
}

/// ## rust function
/// ## Author: Jiangqiu Shen
/// ## Date: 2023-05-11
/// Description: generate all Id types.
pub fn generate_basic_type_definitions() -> proc_macro2::TokenStream {
    quote::quote!(
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
                    impl core::ops::Add for $name {
                        type Output = Self;
                        fn add(self, rhs: Self) -> Self::Output {
                            Self(self.0 + rhs.0)
                        }
                    }
                    impl core::ops::AddAssign for $name {
                        fn add_assign(&mut self, rhs: Self) {
                            self.0 += rhs.0;
                        }
                    }
                    impl core::ops::Sub for $name {
                        type Output = Self;
                        fn sub(self, rhs: Self) -> Self::Output {
                            Self(self.0 - rhs.0)
                        }
                    }
                    impl core::ops::SubAssign for $name {
                        fn sub_assign(&mut self, rhs: Self) {
                            self.0 -= rhs.0;
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

    )
}

/// ## rust function
/// ## Author: Jiangqiu Shen
/// ## Date: 2023-05-11
/// Description: generate all Trait needed for the struct.
pub fn generate_trait_definitions(struct_name: &syn::Ident) -> proc_macro2::TokenStream {
    quote::quote!(
        /// ## rust function
        /// ## Author: Jiangqiu Shen
        /// ## Date: 2023-05-11
        /// Description: JumpCycle: represent the cycles spent on jump
        pub trait JumpCycle {
            /// ## rust function
            /// ## Author: Jiangqiu Shen
            /// ## Date: 2023-05-11
            /// Description: all cycles spend on jump
            fn total(&self) -> usize;
            /// ## rust function
            /// ## Author: Jiangqiu Shen
            /// ## Date: 2023-05-11
            /// Description: the jump that is sequential access
            fn get_one_jump(&self) -> usize;
            /// ## rust function
            /// ## Author: Jiangqiu Shen
            /// ## Date: 2023-05-11
            /// Description: the jump that is random access
            fn get_multi_jump(&self) -> usize;
            /// ## rust function
            /// ## Author: Jiangqiu Shen
            /// ## Date: 2023-05-11
            /// Description: the jump that is sequential access
            fn get_one_jump_mut(&mut self) -> &mut usize;
            /// ## rust function
            /// ## Author: Jiangqiu Shen
            /// ## Date: 2023-05-11
            /// Description: the jump that is random access
            fn get_multi_jump_mut(&mut self) -> &mut usize;
        }
        /// ## rust function
        /// ## Author: Jiangqiu Shen
        /// ## Date: 2023-05-11
        /// Description: AddableJumpCycle: represent the Jump cycle that can be added into the current cycle
        pub trait AddableJumpCycle: JumpCycle {
            /// ## rust function
            /// ## Author: Jiangqiu Shen
            /// ## Date: 2023-05-11
            /// Description: add another JumpCycle into the current JumpCycle
            fn add(&mut self, jump_cycle: &Self);
        }
        /// ## rust function
        /// ## Author: Jiangqiu Shen
        /// ## Date: 2023-05-11
        /// Description: UpdatableJumpCycle: represent the Jump cycle that can be updated
        pub trait UpdatableJumpCycle {
            fn update(
                &mut self,
                row_status: &RowIdWordId,
                loc: &RowLocation,
                size: WordId,
                remap_cycle: usize,
            );
        }
        /// ## rust function
        /// ## Author: Jiangqiu Shen
        /// ## Date: 2023-05-11
        /// Description: a action which can be executed by the struct to apply on all fiends
        pub trait RowCycleAction {
            /// ## rust function
            /// ## Author: Jiangqiu Shen
            /// ## Date: 2023-05-11
            /// Description: the action applied on each field
            fn apply<T: JumpCycle + UpdatableJumpCycle + AddableJumpCycle>(&mut self, item: &T);
        }
        /// ## rust function
        /// ## Author: Jiangqiu Shen
        /// ## Date: 2023-05-11
        /// Description: a action which can be executed by the struct to apply on all fiends
        pub trait RowCycleActionMut {
            /// ## rust function
            /// ## Author: Jiangqiu Shen
            /// ## Date: 2023-05-11
            /// Description: the action applied on each field
            fn apply_mut<T: JumpCycle + UpdatableJumpCycle + AddableJumpCycle>(
                &mut self,
                item: &mut T,
            );
        }
        /// ## rust function
        /// ## Author: Jiangqiu Shen
        /// ## Date: 2023-05-11
        /// Description: a action which can be executed by the struct to apply on all fiends
        pub trait RowCycleActionPairMut {
            /// ## rust function
            /// ## Author: Jiangqiu Shen
            /// ## Date: 2023-05-11
            /// Description: the action applied on each field
            fn apply_pair_mut<T: JumpCycle + UpdatableJumpCycle + AddableJumpCycle>(
                &mut self,
                source: &T,
                target: &mut T,
            );
        }
        /// ## rust function
        /// ## Author: Jiangqiu Shen
        /// ## Date: 2023-05-11
        /// Description: a action which can be executed by the struct to apply on all fiends
        pub trait RowCycleArrayReduce {
            /// ## rust function
            /// ## Author: Jiangqiu Shen
            /// ## Date: 2023-05-11
            /// Description: the action reduce the field on all sources and apply on the target
            fn apply_reduce<T: JumpCycle + UpdatableJumpCycle + AddableJumpCycle>(
                &mut self,
                source: &[#struct_name],
                target: &mut T,
                mapper: impl FnMut(&#struct_name) -> &T,
            );
        }
    )
}

/// ## rust function
/// ## Author: Jiangqiu Shen
/// ## Date: 2023-05-11
/// Description: create the struct definition
pub fn generate_struct_definition(
    struct_ident: &syn::Ident,
    jump_type_variable_idents: impl Iterator<Item = syn::Ident>,
    struct_items: impl Iterator<Item = syn::Type>,
) -> proc_macro2::TokenStream {
    quote::quote!(
        /// ## rust function
        /// ## Author: Jiangqiu Shen
        /// ## Date: 2023-05-11
        /// Description: this struct contains all the JumpCycls that needed to be evaluated in the experiemnts.
        /// to manipulate the filed, use [`apply`], [`apply_mut`], [`apply_pair_mut`], [`apply_reduce`]
        #[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone, Copy)]
        pub struct #struct_ident{
            #(
                pub #jump_type_variable_idents :#struct_items
            ),*
        }

    )
}

/// ## rust function
/// ## Author: Jiangqiu Shen
/// ## Date: 2023-05-11
/// Description: generate the struct impl
pub fn generate_struct_impl(
    struct_ident: &syn::Ident,
    jump_type_variable_idents: impl Iterator<Item = syn::Ident> + Clone,
) -> proc_macro2::TokenStream {
    let jump_type_variable_idents_apply = jump_type_variable_idents.clone();
    let jump_type_variable_idents_apply_mut = jump_type_variable_idents.clone();

    let jump_type_variable_idents_apply_mut_1 = jump_type_variable_idents.clone();
    let jump_type_variable_idents_apply_mut_2 = jump_type_variable_idents.clone();
    let jump_type_variable_idents_apply_reduce_1 = jump_type_variable_idents.clone();
    let jump_type_variable_idents_apply_reduce_2 = jump_type_variable_idents.clone();

    let struct_token_stream = quote::quote!(


        impl #struct_ident {
            /// ## rust function
            /// ## Author: Jiangqiu Shen
            /// ## Date: 2023-05-11
            /// Description: apply the action on all fields
            pub fn apply(&self, action: &mut impl RowCycleAction) {
                #(action.apply(&self.#jump_type_variable_idents_apply);)*
            }
            /// ## rust function
            /// ## Author: Jiangqiu Shen
            /// ## Date: 2023-05-11
            /// Description: apply the action on all fields
            pub fn apply_mut(&mut self, action: &mut impl RowCycleActionMut) {
                #(action.apply_mut(&mut self.#jump_type_variable_idents_apply_mut);)*
            }
            /// ## rust function
            /// ## Author: Jiangqiu Shen
            /// ## Date: 2023-05-11
            /// Description: apply the action on all fields of target and self
            pub fn apply_pair_mut(
                &self,
                target: &mut Self,
                action: &mut impl RowCycleActionPairMut,
            ) {
                #(action.apply_pair_mut(&self.#jump_type_variable_idents_apply_mut_1, &mut target.#jump_type_variable_idents_apply_mut_2);)*
            }
            /// ## rust function
            /// ## Author: Jiangqiu Shen
            /// ## Date: 2023-05-11
            /// Description: for each field , do the reduce and apply on the target
            pub fn apply_reduce(input_array:&[Self],target:&mut Self,action:&mut impl RowCycleArrayReduce){
                #(action.apply_reduce(input_array,&mut target.#jump_type_variable_idents_apply_reduce_1,|item| &item.#jump_type_variable_idents_apply_reduce_2);)*
            }
        }

    );
    struct_token_stream
}
#[derive(serde::Deserialize, Debug)]
struct SimpleConfig {
    channels: usize,
    banks: usize,
    subarrays: usize,
    cols: usize,
    word_size: usize,
    walker_size: usize,
}
pub(crate) fn generate_default_config_inner(
    input: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let file_path: syn::LitStr = syn::parse2(input).unwrap();
    let simple_config: SimpleConfig =
        toml::from_str(&std::fs::read_to_string(file_path.value()).unwrap()).unwrap();
    let mut output = proc_macro2::TokenStream::new();
    let channels = syn::LitInt::new(
        &simple_config.channels.to_string(),
        proc_macro2::Span::call_site(),
    );
    let banks = syn::LitInt::new(
        &simple_config.banks.to_string(),
        proc_macro2::Span::call_site(),
    );
    let subarrays = syn::LitInt::new(
        &simple_config.subarrays.to_string(),
        proc_macro2::Span::call_site(),
    );
    let cols = syn::LitInt::new(
        &simple_config.cols.to_string(),
        proc_macro2::Span::call_site(),
    );
    let word_size = syn::LitInt::new(
        &simple_config.word_size.to_string(),
        proc_macro2::Span::call_site(),
    );
    let walker_size = syn::LitInt::new(
        &simple_config.walker_size.to_string(),
        proc_macro2::Span::call_site(),
    );

    let definations = quote::quote!(
        pub CONFIG_CHANNELS:usize=#channels;
        pub CONFIG_BANKS:usize=#banks;
        pub CONFIG_SUBARRAYS:usize=#subarrays;
        pub CONFIG_COLS:usize=#cols;
        pub CONFIG_WORD_SIZE:usize=#word_size;
        pub CONFIG_WALKER_SIZE:usize=#walker_size;
    );
    output.extend(definations);
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generate_default_config_inner() {
        let input = quote::quote!("src/test.toml");
        let output = generate_default_config_inner(input);
        println!("{}", output);
    }
}
