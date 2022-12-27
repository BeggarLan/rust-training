struct Library {
    books: Vec<Book>,
}

struct Book {
    title: String,
    year: u16,
}

impl Book {
    fn new(title: String, year: u16) -> Book {
        Book{
            title, 
            year
        }
    }
}

impl Library {
    fn new() -> Library {
        unimplemented!()
    }
}

fn main() {
    // let mut vec = vec![1,2];
    // vec.push(3);
    // println!("middle value: {}", vec[vec.len() / 2]);
    // for item in vec.iter() {
    //     println!("item: {item}");
    // }
    // let Library = Library::new();

    let v: Vec<String> = vec![String::from("foo"), String::from("bar")];

    for word in &v {
        println!("word: {word}");
    }

    for word in v {
        println!("word: {word}");
    }

 }