### Korišćenje autorizovanih API-ja (DefaultApi)

**Važno:** Morate postaviti vaš API ključ u Configuration pre nego što napravite autorizovane zahteve. Ako to ne uradite, zahtevi će vratiti grešku 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Kreirajte i konfigurišite API klijent
config = Configuration()
config.host = "https://fastcomments.com/api"

# OBAVEZNO: Postavite vaš API ključ (preuzmite ga sa vašeg FastComments kontrolnog panela)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Kreirajte API instancu sa konfigurisanim klijentom
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Sada možete praviti autorizovane API pozive
try:
    # Primer: Dodavanje SSO korisnika
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user(
        tenant_id="YOUR_TENANT_ID",
        create_apisso_user_data=user_data
    )
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # Uobičajene greške:
    # - 401: API ključ nedostaje ili nije važeći
    # - 400: Validacija zahteva nije uspela
```

### Korišćenje javnih API-ja (PublicApi)

Javni endpointi ne zahtevaju autentifikaciju:

```python
from client import ApiClient, Configuration, PublicApi

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
public_api = PublicApi(api_client)

try:
    response = public_api.get_comments_public(
        tenant_id="YOUR_TENANT_ID",
        url_id="page-url-id"
    )
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Korišćenje SSO (Single Sign-On)

SDK uključuje pomoćne alatke za generisanje sigurnih SSO tokena:

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

# Koristite ovaj token u vašem frontend-u ili ga prosledite API pozivima
print(f"SSO Token: {sso_token}")
```

Za jednostavan SSO (manje sigurno, za testiranje):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

# Kreirajte podatke o korisniku
user_data = SimpleSSOUserData(
    user_id="user-123",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Uobičajeni problemi

1. **401 "missing-api-key" greška**: Uverite se da ste postavili `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` pre nego što kreirate instancu DefaultApi.
2. **Pogrešna API klasa**: Koristite `DefaultApi` za serverske autorizovane zahteve, `PublicApi` za klijentske/javne zahteve.
3. **Greške pri uvozu**: Uverite se da importujete iz ispravnog modula:
   - API klijent: `from client import ...`
   - SSO pomoćni alati: `from sso import ...`