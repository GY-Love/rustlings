fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);
    string_uppercase(data);
}

fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

fn string_uppercase(mut  data: String) {
    data.make_ascii_uppercase();
    println!("{}", data);
}