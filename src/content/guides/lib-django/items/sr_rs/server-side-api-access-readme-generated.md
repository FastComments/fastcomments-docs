Sa instaliranim dodatkom `[api]`, позовите FastComments REST API преко SDK‑а, који је унапред конфигурисан са вашим API кључем и регионом:

```python
from fastcomments_django import admin, public_api, get_manager

admin().get_comments("YOUR_TENANT_ID", ...)     # аутентификовано (DefaultApi)
public_api().get_comments_public(...)            # јавно (PublicApi)

# Генерисати SSO токен за API позиве или предају клијенту:
token = get_manager().sso().token_for(request.user)
```