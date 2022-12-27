#[derive(Copy, Clone, Debug)]
struct Point(i32, i32);

fn main() {
    // {
    //     let p = Point(1,2);
    //     println!("{}", p.0);
    // }
    // println!("{}", p.0);
    
    // let s = String::from("hello");
    // let s2 : String = s;
    // println!("{}", s);

    // let x = 42;
    // let y = x;
    // println!("x: {x}");
    // println!("y: {y}");

    // let p1 = Point(3, 4);
    // let p2 = p1;
    // println!("{}", p1.0);
    // println!("p1: {p1:?}");
    // println!("p2: {p2:?}");


    // let p1 = Point(3, 4);
    // let p2 = Point(10, 20);
    // let p3 = add(p1, p2);
    // println!("{p1:?} + {p2:?} = {p3:?}");

    let mut a: i32 = 10;
    let b: &i32 = &a;

    {
        let c: &mut i32 = &mut a;
        *c = 20;
    }

    println!("a: {a}");
    println!("b: {b}");
}

fn add(p1: Point, p2: Point) -> Point {
    Point(p1.0 + p2.0, p1.1 + p2.1)
}