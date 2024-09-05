# POST /blob/new

required permissions: `MANAGE_ARTICLES`

Expects binary data as body. Saves a new blob which can be retrieved using `/blob/get` endpoint. Replies with json containing the blob id:

```json
{
	"message":"<message>",
	"id":<blob_id>
}
```

## Errors

Note that the server replies with `201 Created` on success.

### 500 Internal server error

If the article database requests fail, the server replies with status code `500 Internal server error` with json body containing only a `message` field.

