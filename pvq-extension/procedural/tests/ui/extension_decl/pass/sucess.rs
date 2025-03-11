use pvq_extension_procedural::extension_decl;

#[extension_decl]
pub mod test_extension {
    #[extension_decl::extension]
    pub trait TestExtension {
        type Value;
        fn test_fn(value: Self::Value) -> bool;
    }
}

fn main() {}
