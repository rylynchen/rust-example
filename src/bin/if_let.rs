enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

fn main() {
    let number = Some(7);
    if let Some(i) = number {
        println!("i is {}", i);
    }

    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("a is foobar");
    }

    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }
    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred");
    }
}