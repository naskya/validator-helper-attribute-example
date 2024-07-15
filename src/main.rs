use test_macro::Foo;
use validator::Validate;

#[derive(Validate, Foo)]
struct Test1 {
    #[validate(nested)]
    a: A,
}

#[derive(Validate, Foo)]
struct Test2 {
    #[validate(nested)]
    #[foo(foo)]
    a: A,
}

#[derive(Validate, Foo)]
struct Test3 {
    #[validate(nested)]
    #[foo = "foo"]
    a: A,
}

#[derive(Validate, Foo)]
struct Test4 {
    #[validate(nested)]
    #[foo]
    a: A,
}

#[derive(Validate)]
struct A {}

fn main() {}
