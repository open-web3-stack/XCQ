use pvq_extension_procedural::{extension_decl, extensions_impl};

#[extension_decl]
pub mod test_extension {
    #[extension_decl::extension]
    pub trait TestExtension {
        type Value;
        fn test_fn(value: Self::Value) -> bool;
    }
}

#[extensions_impl]
mod test_impl {
    // Missing #[extensions_impl::impl_struct] attribute
    pub struct TestImpl;

    #[extensions_impl::extension]
    impl test_extension::TestExtension for TestImpl {
        type Value = u32;
        fn test_fn(value: u32) -> bool {
            value > 0
        }
    }
}

fn main() {}
