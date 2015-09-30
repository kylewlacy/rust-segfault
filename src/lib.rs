extern crate my_crate;
use my_crate::Foo;

pub fn box_foo() -> Box<Foo> {
    Box::new(Foo::new())
}
