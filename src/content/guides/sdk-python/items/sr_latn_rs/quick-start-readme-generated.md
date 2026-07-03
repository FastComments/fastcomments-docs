### Korišćenje autentifikovanih API-ja (DefaultApi)

**Važno:** Morate postaviti vaš API ključ u Configuration pre slanja autentifikovanih zahteva. Ako to ne učinite, zahtevi će propasti sa greškom 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Kreirajte i konfigurišite API klijent
config = Configuration()
config.host = "https://fastcomments.com"

# OBAVEZNO: Postavite vaš API ključ (preuzmite ga sa FastComments kontrolne table)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

# Kreirajte instancu API-ja sa konfigurisanim klijentom
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

### Korišćenje javnih API-ja (PublicApi)

Javni endpointi ne zahtevaju autentifikaciju:

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

`ModerationApi` napaja kontrolnu tablu moderatora. Metode se pozivaju u ime moderatora prosleđivanjem `sso` tokena:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Prebrojite komentare koji čekaju na moderaciju
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

# Potpišite ga vašim API tajnim ključem (HMAC-SHA256)
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# Generišite SSO token koji ćete proslediti widgetu ili API pozivu
sso_token = sso.create_token()

# Koristite ovaj token u vašem frontendu ili ga prosledite API pozivima
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

### Uobičajeni problemi

1. **401 "missing-api-key" greška**: Uverite se da ste postavili `config.api_key = {"api_key": "YOUR_KEY"}` pre kreiranja DefaultApi instance.
2. **Pogrešna API klasa**: Koristite `DefaultApi` za server‑side autentifikovane zahteve, `PublicApi` za klijentske/javne zahteve i `ModerationApi` za zahteve kontrolne table moderatora.
3. **Greške pri uvozu**: Uverite se da uvozite iz ispravnog modula:
   - API klijent: `from client import ...`
   - SSO alati: `from sso import ...`