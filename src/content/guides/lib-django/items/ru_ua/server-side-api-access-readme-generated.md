С установленным дополнительным модулем `[api]` вызовите REST API FastComments через SDK,  
предварительно настроенный с вашим API‑ключом и регионом:

```python
from fastcomments_django import admin, public_api, get_manager

admin().get_comments("YOUR_TENANT_ID", ...)     # аутентифицированный (DefaultApi)
public_api().get_comments_public(...)            # публичный (PublicApi)

# Сгенерировать SSO токен для API вызовов или передачи клиенту:
token = get_manager().sso().token_for(request.user)
```