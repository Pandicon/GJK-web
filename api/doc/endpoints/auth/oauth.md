# GET /auth/oauth

required permissions: None

Logs in user and generates a token, expects `?code=[code]&status=[status]` arguments.

Finishes OAuth2 by requesting access token from Google API and gets users, e-mail address. If user with that address exists, then token is generated and sent back to the user. If the address ends with `@gjk.cz`, then a new user with `GJK_DEFAULT` perms is created for the email and token is generated as well. Token is send in json:

```json
{
	"token":"<token>"
}
```

There can only be 20 tokens per user, and if user creates a new one, the oldest token gets invalidated. Tokens are also invalidated after 100 days. (TODO: check for last use time instead). On server restart or crash, all tokens are invalidated.

## Errors

### 400 Bad request

If the request doesn't contain `code`/`status` fields or if the OAuth failed, the server replies with status code `400 Unauthorized` and plaintext message.

### 403 Forbidden

If the user doesn't exist and e-mail address doesn't end with `@gjk.cz`, the server replies with status code `403 Forbidden` and plaintext message.

### 500 Internal server error

If the user database requests fail, the server replies with status code `500 Internal server error` and plaintext message.
