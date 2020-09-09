use std::collections::HashMap;

fn main() {
    let mut book_reviews = HashMap::with_capacity(10);
    book_reviews.insert("rust book", "good");
    book_reviews.insert("goods", "yes");
    dbg!(book_reviews.clone());

    for key in book_reviews.keys() {
        println!("{}", key);
    }

    for value in book_reviews.values() {
        println!("{}", value);
    }

    if !book_reviews.contains_key("not found") {
        println!("find {}  times", book_reviews.len());
    }

    book_reviews.remove("rust book");

    let to_find = ["rust book", "goods"];
    for book in &to_find {
        match book_reviews.get(book) {
            Some(review) => println!("{}, {}", book, review),
            None => println!("{} is unreviewed.", book),
        }
    }

    assert_eq!(book_reviews["goods"], "yes");
}
