---
Avec l'extra `[api]` installé, appelez l'API REST de FastComments via le SDK,
pré‑configuré avec votre clé API et votre région :

```python
from fastcomments_django import admin, public_api, get_manager

admin().get_comments("YOUR_TENANT_ID", ...)     # authenticated (DefaultApi)
public_api().get_comments_public(...)            # public (PublicApi)

# Generate an SSO token for API calls or client hand-off:
token = get_manager().sso().token_for(request.user)
```
---