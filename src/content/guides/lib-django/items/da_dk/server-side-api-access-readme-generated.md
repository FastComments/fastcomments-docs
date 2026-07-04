Med udvidelsen `[api]` installeret kan du kalde FastComments REST API'en gennem SDK'en, som er forudkonfigureret med din API-nøgle og region:

```python
from fastcomments_django import admin, public_api, get_manager

admin().get_comments("YOUR_TENANT_ID", ...)     # godkendt (DefaultApi)
public_api().get_comments_public(...)            # offentlig (PublicApi)

# Generér en SSO-token til API‑kald eller klientoverdragelse:
token = get_manager().sso().token_for(request.user)
```