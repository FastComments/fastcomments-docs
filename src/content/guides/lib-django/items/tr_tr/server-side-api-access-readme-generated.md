`[api]` eklentisi yüklüyken, FastComments REST API'sine SDK üzerinden erişin;
API anahtarınız ve bölgenizle önceden yapılandırılmış şekilde:

```python
from fastcomments_django import admin, public_api, get_manager

admin().get_comments("YOUR_TENANT_ID", ...)     # authenticated (DefaultApi)
public_api().get_comments_public(...)            # public (PublicApi)

# API çağrıları veya istemciye devir için bir SSO token'ı oluşturun:
token = get_manager().sso().token_for(request.user)
```