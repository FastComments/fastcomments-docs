### Using Authenticated APIs (DefaultApi)

**Ważne:** Musisz ustawić swój klucz API w konfiguracji przed wykonywaniem uwierzytelnionych żądań. Jeśli tego nie zrobisz, żądania zakończą się błędem 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Utwórz i skonfiguruj klienta API
config = Configuration()
config.host = "https://fastcomments.com"

# WYMAGANE: Ustaw swój klucz API (pobierz go z panelu FastComments)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

# Utwórz instancję API z skonfigurowanym klientem
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Teraz możesz wykonywać uwierzytelnione wywołania API
try:
    # Przykład: Dodaj użytkownika SSO
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user("YOUR_TENANT_ID", user_data)
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # Typowe błędy:
    # - 401: Brak klucza API lub jest nieprawidłowy
    # - 400: Walidacja żądania nie powiodła się
```

### Using Public APIs (PublicApi)

Publiczne endpointy nie wymagają uwierzytelnienia:

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

`ModerationApi` napędza panel moderatora. Metody są wywoływane w imieniu moderatora poprzez przekazanie tokenu `sso`:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Policz komentarze oczekujące na moderację
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Using SSO (Single Sign-On)

SDK zawiera narzędzia do generowania bezpiecznych tokenów SSO:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Utwórz dane użytkownika (id, email i nazwa użytkownika są wymagane)
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Podpisz go swoim sekretem API (HMAC-SHA256)
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# Wygeneruj token SSO, aby przekazać go do widgetu lub wywołania API
sso_token = sso.create_token()

# Użyj tego tokenu w frontendzie lub przekaż go w wywołaniach API
print(f"SSO Token: {sso_token}")
```

Dla prostego SSO (mniej bezpiecznego, do testów):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    username="johndoe",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Live Subscriptions (PubSub)

Moduł `pubsub` pozwala subskrybować zdarzenia komentarzy w czasie rzeczywistym (nowe komentarze, głosy, edycje, powiadomienia itp.) przez WebSocket, odzwierciedlając `LiveEventSubscriber` z FastComments Java SDK. Wymaga dodatkowego pakietu `pubsub`, który dodaje klienta WebSocket na szczycie wygenerowanego klienta API:

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
    user_id_ws="a-unique-presence-id",  # np. UUID dla tej sesji
    handle_live_event=handle_live_event,
    on_connection_status_change=lambda connected, last_event_time: print(
        f"connected={connected}"
    ),
    region=None,  # ustaw na "eu" dla regionu UE
)

# ...później, gdy nie chcesz już otrzymywać aktualizacji:
result.close()
```

Subskrybent uruchamia połączenie w tle w wątku demona, automatycznie ponownie łączy się z jitterem i pobiera wszelkie zdarzenia, które zostały pominięte podczas rozłączenia z endpointem event-log przy ponownym połączeniu. Przekaż opcjonalny callback `can_see_comments` (`List[str] -> Dict[str, str]`, zwracający identyfikatory, które użytkownik **NIE** może zobaczyć), aby odfiltrować zdarzenia dla komentarzy, które użytkownik nie ma zakaz wyświetlania. Ustaw `disable_live_commenting=True`, aby `subscribe_to_changes` stało się operacją pustą zwracającą `None`.

### Common Issues

1. **401 "missing-api-key" error**: Upewnij się, że ustawiłeś `config.api_key = {"api_key": "YOUR_KEY"}` przed utworzeniem instancji DefaultApi.
2. **Wrong API class**: Użyj `DefaultApi` do uwierzytelnionych żądań po stronie serwera, `PublicApi` do żądań po stronie klienta/publicznych oraz `ModerationApi` do żądań panelu moderatora.
3. **Import errors**: Upewnij się, że importujesz z właściwego modułu:
   - Klient API: `from client import ...`
   - Narzędzia SSO: `from sso import ...`
   - Subskrypcje na żywo: `from pubsub import ...` (wymaga dodatkowego pakietu `pubsub`)