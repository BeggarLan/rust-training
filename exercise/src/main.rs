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
    // let dangleRef = dangleRef();
    // dangleRef2();
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
    // let ref_y: &i32 = &x;
    *ref_x = 20;

    // println!("x: {x}");
    println!("ref_x: {ref_x}");
}

// fn dangleref() -> &String {
//     let str : String::from("hello");
//     return &str;
// }

// fn dangleRef2() {
//     println!("dangleRef2");
//     let ref_x: &i32;
//     {
//         let x : i32 = 10;
//         ref_x = &x;
//         println!("ref_x: {ref_x}");
//     }
//     println!("ref_x: {ref_x}");
// }

