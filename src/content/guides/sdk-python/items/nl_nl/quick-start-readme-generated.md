### Gebruik van geauthenticeerde API's (DefaultApi)

**Belangrijk:** Je moet je API-sleutel instellen op de Configuration voordat je geauthenticeerde verzoeken doet. Als je dat niet doet, zullen verzoeken falen met een 401-fout.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Maak en configureer de API-client
config = Configuration()
config.host = "https://fastcomments.com"

# VERPLICHT: Stel je API-sleutel in (haal deze op van je FastComments-dashboard)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

# Maak de API‑instantie met de geconfigureerde client
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Nu kun je geauthenticeerde API‑aanroepen doen
try:
    # Voorbeeld: Voeg een SSO‑gebruiker toe
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

### Gebruik van openbare API's (PublicApi)

Openbare eindpunten vereisen geen authenticatie:

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

### Gebruik van het moderatiedashboard (ModerationApi)

De `ModerationApi` voedt het moderatiedashboard. Methoden worden aangeroepen namens een moderator door een `sso`-token door te geven:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Tel de reacties die wachten op moderatie
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Gebruik van SSO (Single Sign-On)

De SDK bevat hulpprogramma's voor het genereren van beveiligde SSO-tokens:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Maak gebruikersgegevens (id, e‑mail en gebruikersnaam zijn vereist)
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Onderteken het met je API‑geheim (HMAC‑SHA256)
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# Genereer de SSO‑token om door te geven aan de widget of een API‑aanroep
sso_token = sso.create_token()

# Gebruik deze token in je frontend of geef door aan API‑aanroepen
print(f"SSO Token: {sso_token}")
```

Voor eenvoudige SSO (minder veilig, voor testen):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    username="johndoe",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Live-abonnementen (PubSub)

De `pubsub` module laat je abonneren op realtime commentaargebeurtenissen (nieuwe reacties, stemmen, bewerkingen, meldingen, enz.) via een WebSocket, en spiegelt de `LiveEventSubscriber` van de FastComments Java SDK. Het vereist de `pubsub` extra, die een WebSocket-client toevoegt bovenop de gegenereerde API-client:

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
    user_id_ws="a-unique-presence-id",  # bijv. een UUID voor deze sessie
    handle_live_event=handle_live_event,
    on_connection_status_change=lambda connected, last_event_time: print(
        f"connected={connected}"
    ),
    region=None,  # stel in op "eu" voor de EU-regio
)

# ...later, wanneer je geen updates meer wilt:
result.close()
```

De abonnee draait de verbinding op een achtergrond‑daemon‑thread, maakt transparant opnieuw verbinding met jitter, en haalt eventuele gebeurtenissen op die gemist zijn tijdens een onderbreking van de event‑log‑endpoint bij het opnieuw verbinden. Geef een optionele `can_see_comments`‑callback (`List[str] -> Dict[str, str]`, die de id's retourneert die de gebruiker NIET mag zien) om gebeurtenissen voor reacties die de gebruiker niet mag bekijken te filteren. Stel `disable_live_commenting=True` in om `subscribe_to_changes` een no‑op te maken die `None` retourneert.

### Veelvoorkomende problemen

1. **401 "missing-api-key" fout**: Zorg ervoor dat je `config.api_key = {"api_key": "YOUR_KEY"}` instelt voordat je de DefaultApi‑instantie maakt.
2. **Verkeerde API‑klasse**: Gebruik `DefaultApi` voor server‑side geauthenticeerde verzoeken, `PublicApi` voor client‑side/openbare verzoeken, en `ModerationApi` voor verzoeken van het moderatiedashboard.
3. **Import‑fouten**: Zorg ervoor dat je importeert uit de juiste module:
   - API‑client: `from client import ...`
   - SSO‑hulpprogramma's: `from sso import ...`
   - Live‑abonnementen: `from pubsub import ...` (vereist de `pubsub` extra)