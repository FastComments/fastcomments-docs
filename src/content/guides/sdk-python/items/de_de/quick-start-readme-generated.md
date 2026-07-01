### Verwendung authentifizierter APIs (DefaultApi)

**Wichtig:** Sie müssen Ihren API‑Schlüssel in der Configuration setzen, bevor Sie authentifizierte Anfragen stellen. Wenn Sie dies nicht tun, schlagen die Anfragen mit einem 401‑Fehler fehl.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Erstelle und konfiguriere den API-Client
config = Configuration()
config.host = "https://fastcomments.com/api"

# ERFORDERLICH: Setzen Sie Ihren API-Schlüssel (erhalten Sie diesen in Ihrem FastComments-Dashboard)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Erstelle die API-Instanz mit dem konfigurierten Client
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Jetzt können Sie authentifizierte API-Aufrufe tätigen
try:
    # Beispiel: Einen SSO-Benutzer hinzufügen
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user("YOUR_TENANT_ID", user_data)
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
    response = public_api.get_comments_public("YOUR_TENANT_ID", "page-url-id")
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Verwendung des Moderations‑Dashboards (ModerationApi)

Die `ModerationApi` betreibt das Moderator‑Dashboard. Methoden werden im Namen eines Moderators aufgerufen, indem ein `sso`‑Token übergeben wird:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Zähle die Kommentare, die auf Moderation warten
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Verwendung von SSO (Single Sign-On)

Das SDK enthält Dienstprogramme zum Erzeugen sicherer SSO‑Tokens:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Benutzer-Daten erstellen
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Erstelle SSO-Instanz mit Ihrem API-Geheimnis
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Generiere das SSO-Token
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

1. **401 "missing-api-key"-Fehler**: Stellen Sie sicher, dass Sie `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` setzen, bevor Sie die DefaultApi-Instanz erstellen.
2. **Falsche API-Klasse**: Verwenden Sie `DefaultApi` für serverseitige authentifizierte Anfragen, `PublicApi` für clientseitige/öffentliche Anfragen und `ModerationApi` für Anfragen des Moderator-Dashboards.
3. **Importfehler**: Stellen Sie sicher, dass Sie aus dem richtigen Modul importieren:
   - API-Client: `from client import ...`
   - SSO-Dienstprogramme: `from sso import ...`