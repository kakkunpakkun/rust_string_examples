use std::any::{Any, TypeId};

fn main() {
    equal_str();
    types_str();
    concat_str();
    length_str();
    case_str();
    repeat_str();
    format_str();
    replace_str();
    trim_str();
    to_i32_str();
    start_and_end_str();
    split_str();
    join_str();
}

// returns true if T is String. Otherwise returns false.
fn is_string<T: ?Sized + Any>(_s: &T) -> bool {
    TypeId::of::<String>() == TypeId::of::<T>()
}

fn equal_str() {
    assert_eq!(String::from("abc"), "abc".to_string());
    assert_eq!("abc", "abc".to_string());
}

fn types_str() {
    let abc: &str = "abc";
    let abc_string = "abc".to_string();

    assert!(!is_string(&abc));
    assert!(is_string(&abc_string));
}

fn concat_str() {
    assert_eq!("abcdef", concat!("abc", "def"));
    assert_eq!("abcdef", format!("{}{}", "abc", "def"));
}

fn length_str() {
    assert_eq!(6, "abcdef".len());
    assert_eq!(6, String::from("abcdef").len());
}

fn case_str() {
    assert_eq!("ABCDEF", "abcdef".to_uppercase());
    assert_eq!("abcdef", "ABcDef".to_lowercase());
}

fn repeat_str() {
    assert_eq!("abcabcabc", "abc".repeat(3));
}

fn format_str() {
    assert_eq!("abc 123 def 456", format!("abc {} {} 456", 123, "def"));
}

fn replace_str() {
    let hello = "goodmorning world!".replace("goodmorning", "hello");
    assert_eq!("hello world!", hello);
    assert!(is_string(&hello));
}

fn trim_str() {
    let hello = "\nhello world! ";
    assert_eq!("hello world!", hello.trim());
    assert_eq!("hello world! ", hello.trim_left());
    assert_eq!("\nhello world!", hello.trim_right());
}

fn to_i32_str() {
    use std::str::FromStr;
    assert_eq!(123, i32::from_str("123").unwrap());
}

fn start_and_end_str() {
    assert!("abcdefg".starts_with("abc"));
    assert!("abcdefg".ends_with("fg"));

    assert!(!"abcdefg".starts_with("cd"));
    assert!(!"abcdefg".ends_with("fg "));
}

fn split_str() {
    let v: Vec<&str> = "I have a pen".split(' ').collect();
    assert_eq!(v, ["I", "have", "a", "pen"]);
}

fn join_str() {
    assert_eq!(["hello", "world"].join(" "), "hello world");
}