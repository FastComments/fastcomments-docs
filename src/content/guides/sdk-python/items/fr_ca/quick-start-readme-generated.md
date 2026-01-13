### Utilisation des API authentifiées (DefaultApi)

**Important :** Vous devez configurer votre clé API dans Configuration avant d'effectuer des requêtes authentifiées. Si vous ne le faites pas, les requêtes échoueront avec une erreur 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Créez et configurez le client API
config = Configuration()
config.host = "https://fastcomments.com/api"

# REQUIS : Définissez votre clé API (obtenez-la depuis votre tableau de bord FastComments)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Créez l'instance de l'API avec le client configuré
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Vous pouvez maintenant effectuer des appels API authentifiés
try:
    # Exemple : Ajouter un utilisateur SSO
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user(
        tenant_id="YOUR_TENANT_ID",
        create_apisso_user_data=user_data
    )
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # Erreurs courantes :
    # - 401 : la clé API est manquante ou invalide
    # - 400 : la validation de la requête a échoué
```

### Utilisation des API publiques (PublicApi)

Les endpoints publics ne nécessitent pas d'authentification :

```python
from client import ApiClient, Configuration, PublicApi

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
public_api = PublicApi(api_client)

try:
    response = public_api.get_comments_public(
        tenant_id="YOUR_TENANT_ID",
        url_id="page-url-id"
    )
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Utilisation du SSO (Single Sign-On)

Le SDK inclut des utilitaires pour générer des jetons SSO sécurisés :

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Créez les données utilisateur
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Créez l'instance SSO avec votre secret API
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Générez le jeton SSO
sso_token = sso.create_token()

# Utilisez ce jeton dans votre frontend ou transmettez-le aux appels API
print(f"SSO Token: {sso_token}")
```

For simple SSO (less secure, for testing):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    user_id="user-123",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Problèmes courants

1. **401 "missing-api-key" erreur** : Assurez-vous d'avoir défini `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` avant de créer l'instance DefaultApi.
2. **Mauvaise classe API** : Utilisez `DefaultApi` pour les requêtes authentifiées côté serveur, `PublicApi` pour les requêtes côté client/publiques.
3. **Erreurs d'importation** : Assurez-vous d'importer depuis le bon module :
   - Client API : `from client import ...`
   - Utilitaires SSO : `from sso import ...`