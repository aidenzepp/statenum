mod helper;
mod macros;

use helper::*;
use macros::*;

#[proc_macro_attribute]
pub fn statenum(_args: TokenStream, item: TokenStream) -> TokenStream {
    _statenum(_args, item)
}
