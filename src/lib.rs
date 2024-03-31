pub mod logger;
pub mod treap;
#[cfg(test)]
mod tests {
    use self::treap::Treap;
    use super::*;

    #[test]
    fn first_test() {
        let mut treap = Treap::<i32, String, u8, u8>::new(0u8);
        treap.insert(1, "one".to_string());
        treap.insert(2, "two".to_string());
        treap.insert(3, "three".to_string());
        treap.insert(4, "four".to_string());
        treap.insert(5, "five".to_string());
        treap.insert(6, "six".to_string());
        treap.insert(7, "seven".to_string());
        treap.insert(8, "eight".to_string());
        treap.insert(9, "nine".to_string());
        treap.insert(10, "ten".to_string());
    }
}
