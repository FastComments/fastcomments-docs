### Utilisation des API authentifiées (DefaultApi)

**Important :** Vous devez définir votre clé API dans la Configuration avant d'effectuer des requêtes authentifiées. Sinon, les requêtes échoueront avec une erreur 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Créez et configurez le client API
config = Configuration()
config.host = "https://fastcomments.com"

# OBLIGATOIRE : définissez votre clé API (obtenez-la depuis votre tableau de bord FastComments)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

# Créez l'instance API avec le client configuré
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Vous pouvez maintenant effectuer des appels API authentifiés
try:
    # Exemple : ajouter un utilisateur SSO
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user("YOUR_TENANT_ID", user_data)
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # Erreurs courantes :
    # - 401 : la clé API est manquante ou invalide
    # - 400 : la validation de la requête a échoué
```

### Utilisation des API publiques (PublicApi)

Les points de terminaison publics ne nécessitent pas d'authentification :

```python
from client import ApiClient, Configuration, PublicApi

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
public_api = PublicApi(api_client)

try:
    response = public_api.get_comments_public("YOUR_TENANT_ID", "page-url-id")
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Utilisation du tableau de bord de modération (ModerationApi)

Le `ModerationApi` alimente le tableau de bord des modérateurs. Les méthodes sont appelées au nom d'un modérateur en passant un jeton `sso` :

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Compter les commentaires en attente de modération
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Utilisation du SSO (Single Sign-On)

Le SDK inclut des utilitaires pour générer des jetons SSO sécurisés :

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Créez les données utilisateur (id, email et nom d'utilisateur sont requis)
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Signez-le avec votre secret API (HMAC-SHA256)
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# Générez le jeton SSO à transmettre au widget ou à un appel API
sso_token = sso.create_token()

# Utilisez ce jeton dans votre frontend ou transmettez-le aux appels API
print(f"SSO Token: {sso_token}")
```

Pour le SSO simple (moins sécurisé, à des fins de test) :

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    username="johndoe",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Souscriptions en direct (PubSub)

Le module `pubsub` vous permet de vous abonner aux événements de commentaires en temps réel (nouveaux commentaires, votes, modifications, notifications, etc.) via un WebSocket, reproduisant le `LiveEventSubscriber` du SDK Java FastComments. Il nécessite l'extra `pubsub`, qui ajoute un client WebSocket au-dessus du client API généré :

```bash
pip install "fastcomments[pubsub] @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0"
```

```python
from pubsub import LiveEventSubscriber

subscriber = LiveEventSubscriber()

def handle_live_event(event):
    print(f"Live event: {event.type}")
    if event.comment:
        print(f"  comment: {event.comment.comment}")

result = subscriber.subscribe_to_changes(
    tenant_id_ws="YOUR_TENANT_ID",
    url_id="page-url-id",
    url_id_ws="page-url-id",
    user_id_ws="a-unique-presence-id",  # par ex. un UUID pour cette session
    handle_live_event=handle_live_event,
    on_connection_status_change=lambda connected, last_event_time: print(
        f"connected={connected}"
    ),
    region=None,  # définir sur "eu" pour la région UE
)

# ...plus tard, lorsque vous ne souhaitez plus de mises à jour :
result.close()
```

Le souscripteur exécute la connexion sur un thread daemon en arrière-plan, se reconnecte de manière transparente avec du jitter, et récupère les événements manqués pendant la déconnexion du point de terminaison du journal d'événements lors de la reconnexion. Passez un rappel optionnel `can_see_comments` (`List[str] -> Dict[str, str]`, renvoyant les identifiants que l'utilisateur ne doit PAS voir) pour filtrer les événements des commentaires que l'utilisateur n'est pas autorisé à voir. Définissez `disable_live_commenting=True` pour que `subscribe_to_changes` devienne une opération nulle qui renvoie `None`.

### Problèmes courants

1. **Erreur 401 "missing-api-key"** : Assurez-vous d'avoir défini `config.api_key = {"api_key": "YOUR_KEY"}` avant de créer l'instance DefaultApi.
2. **Classe API incorrecte** : Utilisez `DefaultApi` pour les requêtes authentifiées côté serveur, `PublicApi` pour les requêtes côté client/public, et `ModerationApi` pour les requêtes du tableau de bord des modérateurs.
3. **Erreurs d'importation** : Assurez-vous d'importer depuis le bon module :
   - Client API : `from client import ...`
   - Utilitaires SSO : `from sso import ...`
   - Souscriptions en direct : `from pubsub import ...` (nécessite l'extra `pubsub`)