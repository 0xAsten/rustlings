// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!

// Note:

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    // ???("blue");
    // ???("red".to_string());
    // ???(String::from("hi"));
    // ???("rust is fun!".to_owned());
    // ???("nice weather".into());
    // ???(format!("Interpolation {}", "Station"));
    // ???(&String::from("abc")[0..1]);
    // ???("  hello there ".trim());
    // ???("Happy Monday!".to_string().replace("Mon", "Tues"));
    // ???("mY sHiFt KeY iS sTiCkY".to_lowercase());
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}