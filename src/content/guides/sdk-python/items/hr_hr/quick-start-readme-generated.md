### Korištenje autentificiranih API-ja (DefaultApi)

**Važno:** Morate postaviti svoj API ključ u Configuration prije slanja autentificiranih zahtjeva. Ako to ne učinite, zahtjevi će vratiti grešku 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Kreiraj i konfiguriraj API klijenta
config = Configuration()
config.host = "https://fastcomments.com/api"

# OBAVEZNO: Postavite svoj API ključ (dohvatite ga s vaše FastComments nadzorne ploče)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Kreiraj instancu API-ja s konfiguriranim klijentom
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Sada možete pozivati autentificirane API metode
try:
    # Primjer: Dodaj SSO korisnika
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
    # - 401: API ključ nedostaje ili je neispravan
    # - 400: Validacija zahtjeva nije uspjela
```

### Korištenje javnih API-ja (PublicApi)

Javni endpointi ne zahtijevaju autentifikaciju:

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

### Korištenje SSO (Single Sign-On)

SDK uključuje alate za generiranje sigurnih SSO tokena:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Kreiraj podatke o korisniku
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Kreiraj SSO instancu s vašim API tajnim ključem
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Generiraj SSO token
sso_token = sso.create_token()

# Koristite ovaj token u frontend-u ili ga proslijedite API pozivima
print(f"SSO Token: {sso_token}")
```

For simple SSO (less secure, for testing):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    user_id="user-123",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Uobičajeni problemi

1. **401 "missing-api-key" greška**: Provjerite da ste postavili `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` prije stvaranja instance DefaultApi.
2. **Pogrešna klasa API-ja**: Koristite `DefaultApi` za serverske autentificirane zahtjeve, `PublicApi` za klijentske/javne zahtjeve.
3. **Greške pri importu**: Provjerite da importirate iz ispravnog modula:
   - API client: `from client import ...`
   - SSO utilities: `from sso import ...`