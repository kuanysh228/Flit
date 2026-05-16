use flit::core::orp::find_orp;

#[test]
fn golden_table() {
    let cases = [
        ("a",              0),
        ("be",             1),
        ("cat",            1),
        ("word",           2),
        ("hello",          2),
        ("foobar",         2),
        ("digital",        2),
        ("together",       3),
        ("beautiful",      3),
        ("strawberry",     3),
        ("comfortable",    3),
        ("uncomfortable",  3),
        ("unbelievably",   3),
        ("extraordinarily", 4),
        ("incomprehensible", 4),
        ("привет",         2),
        ("читать",         2),
        ("скорочтение",    3),
    ];

    for (word, expected) in cases {
        assert_eq!(
            find_orp(word),
            expected,
            "find_orp({word:?}) should be {expected}"
        );
    }
}
