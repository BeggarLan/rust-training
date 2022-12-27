fn main() {
    hello(10);
}

fn hello(n: u32) {
    for x in 1..=n {
        println!("{x}");
    }
}