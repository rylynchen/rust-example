use std::cmp::min;
use std::fmt;
use std::fmt::{Display, Formatter};

fn main() {
    let num = vec![1, 1, 2, 2, 3];
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{:b}", 12);

    println!("{number:>0width$}", number = 1, width = 6);

    #[derive(Debug)]
    struct Structure(i32);

    impl Display for Structure {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "Structure is {}", self.0)
        }
    }

    #[derive(Debug)]
    struct Deep(Structure);

    println!("{}", Structure(3));
    println!("{:?}", Deep(Structure(7)));

    println!("{}", format!("{:08}", 100));

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    println!("{:#?}", peter);

    #[derive(Debug)]
    struct MinMax(i64, i64);

    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    impl fmt::Binary for MinMax {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "({:b}, {:b})", self.0, self.1)
        }
    }

    let minmax = MinMax(0, 14);
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);
    println!("Binary: {:b}", minmax);

    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let vec = &self.0;
            write!(f, "[")?;
            for (count, v) in vec.iter().enumerate() {
                if count != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", v)?;
                if count % 2 == 0 {
                    write!(f, ":")?;
                }
            }
            write!(f, "]")
        }
    }

    let list = List(vec![1, 2, 3]);
    println!("{}", list);
}
