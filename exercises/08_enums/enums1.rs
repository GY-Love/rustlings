#[derive(Debug)]
enum Message {
    Quit,
    Echo(String), 
    Move { x: i32, y: i32 }, 
    ChangeColor(u8, u8, u8), 
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo("Hello, World!".to_string()));
    println!("{:?}", Message::Move { x: 10, y: 20 });
    println!("{:?}", Message::ChangeColor(255, 0, 128));
}