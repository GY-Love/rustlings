fn main() {
    let mut res = 42;
    let option = Some(12);
    
    // Fix Clippy lint by using pattern matching with `if let`
    if let Some(x) = option {
        res += x;
    }

    println!("{res}");
}
