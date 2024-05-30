# POST /auth/logout_all

required permissions: `<be logged in>`

invalidates all user's tokens and replies with json

```json
{
	"message":"<message>",
}
```

## Errors

### 400 Bad request

If user doesn't send the token or the token is invalid the server replies with status code 400 but with the same body json (with different message).

### 500 Internal server error

This shouldn't happen, but the server might reply with status code 500 when the internal state is broken.
