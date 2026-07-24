### Bruger af godkendte API'er (DefaultApi)

**Vigtigt:** Du skal indstille din API-nøgle i Configuration, før du foretager godkendte anmodninger. Hvis du ikke gør det, vil anmodninger fejle med en 401-fejl.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Opret og konfigurer API-klienten
config = Configuration()
config.host = "https://fastcomments.com"

# KRÆVET: Indstil din API-nøgle (hent den fra dit FastComments-dashboard)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

# Opret API‑instansen med den konfigurerede klient
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Nu kan du foretage godkendte API‑kald
try:
    # Eksempel: Tilføj en SSO‑bruger
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user("YOUR_TENANT_ID", user_data)
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # Almindelige fejl:
    # - 401: API-nøgle mangler eller er ugyldig
    # - 400: Anmodningsvalidering mislykkedes
```

### Brug af offentlige API'er (PublicApi)

Offentlige slutpunkter kræver ikke godkendelse:

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

### Brug af moderationsdashboardet (ModerationApi)

`ModerationApi` driver moderatordashboardet. Metoder kaldes på vegne af en moderator ved at videregive en `sso`-token:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Tæl kommentarerne, der venter på moderation
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Brug af SSO (Single Sign-On)

SDK'en indeholder værktøjer til at generere sikre SSO-token:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Opret brugerdata (id, email og brugernavn er påkrævet)
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Signer den med din API‑hemmelighed (HMAC‑SHA256)
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# Generer SSO‑tokenet til at videregive til widget’en eller et API‑kald
sso_token = sso.create_token()

# Brug dette token i din frontend eller videregiv til API‑kald
print(f"SSO Token: {sso_token}")
```

For simpel SSO (mindre sikker, til test):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    username="johndoe",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Live-abonnementer (PubSub)

`pubsub`-modulet lader dig abonnere på realtidskommentarbegivenheder (nye kommentarer, stemmer, redigeringer, notifikationer osv.) over en WebSocket, som spejler FastComments Java SDK's `LiveEventSubscriber`. Det kræver `pubsub`-ekstraen, som tilføjer en WebSocket-klient oven på den genererede API-klient:

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
    user_id_ws="a-unique-presence-id",  # f.eks. en UUID for denne session
    handle_live_event=handle_live_event,
    on_connection_status_change=lambda connected, last_event_time: print(
        f"connected={connected}"
    ),
    region=None,  # sæt til "eu" for EU‑regionen
)

# ...senere, når du ikke længere ønsker opdateringer:
result.close()
```

Abonnenten kører forbindelsen på en baggrunds‑daemon‑tråd, genopretter automatisk med jitter, og henter eventuelle begivenheder, der gik glip af, mens den var frakoblet fra event‑log‑slutpunktet ved genforbindelse. Videregiv en valgfri `can_see_comments`‑callback (`List[str] -> Dict[str, str]`, som returnerer de id'er brugeren IKKE må se) for at filtrere begivenheder for kommentarer, som brugeren ikke har tilladelse til at se. Sæt `disable_live_commenting=True` for at gøre `subscribe_to_changes` til en ingen‑operation, der returnerer `None`.

### Almindelige problemer

1. **401 "missing-api-key" fejl**: Sørg for at du indstiller `config.api_key = {"api_key": "YOUR_KEY"}` før du opretter DefaultApi‑instansen.
2. **Forkert API‑klasse**: Brug `DefaultApi` til server‑side godkendte anmodninger, `PublicApi` til klient‑side/offentlige anmodninger, og `ModerationApi` til anmodninger fra moderatordashboardet.
3. **Import‑fejl**: Sørg for at du importerer fra det korrekte modul:
   - API client: `from client import ...`
   - SSO utilities: `from sso import ...`
   - Live subscriptions: `from pubsub import ...` (needs the `pubsub` extra)