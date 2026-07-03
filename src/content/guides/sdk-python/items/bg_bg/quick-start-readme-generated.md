### Използване на автентифицирани API (DefaultApi)

**Важно:** Трябва да зададете вашия API ключ в Configuration преди да правите автентифицирани заявки. Ако не го направите, заявките ще се провалят с грешка 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Създаване и конфигуриране на API клиента
config = Configuration()
config.host = "https://fastcomments.com"

# ЗАДЪЛЖИТЕЛНО: Задайте вашия API ключ (вземете го от таблото на FastComments)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

# Създаване на API инстанция с конфигурирания клиент
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Сега можете да правите автентифицирани API повиквания
try:
    # Пример: Добавяне на SSO потребител
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user("YOUR_TENANT_ID", user_data)
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # Чести грешки:
    # - 401: Липсващ или невалиден API ключ
    # - 400: Валидацията на заявката е неуспешна
```

### Използване на публични API (PublicApi)

Публичните крайни точки не изискват автентификация:

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

### Използване на таблото за модерация (ModerationApi)

`ModerationApi` задвижва таблото за модератори. Методи се извикват от името на модератор, като се предава `sso` токен:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Броене на коментарите, чакащи модерация
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Използване на SSO (Single Sign-On)

SDK включва помощни средства за генериране на сигурни SSO токени:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Създаване на потребителски данни (id, email и username са задължителни)
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Подпишете го с вашия API таен ключ (HMAC‑SHA256)
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# Генериране на SSO токен за предаване към уиджета или API повикване
sso_token = sso.create_token()

# Използвайте този токен във вашия фронтенд или го предайте към API повиквания
print(f"SSO Token: {sso_token}")
```

За прост SSO (по‑малко сигурен, за тестване):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    username="johndoe",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Чести проблеми

1. **401 грешка "missing-api-key"**: Уверете се, че сте задали `config.api_key = {"api_key": "YOUR_KEY"}` преди създаването на инстанция на DefaultApi.
2. **Грешен API клас**: Използвайте `DefaultApi` за сървърно‑странични автентифицирани заявки, `PublicApi` за клиентски/публични заявки и `ModerationApi` за заявки към таблото за модератори.
3. **Грешки при импорт**: Уверете се, че импортирате от правилния модул:
   - API клиент: `from client import ...`
   - SSO помощни средства: `from sso import ...`