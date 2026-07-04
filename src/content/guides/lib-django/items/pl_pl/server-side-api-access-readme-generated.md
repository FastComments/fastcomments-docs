---
Po zainstalowaniu dodatku `[api]` wywołaj REST API FastComments poprzez SDK,
wstępnie skonfigurowane z Twoim kluczem API i regionem:

```python
from fastcomments_django import admin, public_api, get_manager

admin().get_comments("YOUR_TENANT_ID", ...)     # authenticated (DefaultApi)
public_api().get_comments_public(...)            # public (PublicApi)

# Generate an SSO token for API calls or client hand-off:
token = get_manager().sso().token_for(request.user)
```
---