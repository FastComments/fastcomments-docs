### Using Authenticated APIs (DefaultApi)

**Важно:** Вы должны установить ваш API‑ключ в объект Configuration перед выполнением аутентифицированных запросов. Если вы этого не сделаете, запросы завершятся ошибкой 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Создать и настроить клиент API
config = Configuration()
config.host = "https://fastcomments.com"

# ОБЯЗАТЕЛЬНО: Установите ваш API‑ключ (получите его в панели FastComments)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

# Создать экземпляр API с настроенным клиентом
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Теперь вы можете выполнять аутентифицированные запросы к API
try:
    # Пример: добавить SSO‑пользователя
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user("YOUR_TENANT_ID", user_data)
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # Общие ошибки:
    # - 401: API‑ключ отсутствует или неверен
    # - 400: Ошибка валидации запроса
```

### Using Public APIs (PublicApi)

Публичные эндпоинты не требуют аутентификации:

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

### Using the Moderation Dashboard (ModerationApi)

`ModerationApi` обеспечивает работу панели модератора. Методы вызываются от имени модератора путем передачи токена `sso`:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Подсчитать комментарии, ожидающие модерации
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Using SSO (Single Sign-On)

SDK включает утилиты для генерации защищённых токенов SSO:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Создать данные пользователя (id, email и username обязательны)
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Подписать его с помощью вашего API‑секрета (HMAC‑SHA256)
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# Сгенерировать токен SSO для передачи в виджет или API‑запрос
sso_token = sso.create_token()

# Использовать этот токен во frontend'е или передавать в API‑запросах
print(f"SSO Token: {sso_token}")
```

Для простого SSO (менее безопасного, для тестирования):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    username="johndoe",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Common Issues

1. **Ошибка 401 "missing-api-key"**: Убедитесь, что вы установили `config.api_key = {"api_key": "YOUR_KEY"}` перед созданием экземпляра DefaultApi.
2. **Неправильный класс API**: Используйте `DefaultApi` для серверных аутентифицированных запросов, `PublicApi` — для клиентских/публичных запросов, и `ModerationApi` — для запросов панели модератора.
3. **Ошибки импорта**: Убедитесь, что импортируете из правильного модуля:
   - Клиент API: `from client import ...`
   - Утилиты SSO: `from sso import ...`