---
### Korištenje autentificiranih API-ja (DefaultApi)

**Važno:** Morate postaviti svoj API ključ u Configuration prije nego što napravite autentificirane zahtjeve. Ako to ne učinite, zahtjevi će propasti s greškom 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Stvorite i konfigurirajte API klijent
config = Configuration()
config.host = "https://fastcomments.com"

# OBAVEZNO: Postavite svoj API ključ (preuzmite ga iz FastComments nadzorne ploče)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

# Stvorite instancu API-ja s konfiguriranim klijentom
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
    # Česte greške:
    # - 401: API ključ nedostaje ili je nevažeći
    # - 400: Validacija zahtjeva nije uspjela
```

### Korištenje javnih API-ja (PublicApi)

Javni krajnji točke ne zahtijevaju autentifikaciju:

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

### Korištenje nadzorne ploče za moderiranje (ModerationApi)

`ModerationApi` pokreće moderatorsku nadzornu ploču. Metode se pozivaju u ime moderatora prosljeđivanjem `sso` tokena:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Izbrojite komentare koji čekaju moderaciju
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Korištenje SSO (Single Sign-On)

SDK uključuje alate za generiranje sigurnih SSO tokena:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Stvorite podatke korisnika (id, email i korisničko ime su obavezni)
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Potpišite ga svojim API tajnim ključeom (HMAC-SHA256)
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# Generirajte SSO token koji ćete proslijediti widgetu ili API pozivu
sso_token = sso.create_token()

# Koristite ovaj token u svom frontendu ili ga proslijedite API pozivima
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

### Uobičajeni problemi

1. **401 "missing-api-key" greška**: Provjerite jeste li postavili `config.api_key = {"api_key": "YOUR_KEY"}` prije stvaranja instance DefaultApi.
2. **Pogrešna API klasa**: Koristite `DefaultApi` za server‑side autentificirane zahtjeve, `PublicApi` za klijentske/javne zahtjeve i `ModerationApi` za zahtjeve moderacijske nadzorne ploče.
3. **Pogreške pri uvozu**: Provjerite uvozite iz ispravnog modula:
   - API klijent: `from client import ...`
   - SSO alati: `from sso import ...`
---