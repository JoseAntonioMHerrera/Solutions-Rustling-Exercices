// macros4.rs
// Make me compile! Execute `rustlings hint macros4` for hints :)

#[macro_export]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
    ($($val2:expr),*) => {
        $(
            println!("Look at this final macro: {}", $val2);
        )*

    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
    my_macro!(8,8,8,8,8);
}
