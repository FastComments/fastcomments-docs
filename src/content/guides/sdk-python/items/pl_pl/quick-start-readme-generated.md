### Korzystanie z uwierzytelnionych API (DefaultApi)

**Ważne:** Musisz ustawić swój klucz API w Configuration przed wykonywaniem uwierzytelnionych żądań. Jeśli tego nie zrobisz, żądania zakończą się błędem 401.

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

    response = api.add_sso_user(
        tenant_id="YOUR_TENANT_ID",
        create_apisso_user_data=user_data
    )
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # Typowe błędy:
    # - 401: Brakujący lub nieprawidłowy klucz API
    # - 400: Walidacja żądania nie powiodła się
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
    response = public_api.get_comments_public(
        tenant_id="YOUR_TENANT_ID",
        url_id="page-url-id"
    )
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

# Użyj tego tokenu w frontendzie lub przekaż do wywołań API
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

### Częste problemy

1. **401 "missing-api-key" error**: Upewnij się, że ustawisz `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` przed utworzeniem instancji DefaultApi.
2. **Zła klasa API**: Użyj `DefaultApi` dla uwierzytelnionych żądań po stronie serwera, `PublicApi` dla żądań po stronie klienta/publicznych.
3. **Błędy importu**: Upewnij się, że importujesz z poprawnego modułu:
   - API client: `from client import ...`
   - SSO utilities: `from sso import ...`