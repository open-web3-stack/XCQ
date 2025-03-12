#[test]
fn test_macros() {
    let t = trybuild::TestCases::new();
    // Test successful cases
    t.pass("tests/ui/extension_decl/pass/*.rs");
    t.pass("tests/ui/extensions_impl/pass/*.rs");

    // Test failing cases
    t.compile_fail("tests/ui/extension_decl/*.rs");
    t.compile_fail("tests/ui/extensions_impl/*.rs");
}
