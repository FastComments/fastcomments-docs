### Geverifieerde API's gebruiken (DefaultApi)

**Belangrijk:** U moet uw API-sleutel instellen in de Configuration voordat u geverifieerde verzoeken doet. Als u dat niet doet, zullen verzoeken mislukken met een 401-fout.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Maak en configureer de API-client
config = Configuration()
config.host = "https://fastcomments.com/api"

# VEREIST: Stel uw API-sleutel in (verkrijg deze van uw FastComments-dashboard)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Maak een API-instantie met de geconfigureerde client
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Nu kunt u geverifieerde API-aanroepen doen
try:
    # Voorbeeld: Voeg een SSO-gebruiker toe
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
    # Veelvoorkomende fouten:
    # - 401: API-sleutel ontbreekt of is ongeldig
    # - 400: Validatie van verzoek is mislukt
```

### Publieke API's gebruiken (PublicApi)

Openbare endpoints vereisen geen authenticatie:

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

### Het moderatie-dashboard gebruiken (ModerationApi)

De `ModerationApi` verzorgt het moderatiedashboard. Methoden worden namens een moderator aangeroepen door een `sso`-token mee te geven:

```python
from client import ApiClient, Configuration, ModerationApi

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Tel de reacties die wachten op moderatie
    response = moderation_api.get_count(sso="SSO_TOKEN")
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### SSO gebruiken (Single Sign-On)

De SDK bevat hulpprogramma's voor het genereren van veilige SSO-tokens:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Maak gebruikersgegevens aan
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Maak een SSO-instantie met uw API-secret
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Genereer het SSO-token
sso_token = sso.create_token()

# Gebruik dit token in uw frontend of geef het door aan API-aanroepen
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

1. **401 "missing-api-key" fout**: Zorg ervoor dat u `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` instelt voordat u de DefaultApi-instantie maakt.
2. **Verkeerde API-klasse**: Gebruik `DefaultApi` voor server-side geauthenticeerde verzoeken, `PublicApi` voor client-side/publieke verzoeken, en `ModerationApi` voor verzoeken van het moderatiedashboard.
3. **Importfouten**: Zorg ervoor dat u importeert uit de juiste module:
   - API client: `from client import ...`
   - SSO utilities: `from sso import ...`