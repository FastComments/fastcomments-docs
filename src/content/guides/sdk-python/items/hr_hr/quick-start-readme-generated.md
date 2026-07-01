### Korištenje autentificiranih API-ja (DefaultApi)

**Važno:** Morate postaviti svoj API ključ u Configuration prije izvršavanja autentificiranih zahtjeva. Ako to ne učinite, zahtjevi će propasti s 401 greškom.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Kreirajte i konfigurirajte API klijent
config = Configuration()
config.host = "https://fastcomments.com/api"

# OBAVEZNO: Postavite svoj API ključ (preuzmite ga iz FastComments nadzorne ploče)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Kreirajte API instancu s konfiguriranim klijentom
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Sada možete izvršavati autentificirane API pozive
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
    # Uobičajene greške:
    # - 401: API key is missing or invalid
    # - 400: Request validation failed
```

### Korištenje javnih API-ja (PublicApi)

Javni krajnji točke ne zahtijevaju autentifikaciju:

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

### Korištenje moderacijskog nadzornog panela (ModerationApi)

`ModerationApi` pokreće moderacijski nadzorni panel. Metode se pozivaju u ime moderatora prosljeđivanjem `sso` tokena:

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

### Korištenje SSO (Single Sign-On)

SDK uključuje alate za generiranje sigurnih SSO tokena:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Kreirajte podatke o korisniku
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Kreirajte SSO instancu s vašim API tajnom
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Generirajte SSO token
sso_token = sso.create_token()

# Koristite ovaj token u vašem frontendu ili ga proslijedite API pozivima
print(f"SSO Token: {sso_token}")
```

Za jednostavni SSO (manje siguran, za testiranje):

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

1. **401 "missing-api-key" greška**: Provjerite jeste li postavili `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` prije stvaranja DefaultApi instance.
2. **Pogrešna API klasa**: Koristite `DefaultApi` za server-side autentificirane zahtjeve, `PublicApi` za klijentske/javne zahtjeve i `ModerationApi` za zahtjeve moderacijskog nadzornog panela.
3. **Greške pri uvozu**: Provjerite da uvozite iz ispravnog modula:
   - API klijent: `from client import ...`
   - SSO alati: `from sso import ...`