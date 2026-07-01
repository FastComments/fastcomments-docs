### Uporaba avtenticiranih API-jev (DefaultApi)

**Pomembno:** Pred izvajanjem avtenticiranih zahtev morate nastaviti svoj API ključ v konfiguraciji. Če tega ne storite, bodo zahteve spodletelo z napako 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Ustvari in konfiguriraj API odjemalca
config = Configuration()
config.host = "https://fastcomments.com/api"

# OBVEZNO: Nastavite svoj API ključ (ga pridobite v nadzorni plošči FastComments)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Ustvari API primerek s konfiguriranim odjemalcem
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Zdaj lahko izvajate avtenticirane API klice
try:
    # Primer: Dodaj SSO uporabnika
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user("YOUR_TENANT_ID", user_data)
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # Pogoste napake:
    # - 401: API ključ manjka ali je neveljaven
    # - 400: Preverjanje zahteve je spodletelo
```

### Uporaba javnih API-jev (PublicApi)

Javni končni naslovi ne zahtevajo avtentikacije:

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

### Uporaba nadzorne plošče za moderiranje (ModerationApi)

`ModerationApi` poganja nadzorno ploščo moderatorjev. Metode se kličejo v imenu moderatorja z podajanjem `sso` žetona:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Šteje komentarje, ki čakajo na moderiranje
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Uporaba SSO (Single Sign-On)

SDK vključuje pripomočke za ustvarjanje varnih SSO žetonov:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Ustvari podatke uporabnika
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Ustvari SSO primerek z vašim API skrivnim ključem
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Ustvari SSO žeton
sso_token = sso.create_token()

# Uporabite ta žeton v vašem frontendu ali ga posredujte API zahtevam
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

1. **401 "missing-api-key" napaka**: Prepričajte se, da ste nastavili `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` pred ustvarjanjem `DefaultApi` primerka.
2. **Napačen API razred**: Uporabite `DefaultApi` za strežniške avtenticirane zahteve, `PublicApi` za odjemalske/javne zahteve in `ModerationApi` za zahteve nadzorne plošče moderatorjev.
3. **Napake pri uvozu**: Prepričajte se, da uvažate iz pravilnega modula:
   - API odjemalec: `from client import ...`
   - SSO pripomočki: `from sso import ...`