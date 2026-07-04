Avec l'extra `[api]` installé, appelez l'API REST FastComments via le SDK, préconfiguré avec votre clé d'API et votre région :

```python
from fastcomments_django import admin, public_api, get_manager

admin().get_comments("YOUR_TENANT_ID", ...)     # authentifié (DefaultApi)
public_api().get_comments_public(...)            # public (PublicApi)

# Générer un jeton SSO pour les appels API ou la remise au client :
token = get_manager().sso().token_for(request.user)
```