#[test]
fn trybuild() {
    let t = trybuild::TestCases::new();
    t.pass("trybuild/ok/*.rs");
    t.compile_fail("trybuild/err/*.rs");
}
