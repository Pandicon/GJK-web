# GET /auth/me

required permissions: `NONE`

gets info about the user (expects a token) and returns it in a json like this:

```json
{
	"mail":"mail0@gjk.cz",
	"perms":3
}
```

## Errors

### 400 Bad request

If the request doesn't contain a token or if the token isn't valid, a different json and status code `400 Bad request` is sent back:

```json
{
	"message":"<message>",
}
```

### 500 Internal server error

If the user database requests fail, the server replies with status code `500 Internal server error` with a json body:

```json
{
	"message":"<message>",
}
```
