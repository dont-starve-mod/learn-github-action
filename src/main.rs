const DYN_MAGIC_NUMBER: Option<&'static str> = option_env!("DYN_MAGIC_NUMBER");

fn main() {
    println!("Hello, world!");
    println!("{DYN_MAGIC_NUMBER:?}");
}