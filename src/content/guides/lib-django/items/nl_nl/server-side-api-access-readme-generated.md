---
Met de `[api]` extra geïnstalleerd, roep de FastComments REST API aan via de SDK,
vooraf geconfigureerd met uw API‑sleutel en regio:

```python
from fastcomments_django import admin, public_api, get_manager

admin().get_comments("YOUR_TENANT_ID", ...)     # authenticated (DefaultApi)
public_api().get_comments_public(...)            # public (PublicApi)

# Generate an SSO token for API calls or client hand-off:
token = get_manager().sso().token_for(request.user)
```
---