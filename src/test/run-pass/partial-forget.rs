use std::mem::{forget,drop};

struct Foo;
struct Bar(Foo, Foo);

pub fn main() {
    let bar = Bar(Foo, Foo);
    let foo0 = bar.0;
    unsafe { forget(bar) };
    drop(foo0);
}
