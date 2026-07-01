### Gebruik geauthentiseerde API's (DefaultApi)

**Belangrijk:** U moet uw API‑sleutel instellen in de Configuration voordat u geauthentiseerde verzoeken maakt. Als u dat niet doet, zullen verzoeken falen met een 401‑fout.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Maak en configureer de API-client
config = Configuration()
config.host = "https://fastcomments.com/api"

# VERPLICHT: Stel uw API-sleutel in (haal deze op vanuit uw FastComments-dashboard)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Maak de API-instantie met de geconfigureerde client
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Nu kunt u geauthentiseerde API‑aanroepen doen
try:
    # Voorbeeld: Voeg een SSO-gebruiker toe
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user("YOUR_TENANT_ID", user_data)
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # Veelvoorkomende fouten:
    # - 401: API-sleutel ontbreekt of is ongeldig
    # - 400: Validatie van verzoek mislukt
```

### Gebruik openbare API's (PublicApi)

Openbare eindpunten vereisen geen authenticatie:

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

### Gebruik het moderatiedashboard (ModerationApi)

`ModerationApi` voedt het moderatiedashboard. Methodes worden uitgevoerd namens een moderator door een `sso`‑token door te geven:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Tel de reacties die op moderatie wachten
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Gebruik SSO (Single Sign-On)

De SDK bevat hulpprogramma's voor het genereren van veilige SSO‑tokens:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Maak gebruikersgegevens
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Maak een SSO-instantie met uw API-geheim
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Genereer de SSO-token
sso_token = sso.create_token()

# Gebruik dit token in uw frontend of geef het door aan API‑aanroepen
print(f"SSO Token: {sso_token}")
```

Voor eenvoudige SSO (minder veilig, voor testen):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    user_id="user-123",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Veelvoorkomende problemen

1. **401 "missing-api-key" fout**: Zorg ervoor dat u `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` instelt voordat u de DefaultApi‑instantie maakt.
2. **Verkeerde API-klasse**: Gebruik `DefaultApi` voor server‑side geauthentiseerde verzoeken, `PublicApi` voor client‑side/openbare verzoeken, en `ModerationApi` voor verzoeken vanuit het moderatiedashboard.
3. **Importfouten**: Zorg ervoor dat u importeert vanuit de juiste module:
   - API-client: `from client import ...`
   - SSO‑hulpmiddelen: `from sso import ...`