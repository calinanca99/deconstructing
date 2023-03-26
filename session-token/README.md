# Session Token

Deconstructing how session tokens work.

## Workflow

1. Send GET request to "http://localhost:4000/home"
1. Sign up by sending a POST request to "http://localhost:4000/signup"
1. Login by sending a POST request to "http://localhost:4000/login". If the
   username and password combination is correct, then you receive a token
1. Send GET request to "http://localhost:4000/home" with the "Authorization"
   header set ("Authorization: Bearer \<yoursessiontoken\>")

## Extra

- [ ] Expiration for token
- [ ] Permissions
  - Good to explore [JWTs](https://jwt.io/introduction)
