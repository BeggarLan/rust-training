fn main() {
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a: {a:?}");

    a[3] = 30;

    let s: &[i32] = &a[2..4];
    println!("s: {:?}", s);
}
