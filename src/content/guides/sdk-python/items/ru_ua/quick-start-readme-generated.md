### Using Authenticated APIs (DefaultApi)

**Важно:** Вы должны установить ваш API‑ключ в Configuration перед выполнением аутентифицированных запросов. Если не сделаете, запросы завершатся ошибкой 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Создать и настроить клиент API
config = Configuration()
config.host = "https://fastcomments.com/api"

# ОБЯЗАТЕЛЬНО: Установите ваш API-ключ (получите его в панели FastComments)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Создать экземпляр API с настроенным клиентом
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Теперь вы можете выполнять аутентифицированные вызовы API
try:
    # Пример: Добавить SSO‑пользователя
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
    # - 401: API‑ключ отсутствует или недействителен
    # - 400: Ошибка валидации запроса
```

### Using Public APIs (PublicApi)

Публичные эндпоинты не требуют аутентификации:

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

### Using the Moderation Dashboard (ModerationApi)

`ModerationApi` обеспечивает работу панели модератора. Методы вызываются от имени модератора путем передачи токена `sso`:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com/api"

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

SDK включает утилиты для генерации безопасных токенов SSO:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Создать данные пользователя
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Создать экземпляр SSO с вашим секретом API
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Сгенерировать токен SSO
sso_token = sso.create_token()

# Использовать этот токен во Frontend или передавать в вызовы API
print(f"SSO Token: {sso_token}")
```

Для простого SSO (меньше безопасности, только для тестирования):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    user_id="user-123",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Common Issues

1. **401 ошибка "missing-api-key"**: Убедитесь, что вы установили `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` перед созданием экземпляра DefaultApi.  
2. **Неправильный класс API**: Используйте `DefaultApi` для серверных аутентифицированных запросов, `PublicApi` для клиентских/публичных запросов и `ModerationApi` для запросов панели модератора.  
3. **Ошибка импорта**: Убедитесь, что импортируете из правильного модуля:  
   - клиент API: `from client import ...`  
   - утилиты SSO: `from sso import ...`