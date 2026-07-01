### Brug af autentificerede API'er (DefaultApi)

**Vigtigt:** Du skal indstille din API-nøgle i Configuration, før du foretager autentificerede anmodninger. Hvis du ikke gør det, vil anmodninger fejle med en 401-fejl.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Opret og konfigurer API-klienten
config = Configuration()
config.host = "https://fastcomments.com/api"

# KRAV: Indstil din API-nøgle (hent den fra dit FastComments-dashboard)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Opret API‑instansen med den konfigurerede klient
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Nu kan du foretage autentificerede API‑kald
try:
    # Eksempel: Tilføj en SSO‑bruger
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user("YOUR_TENANT_ID", user_data)
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # Almindelige fejl:
    # - 401: API-nøgle mangler eller er ugyldig
    # - 400: Anmodningsvalidering mislykkedes
```

### Brug af offentlige API'er (PublicApi)

Offentlige endepunkter kræver ingen godkendelse:

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

### Brug af moderationsdashboardet (ModerationApi)

`ModerationApi` driver moderatordashboardet. Metoder kaldes på vegne af en moderator ved at videregive et `sso`‑token:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Tæl kommentarerne, der afventer moderering
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Brug af SSO (Single Sign-On)

SDK'en indeholder værktøjer til at generere sikre SSO‑tokens:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Opret brugerdata
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Opret SSO‑instans med din API‑hemmelighed
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Generér SSO‑tokenet
sso_token = sso.create_token()

# Brug dette token i din frontend eller videregiv til API‑kald
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

1. **401 "missing-api-key"-fejl**: Sørg for at du indstiller `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` før du opretter DefaultApi‑instansen.
2. **Forkert API-klasse**: Brug `DefaultApi` til server-side autentificerede anmodninger, `PublicApi` til klient-side/offentlige anmodninger, og `ModerationApi` til anmodninger fra moderatordashboardet.
3. **Importfejl**: Sørg for at du importerer fra det korrekte modul:
   - API-klient: `from client import ...`
   - SSO-værktøjer: `from sso import ...`