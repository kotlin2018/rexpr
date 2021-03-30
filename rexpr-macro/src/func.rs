use proc_macro2::{Ident, Span};
use quote::quote;
use quote::ToTokens;
use syn;
use syn::{AttributeArgs, ItemFn};

use crate::proc_macro::TokenStream;

pub(crate) fn impl_fn(f:&ItemFn, args:&AttributeArgs) ->TokenStream{

    return f.to_token_stream().into();
}