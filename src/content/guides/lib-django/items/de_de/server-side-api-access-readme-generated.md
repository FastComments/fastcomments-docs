Mit dem `[api]`‑Extra installiert, rufen Sie die FastComments REST API über das SDK auf, das mit Ihrem API‑Schlüssel und Ihrer Region vorkonfiguriert ist:

```python
from fastcomments_django import admin, public_api, get_manager

admin().get_comments("YOUR_TENANT_ID", ...)     # authentifiziert (DefaultApi)
public_api().get_comments_public(...)            # öffentlich (PublicApi)

# Generieren Sie ein SSO‑Token für API‑Aufrufe oder die Weitergabe an den Client:
token = get_manager().sso().token_for(request.user)
```