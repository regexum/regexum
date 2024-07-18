#![allow(unused_variables, dead_code)]
use regexum::Patternize;

#[test]
fn simple_test() {
    #[derive(Patternize)]
    enum Enum {
        #[pattern("x")]
        X,
        Y,
    }

    assert_eq!(Enum::patterns(), &["x"]);
}
