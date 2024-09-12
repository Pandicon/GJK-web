# Authentication

## OAuth through Google

The authentication starts by redirecting the client to the [/auth/redirect](endpoints/auth/redirect.md) endpoint, which redirects the client forward to the Google oauth page. Afterwards, the client is redirected back to the page specified in `oauth.json` configuration file. That page receives `?code=[code]&status=[status]` url query arguments, which can be forwarded to [/auth/oauth](endpoints/auth/oauth.md) endpoint to obtains a *token*.

Client now can make requests like to access endpoints which require user permissions:

```
GET /endpoint HTTP/1.1
Authorization: Bearer <token>
Header2: header2 value

body
```

To log out, user can then send a request to [/auth/logout](endpoints/auth/logout.md) or [/auth/logout_all](endpoints/auth/logout_all.md).

