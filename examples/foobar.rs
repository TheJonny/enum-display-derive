#[macro_use]
extern crate enum_display_derive;

#[derive(Display)]
pub enum FooBar {
    Foo,
    Bar(),
    FooBar(i32),
}

fn main() {
    println!("{}", FooBar::FooBar(42));
}
