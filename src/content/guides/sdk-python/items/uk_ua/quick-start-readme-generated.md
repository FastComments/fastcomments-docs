### Використання аутентифікованих API (DefaultApi)

**Важливо:** Ви повинні встановити ваш API‑ключ у Configuration перед виконанням аутентифікованих запитів. Якщо ви цього не зробите, запити зазнають невдачі з помилкою 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Створити та налаштувати клієнт API
config = Configuration()
config.host = "https://fastcomments.com/api"

# ОБОВ'ЯЗКОВО: Встановити ваш API‑ключ (отримайте його у вашій панелі FastComments)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Створити екземпляр API з налаштованим клієнтом
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Тепер ви можете робити аутентифіковані виклики API
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
    # Поширені помилки:
    # - 401: API‑ключ відсутній або недійсний
    # - 400: Валідація запиту не пройшла
```

### Використання публічних API (PublicApi)

Публічні кінцеві точки не потребують аутентифікації:

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

### Використання панелі модерації (ModerationApi)

`ModerationApi` забезпечує функціонування панелі модератора. Методи викликаються від імені модератора шляхом передачі токену `sso`:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Порахувати коментарі, що очікують модерації
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Використання SSO (Single Sign-On)

SDK містить утиліти для генерування захищених SSO токенів:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Створити дані користувача
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Створити інстанс SSO з вашим API секретом
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Згенерувати SSO токен
sso_token = sso.create_token()

# Використайте цей токен у вашому фронтенді або передайте у виклики API
print(f"SSO Token: {sso_token}")
```

Для простого SSO (менш безпечний, для тестування):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    user_id="user-123",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Поширені проблеми

1. **401 "missing-api-key" error**: Переконайтеся, що ви встановили `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` перед створенням екземпляру DefaultApi.
2. **Wrong API class**: Використовуйте `DefaultApi` для серверних аутентифікованих запитів, `PublicApi` для клієнтських/публічних запитів, і `ModerationApi` для запитів панелі модератора.
3. **Import errors**: Переконайтеся, що ви імпортуєте з правильного модуля:
   - API client: `from client import ...`
   - SSO utilities: `from sso import ...`