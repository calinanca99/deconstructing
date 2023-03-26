# Session-based auth

Deconstructing how session-based auth works.

## Workflow

1. Send GET request to "http://localhost:4000/home"
1. Sign up by sending a POST request to "http://localhost:4000/signup"
1. Login by sending a POST request to "http://localhost:4000/login". If the
   username and password combination is correct, then you receive a session id
1. Send GET request to "http://localhost:4000/home" with the "Authorization"
   header set ("Authorization: Bearer \<yoursessionid\>")

## Extra

- [ ] Add the session session id as a cookie in the response header
- [ ] Expiration for session id
- [ ] Add a /logout endpoint and cancel the session
- [ ] Permissions
  - Good to explore [JWTs](https://jwt.io/introduction)

## Resources

- [Difference between cookies, sessions and tokens](https://youtu.be/GhrvZ5nUWNg)
- [HTTP Cookies Crash Course](https://youtu.be/sovAIX4doOE)
