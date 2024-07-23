#[rustfmt::skip]
#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    // Handle the Option correctly by using pattern matching
    if my_option.is_none() {
        println!("The option is None.");
    }

    // Fix array syntax by adding the missing comma
    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {my_arr:?}");

    // Fix the `resize` method issue by correctly resizing the vector to be empty
    let mut my_vec = vec![1, 2, 3, 4, 5];
    my_vec.clear();
    println!("This Vec is empty, see? {my_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    // Use std::mem::swap to swap values
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {value_a}; value b: {value_b}");
}
