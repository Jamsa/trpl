// a crate that include a derive macro
// we treat this module as a crate, user can use this crate without the macros
// so we defined the macros in hello_macro_derive as a feature
pub trait HelloMacro {
    fn hello_macro();
}
