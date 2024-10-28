#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            length: size,
            width: size,
        }
    }

    fn new(length: u32, width: u32) -> Self {
        Self { length, width }
    }
}

fn main() {
    let rect1 = Rectangle::square(10);
    let rect2 = Rectangle::new(10, 20);

    println!("{rect1:#?} \n{rect2:#?}");
}
