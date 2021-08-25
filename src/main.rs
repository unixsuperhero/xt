mod app;
mod area;
mod csv_loader;
mod database;
mod sutoa;

use {csv_loader::CsvLoader, database::Database};

fn main() {
    let mut db = Database::new();
    //println!("db before: {:?}\n", &db);

    let tb = match std::env::args().nth(1) {
        Some(file) => CsvLoader::from_path(&file),
        None => CsvLoader::from_path(&"test/simple.csv"),
    }
    .unwrap();

    db.load_table(&tb);
    //println!("db after: {:?}\n", &db);
}
