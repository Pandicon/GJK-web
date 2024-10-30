# GET /article/articles

required permissions: `NONE`

Expects url parameter like this:

```
/article/articles?page=4
```

gets a list of `page`-th set of 10 (hardcoded page size) articles as json:

```json
{
	"articles":[
		{
			"id":42,
			"thumbnail_id":42,
			"timestamp":1716403351,
			"title":"Název úžasného článku",
			"author_name":"Anonym :D",
			"content":"Text úžasného článku",
			"tags":["úžasné články", "pro studenty"],
		},
		// ...
	]
}
```

## Errors

### 400 Bad request

If the user sends invalid body the server replies with status code 400 but with plaintext message.

### 500 Internal server error

If the user database requests fail, the server replies with status code `500 Internal server error` with a json body:

```json
{
	"message":"<message>",
}
```

