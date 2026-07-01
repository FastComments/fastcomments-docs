### Korišćenje autentifikovanih API‑ja (DefaultApi)

**Važno:** Morate postaviti vaš API ključ u Configuration prije nego što izvršite autentifikovane zahtjeve. Ako to ne učinite, zahtjevi će se završiti greškom 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Kreirajte i konfigurirajte API klijent
config = Configuration()
config.host = "https://fastcomments.com/api"

# OBAVEZNNO: Postavite vaš API ključ (preuzmite ga sa vašeg FastComments kontrolnog panela)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Kreirajte API instancu s konfiguriranim klijentom
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Sada možete izvršavati autentifikovane API pozive
try:
    # Primjer: Dodajte SSO korisnika
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
    # - 401: API ključ nedostaje ili je neispravan
    # - 400: Validacija zahtjeva nije uspjela
```

### Korišćenje javnih API‑ja (PublicApi)

Javni endpointi ne zahtijevaju autentifikaciju:

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

### Korišćenje upravljačke ploče za moderaciju (ModerationApi)

`ModerationApi` omogućava rad moderacijske kontrolne ploče. Metode se pozivaju u ime moderatora prosljeđivanjem `sso` tokena:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Prebrojite komentare koji čekaju moderaciju
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Korišćenje SSO (Single Sign-On)

SDK uključuje alate za generisanje sigurnih SSO tokena:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Kreirajte podatke o korisniku
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Kreirajte SSO instancu sa vašim API tajnim ključem
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Generišite SSO token
sso_token = sso.create_token()

# Upotrijebite ovaj token u vašem frontendu ili ga proslijedite API pozivima
print(f"SSO Token: {sso_token}")
```

Za jednostavno SSO (manje sigurno, za testiranje):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    user_id="user-123",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Česte poteškoće

1. **401 „missing‑api‑key“ greška**: Provjerite da ste postavili `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` prije kreiranja DefaultApi instance.
2. **Pogrešna API klasa**: Koristite `DefaultApi` za server‑side autentifikovane zahtjeve, `PublicApi` za client‑side/javne zahtjeve, i `ModerationApi` za zahtjeve moderacijske kontrolne ploče.
3. **Greške pri uvozu**: Provjerite da **uvezete** iz ispravnog **modula**:
   - API klijent: `from client import ...`
   - SSO alati: `from sso import ...`