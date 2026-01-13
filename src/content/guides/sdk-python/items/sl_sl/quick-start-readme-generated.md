### Uporaba avtenticiranih API-jev (DefaultApi)

**Pomembno:** Pred izvajanjem avtenticiranih zahtev morate na Configuration nastaviti svoj API ključ. Če tega ne storite, bodo zahteve vrnile napako 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Ustvari in konfiguriraj API odjemalca
config = Configuration()
config.host = "https://fastcomments.com/api"

# OBVEZNO: Nastavite svoj API ključ (pridobite ga na nadzorni plošči FastComments)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Ustvari instanco API-ja s konfiguriranim odjemalcem
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Zdaj lahko izvajate avtenticirane klice API-ja
try:
    # Primer: Dodaj SSO uporabnika
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
    # Pogoste napake:
    # - 401: API ključ manjka ali ni veljaven
    # - 400: Preverjanje zahteve ni uspelo
```

### Uporaba javnih API-jev (PublicApi)

Javni endpointi ne zahtevajo avtentikacije:

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

### Uporaba SSO (Single Sign-On)

SDK vključuje pripomočke za generiranje varnih SSO žetonov:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Ustvari podatke o uporabniku
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Ustvari SSO instanco z vašim API skrivnim ključem
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Generiraj SSO žeton
sso_token = sso.create_token()

# Uporabite ta žeton v frontend-u ali ga posredujte klicem API-ja
print(f"SSO Token: {sso_token}")
```

Za preprost SSO (manj varen, za testiranje):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

# Ustvari podatke o uporabniku
user_data = SimpleSSOUserData(
    user_id="user-123",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Pogoste težave

1. **401 napaka "missing-api-key"**: Poskrbite, da nastavite `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` pred ustvarjanjem instance DefaultApi.
2. **Napačen razred API-ja**: Uporabite `DefaultApi` za strežniške avtenticirane zahteve, `PublicApi` za odjemalske/javne zahteve.
3. **Napake pri uvozu**: Prepričajte se, da uvažate iz pravilnega modula:
   - API client: `from client import ...`
   - SSO utilities: `from sso import ...`