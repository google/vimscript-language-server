use crate::parse;
use expect_test::expect_file;
use std::path::PathBuf;

#[derive(Debug)]
struct TestCase {
    vim: PathBuf,
    ast: PathBuf,
}

fn read_test_cases() -> Vec<TestCase> {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("test_data/parser/");

    let mut entries = std::fs::read_dir(d)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()
        .unwrap();
    entries.sort();
    println!("{:?}", entries);

    let before = entries
        .iter()
        .filter(|path| path.to_str().unwrap().ends_with(".vim"));
    let after = entries
        .iter()
        .filter(|path| path.to_str().unwrap().ends_with(".ast"));

    before
        .zip(after)
        .map(|pair| TestCase {
            vim: pair.0.clone(),
            ast: pair.1.clone(),
        })
        .collect()
}

#[derive(PartialEq, Eq)]
#[doc(hidden)]
pub struct PrettyString<'a>(pub &'a str);

/// Make diff to display string as multi-line string
impl<'a> std::fmt::Debug for PrettyString<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

#[test]
fn parser() {
    println!("Running");
    for case in read_test_cases() {
        println!("Testing {:?}", case);
        let content = std::fs::read_to_string(&case.vim).unwrap();
        let parsed = parse(&content);

        let debug_dump = format!("{:#?}", parsed.syntax());
        expect_file![&case.ast].assert_eq(&debug_dump);
    }
}
