---
С инсталирания екстра `[api]`, извикайте FastComments REST API чрез SDK‑то, предварително конфигурирано с вашия API ключ и регион:

```python
from fastcomments_django import admin, public_api, get_manager

admin().get_comments("YOUR_TENANT_ID", ...)     # автентифициран (DefaultApi)
public_api().get_comments_public(...)            # публичен (PublicApi)

# Генерирайте SSO токен за API повиквания или предаване към клиент:
token = get_manager().sso().token_for(request.user)
```
---