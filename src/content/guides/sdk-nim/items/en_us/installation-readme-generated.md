### Using Nimble

```bash
nimble install fastcomments
```

### Building from Source

```bash
nimble build
```

### Library Contents

This library contains the generated API client and the SSO utilities to make working with the API easier.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Public vs Secured APIs

For the API client, there are two API modules, `api_default` and `api_public`. The `api_default` contains methods that require your API key, and `api_public` contains api calls
that can be made directly from a browser/mobile device/etc without authentication.