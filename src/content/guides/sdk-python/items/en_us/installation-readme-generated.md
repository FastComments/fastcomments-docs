### PyPI

```bash
pip install fastcomments
```

### Library Contents

This library contains two modules: the generated API client and the core Python library, which includes handwritten utilities to make working with the API easier, including SSO support.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Public vs. Secured APIs

For the API client, there are two classes, `DefaultApi` and `PublicApi`. The `DefaultApi` contains methods that require your API key, and `PublicApi` contains API calls that can be made directly from a browser/mobile device/etc without authentication.