use std::fs::File;
use std::io::{Write, BufReader, BufRead};


struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    // TODO: Implement this function
    // Hint: Use File::create() and write!() macro
    let mut file = File::create(filename).unwrap();

    for book in books {
        let line = format!("{},{},{}\n", book.title, book.author, book.year);
        let write_outcome = file.write_all(line.as_bytes());
    }
}

fn load_books(filename: &str) -> Vec<Book> {
    // TODO: Implement this function
    // Hint: Use File::open() and BufReader
    let mut books = Vec::new();
    let file_outcome = File::open(filename).unwrap();
    let file = file_outcome;

    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(text) = line {
            let parts: Vec<&str> = text.trim().split(',').collect();

            if parts.len() == 3 {
                let title = parts[0].to_string();
                let author = parts[1].to_string();

                if let Ok(year) = parts[2].parse::<u16>() {
                    let book = Book {
                        title,
                        author,
                        year,
                    };
                    books.push(book);
            }
        }
    }
   }
   books
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}