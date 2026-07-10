### Использование аутентифицированных API (DefaultApi)

**Важно:** Вы должны установить ваш API‑ключ в Configuration перед выполнением аутентифицированных запросов. Если вы этого не сделаете, запросы завершатся ошибкой 401.

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
    # - 400: Ошибка проверки запроса
```

### Использование публичных API (PublicApi)

Публичные конечные точки не требуют аутентификации:

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

### Использование панели модерации (ModerationApi)

`ModerationApi` обеспечивает работу панели модератора. Методы вызываются от имени модератора, передавая токен `sso`:

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

### Использование SSO (Single Sign-On)

SDK включает утилиты для генерации безопасных SSO‑токенов:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Создать данные пользователя (id, email и username обязательны)
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Подписать его вашим API‑секретом (HMAC‑SHA256)
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# Сгенерировать SSO‑токен для передачи в виджет или API‑вызов
sso_token = sso.create_token()

# Использовать этот токен во фронтенде или передавать в API‑вызовы
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

### Живые подписки (PubSub)

Модуль `pubsub` позволяет подписываться на события комментариев в реальном времени (новые комментарии, голоса, правки, уведомления и т.д.) через WebSocket, аналогично `LiveEventSubscriber` из Java‑SDK FastComments. Для этого требуется дополнительный пакет `pubsub`, который добавляет WebSocket‑клиент поверх сгенерированного API‑клиента:

```bash
pip install "fastcomments[pubsub] @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0"
```

```python
from pubsub import LiveEventSubscriber

subscriber = LiveEventSubscriber()

def handle_live_event(event):
    print(f"Live event: {event.type}")
    if event.comment:
        print(f"  comment: {event.comment.comment}")

result = subscriber.subscribe_to_changes(
    tenant_id_ws="YOUR_TENANT_ID",
    url_id="page-url-id",
    url_id_ws="page-url-id",
    user_id_ws="a-unique-presence-id",  # например, UUID для этой сессии
    handle_live_event=handle_live_event,
    on_connection_status_change=lambda connected, last_event_time: print(
        f"connected={connected}"
    ),
    region=None,  # set to "eu" for the EU region
)

# ... позже, когда обновления больше не нужны:
result.close()
```

Подписчик запускает соединение в фоновом демоническом потоке, автоматически переподключается с джиттером и получает любые события, пропущенные во время отключения, с конечной точки журнала событий при повторном подключении. Передайте необязательный обратный вызов `can_see_comments` (`List[str] -> Dict[str, str]`, возвращающий идентификаторы, которые пользователь НЕ ДОЛЖЕН видеть) для фильтрации событий комментариев, недоступных пользователю. Установите `disable_live_commenting=True`, чтобы `subscribe_to_changes` стал пустой операцией, возвращающей `None`.

### Распространённые проблемы

1. **Ошибка 401 "missing-api-key"**: Убедитесь, что вы установили `config.api_key = {"api_key": "YOUR_KEY"}` перед созданием экземпляра DefaultApi.
2. **Неправильный класс API**: Используйте `DefaultApi` для серверных аутентифицированных запросов, `PublicApi` для клиентских/публичных запросов и `ModerationApi` для запросов панели модератора.
3. **Ошибки импорта**: Убедитесь, что импортируете из правильного модуля:
   - API‑клиент: `from client import ...`
   - Утилиты SSO: `from sso import ...`
   - Живые подписки: `from pubsub import ...` (требуется дополнительный пакет `pubsub`)