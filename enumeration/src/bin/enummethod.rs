#[derive(Debug)]
enum Message {
    Quit,
    Move { x: u32, y: u32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let m = Message::Write(String::from("jibesh"));
    m.call();
}
