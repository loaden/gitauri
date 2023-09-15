pub fn times(left: usize, right: usize) -> usize {
    left * right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = times(2, 3);
        assert_eq!(result, 6);
    }
}
