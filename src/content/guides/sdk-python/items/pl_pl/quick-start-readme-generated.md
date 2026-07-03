### Korzystanie z uwierzytelnionych API (DefaultApi)

**Ważne:** Musisz ustawić swój klucz API w Configuration przed wykonywaniem uwierzytelnionych żądań. Jeśli tego nie zrobisz, żądania zakończą się błędem 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Utwórz i skonfiguruj klienta API
config = Configuration()
config.host = "https://fastcomments.com"

# WYMAGANE: Ustaw swój klucz API (pobierz go z panelu FastComments)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

# Utwórz instancję API przy użyciu skonfigurowanego klienta
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
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
public_api = PublicApi(api_client)

try:
    response = public_api.get_comments_public("YOUR_TENANT_ID", "page-url-id")
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Korzystanie z panelu moderacji (ModerationApi)

`ModerationApi` obsługuje panel moderatora. Metody są wywoływane w imieniu moderatora poprzez przekazanie tokenu `sso`:

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

### Korzystanie z SSO (Single Sign-On)

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

# Podpisz go używając swojego sekretu API (HMAC-SHA256)
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# Wygeneruj token SSO, aby przekazać go do widgetu lub wywołania API
sso_token = sso.create_token()

# Użyj tego tokenu w frontendzie lub przekaż do wywołań API
print(f"SSO Token: {sso_token}")
```

For simple SSO (less secure, for testing):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    username="johndoe",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Typowe problemy

1. **401 "missing-api-key" error**: Upewnij się, że ustawiłeś `config.api_key = {"api_key": "YOUR_KEY"}` przed utworzeniem instancji DefaultApi.
2. **Wrong API class**: Użyj `DefaultApi` do żądań uwierzytelnionych po stronie serwera, `PublicApi` do żądań po stronie klienta/publicznych oraz `ModerationApi` do żądań panelu moderatora.
3. **Import errors**: Upewnij się, że importujesz z właściwego modułu:
   - Klient API: `from client import ...`
   - Narzędzia SSO: `from sso import ...`