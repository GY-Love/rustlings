mod macros {
    // Define the macro inside the module
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }

    // Use the macro within the module scope
    pub fn use_macro() {
        my_macro!();
    }
}

fn main() {
    // Call the function that uses the macro
    macros::use_macro();
}
