---
S instaliranim dodatkom `[api]`, pozovite FastComments REST API putem SDK-a,
pre‑konfiguriranog s vašim API ključem i regijom:

```python
from fastcomments_django import admin, public_api, get_manager

admin().get_comments("YOUR_TENANT_ID", ...)     # autentificirano (DefaultApi)
public_api().get_comments_public(...)            # javno (PublicApi)

# Generirajte SSO token za API pozive ili predaju klijentu:
token = get_manager().sso().token_for(request.user)
```
---