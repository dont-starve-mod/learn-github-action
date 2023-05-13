const DYN_NUMBER: Option<&'static str> = option_env!("DYN_NUMBER");

fn main() {
    println!("Hello, world!");
    println!("{DYN_NUMBER:?}");
}