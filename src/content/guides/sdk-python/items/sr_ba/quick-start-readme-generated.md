### Korišćenje autentifikovanih API‑ja (DefaultApi)

**Važno:** Morate postaviti svoj API ključ u Configuration prije nego što napravite autentifikovane zahtjeve. Ako to ne učinite, zahtjevi će propasti s greškom 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Kreirajte i konfigurišite API klijent
config = Configuration()
config.host = "https://fastcomments.com"

# OBAVEZNO: Postavite svoj API ključ (preuzmite ga sa FastComments kontrolne table)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

# Kreirajte API instancu s konfiguriranim klijentom
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
    # - 400: Validacija zahtjeva nije uspjela
```

### Korišćenje javnih API‑ja (PublicApi)

Javni endpointi ne zahtijevaju autentifikaciju:

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

### Korišćenje moderacijskog dashboarda (ModerationApi)

`ModerationApi` pokreće moderacijski dashboard. Metode se pozivaju u ime moderatora prosljeđivanjem `sso` tokena:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Broj komentara koji čekaju moderaciju
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Korišćenje SSO (Single Sign-On)

SDK uključuje alate za generisanje sigurnih SSO tokena:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Kreirajte podatke o korisniku (id, email i korisničko ime su obavezni)
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Potpišite ga svojim API tajnim (HMAC-SHA256)
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# Generišite SSO token koji ćete proslijediti widgetu ili API pozivu
sso_token = sso.create_token()

# Koristite ovaj token u frontendu ili ga proslijedite API pozivima
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

### Live pretplate (PubSub)

Modul `pubsub` vam omogućava pretplatu na događaje komentara u realnom vremenu (novi komentari, glasovi, izmjene, obavijesti, itd.) preko WebSocket‑a, što je analogno `LiveEventSubscriber`‑u FastComments Java SDK‑a. Potreban je dodatak `pubsub`, koji dodaje WebSocket klijent na generirani API klijent:

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
    region=None,  # postaviti na "eu" za EU regiju
)

# ...kasnije, kada više ne želite ažuriranja:
result.close()
```

Pretplatnik pokreće vezu na pozadinskoj daemon niti, transparentno se ponovo povezuje s jitter‑om i dohvaća sve događaje propuštene dok je bio odspojen od endpointa event‑log pri ponovnom povezivanju. Proslijedite opcionalni `can_see_comments` callback (`List[str] -> Dict[str, str]`), koji vraća ID‑ove koje korisnik NE smije vidjeti, kako biste filtrirali događaje za komentare koje korisnik ne smije vidjeti. Postavite `disable_live_commenting=True` da `subscribe_to_changes` postane no‑op koji vraća `None`.

### Česti problemi

1. **401 "missing-api-key" greška**: Provjerite da ste postavili `config.api_key = {"api_key": "YOUR_KEY"}` prije kreiranja DefaultApi instance.
2. **Pogrešna API klasa**: Koristite `DefaultApi` za server‑side autentifikovane zahtjeve, `PublicApi` za klijentske/javne zahtjeve i `ModerationApi` za zahtjeve moderacijskog dashboarda.
3. **Greške pri uvozu**: Provjerite da uvozite iz ispravnog modula:
   - API klijent: `from client import ...`
   - SSO alati: `from sso import ...`
   - Live pretplate: `from pubsub import ...` (potreban je `pubsub` dodatak)

---