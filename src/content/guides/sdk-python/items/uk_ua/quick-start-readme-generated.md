### Використання автентифікованих API (DefaultApi)

**Important:** Ви повинні встановити ваш API-ключ у Configuration перед виконанням автентифікованих запитів. Якщо ви цього не зробите, запити завершаться помилкою 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Створіть та налаштуйте клієнт API
config = Configuration()
config.host = "https://fastcomments.com"

# ОБОВ’ЯЗКОВО: Встановіть ваш API-ключ (отримайте його у вашій панелі FastComments)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

# Створіть екземпляр API з налаштованим клієнтом
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Тепер ви можете робити автентифіковані виклики API
try:
    # Приклад: Додати SSO користувача
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user("YOUR_TENANT_ID", user_data)
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # Загальні помилки:
    # - 401: API key відсутній або недійсний
    # - 400: Перевірка запиту не вдалася
```

### Використання публічних API (PublicApi)

Публічні кінцеві точки не вимагають автентифікації:

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

### Використання панелі модерації (ModerationApi)

`ModerationApi` забезпечує роботу панелі модератора. Методи викликаються від імені модератора шляхом передачі токена `sso`:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Підрахувати коментарі, що очікують модерації
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Використання SSO (Single Sign-On)

SDK включає утиліти для генерування безпечних SSO токенів:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Створити дані користувача (id, email та username обов’язкові)
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Підписати його вашим API секретом (HMAC-SHA256)
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# Згенерувати SSO токен для передачі у віджет або API виклик
sso_token = sso.create_token()

# Використовуйте цей токен у вашому фронтенді або передайте у API виклики
print(f"SSO Token: {sso_token}")
```

Для простого SSO (менш безпечного, для тестування):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    username="johndoe",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Поширені проблеми

1. **401 "missing-api-key" error**: Переконайтеся, що ви встановили `config.api_key = {"api_key": "YOUR_KEY"}` перед створенням екземпляра DefaultApi.
2. **Wrong API class**: Використовуйте `DefaultApi` для серверних автентифікованих запитів, `PublicApi` для клієнтських/публічних запитів, та `ModerationApi` для запитів панелі модератора.
3. **Import errors**: Переконайтеся, що ви імпортуєте з правильного модуля:
   - API client: `from client import ...`
   - SSO utilities: `from sso import ...`