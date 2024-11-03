# POST /article/edit

\[!\] this endpoint will probably change

Required permissions: `MANAGE_ARTICLES` and has to be the article's author

Expects the same JSON body as [/article/new](new.md) along with an id of the article to edit.

Edits the article and responds with JSON:
```json
{
    "message": "<message>"
}
```

## Errors
Note that the server replies with `200 OK` on success.

### 400 Bad request

If the user sends invalid body the server replies with status code 400 but with plaintext message.

### 403 Forbidden
If the user has insufficient permissions (can not edit articles) or is not the author of the post they are trying to edit, a response with `403` will be sent along with a message, for example:
```json
{
    "message": "Only the author can edit an article."
}
```

### 404 Not found
If the article to be edited does not exist, the server responds with a `404` status code along with a message, for example:
```json
{
    "message": "The article with id 17359 does not exist."
}
```

### 500 Internal server error
If the article database requests fail, the server replies with status code `500 Internal server error` with the same json body.
