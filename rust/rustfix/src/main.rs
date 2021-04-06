trait Foo {
    fn foo(&self, _: Box<dyn Foo>);
}

fn main() {
  for i in 10..0 {
    println!("i is {}", i);
  }
}
