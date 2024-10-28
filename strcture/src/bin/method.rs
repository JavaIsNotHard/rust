#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&mut self) -> u32 {
        self.length = 40;
        return self.length * self.width;
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        return self.width > rectangle.width && self.length > rectangle.length;
    }
}

fn main() {
    let mut rect1 = Rectangle {
        length: 20,
        width: 40,
    };

    let rect2 = Rectangle {
        length: 10,
        width: 20,
    };

    let rect3 = Rectangle {
        length: 30,
        width: 50,
    };

    println!("The area of the rectangle {rect1:?} is {}", rect1.area());

    if rect1.can_hold(&rect2) {
        println!("the rectange {rect1:?} can hold {rect2:?}");
    }

    if rect1.can_hold(&rect3) {
        println!("the rectange {rect1:?} can hold {rect3:?}");
    } else {
        println!("the rectange {rect1:?} cannot hold {rect3:?}");
    }
}
