### Using Authenticated APIs (DefaultApi)

**Important:** You must set your API key on the Configuration before making authenticated requests. If you don't, requests will fail with a 401 error.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Kreirajte i konfigurišite API klijent
config = Configuration()
config.host = "https://fastcomments.com"

# OBAVEZNO: Postavite svoj API ključ (preuzmite ga sa FastComments kontrolne table)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

# Kreirajte API instancu sa konfigurisanim klijentom
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Sada možete vršiti autentifikovane API pozive
try:
    # Primer: Dodavanje SSO korisnika
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user("YOUR_TENANT_ID", user_data)
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # Uobičajene greške:
    # - 401: API ključ nedostaje ili je neispravan
    # - 400: Validacija zahteva nije uspela
```

### Using Public APIs (PublicApi)

Public endpoints don't require authentication:

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

### Using the Moderation Dashboard (ModerationApi)

The `ModerationApi` powers the moderator dashboard. Methods are called on behalf of a moderator by passing an `sso` token:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Prebrojite komentare koji čekaju moderaciju
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Using SSO (Single Sign-On)

The SDK includes utilities for generating secure SSO tokens:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Kreirajte podatke o korisniku (id, email i korisničko ime su obavezni)
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Potpišite ga svojim API tajnim ključem (HMAC-SHA256)
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# Generišite SSO token koji ćete proslediti widgetu ili API pozivu
sso_token = sso.create_token()

# Koristite ovaj token u svom frontendu ili ga prosledite API pozivima
print(f"SSO Token: {sso_token}")
```

For simple SSO (less secure, for testing):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    username="johndoe",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Live Subscriptions (PubSub)

The `pubsub` module lets you subscribe to real-time comment events (new comments, votes, edits, notifications, etc.) over a WebSocket, mirroring the FastComments Java SDK's `LiveEventSubscriber`. It requires the `pubsub` extra, which adds a WebSocket client on top of the generated API client:

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
    user_id_ws="a-unique-presence-id",  # npr. UUID za ovu sesiju
    handle_live_event=handle_live_event,
    on_connection_status_change=lambda connected, last_event_time: print(
        f"connected={connected}"
    ),
    region=None,  # postavite na "eu" za EU region
)

# ...kasnije, kada više ne želite ažuriranja:
result.close()
```

The subscriber runs the connection on a background daemon thread, transparently reconnects with jitter, and fetches any events missed while disconnected from the event-log endpoint on reconnect. Pass an optional `can_see_comments` callback (`List[str] -> Dict[str, str]`, returning the ids the user may NOT see) to filter out events for comments the user is not allowed to view. Set `disable_live_commenting=True` to make `subscribe_to_changes` a no-op that returns `None`.

### Common Issues

1. **401 "missing-api-key" error**: Make sure you set `config.api_key = {"api_key": "YOUR_KEY"}` before creating the DefaultApi instance.
2. **Wrong API class**: Use `DefaultApi` for server-side authenticated requests, `PublicApi` for client-side/public requests, and `ModerationApi` for moderator dashboard requests.
3. **Import errors**: Make sure you're importing from the correct module:
   - API client: `from client import ...`
   - SSO utilities: `from sso import ...`
   - Live subscriptions: `from pubsub import ...` (needs the `pubsub` extra)
---