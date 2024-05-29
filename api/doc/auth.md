# Authentication

## OAuth through Google

The authentication starts by redirecting user to

```
https://accounts.google.com/o/oauth2/v2/auth?scope=https%3A//www.googleapis.com/auth/userinfo.email&access_type=offline&include_granted_scopes=true&response_type=code&state=[STATE]&redirect_uri=[REDIRECT_URI]&client_id=[CLIENT_ID]
```

where `state` is anything, `redirect_uri` is uri the user should be redirected back to with `code` and `state` arguments and `client_id` is Google application client ID.

Then the client sends a request to [/auth/oauth](endpoints/auth/oauth.md) with `code` and `state` and obtains a *token*.

Client now can make requests like to access endpoints which require user permissions:

```
GET /endpoint HTTP/1.1
Authorization: Bearer <token>
Header2: header2 value

body
```

To log out, user can then send a request to [/auth/logout](endpoints/auth/logout.md) or [/auth/logout_all](endpoints/auth/logout_all.md).
