#!/usr/bin/python3
import sqlite3

con = sqlite3.connect("./articles.db")
c = con.cursor()
print("============ article =============")
c.execute("SELECT * FROM article")
for x in c.fetchall():
	print(x)
print("============ article_meta =============")
c.execute("SELECT * FROM article_meta")
for x in c.fetchall():
	print(x)
