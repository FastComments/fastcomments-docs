### Использование аутентифицированных API (DefaultApi)

**Важно:** Вы должны установить ваш API-ключ в Configuration перед выполнением аутентифицированных запросов. Если не сделать этого, запросы вернут ошибку 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Создайте и настройте клиент API
config = Configuration()
config.host = "https://fastcomments.com/api"

# ОБЯЗАТЕЛЬНО: Установите ваш API-ключ (получите его на панели FastComments)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Создайте экземпляр API с настроенным клиентом
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Теперь вы можете выполнять аутентифицированные вызовы API
try:
    # Пример: Добавить SSO-пользователя
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
    # Частые ошибки:
    # - 401: API-ключ отсутствует или недействителен
    # - 400: Ошибка валидации запроса
```

### Использование публичных API (PublicApi)

Публичные конечные точки не требуют аутентификации:

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

### Использование панели модерации (ModerationApi)

`ModerationApi` отвечает за работу панели модерации. Методы вызываются от имени модератора путём передачи токена `sso`:

```python
from client import ApiClient, Configuration, ModerationApi

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Подсчитать количество комментариев, ожидающих модерации
    response = moderation_api.get_count(sso="SSO_TOKEN")
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Использование SSO (Single Sign-On)

SDK содержит утилиты для генерации безопасных SSO-токенов:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Создать данные пользователя
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Создать экземпляр SSO с вашим API-секретом
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Сгенерировать SSO-токен
sso_token = sso.create_token()

# Используйте этот токен на фронтенде или передайте в вызовы API
print(f"SSO Token: {sso_token}")
```

Для простого SSO (менее безопасно, для тестирования):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    user_id="user-123",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Частые проблемы

1. **401 "missing-api-key" error**: Убедитесь, что вы установили `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` перед созданием экземпляра DefaultApi.
2. **Wrong API class**: Используйте `DefaultApi` для серверных аутентифицированных запросов, `PublicApi` для клиентских/публичных запросов и `ModerationApi` для запросов панели модерации.
3. **Import errors**: Убедитесь, что вы импортируете из правильного модуля:
   - API client: `from client import ...`
   - SSO utilities: `from sso import ...`