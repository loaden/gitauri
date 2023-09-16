pub fn greet(name: &str) -> String {
    format!(
        "Hello, {}! You've been greeted from {} {}!",
        name,
        base::G_APPNAME,
        base::add(1, 2) + times(2, 3)
    )
}

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
