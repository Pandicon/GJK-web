#!/usr/bin/python3
import argparse
import sqlite3
import sys

argp = argparse.ArgumentParser(description='Database migration script')
argp.add_argument('migration_file', type=str)
a = argp.parse_args()
with open(a.migration_file, "r") as f:
	if f.readline().strip() != "migrate":
		raise RuntimeError("invalid migration file")
	db_file = f.readline().strip()
	table = f.readline().strip()
	fields = f.readline().strip()
	val_mut = f.readline().strip()
print("------ migrating ------")
print(f"db: {db_file} table: {table} -> backup file: {db_file}.backup")
print(f"new fields: {fields}")
print(f"value mutator: {val_mut}")
if input("proceed? [y/N]> ").lower() not in ("y", "ye", "yes"):
	sys.exit(1)
print(f"creating a backup...")
with open(db_file, "rb") as f:
	with open(db_file+".backup", "wb") as fw:
		fw.write(f.read())
print(f"opening database...")
con = sqlite3.connect(db_file)
c = con.cursor()
print(f"loading data...")
c.execute(f"SELECT * FROM {table};")
data = []
for x in c.fetchall():
	data.append(x)
print(f"removing and recreating table...")
c.execute(f"DROP TABLE {table};")
c.execute(f"CREATE TABLE {table} {fields};")
print(f"inserting new data...")
def esc_str(s):
	return s.replace("'", "''")
for x in data:
	c.execute(f"INSERT INTO {table} VALUES {eval(val_mut)};")
con.commit()
