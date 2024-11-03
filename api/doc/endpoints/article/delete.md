# DELETE /article/delete

\[!\] this endpoint will probably change

Required permissions: `MANAGE_ARTICLES` and has to be the article's author

Expects url parameter like this:
```
/article/delete?id=4
```

Deletes the article and responds with no content (`204 No Content`).

## Errors
Note that the server replies with `204 No Content` on success.

### 400 Bad request
If the user sends invalid body the server replies with status code 400 but with plaintext message.

### 403 Forbidden
If the user has insufficient permissions (can not delete articles) or is not the author of the post they are trying to delete, a response with `403` will be sent along with a message, for example:
```json
{
    "message": "Only the author can delete an article."
}
```

### 404 Not found
If the article to be deleted does not exist, the server responds with a `404` status code along with a message, for example:
```json
{
    "message": "The article with id 17359 does not exist."
}
```

### 500 Internal server error
If the article database requests fail, the server replies with status code `500 Internal server error` with the same json body.
