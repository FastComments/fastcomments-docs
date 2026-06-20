### Uporaba avtenticiranih API-jev (DefaultApi)

**Pomembno:** Pred ustvarjanjem avtenticiranih zahtev morate nastaviti svoj API ključ v Configuration. Če tega ne storite, bodo zahtevki spodleteli z napako 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Ustvarite in konfigurirajte API odjemalca
config = Configuration()
config.host = "https://fastcomments.com/api"

# OBVEZNO: Nastavite svoj API ključ (pridobite ga na nadzorni plošči FastComments)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Ustvarite instanco API z konfiguriranim odjemalcem
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
    # - 401: API ključ manjka ali je neveljaven
    # - 400: Validacija zahtevka ni uspela
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

### Uporaba nadzorne plošče moderacije (ModerationApi)

The `ModerationApi` powers the moderator dashboard. Methods are called on behalf of a moderator by passing an `sso` token:

```python
from client import ApiClient, Configuration, ModerationApi

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Preštejte komentarje, ki čakajo na moderacijo
    response = moderation_api.get_count(sso="SSO_TOKEN")
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Uporaba SSO (enotna prijava)

SDK vsebuje pripomočke za ustvarjanje varnih SSO žetonov:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Ustvarite podatke o uporabniku
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Ustvarite SSO instanco z vašim API skrivnostjo
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Generirajte SSO žeton
sso_token = sso.create_token()

# Ta žeton uporabite v frontend aplikaciji ali ga posredujte klicem API-ja
print(f"SSO Token: {sso_token}")
```

Za preprosto SSO (manj varno, za testiranje):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    user_id="user-123",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Pogoste težave

1. **401 "missing-api-key" error**: Prepričajte se, da nastavite `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` pred ustvarjanjem instance DefaultApi.
2. **Wrong API class**: Uporabite `DefaultApi` za strežniške avtenticirane zahteve, `PublicApi` za odjemalske/javne zahteve in `ModerationApi` za zahteve nadzorne plošče moderatorja.
3. **Import errors**: Prepričajte se, da uvažate iz pravilnega modula:
   - API client: `from client import ...`
   - SSO utilities: `from sso import ...`