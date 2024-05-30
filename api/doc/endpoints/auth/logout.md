# POST /auth/logout

required permissions: `<be logged in>`

invalidates used token and replies with json

```json
{
	"message":"<message>",
}
```

## Errors

### 400 Bad request

If user doesn't send the token or the token is invalid the server replies with status code 400 but with the same body json (with different message).
