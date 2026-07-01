### Използване на удостоверени API (DefaultApi)

**Важно:** Трябва да зададете вашия API ключ в Configuration преди да правите удостоверени заявки. Ако не го направите, заявките ще се провалят с грешка 401.

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

    response = api.add_sso_user("YOUR_TENANT_ID", user_data)
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # Common errors:
    # - 401: API key is missing or invalid
    # - 400: Request validation failed
```

### Използване на публични API (PublicApi)

Публичните крайни точки не изискват удостоверяване:

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

### Използване на таблото за модериране (ModerationApi)

`ModerationApi` захранва таблото за модератори. Методите се извикват от името на модератор, като се предава `sso` токен:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Count the comments awaiting moderation
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Използване на SSO (Single Sign-On)

SDK‑то включва помощни функции за генериране на сигурни SSO токени:

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

За прост SSO (по‑малко сигурен, за тестове):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    user_id="user-123",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Чести проблеми

1. **401 "missing-api-key" грешка**: Уверете се, че сте задали `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` преди създаването на инстанцията DefaultApi.  
2. **Грешен API клас**: Използвайте `DefaultApi` за сървърно‑странични удостоверени заявки, `PublicApi` за клиентско/публично заявки и `ModerationApi` за заявки от таблото за модериране.  
3. **Грешки при импортиране**: Уверете се, че импортирате от правилния модул:  
   - API клиент: `from client import ...`  
   - SSO помощни функции: `from sso import ...`