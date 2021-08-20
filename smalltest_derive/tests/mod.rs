use smalltest::*;
use smalltest_derive::*;

#[derive(Debug, Arbitrary, PartialEq, Eq)]
struct Foo {
    a: u8,
    b: u8,
    c: u8,
}

#[test]
fn foo_generates_some_values() {
    assert_eq!(
        Foo::any().take(10).collect::<Vec<_>>(),
        vec![
            Foo { a: 0, b: 0, c: 0 },
            Foo { a: 0, b: 0, c: 1 },
            Foo { a: 1, b: 0, c: 0 },
            Foo { a: 0, b: 1, c: 0 },
            Foo { a: 1, b: 0, c: 1 },
            Foo { a: 2, b: 0, c: 0 },
            Foo { a: 0, b: 0, c: 2 },
            Foo { a: 1, b: 1, c: 0 },
            Foo { a: 2, b: 0, c: 1 },
            Foo { a: 3, b: 0, c: 0 },
        ]
    );
}

#[derive(Debug, PartialEq, Eq, Arbitrary)]
enum Bar {
    Bar1,
    Bar2,
}

#[test]
fn bar_generates_all_variants() {
    let variants: Vec<Bar> = Bar::any().collect();
    assert_eq!(variants, vec![Bar::Bar1, Bar::Bar2,])
}
