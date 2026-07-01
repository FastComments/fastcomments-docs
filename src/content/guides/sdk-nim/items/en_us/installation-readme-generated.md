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

For the API client, there are three API modules, `api_default`, `api_public`, and `api_moderation`. The `api_default` contains methods that require your API key, and `api_public` contains api calls
that can be made directly from a browser/mobile device/etc without authentication. The `api_moderation` module contains methods for the moderator dashboard.

The `api_moderation` module provides an extensive suite of live and fast moderation APIs. Every `api_moderation` method accepts an `sso` parameter and can authenticate via SSO or a FastComments.com session cookie.