#!/usr/bin/python3
import argparse
import sqlite3
import sys

argp = argparse.ArgumentParser(description='Add user utility')
argp.add_argument('mail', type=str)
argp.add_argument('name', type=str)
argp.add_argument('perms', type=int)
argp.add_argument('--list', action='store_true')
a = argp.parse_args()
con = sqlite3.connect("./userdb.db")
c = con.cursor()
if a.list:
	c.execute("SELECT * FROM user")
	for x in c.fetchall():
		print(x)
	sys.exit(0)
name = None if a.name == "0" else a.name
c.execute(f"INSERT INTO user(mail, name, perms) VALUES (?, ?, {a.perms});", (a.mail, name,))
con.commit()
