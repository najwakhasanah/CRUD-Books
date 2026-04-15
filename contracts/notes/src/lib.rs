#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Env, String, Symbol, Vec
};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Book {
    pub id: u64,
    pub title: String,
    pub category: String,
    pub isbn: String,
    pub created_at: u64,
    pub updated_at: u64,
}

const BOOK_DATA: Symbol = symbol_short!("BOOK_DATA");

#[contract]
pub struct BooksContract;

#[contractimpl]
impl BooksContract {
    pub fn get_books(env: Env) -> Vec<Book> {
        env.storage().instance()
            .get(&BOOK_DATA)
            .unwrap_or(Vec::new(&env))
    }

    pub fn create_book(env: Env, title: String, category: String, isbn: String) -> String {
        let mut books: Vec<Book> = env.storage().instance()
            .get(&BOOK_DATA)
            .unwrap_or(Vec::new(&env));

        let now = env.ledger().timestamp();
        let book = Book {
            id: env.prng().gen::<u64>(),
            title,
            category,
            isbn,
            created_at: now,
            updated_at: now,
        };

        books.push_back(book);
        env.storage().instance().set(&BOOK_DATA, &books);
        String::from_str(&env, "Buku berhasil ditambahkan")
    }

    pub fn update_book(
        env: Env,
        id: u64,
        new_title: String,
        new_category: String,
        new_isbn: String,
    ) -> String {
        let mut books: Vec<Book> = env.storage().instance()
            .get(&BOOK_DATA)
            .unwrap_or(Vec::new(&env));

        let now = env.ledger().timestamp();
        for i in 0..books.len() {
            if books.get(i).unwrap().id == id {
                let old_book = books.get(i).unwrap();
                let updated_book = Book {
                    id,
                    title: new_title,
                    category: new_category,
                    isbn: new_isbn,
                    created_at: old_book.created_at,
                    updated_at: now,
                };
                books.set(i, updated_book);
                env.storage().instance().set(&BOOK_DATA, &books);
                return String::from_str(&env, "Buku berhasil diupdate");
            }
        }
        String::from_str(&env, "Buku tidak ditemukan")
    }

    pub fn delete_book(env: Env, id: u64) -> String {
        let mut books: Vec<Book> = env.storage().instance()
            .get(&BOOK_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..books.len() {
            if books.get(i).unwrap().id == id {
                books.remove(i);
                env.storage().instance().set(&BOOK_DATA, &books);
                return String::from_str(&env, "Buku berhasil dihapus");
            }
        }
        String::from_str(&env, "Buku tidak ditemukan")
    }

    pub fn get_book_by_id(env: Env, id: u64) -> Option<Book> {
        let books: Vec<Book> = env.storage().instance()
            .get(&BOOK_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..books.len() {
            let book = books.get(i).unwrap();
            if book.id == id {
                return Some(book);
            }
        }
        None
    }
}

mod test;