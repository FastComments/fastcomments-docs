---
Con l'extra `[api]` installato, chiama l'API REST di FastComments tramite l'SDK, pre‑configurato con la tua chiave API e la regione:

```python
from fastcomments_django import admin, public_api, get_manager

admin().get_comments("YOUR_TENANT_ID", ...)     # authenticated (DefaultApi)
public_api().get_comments_public(...)            # public (PublicApi)

# Generate an SSO token for API calls or client hand-off:
token = get_manager().sso().token_for(request.user)
```
---