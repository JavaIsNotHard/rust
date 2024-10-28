#[derive(Debug)]
struct Rectangle {
    length: u32,
    breadth: u32,
}

fn create_rectangle(length: u32, breadth: u32) -> Rectangle {
    Rectangle { length, breadth }
}

fn calculate_area(rectangle: &Rectangle) -> u32 {
    return rectangle.length * rectangle.breadth;
}

fn calculate_area_tuple(dimensions: (u32, u32)) -> u32 {
    return dimensions.0 * dimensions.1;
}

fn main() {
    let rectangle = create_rectangle(10, 20);
    let area = calculate_area(&rectangle);
    let rect1 = (10, 20);
    let _area_two = calculate_area_tuple(rect1);
    println!("{area}");

    println!("The rectange is {rectangle:#?}");

    dbg!(&rectangle);
}
