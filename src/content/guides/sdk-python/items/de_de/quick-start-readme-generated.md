### Verwendung authentifizierter APIs (DefaultApi)

**Wichtig:** Du musst deinen API-Schlüssel in der Configuration setzen, bevor du authentifizierte Anfragen durchführst. Wenn du das nicht tust, schlagen Anfragen mit einem 401-Fehler fehl.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Erstelle und konfiguriere den API-Client
config = Configuration()
config.host = "https://fastcomments.com/api"

# ERFORDERLICH: Setze deinen API-Schlüssel (diesen bekommst du in deinem FastComments-Dashboard)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Erstelle die API-Instanz mit dem konfigurierten Client
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Jetzt kannst du authentifizierte API-Aufrufe machen
try:
    # Beispiel: Einen SSO-Benutzer hinzufügen
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
    # Häufige Fehler:
    # - 401: API-Schlüssel fehlt oder ist ungültig
    # - 400: Anfragevalidierung fehlgeschlagen
```

### Verwendung öffentlicher APIs (PublicApi)

Öffentliche Endpunkte erfordern keine Authentifizierung:

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

### Verwendung von SSO (Single Sign-On)

Das SDK enthält Hilfsfunktionen zum Erzeugen sicherer SSO-Tokens:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Erstelle Benutzerdaten
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Erstelle eine SSO-Instanz mit deinem API-Geheimnis
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Generiere das SSO-Token
sso_token = sso.create_token()

# Verwende dieses Token im Frontend oder übergib es an API-Aufrufe
print(f"SSO Token: {sso_token}")
```

Für einfaches SSO (weniger sicher, zum Testen):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    user_id="user-123",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Häufige Probleme

1. **401 "missing-api-key" Fehler**: Stelle sicher, dass du `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` setzt, bevor du die DefaultApi-Instanz erstellst.
2. **Falsche API-Klasse**: Verwende `DefaultApi` für serverseitige authentifizierte Anfragen, `PublicApi` für clientseitige/öffentliche Anfragen.
3. **Importfehler**: Stelle sicher, dass du aus dem richtigen Modul importierst:
   - API client: `from client import ...`
   - SSO utilities: `from sso import ...`