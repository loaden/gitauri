use gitauri::*;

#[test]
fn test_for_greeting() {
    assert_eq!(greet(""), "Please input your name!");
    assert_eq!(greet("   "), "Please input your name!");
}

#[test]
fn test_for_times() {
    assert_eq!(times(2, 3), 5);
    assert_eq!(times(7, 9), 60);
}