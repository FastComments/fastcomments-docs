### Utilizzo delle API autenticate (DefaultApi)

**Importante:** È necessario impostare la propria API key sulla Configuration prima di effettuare richieste autenticate. In caso contrario, le richieste falliranno con un errore 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Create and configure the API client
config = Configuration()
config.host = "https://fastcomments.com"

# REQUIRED: Set your API key (get this from your FastComments dashboard)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

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

### Utilizzo delle API pubbliche (PublicApi)

Gli endpoint pubblici non richiedono autenticazione:

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

### Utilizzo della dashboard di moderazione (ModerationApi)

La `ModerationApi` alimenta la dashboard del moderatore. I metodi vengono chiamati per conto di un moderatore passando un token `sso`:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Count the comments awaiting moderation
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Utilizzo di SSO (Single Sign-On)

L'SDK include utilità per generare token SSO sicuri:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Create user data (id, email, and username are required)
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Sign it with your API secret (HMAC-SHA256)
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# Generate the SSO token to pass to the widget or an API call
sso_token = sso.create_token()

# Use this token in your frontend or pass to API calls
print(f"SSO Token: {sso_token}")
```

Per un SSO semplice (meno sicuro, per test):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    username="johndoe",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Sottoscrizioni live (PubSub)

Il modulo `pubsub` consente di sottoscrivere eventi di commenti in tempo reale (nuovi commenti, voti, modifiche, notifiche, ecc.) tramite WebSocket, replicando il `LiveEventSubscriber` dell'SDK Java di FastComments. Richiede l'extra `pubsub`, che aggiunge un client WebSocket sopra il client API generato:

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
    user_id_ws="a-unique-presence-id",  # e.g. a UUID for this session
    handle_live_event=handle_live_event,
    on_connection_status_change=lambda connected, last_event_time: print(
        f"connected={connected}"
    ),
    region=None,  # set to "eu" for the EU region
)

# ...later, when you no longer want updates:
result.close()
```

Il subscriber esegue la connessione su un thread daemon in background, si riconnette in modo trasparente con jitter e recupera dalla endpoint del registro eventi eventuali eventi persi durante la disconnessione. Passare una callback opzionale `can_see_comments` (`List[str] -> Dict[str, str]`, che restituisce gli id che l'utente NON può vedere) per filtrare gli eventi relativi ai commenti che l'utente non è autorizzato a visualizzare. Impostare `disable_live_commenting=True` per rendere `subscribe_to_changes` una no-op che restituisce `None`.

### Problemi comuni

1. **Errore 401 "missing-api-key"**: Assicurarsi di aver impostato `config.api_key = {"api_key": "YOUR_KEY"}` prima di creare l'istanza DefaultApi.
2. **Classe API errata**: Usare `DefaultApi` per le richieste autenticate lato server, `PublicApi` per le richieste pubbliche lato client e `ModerationApi` per le richieste della dashboard del moderatore.
3. **Errori di import**: Assicurarsi di importare dal modulo corretto:
   - Client API: `from client import ...`
   - Utilità SSO: `from sso import ...`
   - Sottoscrizioni live: `from pubsub import ...` (richiede l'extra `pubsub`)