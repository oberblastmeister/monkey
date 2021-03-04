#[test]
fn trybuild() {
    let t = trybuild::TestCases::new();
    t.pass("trybuild/01-recognize.rs");
}
