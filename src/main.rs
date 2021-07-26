mod area;
mod app;
mod database;
mod csv_loader;

use {
    database::Database,
    csv_loader::CsvLoader,
};

fn main() {
    let mut db = Database::new();
    println!("db before: {:?}\n", &db);

    let tb = CsvLoader::from_path("test/large.csv").unwrap();
    println!("builder: \r\t{:?}\n", tb);

    db.load_table(&tb);
    println!("db after: {:?}\n", &db);

}
