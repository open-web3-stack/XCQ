use pvq_extension_procedural::extension_decl;

// This should fail because extension_decl can only be used on modules
#[extension_decl]
struct InvalidUsage {
    field: u32,
}

fn main() {}
