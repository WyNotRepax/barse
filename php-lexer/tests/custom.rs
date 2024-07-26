use util::assert_lex_same;

mod util;
#[test]
fn test_yield_from() {
    assert_lex_same("<?php yield from");
}
