With the `[api]` extra installed, call the FastComments REST API through the SDK,
pre-configured with your API key and region:

```python
from fastcomments_django import admin, public_api, get_manager

admin().get_comments("YOUR_TENANT_ID", ...)     # аутентифицирано (DefaultApi)
public_api().get_comments_public(...)            # публично (PublicApi)

# Генерирайте SSO токен за API заявки или предаване към клиент:
token = get_manager().sso().token_for(request.user)
```