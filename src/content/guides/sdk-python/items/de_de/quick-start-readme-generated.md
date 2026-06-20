### Verwendung authentifizierter APIs (DefaultApi)

**Wichtig:** Sie müssen Ihren API-Schlüssel in der Configuration setzen, bevor Sie authentifizierte Anfragen stellen. Wenn Sie das nicht tun, schlagen Anfragen mit einem 401-Fehler fehl.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Erstelle und konfiguriere den API-Client
config = Configuration()
config.host = "https://fastcomments.com/api"

# ERFORDERLICH: Setzen Sie Ihren API-Schlüssel (diesen finden Sie im FastComments-Dashboard)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Erstelle die API-Instanz mit dem konfigurierten Client
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Nun können Sie authentifizierte API-Aufrufe durchführen
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
    # - 400: Anforderungsvalidierung fehlgeschlagen
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

### Verwendung des Moderations-Dashboards (ModerationApi)

Die `ModerationApi` betreibt das Moderator-Dashboard. Methoden werden im Namen eines Moderators durch Übergeben eines `sso`-Tokens aufgerufen:

```python
from client import ApiClient, Configuration, ModerationApi

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Zähle die zur Moderation wartenden Kommentare
    response = moderation_api.get_count(sso="SSO_TOKEN")
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

# Erstelle eine SSO-Instanz mit Ihrem API-Geheimnis
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Erzeuge das SSO-Token
sso_token = sso.create_token()

# Verwenden Sie dieses Token in Ihrem Frontend oder übergeben Sie es an API-Aufrufe
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

1. **401 "missing-api-key" Fehler**: Stellen Sie sicher, dass Sie `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` setzen, bevor Sie die DefaultApi-Instanz erstellen.
2. **Falsche API-Klasse**: Verwenden Sie `DefaultApi` für serverseitige authentifizierte Anfragen, `PublicApi` für clientseitige / öffentliche Anfragen und `ModerationApi` für Anfragen des Moderator-Dashboards.
3. **Importfehler**: Stellen Sie sicher, dass Sie aus dem richtigen Modul importieren:
   - API-Client: `from client import ...`
   - SSO-Dienstprogramme: `from sso import ...`