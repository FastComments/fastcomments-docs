### Utilisation des API authentifiées (DefaultApi)

**Important :** Vous devez définir votre clé API dans la Configuration avant d'effectuer des requêtes authentifiées. Sinon, les requêtes échoueront avec une erreur 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Create and configure the API client
config = Configuration()
config.host = "https://fastcomments.com/api"

# REQUIRED: Set your API key (get this from your FastComments dashboard)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Create the API instance with the configured client
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Now you can make authenticated API calls
try:
    # Example: Add an SSO user
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user("YOUR_TENANT_ID", user_data)
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # Common errors:
    # - 401: API key is missing or invalid
    # - 400: Request validation failed
```

### Utilisation des API publiques (PublicApi)

Les points de terminaison publics ne nécessitent pas d'authentification :

```python
from client import ApiClient, Configuration, PublicApi

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
public_api = PublicApi(api_client)

try:
    response = public_api.get_comments_public("YOUR_TENANT_ID", "page-url-id")
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Utilisation du tableau de bord de modération (ModerationApi)

`ModerationApi` alimente le tableau de bord des modérateurs. Les méthodes sont appelées au nom d'un modérateur en transmettant un jeton `sso` :

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Count the comments awaiting moderation
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Utilisation du SSO (Single Sign-On)

Le SDK inclut des utilitaires pour générer des jetons SSO sécurisés :

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Create user data
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Create SSO instance with your API secret
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Generate the SSO token
sso_token = sso.create_token()

# Use this token in your frontend or pass to API calls
print(f"SSO Token: {sso_token}")
```

Pour un SSO simple (moins sécurisé, à des fins de test) :

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

1. **Erreur 401 « missing-api-key »** : Assurez‑vous de définir `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` avant de créer l'instance DefaultApi.  
2. **Classe d'API incorrecte** : Utilisez `DefaultApi` pour les requêtes authentifiées côté serveur, `PublicApi` pour les requêtes côté client/public, et `ModerationApi` pour les requêtes du tableau de bord des modérateurs.  
3. **Erreurs d'importation** : Assurez‑vous d'importer depuis le bon module :  
   - Client API : `from client import ...`  
   - Utilitaires SSO : `from sso import ...`