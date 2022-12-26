fn main() {
    println!("exercise!");
    let str: String = "lanweihua".into();
    let mut x: i32 = 6;
    println!("{x}");
    while x != 1 {
        if x % 2 == 0 {
            x = x / 2;
        } else {
            x = 3 * x + 1;
        }
        println!(" -> {x}");
    }
    println!();
    arrayTest();
    tupleTest();
    refTest();
}

fn arrayTest() {
    let mut x : [i32; 10] = [42; 10];
    x[5] = 100;
    println!("x = {:?}", x);
}

fn tupleTest() {
    let mut t: (i8, bool) = (8, false);
    t.0 = 1;
    println!("t = {:?}", t);
    println!("first index value: {}", t.0);
    println!("second index value: {}", t.1);
}

fn refTest() {
    println!("refTest");

    let mut x: i32 = 10;
    let ref_x: &mut i32 = &mut x;
    let ref_y: &mut i32 = &mut x;
    *ref_x = 20;

    // println!("x: {x}");
    println!("ref_x: {ref_x}");
}
