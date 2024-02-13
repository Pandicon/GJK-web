# POST /article/new

\[!\] this endpoint will probably change

required permissions: `MANAGE_ARTICLES`

Expects json body

```json
{
	"title":"<string title>",
	"author":"<string author>",
	"content":"<string content>",
	"tags":["<string tag>", "<string tag 2> ..."],
}
```

adds an article and responds with json

```json
{
	"message":"<message>",
}
```

## Errors

Note that the server replies with `201 Created` on success.

### 400 Bad request

If the user sends invalid body the server replies with status code 400 but with plaintext message.

### 500 Internal server error

If the article database requests fail, the server replies with status code `500 Internal server error` with the same json body.
