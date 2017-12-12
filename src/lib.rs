#[macro_export]
macro_rules! rkl {
    ($($i:tt $($arg:ident) * = $body:expr)*) => {
        $(
            let $i = |$($arg,)*| $body;
        )*
    };
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        rkl!{
            add a b = a + b
        }
        assert_eq!(add(2f32, 2f32), 4f32);
    }
}
