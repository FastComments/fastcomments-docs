### Korzystanie z uwierzytelnionych API (DefaultApi)

**Ważne:** Musisz ustawić swój klucz API w konfiguracji przed wykonywaniem uwierzytelnionych żądań. Jeśli tego nie zrobisz, żądania zakończą się błędem 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Utwórz i skonfiguruj klienta API
config = Configuration()
config.host = "https://fastcomments.com/api"

# WYMAGANE: Ustaw swój klucz API (pobierz go z panelu FastComments)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

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
    # - 400: Niepowodzenie walidacji żądania
```

### Korzystanie z publicznych API (PublicApi)

Publiczne endpointy nie wymagają uwierzytelnienia:

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

### Korzystanie z panelu moderacji (ModerationApi)

API `ModerationApi` napędza panel moderatora. Metody są wywoływane w imieniu moderatora poprzez przekazanie tokenu `sso`:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Zlicz komentarze czekające na moderację
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Korzystanie z SSO (Single Sign-On)

SDK zawiera narzędzia do generowania bezpiecznych tokenów SSO:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Utwórz dane użytkownika
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Utwórz instancję SSO z Twoim sekretem API
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Wygeneruj token SSO
sso_token = sso.create_token()

# Użyj tego tokenu w frontendzie lub przekaż go w wywołaniach API
print(f"SSO Token: {sso_token}")
```

Dla prostego SSO (mniej bezpieczne, do testów):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    user_id="user-123",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Typowe problemy

1. **Błąd 401 "missing-api-key"**: Upewnij się, że ustawiłeś `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` przed utworzeniem instancji DefaultApi.
2. **Nieprawidłowa klasa API**: Używaj `DefaultApi` do serwerowych uwierzytelnionych żądań, `PublicApi` do żądań po stronie klienta/publicznych oraz `ModerationApi` do żądań z panelu moderatora.
3. **Błędy importu**: Upewnij się, że importujesz z właściwego modułu:
   - Klient API: `from client import ...`
   - Narzędzia SSO: `from sso import ...`