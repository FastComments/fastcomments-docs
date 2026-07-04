With the `[api]` extra installed, call the FastComments REST API through the SDK,
pre-configured with your API key and region:

```python
from fastcomments_django import admin, public_api, get_manager

admin().get_comments("YOUR_TENANT_ID", ...)     # kimlik doğrulandı (DefaultApi)
public_api().get_comments_public(...)            # herkese açık (PublicApi)

# API çağrıları veya istemci devri için bir SSO token'ı oluşturun:
token = get_manager().sso().token_for(request.user)
```