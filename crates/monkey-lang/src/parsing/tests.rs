use std::{
    fs,
    path::{Path, PathBuf},
};

use itertools::{Either, Itertools};

use crate::{ast, parse, tokenize, ParseError, ParseResult};

#[test]
fn parser() {
    dir_tests(snapshots_dir(), &["parser/ok"], |path, text| {
        let parse = parse::<ast::File>(&text);
        assert_success(&parse, path);
        format!("{:#?}", parse)
    })
}

#[test]
fn lexer() {
    dir_tests(snapshots_dir(), &["lexer/ok"], |path, text| {
        let (tokens, errors) = partition_tokenize(&text);
        assert_errors_are_absent(&errors, path);
        format!("{:#?}", tokens)
    })
}

fn partition_tokenize(input: &str) -> (Vec<ast::Token>, Vec<ParseError>) {
    tokenize(input).into_iter().partition_map(|r| match r {
        Ok(v) => Either::Left(v),
        Err(v) => Either::Right(v),
    })
}

fn assert_errors_are_absent(errors: &[ParseError], path: &Path) {
    assert_eq!(
        errors,
        &[] as &[ParseError],
        "There should be no errors in the path {}",
        path.display()
    )
}

fn assert_tokens_success(res: Vec<ParseResult<ast::Token>>, path: &Path) {
    let errors = res.into_iter().filter_map(|res| res.ok()).collect::<Vec<_>>();
    assert_eq!(errors, &[], "There should be no errors in the path {}", path.display());
}

fn assert_success(res: &ParseResult<ast::File>, path: &Path) {
    if let Err(e) = res {
        panic!("The path {} failed:\n{}", path.display(), e)
    }
}

fn dir_tests<P, F>(test_data_dir: P, paths: &[&str], f: F)
where
    P: AsRef<Path>,
    F: Fn(&Path, &str) -> String,
{
    for (path, input_code) in collect_monkey_files(test_data_dir.as_ref(), paths) {
        assert_path(&path, &f(&path, &input_code))
    }
}

fn assert_path<P>(path: P, actual: &str)
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(path.parent().expect("There is no parent for the given path"));
    settings.set_prepend_module_to_snapshot(false);
    settings.set_input_file(path);
    settings.bind(|| {
        let name =
            path.file_stem().expect("No file stem").to_str().expect("Could not turn path to str");
        insta::assert_snapshot!(name, actual);
    })
}

fn collect_monkey_files(root_dir: &Path, paths: &[&str]) -> Vec<(PathBuf, String)> {
    paths
        .iter()
        .flat_map(|path| {
            let path = root_dir.join(path);
            files_in_dir(&path, "mk").into_iter()
        })
        .map(|path| {
            let text = read_text(&path);
            (path, text)
        })
        .collect()
}

fn files_in_dir(dir: &Path, extension: &str) -> Vec<PathBuf> {
    let mut acc = Vec::new();
    for file in fs::read_dir(&dir).unwrap() {
        let file = file.unwrap();
        let path = file.path();
        if path.extension().unwrap_or_default() == extension {
            acc.push(path);
        }
    }
    acc.sort();
    acc
}

/// Read file and normalize newlines.
///
/// `rustc` seems to always normalize `\r\n` newlines to `\n`:
///
/// ```
/// let s = "
/// ";
/// assert_eq!(s.as_bytes(), &[10]);
/// ```
///
/// so this should always be correct.
fn read_text(path: &Path) -> String {
    fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("File at {:?} should be valid", path))
        .replace("\r\n", "\n")
}

fn snapshots_dir() -> PathBuf {
    project_root().join("crates/monkey-lang/src/parsing/snapshots")
}

fn project_root() -> PathBuf {
    let dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(dir).parent().unwrap().parent().unwrap().to_owned()
}
