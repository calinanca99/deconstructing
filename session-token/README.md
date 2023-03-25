# Session Token

Deconstructing how session tokens work.

## Workflow

1. Send first GET request -> Get "not logged id"
2. Log in -> Receive session token
3. Send another GET request with the session token -> Get "dynamic" content

## Extra

- Expiration for token
- Permissions
  - Good to explore JWTs
