# GET /admin/users

required permissions: `ADMIN`

gets a list of users as json:

```json
{
	"users":[
		{
			"mail":"mail0@gjk.cz",
			"perms":3
		},
		// ...
	]
}
```

## Errors

### 500 Internal server error

If the user database requests fail, the server replies with status code `500 Internal server error` with a json body:

```json
{
	"message":"<message>",
}
```
