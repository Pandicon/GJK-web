# `migrate.py`

```sh
migrate.py <migration_file>
```

Migrates a table in some database according to a migration file. (always creates a backup file)
You can find migration files in the migrations subfolder.
Migration file looks like this:

```
migrate
<database_file>
<table>
<new fields>
<value mutator>
```

`<new fields>` are the new columns of the table in the SQL format - directly injected into a `CREATE RABLE` command.
`<value mutator>` is a python expression which is `eval`'d for each tuple of values that were found in the database previously. It should evaluate to a tuple of values to newly insert in the SQLite format.

here's an example:
```
migrate
./userdb.db
user
(mail TEXT NOT NULL, name TEXT, perms INTEGER NOT NULL)
f"('{esc_str(x[0])}', NULL, {x[1]})"
```
it migrates the table user in userdb and adds a name field, initialized to NULL in all old entries

