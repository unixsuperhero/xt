
# Intro

This is a rewrite of `unixsuperhero/etl`.  I tend to rewrite code, because the
final draft is (almost) always better than the rough draft.  Sure, the entire
history _could_ be tracked in git.  But everything up until now was just a
spike.  This is a new design, so I am using a new repo.


# todo

- fix the csv loader to make a TableBuilder
  - use it to init the db

# Eventual Design

All of the filetype loader plugins should build TableBuilder objects.
- By that time, TableBuilder should either push directly into the database's cell slab
  * this is the only way i see it working right now
  * there will likely be an associative function like
  * `Database::init_with(cells: Slab<String>, tables: Slab<Table>, columns: Slab<Column>)`
    or something
- or, have an intermediate cell slab.
  * i don't like this because it feels like we are writing a DatabaseBuilder object instead.
  * which i am fine with, but we would have to rewrite it for the different scope.


# Ideas

- make a `Cash<T: Hash + Eq>` object which takes a slab and adds the reverse lookup functionality to it
  * use it for the cell and column slabs
- make a `Graph<T>` object which is a lot like a git commit.  it knows about
  itself and has a pointer to its parent.
  * not sure about this
