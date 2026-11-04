### Korišćenje autentifikovanih API-ja (DefaultApi)

**Važno:** Morate postaviti svoj API ključ na Configuration pre slanja autentifikovanih zahteva. Ako to ne uradite, zahtevi će pasti sa 401 greškom.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Kreirajte i konfigurišite API klijenta
config = Configuration()
config.host = "https://fastcomments.com"

# OBAVEZNO: Postavite svoj API ključ (preuzmite ga iz FastComments kontrolne table)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

# Kreirajte API instancu sa konfigurisanim klijentom
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Sada možete slati autentifikovane API pozive
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
    # Česte greške:
    # - 401: API ključ nedostaje ili je nevažeći
    # - 400: Validacija zahteva nije uspela
```

### Korišćenje javnih API-ja (PublicApi)

Javni endpoint-i ne zahtevaju autentifikaciju:

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

`ModerationApi` napaja moderatorsku kontrolnu tablu. Metode se pozivaju u ime moderatora prosleđivanjem `sso` tokena:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Prebroj komentare koji čekaju moderaciju
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Korišćenje SSO-a (Single Sign-On)

SDK uključuje pomoćne funkcije za generisanje sigurnih SSO tokena:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Kreirajte podatke o korisniku (id, email i username su obavezni)
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Potpišite ih svojim API sekretom (HMAC-SHA256)
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# Generišite SSO token koji ćete proslediti vidžetu ili API pozivu
sso_token = sso.create_token()

# Koristite ovaj token u svom frontendu ili ga prosledite API pozivima
print(f"SSO Token: {sso_token}")
```

Za jednostavan SSO (manje siguran, za testiranje):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    username="johndoe",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

Live pretplate (PubSub)

Modul `pubsub` vam omogućava da se pretplatite na događaje komentara u realnom vremenu (novi komentari, glasovi, izmene, notifikacije itd.) preko WebSocket-a, preslikavajući `LiveEventSubscriber` iz FastComments Java SDK-a. Zahteva `pubsub` ekstra, koji dodaje WebSocket klijent na generisani API klijent:

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

Subscriber pokreće vezu na pozadinskom daemon thread-u, transparentno se ponovo povezuje sa jitter-om i na ponovno povezivanje dohvata sve događaje propuštene tokom prekida veze sa event-log endpoint-a. Prosledite opcioni `can_see_comments` callback (`List[str] -> Dict[str, str]`, koji vraća id-jeve koje korisnik NE sme da vidi) da biste filtrirali događaje za komentare koje korisnik nema pravo da vidi. Postavite `disable_live_commenting=True` da `subscribe_to_changes` postane no-op koji vraća `None`.

### Česti problemi

1. **401 "missing-api-key" greška**: Proverite da ste postavili `config.api_key = {"api_key": "YOUR_KEY"}` pre kreiranja DefaultApi instance.
2. **Pogrešna API klasa**: Koristite `DefaultApi` za server-side autentifikovane zahteve, `PublicApi` za client-side/javne zahteve, i `ModerationApi` za zahteve moderatorske kontrolne table.
3. **Greške pri importu**: Proverite da importujete iz ispravnog modula:
   - API klijent: `from client import ...`
   - SSO pomoćne funkcije: `from sso import ...`
   - Live pretplate: `from pubsub import ...` (potreban je `pubsub` ekstra)