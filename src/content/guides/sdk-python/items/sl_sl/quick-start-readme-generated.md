### Uporaba avtoriziranih API-jev (DefaultApi)

**Pomembno:** Morate nastaviti svoj API ključ v konfiguraciji, preden pošljete avtorizirane zahteve. Če tega ne storite, bodo zahteve spodletela z napako 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Ustvari in konfiguriraj API odjemalca
config = Configuration()
config.host = "https://fastcomments.com"

# OBVEZNO: Nastavi svoj API ključ (pridobi ga v nadzorni plošči FastComments)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

# Ustvari API instanco s konfiguriranim odjemalcem
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Zdaj lahko izvajaš avtorizirane API klice
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
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
public_api = PublicApi(api_client)

try:
    response = public_api.get_comments_public("YOUR_TENANT_ID", "page-url-id")
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Uporaba nadzorne plošče za moderiranje (ModerationApi)

API `ModerationApi` poganja nadzorno ploščo moderatorja. Metode se kličejo v imenu moderatorja z dodajanjem `sso` žetona:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Preštej komentarje, ki čakajo na moderacijo
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Uporaba SSO (enojna prijava)

SDK vključuje pripomočke za ustvarjanje varnih SSO žetonov:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Ustvari podatke uporabnika (zahtevani so id, e-pošta in uporabniško ime)
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Podpiši ga s svojim API skrivnim ključem (HMAC-SHA256)
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# Ustvari SSO žeton, ki ga posreduješ v gradniku ali API klicu
sso_token = sso.create_token()

# Uporabi ta žeton v svojem vmesniku ali ga posreduj v API klicih
print(f"SSO Token: {sso_token}")
```

Za enostavni SSO (manj varen, za testiranje):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    username="johndoe",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Pogoste težave

1. **Napaka 401 "missing-api-key"**: Prepričajte se, da ste nastavili `config.api_key = {"api_key": "YOUR_KEY"}` pred ustvarjanjem instance DefaultApi.
2. **Napačen API razred**: Uporabite `DefaultApi` za strežniške avtorizirane zahteve, `PublicApi` za odjemalske/javne zahteve in `ModerationApi` za zahteve nadzorne plošče moderatorja.
3. **Napake pri uvozu**: Prepričajte se, da uvažate iz pravilnega modula:
   - API odjemalec: `from client import ...`
   - SSO pripomočki: `from sso import ...`