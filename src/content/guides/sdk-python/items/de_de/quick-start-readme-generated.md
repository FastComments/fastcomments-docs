### Verwendung authentifizierter APIs (DefaultApi)

**Wichtig:** Du musst deinen API‑Schlüssel in der Configuration setzen, bevor du authentifizierte Anfragen machst. Wenn du das nicht tust, schlagen die Anfragen mit einem 401‑Fehler fehl.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Erstelle und konfiguriere den API-Client
config = Configuration()
config.host = "https://fastcomments.com"

# ERFORDERLICH: Setze deinen API-Schlüssel (erhalte ihn im FastComments-Dashboard)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

# Erstelle die API-Instanz mit dem konfigurierten Client
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Jetzt kannst du authentifizierte API-Aufrufe tätigen
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

Öffentliche Endpunkte benötigen keine Authentifizierung:

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

### Verwendung des Moderations‑Dashboards (ModerationApi)

Die `ModerationApi` betreibt das Moderator‑Dashboard. Methoden werden im Namen eines Moderators aufgerufen, indem ein `sso`‑Token übergeben wird:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

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

Das SDK enthält Hilfsprogramme zum Erzeugen sicherer SSO‑Token:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Erstelle Benutzerdaten (id, E‑Mail und Benutzername sind erforderlich)
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Signiere es mit deinem API-Geheimnis (HMAC‑SHA256)
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# Generiere das SSO‑Token, um es an das Widget oder einen API‑Aufruf zu übergeben
sso_token = sso.create_token()

# Verwende dieses Token in deinem Frontend oder übergebe es an API‑Aufrufe
print(f"SSO Token: {sso_token}")
```

Für einfaches SSO (weniger sicher, zum Testen):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    username="johndoe",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Live‑Abonnements (PubSub)

Das `pubsub`‑Modul ermöglicht das Abonnieren von Echtzeit‑Kommentarereignissen (neue Kommentare, Stimmen, Bearbeitungen, Benachrichtigungen usw.) über einen WebSocket und spiegelt den `LiveEventSubscriber` des FastComments Java‑SDKs wider. Es erfordert das `pubsub`‑Extra, das einen WebSocket‑Client zusätzlich zum generierten API‑Client hinzufügt:

```bash
pip install "fastcomments[pubsub] @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0"
```

```python
from pubsub import LiveEventSubscriber

subscriber = LiveEventSubscriber()

def handle_live_event(event):
    print(f"Live event: {event.type}")
    if event.comment:
        print(f"  comment: {event.comment.comment}")

result = subscriber.subscribe_to_changes(
    tenant_id_ws="YOUR_TENANT_ID",
    url_id="page-url-id",
    url_id_ws="page-url-id",
    user_id_ws="a-unique-presence-id",  # z.B. eine UUID für diese Sitzung
    handle_live_event=handle_live_event,
    on_connection_status_change=lambda connected, last_event_time: print(
        f"connected={connected}"
    ),
    region=None,  # setze auf "eu" für die EU-Region
)

# ...später, wenn du keine Updates mehr möchtest:
result.close()
```

Der Subscriber führt die Verbindung in einem Hintergrund‑Daemon‑Thread aus, stellt transparent mit Jitter wieder her und holt alle Ereignisse nach, die während einer Trennung vom Event‑Log‑Endpunkt verpasst wurden. Übergebe einen optionalen `can_see_comments`‑Callback (`List[str] -> Dict[str, str]`, der die IDs zurückgibt, die der Benutzer **nicht** sehen darf), um Ereignisse für Kommentare zu filtern, die der Benutzer nicht sehen darf. Setze `disable_live_commenting=True`, um `subscribe_to_changes` zu einem No‑Op zu machen, das `None` zurückgibt.

### Häufige Probleme

1. **401 „missing-api-key“ Fehler**: Stelle sicher, dass du `config.api_key = {"api_key": "YOUR_KEY"}` setzt, bevor du die DefaultApi‑Instanz erstellst.  
2. **Falsche API‑Klasse**: Verwende `DefaultApi` für serverseitige authentifizierte Anfragen, `PublicApi` für clientseitige/öffentliche Anfragen und `ModerationApi` für Anfragen des Moderator‑Dashboards.  
3. **Import‑Fehler**: Stelle sicher, dass du aus dem richtigen Modul importierst:  
   - API‑Client: `from client import ...`  
   - SSO‑Hilfsprogramme: `from sso import ...`  
   - Live‑Abonnements: `from pubsub import ...` (benötigt das `pubsub`‑Extra)