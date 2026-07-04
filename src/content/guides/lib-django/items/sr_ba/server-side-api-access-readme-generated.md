Sa instaliranim dodatkom `[api]`, pozovite FastComments REST API putem SDK‑a, prekonfigurisanim sa vašim API ključem i regionom:

```python
from fastcomments_django import admin, public_api, get_manager

admin().get_comments("YOUR_TENANT_ID", ...)     # autentifikovan (DefaultApi)
public_api().get_comments_public(...)            # javni (PublicApi)

# Generišite SSO token za API pozive ili prenosa klijenta:
token = get_manager().sso().token_for(request.user)
```