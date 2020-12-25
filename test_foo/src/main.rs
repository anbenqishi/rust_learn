struct Foo { a: i32 }
impl Foo {
    fn bar(&mut self, val: i32) {
        self.a = val + 42;
    }
}

fn main() {
  let mut foo = Foo { a: 0 };
  foo.bar(foo.a);
}