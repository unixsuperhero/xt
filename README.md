
# Intro

This is a rewrite of `unixsuperhero/etl`.


# Eventual Design

All of the filetype loader plugins should build TableBuilder objects.
- By that time, TableBuilder should either push directly into the database's cell slab
  * this is the only way i see it working right now
  * there will likely be an associative function like
  * `Database::init_with(cells: Slab<String>, tables: Slab<Table>, columns: Slab<Column>)`
    or something
