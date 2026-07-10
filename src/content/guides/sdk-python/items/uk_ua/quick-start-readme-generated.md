### Використання автентифікованих API (DefaultApi)

**Important:** Ви повинні встановити ваш API‑ключ у Configuration перед виконанням автентифікованих запитів. Якщо ви цього не зробите, запити завершаться помилкою 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Створити та налаштувати API‑клієнт
config = Configuration()
config.host = "https://fastcomments.com"

# ОБОВ'ЯЗКОВО: Встановіть ваш API‑ключ (отримайте його у вашій панелі FastComments)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

# Створити екземпляр API з налаштованим клієнтом
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Тепер ви можете виконувати автентифіковані API‑виклики
try:
    # Приклад: Додати SSO‑користувача
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
    # - 400: Не вдалося перевірити запит
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

`ModerationApi` забезпечує роботу панелі модератора. Методи викликаються від імені модератора шляхом передачі токену `sso`:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Count the comments awaiting moderation
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Використання SSO (Single Sign-On)

SDK включає утиліти для генерації безпечних SSO‑токенів:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Create user data (id, email, and username are required)
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Sign it with your API secret (HMAC-SHA256)
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# Generate the SSO token to pass to the widget or an API call
sso_token = sso.create_token()

# Use this token in your frontend or pass to API calls
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

### Живі підписки (PubSub)

Модуль `pubsub` дозволяє підписатися на події коментарів у реальному часі (нові коментарі, голоси, редагування, сповіщення тощо) через WebSocket, повторюючи `LiveEventSubscriber` Java SDK FastComments. Для цього потрібен додатковий пакет `pubsub`, який додає WebSocket‑клієнт до згенерованого API‑клієнта:

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
    user_id_ws="a-unique-presence-id",  # напр., UUID для цієї сесії
    handle_live_event=handle_live_event,
    on_connection_status_change=lambda connected, last_event_time: print(
        f"connected={connected}"
    ),
    region=None,  # встановити "eu" для регіону EU
)

# ...пізніше, коли ви більше не хочете оновлень:
result.close()
```

Підписник запускає з’єднання у фоновому демон‑потоку, прозоро перепідключається з джиттером і отримує будь‑які події, пропущені під час відключення, з кінцевої точки event‑log при перепідключенні. Передайте необов’язковий зворотний виклик `can_see_comments` (`List[str] -> Dict[str, str]`, що повертає ідентифікатори, які користувач **НЕ** може бачити), щоб фільтрувати події для коментарів, які користувач не має права переглядати. Встановіть `disable_live_commenting=True`, щоб `subscribe_to_changes` не виконувало нічого і повертало `None`.

### Поширені проблеми

1. **401 "missing-api-key" error**: Переконайтеся, що ви встановили `config.api_key = {"api_key": "YOUR_KEY"}` перед створенням екземпляра DefaultApi.
2. **Wrong API class**: Використовуйте `DefaultApi` для серверних автентифікованих запитів, `PublicApi` для клієнтських/публічних запитів, та `ModerationApi` для запитів панелі модератора.
3. **Import errors**: Переконайтеся, що ви імпортуєте з правильного модуляля:
   - API‑клієнт: `from client import ...`
   - Утиліти SSO: `from sso import ...`
   - Живі підписки: `from pubsub import ...` (потрібен додаток `pubsub`)