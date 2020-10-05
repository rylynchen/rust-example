use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})\n({} {})", &self.0, &self.1, &self.2, &self.3)
    }
}

fn transpose(pair: Matrix) -> Matrix {
    Matrix(pair.0, pair.2, pair.1, pair.3)
}

fn main() {
    println!("1 - 2 = {}", 3u8 - 2);
    println!("NOT true is {}", !true);

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);
    println!("{}", transpose(matrix));
}