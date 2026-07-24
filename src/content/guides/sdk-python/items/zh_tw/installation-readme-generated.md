### Install from GitHub

Install directly from a release tag (recommended, fully reproducible):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Pin the tag rather than a branch so builds are deterministic. The same form works in `requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Each tagged [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) also has a built wheel attached if you prefer to install a binary artifact directly.

### Library Contents

This library contains two modules: the generated API client and the core Python library which contains hand‑written utilities to make working with the API easier, including SSO support.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Public vs Secured APIs

For the API client, there are three classes, `DefaultApi`, `PublicApi`, and `ModerationApi`. The `DefaultApi` contains methods that require your API key, and `PublicApi` contains methods that can be made directly from a browser/mobile device/etc without authentication. The `ModerationApi` provides an extensive suite of live and fast moderation APIs. Every `ModerationApi` method accepts an `sso` parameter and can authenticate via SSO or a FastComments.com session cookie.