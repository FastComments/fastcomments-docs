Com o extra `[api]` instalado, chame a API REST do FastComments através do SDK,
pré-configurada com sua chave de API e região:

```python
from fastcomments_django import admin, public_api, get_manager

admin().get_comments("YOUR_TENANT_ID", ...)     # autenticado (DefaultApi)
public_api().get_comments_public(...)            # público (PublicApi)

# Gere um token SSO para chamadas de API ou entrega ao cliente:
token = get_manager().sso().token_for(request.user)
```