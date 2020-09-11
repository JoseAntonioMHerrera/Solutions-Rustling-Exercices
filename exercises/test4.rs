// test4.rs
// This test covers the sections:
// - Modules
// - Macros

mod macros {
    #[macro_export]
    macro_rules! my_macro {
        ($val:literal) => {
            format!("Hello {}",$val)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_macro_world() {
        assert_eq!(my_macro!("world!"), "Hello world!");
    }

    #[test]
    fn test_my_macro_goodbye() {
        assert_eq!(my_macro!("goodbye!"), "Hello goodbye!");
    }
}
