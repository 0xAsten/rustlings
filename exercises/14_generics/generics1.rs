// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.

// Note: Specifying the type of the vector to be a reference to a string.

fn main() {
    // let mut shopping_list: Vec<???> = Vec::new();
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}