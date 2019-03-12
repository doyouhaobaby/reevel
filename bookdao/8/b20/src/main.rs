fn main() {
    let a = "hello xiaoniuge";
    assert_eq!(a.find('e'), Some(1));
    assert_eq!(a.find("xiao"), Some(6));
    assert_eq!(a.find("notfound"), None);

    assert_eq!(a.rfind("e"), Some(a.len() - 1));
    assert_eq!(a.find(char::is_whitespace), Some(5));
    assert_eq!(a.find(char::is_lowercase), Some(0));
    assert_eq!(a.find(char::is_uppercase), None);
}
