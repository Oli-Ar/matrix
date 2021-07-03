#[test]
fn trybuild() {
    let t = trybuild::TestCases::new();

    t.compile_fail("./tests/trybuild/diff_len_array.rs");
    t.compile_fail("./tests/trybuild/multi_dim_array.rs");
    t.compile_fail("./tests/trybuild/add_incompatible.rs");
}
