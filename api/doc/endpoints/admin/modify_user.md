# POST /admin/modify_user

required permissions: `ADMIN`

Expects json body

```json
{
	"mail":"<string mail>",
	"perms":<uint perms>
}
```

updates user with mail `mail` permissions and replies with json

```json
{
	"message":"<message>",
}
```

## Errors

Note that the server replies with `201 Created` on success.

### 400 Bad request

If the user sends invalid body the server replies with status code 400 but with plaintext message.

### 409 Conflict

If the user doesn't exists, the server replies with status code `409 Conflict` with the same json body.

### 500 Internal server error

If the user database requests fail, the server replies with status code `500 Internal server error` with the same json body.
