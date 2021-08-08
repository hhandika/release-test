fn main() {
    let n = it_works(5);
    println!("Hello, world! {}", n);
}

fn it_works(n: usize) -> usize {
    n * 2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_it_works() {
        let n: usize = 10;
        let res = it_works(5);
        assert_eq!(n, res);
    }
}
