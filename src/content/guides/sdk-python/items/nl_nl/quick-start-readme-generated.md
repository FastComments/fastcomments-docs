### Gebruik geauthenticeerde API's (DefaultApi)

**Belangrijk:** Je moet je API‑sleutel op de Configuration instellen voordat je geauthenticeerde verzoeken doet. Als je dat niet doet, zullen verzoeken falen met een 401‑fout.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Maak en configureer de API-client
config = Configuration()
config.host = "https://fastcomments.com"

# VERPLICHT: Stel je API-sleutel in (haal deze op vanuit je FastComments-dashboard)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

# Maak de API-instantie met de geconfigureerde client
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Nu kun je geauthenticeerde API-aanroepen doen
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
    # - 400: Verzoekvalidatie mislukt
```

### Gebruik openbare API's (PublicApi)

Openbare endpoints vereisen geen authenticatie:

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

### Gebruik het moderatiedashboard (ModerationApi)

De `ModerationApi` levert de functionaliteit voor het moderator‑dashboard. Methodes worden aangeroepen namens een moderator door een `sso`‑token mee te geven:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Tel de reacties die wacht op moderatie
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Gebruik SSO (Single Sign-On)

De SDK bevat hulpprogramma's voor het genereren van veilige SSO‑tokens:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Maak gebruikersgegevens (id, e‑mail en gebruikersnaam zijn vereist)
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Onderteken met je API-geheim (HMAC-SHA256)
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# Genereer de SSO-token om door te geven aan de widget of een API‑aanroep
sso_token = sso.create_token()

# Gebruik deze token in je frontend of geef door aan API‑aanroepen
print(f"SSO Token: {sso_token}")
```

Voor eenvoudig SSO (minder veilig, voor testen):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    username="johndoe",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Veelvoorkomende problemen

1. **401 "missing-api-key" fout**: Zorg ervoor dat je `config.api_key = {\"api_key\": \"YOUR_KEY\"}` instelt voordat je de DefaultApi‑instantie maakt.
2. **Verkeerde API-klasse**: Gebruik `DefaultApi` voor server‑side geauthenticeerde verzoeken, `PublicApi` voor client‑side/openbare verzoeken, en `ModerationApi` voor verzoeken van het moderator‑dashboard.
3. **Importfouten**: Zorg ervoor dat je importeert vanuit de juiste module:
   - API‑client: `from client import ...`
   - SSO‑hulpmiddelen: `from sso import ...`