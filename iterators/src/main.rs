#![allow(unused)]

#[derive(Debug)]
struct Library {
    books: Vec<(u32, String)>,
}

impl Library {
    fn new() -> Self {
        Self { books: Vec::new() }
    }

    fn add_book(&mut self, id: u32, name: &str) {
        self.books.push((id, name.to_string()));
    }
}

impl Iterator for Library {
    type Item = Vec<(u32, String)>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.books.pop() {
            Some(book) => Some(vec![book]),
            None => None,
        }
    }
}
fn main() {
    let mut lib = Library::new();
    lib.add_book(1, "good one");
    lib.add_book(2, "second book");
    println!("{:?}", lib.next()); // next() consumes the base iter

    // lets check clousers too

    // let somthing = lib.books.get(4).unwrap_or_else(|| match lib.books.get(0) {
    //     Some(x) => &x,
    //     None => panic!("Err:: Library is empty"),
    // });

    // println!("{:?}", somthing);

    lib.add_book(1, "good one");
    lib.add_book(2, "second book");

    let filtered_books: Vec<&(u32, String)> = lib
        .books
        .iter() // make an iter
        .inspect(|book| println!("Filtering length  more than 10 :: curr book :: {}", book.1))
        .filter(|(_0, _1)| _1.len() > 10) // We don't want months more than 5 bytes in length.
        .inspect(|book| println!("Filtered books :: {}", book.1))
        .filter(|(_0, _1)| _1.contains("s")) // Also we only like months with the letter u
        .collect();

    //println!("Filter :: {:?}", filtered_books);
    //println!("Lib:: {:?}", lib);

    // here is the use case dbg! , it let dose the code what its doing , it also prints it what happened

    let vec = vec![1, 2, 3];
    let doubble_vec = dbg!(vec.iter().map(|x| x * 2).collect::<Vec<i32>>());

    // lets see the case for inspect. its same as debug but you can do additional things here

    let tripple_vec = vec
        .iter()
        .inspect(|first_item| println!("The first item is {}", first_item))
        .map(|x| x * 3)
        .inspect(|tripple_item| println!("the tripple item is {}", tripple_item))
        .collect::<Vec<i32>>();

    // check for fold too

    dbg!(vec.iter().fold(0, |sum, x| sum + x)); // here initial value for sum is 0
}
