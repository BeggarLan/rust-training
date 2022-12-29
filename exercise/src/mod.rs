mod a {
    pub fn haha() {
      println!("a haha");  
    }
}

mod b {
    pub fn haha() {
      println!("b haha");  
    }
}

fn main() {
    a::haha();
    b::haha();
}
