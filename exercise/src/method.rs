struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn inc_width(&mut self, delta: u32) {
        self.width += delta;
    }
}

fn pick_one<T>(a: T, b: T) -> T {
    if std::process::id() % 2 == 0 {
        a
    } else {
        b
    }
}

fn main() {
    let mut rect: Rectangle = Rectangle{width: 10, height: 20};
    // let mut rect = Rectangle{width: 10, height: 20}; // 省略类型也可以
    println!("old area: {}", rect.area());
    rect.inc_width(5);
    println!("new area: {}", rect.area());

    println!("{}",pick_one("head", "tail"));
    println!("{}",pick_one(500, 1000));
}