#[test]
fn test_macros() {
    let t = trybuild::TestCases::new();
    // Test successful cases
    t.pass("tests/ui/*.rs");

    // Test failing cases
    t.compile_fail("tests/ui/fail/*.rs");
}
