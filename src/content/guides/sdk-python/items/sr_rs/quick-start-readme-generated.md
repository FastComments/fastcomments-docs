### Korišćenje autentifikovanih API‑ja (DefaultApi)

**Važno:** Morate postaviti vaš API ključ u Configuration pre nego što izvršite autentifikovane zahteve. Ako to ne uradite, zahtevi će propasti sa greškom 401.

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

### Korišćenje javnih API‑ja (PublicApi)

Javni krajnji poeni ne zahtevaju autentifikaciju:

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

### Korišćenje kontrolne table za moderaciju (ModerationApi)

`ModerationApi` pokreće kontrolnu tablu moderatora. Metode se pozivaju u ime moderatora prosleđivanjem `sso` tokena:

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

### Korišćenje SSO (Single Sign-On)

SDK uključuje alate za generisanje sigurnih SSO tokena:

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

Za jednostavni SSO (manje siguran, za testiranje):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    username="johndoe",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Pretplate u realnom vremenu (PubSub)

`pubsub` modul vam omogućava da se pretplatite na događaje komentara u realnom vremenu (novi komentari, glasanja, izmene, obaveštenja, itd.) preko WebSocket‑a, što je analogno `LiveEventSubscriber`‑u iz FastComments Java SDK‑a. Zahteva `pubsub` dodatak, koji dodaje WebSocket klijent na vrh generisanog API klijenta:

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

Pretplatnik pokreće vezu na pozadinskoj daemon niti, transparentno se ponovo povezuje uz jitter i preuzima sve događaje koje je propustio dok je bio isključen sa endpoint‑a za log događaja prilikom ponovnog povezivanja. Prosledite opcioni `can_see_comments` callback (`List[str] -> Dict[str, str]`, koji vraća ID‑ove koje korisnik NE sme da vidi) da filtrirate događaje za komentare koje korisnik nije ovlašćen da vidi. Postavite `disable_live_commenting=True` da `subscribe_to_changes` postane no‑op koji vraća `None`.

### Česti problemi

1. **401 greška "missing-api-key"**: Uverite se da ste postavili `config.api_key = {"api_key": "YOUR_KEY"}` pre kreiranja DefaultApi instance.
2. **Pogrešna API klasa**: Koristite `DefaultApi` za server‑side autentifikovane zahteve, `PublicApi` za klijentske/javne zahteve i `ModerationApi` za zahteve kontrolne table moderatora.
3. **Greške pri uvozu**: Uverite se da uvozite iz ispravnog modula:
   - API klijent: `from client import ...`
   - SSO alati: `from sso import ...`
   - Live pretplate: `from pubsub import ...` (zahteva `pubsub` dodatak)