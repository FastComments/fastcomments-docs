### Korzystanie z uwierzytelnianych API (DefaultApi)

**Ważne:** Przed wykonaniem uwierzytelnianych żądań musisz ustawić swój klucz API w konfiguracji. Jeśli tego nie zrobisz, żądania zakończą się błędem 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Utwórz i skonfiguruj klienta API
config = Configuration()
config.host = "https://fastcomments.com"

# WYMAGANE: Ustaw swój klucz API (pobierz go z panelu FastComments)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

# Utwórz instancję API ze skonfigurowanym klientem
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Teraz możesz wykonywać uwierzytelniane wywołania API
try:
    # Przykład: Dodanie użytkownika SSO
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user("YOUR_TENANT_ID", user_data)
    print(f"Użytkownik utworzony: {response}")

except Exception as e:
    print(f"Błąd: {e}")
    # Typowe błędy:
    # - 401: brakujący lub nieprawidłowy klucz API
    # - 400: walidacja żądania nie powiodła się
```

### Korzystanie z publicznych API (PublicApi)

Publiczne punkty końcowe nie wymagają uwierzytelnienia:

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
    print(f"Błąd: {e}")
```

### Korzystanie z panelu moderacji (ModerationApi)

`ModerationApi` zasila panel moderatora. Metody są wywoływane w imieniu moderatora poprzez przekazanie tokenu `sso`:

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
    print(f"Błąd: {e}")
```

### Korzystanie z SSO (Single Sign-On)

SDK zawiera narzędzia do generowania bezpiecznych tokenów SSO:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Utwórz dane użytkownika (id, email i username są wymagane)
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Podpisz je swoim sekretem API (HMAC-SHA256)
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# Wygeneruj token SSO do przekazania widgetowi lub wywołaniu API
sso_token = sso.create_token()

# Użyj tego tokenu w swoim frontendzie lub przekaż go do wywołań API
print(f"Token SSO: {sso_token}")
```

Dla prostego SSO (mniej bezpieczne, do testów):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    username="johndoe",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

Subskrypcje na żywo (PubSub)

Moduł `pubsub` pozwala subskrybować zdarzenia komentarzy w czasie rzeczywistym (nowe komentarze, głosy, edycje, powiadomienia itp.) przez WebSocket, odwzorowując `LiveEventSubscriber` z FastComments Java SDK. Wymaga dodatku `pubsub`, który dodaje klienta WebSocket do wygenerowanego klienta API:

```bash
pip install "fastcomments[pubsub] @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0"
```

```python
from pubsub import LiveEventSubscriber

subscriber = LiveEventSubscriber()

def handle_live_event(event):
    print(f"Zdarzenie na żywo: {event.type}")
    if event.comment:
        print(f"  komentarz: {event.comment.comment}")

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

Subskrybent uruchamia połączenie w tle na wątku demona, automatycznie ponawia połączenie z losowym opóźnieniem (jitter) i po ponownym połączeniu pobiera z punktu końcowego event-log wszelkie zdarzenia, które zostały pominięte podczas rozłączenia. Przekaż opcjonalne wywołanie zwrotne `can_see_comments` (`List[str] -> Dict[str, str]`, zwracające identyfikatory, których użytkownik NIE może zobaczyć), aby odfiltrować zdarzenia dotyczące komentarzy, których użytkownik nie ma prawa oglądać. Ustaw `disable_live_commenting=True`, aby `subscribe_to_changes` stało się operacją pustą (no-op) zwracającą `None`.

### Częste problemy

1. **Błąd 401 "missing-api-key"**: Upewnij się, że ustawiłeś `config.api_key = {"api_key": "YOUR_KEY"}` przed utworzeniem instancji DefaultApi.
2. **Nieprawidłowa klasa API**: Używaj `DefaultApi` dla uwierzytelnianych żądań po stronie serwera, `PublicApi` dla żądań publicznych/po stronie klienta, a `ModerationApi` dla żądań panelu moderatora.
3. **Błędy importu**: Upewnij się, że importujesz z właściwego modułu:
   - Klient API: `from client import ...`
   - Narzędzia SSO: `from sso import ...`
   - Subskrypcje na żywo: `from pubsub import ...` (wymaga dodatku `pubsub`)