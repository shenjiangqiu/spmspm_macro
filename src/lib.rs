pub(crate) mod common;
mod jump_cycles;
mod jump_cycles_struct;

/// ## rust function
/// ## Author: Jiangqiu Shen
/// ## Date: 2023-05-11
/// Description: generate a struct and all functions.the struct contains all the jump cycles.all filed should impl JumpCycle,UpdatableJumpCycle,AddableJumpCycle.
#[proc_macro]
pub fn jump_cycles(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = proc_macro2::TokenStream::from(_item);
    let output = jump_cycles::jump_cycles_inner(item);
    output.into()
}
/// ## rust function
/// ## Author: Jiangqiu Shen
/// ## Date: 2023-05-11
/// Description: add the functions to the struct. all filed should impl JumpCycle,UpdatableJumpCycle,AddableJumpCycle.
#[proc_macro_derive(JumpCyclesStruct)]
pub fn jump_cycles_struct(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let output = jump_cycles_struct::jump_cycles_struct_inner(input);
    output.into()
}
