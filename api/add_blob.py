#!/usr/bin/python3
import sqlite3
import sys

con = sqlite3.connect("./blobs.db")
c = con.cursor()
if len(sys.argv) < 2:
	c.execute("SELECT * FROM blobs")
	for x in c.fetchall():
		print(x)
	sys.exit(0)
with open(sys.argv[1], "rb") as f:
	data = f.read()
c.execute(f"INSERT INTO blobs VALUES (?);", (data,))
con.commit()
