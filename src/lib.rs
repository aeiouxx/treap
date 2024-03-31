pub mod logger;
pub mod treap;
#[cfg(test)]
mod tests {
    use self::treap::Treap;
    use super::*;

    #[test]
    fn first_test() {
        let mut treap = Treap::<i32, String, u8, u8>::new(0u8);
        for i in 0..14 {
            print!("Inserting: {}\n", i);
            treap.insert(i, i.to_string());
            treap.print();
        }
        println!("Height: {}", treap.height());
    }
    #[test]
    fn benchmark() {
        let iterations = 10_000;
        let treap_elements = 1_023;
        let mut height_avg = 0;
        for _ in 1..=iterations {
            let mut treap = Treap::<i32, String>::new(0);
            for i in 1..=treap_elements {
                treap.insert(i, i.to_string());
            }
            let height = treap.height();
            height_avg += height;
        }
        println!("Height average: {}", height_avg as f32 / iterations as f32);
    }
}
