# POST /admin/delete_user

required permissions: `ADMIN`

Expects json body

```json
{
	"mail":"<string mail>"
}
```

deletes user with mail `mail` and replies with json

```json
{
	"message":"<message>",
}
```

## Errors

### 400 Bad request

If the user sends invalid body the server replies with status code 400 but with plaintext message.

### 409 Conflict

If the user doesn't exist, the server replies with status code `409 Conflict` with the same json body.

### 500 Internal server error

If the user database requests fail, the server replies with status code `500 Internal server error` with the same json body.
