fn main() {
    let x : i16 = 2;
    let y : i8 = 3;
    // 用into去处理隐式转换问题
    println!("{}", multiply(x, y.into()));
}

fn multiply(x: i16, y: i16) -> i16{
    x * y
}