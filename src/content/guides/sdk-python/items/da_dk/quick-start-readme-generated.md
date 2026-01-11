### Brug af autentificerede API'er (DefaultApi)

**Vigtigt:** Du skal sætte din API-nøgle på Configuration, før du foretager autentificerede anmodninger. Hvis du ikke gør det, vil anmodninger fejle med en 401-fejl.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Opret og konfigurer API-klienten
config = Configuration()
config.host = "https://fastcomments.com/api"

# PÅKRÆVET: Angiv din API-nøgle (hent den fra dit FastComments-dashboard)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Opret API-instansen med den konfigurerede klient
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Nu kan du lave autentificerede API-opkald
try:
    # Eksempel: Tilføj en SSO-bruger
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
    # Almindelige fejl:
    # - 401: API-nøgle mangler eller er ugyldig
    # - 400: Anmodningsvalidering mislykkedes
```

### Brug af offentlige API'er (PublicApi)

Offentlige endpoints kræver ikke autentificering:

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

### Brug af SSO (Single Sign-On)

SDK'en inkluderer værktøjer til at generere sikre SSO-tokens:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Opret brugerdata
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Opret SSO-instans med din API-secret
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Generer SSO-tokenet
sso_token = sso.create_token()

# Brug dette token i din frontend eller send det med API-opkald
print(f"SSO Token: {sso_token}")
```

For simpel SSO (mindre sikker, til test):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    user_id="user-123",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Almindelige problemer

1. **401 "missing-api-key" error**: Sørg for, at du indstiller `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` før du opretter DefaultApi-instanten.
2. **Wrong API class**: Brug `DefaultApi` til server-side autentificerede forespørgsler, `PublicApi` til klient-side/offentlige forespørgsler.
3. **Importfejl**: Sørg for, at du importerer fra det korrekte modul:
   - API-klient: `from client import ...`
   - SSO-værktøjer: `from sso import ...`