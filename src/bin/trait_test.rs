fn main() {
    let f = F {};
    f.foo(&B {});

    let f_boxed: Box<dyn Foo> = Box::new(f);
    f_boxed.foo(&B {});
}

trait Foo {
    fn foo(&self, b: &dyn Bar);
}

trait Bar {
    fn bar(&self);
}

struct F {}

impl Foo for F {
    fn foo(&self, b: &dyn Bar) {
        b.bar();
    }
}

struct B {}

impl Bar for B {
    fn bar(&self) {
        println!("Hi from B!")
    }
}
