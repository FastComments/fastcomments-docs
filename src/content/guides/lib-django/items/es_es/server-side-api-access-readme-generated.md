---
Con el extra `[api]` instalado, llama a la API REST de FastComments a través del SDK,
preconfigurado con tu clave API y región:

```python
from fastcomments_django import admin, public_api, get_manager

admin().get_comments("YOUR_TENANT_ID", ...)     # autenticado (DefaultApi)
public_api().get_comments_public(...)            # público (PublicApi)

# Genera un token SSO para llamadas API o entrega al cliente:
token = get_manager().sso().token_for(request.user)
```
---