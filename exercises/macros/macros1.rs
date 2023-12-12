// macros1.rs
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a hint.

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

macro_rules! usage {
    ( $prog:expr, $tp:ty, $id:ident ) => {
        let t: $tp = 3;
        let g: $id = 4.to_string();
        println!("{t}, {g}");
        println!("Usage: {} <options>", $prog);
    };
}

type Wide32Int = i32;

fn main() {
    usage!("rustlings", i32, String);
    my_macro!();
}
