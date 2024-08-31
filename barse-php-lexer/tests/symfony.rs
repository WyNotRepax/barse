
use glob::glob;
use util::assert_file_lex_same;

mod util;
#[test]
fn test_symfony() {
    for path in glob("tests/test_data/symfony/src/**/*.php").unwrap() {
        let path = path.unwrap();
        assert_file_lex_same(path);
    }
}
