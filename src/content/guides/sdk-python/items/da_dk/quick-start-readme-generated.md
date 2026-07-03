### Using Authenticated APIs (DefaultApi)

**Vigtigt:** Du skal indstille din API‑nøgle på Configuration, før du foretager autentificerede anmodninger. Hvis du ikke gør det, vil anmodninger fejle med en 401‑fejl.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Opret og konfigurer API-klienten
config = Configuration()
config.host = "https://fastcomments.com"

# KRAV: Angiv din API-nøgle (hent denne fra dit FastComments-dashboard)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

# Opret API‑instansen med den konfigurerede klient
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Nu kan du foretage autentificerede API‑opkald
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

### Using Public APIs (PublicApi)

Offentlige endpoints kræver ikke godkendelse:

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

### Using the Moderation Dashboard (ModerationApi)

`ModerationApi` driver moderator‑dashboardet. Metoder kaldes på vegne af en moderator ved at videregive en `sso`‑token:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Tæl de kommentarer, der venter på moderation
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Using SSO (Single Sign-On)

SDK’en indeholder værktøjer til at generere sikre SSO‑tokens:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Opret brugerdata (id, e‑mail og brugernavn er påkrævet)
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Signer den med din API-hemmelighed (HMAC‑SHA256)
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# Generér SSO‑tokenet til at videregive til widget’en eller et API‑opkald
sso_token = sso.create_token()

# Brug dette token i din frontend eller videregiv det til API‑opkald
print(f"SSO Token: {sso_token}")
```

For simpel SSO (mindre sikker, til test):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    username="johndoe",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Common Issues

1. **401 "missing-api-key" fejl**: Sørg for, at du angiver `config.api_key = {"api_key": "YOUR_KEY"}` inden du opretter DefaultApi‑instansen.
2. **Forkert API-klasse**: Brug `DefaultApi` til server‑side autentificerede anmodninger, `PublicApi` til klient‑side/offentlige anmodninger, og `ModerationApi` til anmodninger fra moderator‑dashboardet.
3. **Import-fejl**: Sørg for, at du importerer fra den korrekte modul:
   - API-klient: `from client import ...`
   - SSO-værktøjer: `from sso import ...`