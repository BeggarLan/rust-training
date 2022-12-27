fn main() {
    let s1: &str = "hello";
    println!("s1: {s1}");
    // s1 = "world"; // &str是immutable的
    
    let mut s2: String = String::from("hello");
    println!("s2: {s2}");

    s2.push_str(s1);
    println!("s2: {s2}");
}