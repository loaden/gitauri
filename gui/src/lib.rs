pub fn greet(name: &str) -> String {
    if name.len() == 0 || name.trim().len() == 0 {
        return "Please input your name!".to_owned()
    }
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
