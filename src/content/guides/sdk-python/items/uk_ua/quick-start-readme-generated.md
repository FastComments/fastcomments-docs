### Використання автентифікованих API (DefaultApi)

**Важливо:** Ви маєте встановити свій API-ключ у Configuration перед виконанням автентифікованих запитів. Якщо ні, запити завершаться помилкою 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Створіть і налаштуйте клієнт API
config = Configuration()
config.host = "https://fastcomments.com/api"

# ОБОВ'ЯЗКОВО: Встановіть свій API-ключ (отримайте його на панелі керування FastComments)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Створіть екземпляр API з налаштованим клієнтом
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Тепер ви можете виконувати автентифіковані виклики API
try:
    # Приклад: Додати користувача SSO
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
    # Поширені помилки:
    # - 401: API-ключ відсутній або недійсний
    # - 400: Перевірка запиту не пройшла
```

### Використання публічних API (PublicApi)

Публічні кінцеві точки не потребують автентифікації:

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

### Використання панелі модерації (ModerationApi)

`ModerationApi` забезпечує роботу панелі модератора. Методи викликаються від імені модератора шляхом передачі токена `sso`:

```python
from client import ApiClient, Configuration, ModerationApi

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Порахуйте коментарі, що очікують модерації
    response = moderation_api.get_count(sso="SSO_TOKEN")
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Використання SSO (Single Sign-On)

SDK включає утиліти для генерації безпечних SSO токенів:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Створіть дані користувача
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Створіть екземпляр SSO з вашим API секретом
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Згенеруйте SSO токен
sso_token = sso.create_token()

# Використайте цей токен у вашому фронтенді або передайте в виклики API
print(f"SSO Token: {sso_token}")
```

Для простого SSO (менш безпечно, для тестування):

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
2. **Wrong API class**: Використовуйте `DefaultApi` для автентифікованих запитів на сервері, `PublicApi` для клієнтських/публічних запитів і `ModerationApi` для запитів панелі модератора.
3. **Import errors**: Переконайтеся, що ви імпортуєте з правильного модуля:
   - API client: `from client import ...`
   - SSO utilities: `from sso import ...`