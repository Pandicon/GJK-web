# GET /blob/get

required permissions: `NONE`

Expects url parameter like this:

```
/article/articles?id=4
```

gets a blob with id `id` as raw binary data

png, jpeg and pdf formats are detected through file type magics and sent in the `content-type` http header.

## Errors

### 400 Bad request

If the user sends invalid url parameters, the server replies with status code `400 Bad request`.

### 500 Internal server error

If the user database requests fail, the server replies with status code `500 Internal server error` with a json body:

```json
{
	"message":"<message>",
}
```

