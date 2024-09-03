#!/usr/bin/python3
import json
import sqlite3
import sys

if len(sys.argv) < 2:
	print("usage: load_old_posts_json.py <old_posts_json_file> [options]")
	sys.exit()
with open(sys.argv[1], "r") as f:
	data = json.load(f)
con = sqlite3.connect("./articles.db")
c = con.cursor()
if "--clear" in sys.argv:
	c.execute("DELETE FROM article")
	c.execute("DELETE FROM article_meta")
for post_i,post in enumerate(data[:10]):
	content = "<br>".join(s.replace("'", "''") for s in post['contents'])
	tags = ';'.join([post['categories']])
	cmd = f"INSERT INTO article VALUES ('{post['title']}', '{post['author']}', '{content}', '{tags}')"
	try:
		c.execute(cmd)
	except Exception as e:
		print(cmd)
		print(e)
		sys.exit(1)
	timestamp = post['unix'] if post['unix'] > len(data) else post_i
	cmd = f"INSERT INTO article_meta VALUES ({c.lastrowid}, {timestamp});"
	try:
		c.execute(cmd)
	except Exception as e:
		print(cmd)
		print(e)
		sys.exit(1)
con.commit()

