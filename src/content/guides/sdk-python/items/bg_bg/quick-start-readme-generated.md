### Използване на автентифицирани API (DefaultApi)

**Важно:** Трябва да зададете вашия API ключ в Configuration преди да правите автентифицирани заявки. Ако не го направите, заявките ще се провалят с грешка 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Create and configure the API client
config = Configuration()
config.host = "https://fastcomments.com/api"

# REQUIRED: Set your API key (get this from your FastComments dashboard)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Create the API instance with the configured client
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Now you can make authenticated API calls
try:
    # Example: Add an SSO user
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
    # Common errors:
    # - 401: API key is missing or invalid
    # - 400: Request validation failed
```

### Използване на публични API (PublicApi)

Публичните крайни точки не изискват автентикация:

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

### Използване на таблото за модерация (ModerationApi)

`ModerationApi` захранва таблото за модерация. Методите се извикват от име на модератор чрез предаване на `sso` токен:

```python
from client import ApiClient, Configuration, ModerationApi

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Count the comments awaiting moderation
    response = moderation_api.get_count(sso="SSO_TOKEN")
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Използване на SSO (Single Sign-On)

SDK включва помощни средства за генериране на сигурни SSO токени:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Create user data
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Create SSO instance with your API secret
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Generate the SSO token
sso_token = sso.create_token()

# Use this token in your frontend or pass to API calls
print(f"SSO Token: {sso_token}")
```

За прост SSO (по-малко сигурен, за тестване):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    user_id="user-123",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Често срещани проблеми

1. **401 "missing-api-key" грешка**: Уверете се, че сте задали `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` преди да създадете екземпляра на DefaultApi.
2. **Неправилен API клас**: Използвайте `DefaultApi` за сървърни автентифицирани заявки, `PublicApi` за клиентски/публични заявки, и `ModerationApi` за заявки на таблото за модерация.
3. **Грешки при импорт**: Уверете се, че импортирате от правилния модул:
   - API клиент: `from client import ...`
   - SSO помощни средства: `from sso import ...`