### Използване на автентикирани API (DefaultApi)

**Важно:** Трябва да зададете своя API ключ в Configuration, преди да правите автентикирани заявки. Ако не го направите, заявките ще се провалят с грешка 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Създайте и конфигурирайте API клиента
config = Configuration()
config.host = "https://fastcomments.com"

# ЗАДЪЛЖИТЕЛНО: Задайте своя API ключ (вземете го от вашия FastComments dashboard)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

# Създайте API инстанцията с конфигурирания клиент
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Сега можете да правите автентикирани API повиквания
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
    # - 401: API ключът липсва или е невалиден
    # - 400: Валидацията на заявката се провали
```

### Използване на публични API (PublicApi)

Публичните крайни точки не изискват автентикация:

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

### Използване на дъборда за модериране (ModerationApi)

`ModerationApi` захранва дъборда на модератора. Методите се извикват от името на модератор чрез подаване на `sso` токен:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Пребройте коментарите, чакащи модериране
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Използване на SSO (Single Sign-On)

SDK включва помощни инструменти за генериране на сигурни SSO токени:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Създайте потребителски данни (id, email и username са задължителни)
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Подпишете ги с вашия API секрет (HMAC-SHA256)
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# Генерирайте SSO токена, който да подадете на уиджета или на API повикване
sso_token = sso.create_token()

# Използвайте този токен във вашия frontend или го подавайте на API повиквания
print(f"SSO Token: {sso_token}")
```

За прост SSO (по-несигурен, за тестване):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    username="johndoe",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Абонаменти на живо (PubSub)

Модулът `pubsub` ви позволява да се абонирате за събития на коментари в реално време (нови коментари, гласове, редакции, известия и т.н.) чрез WebSocket, отразявайки `LiveEventSubscriber` на FastComments Java SDK. Изисква екстрата `pubsub`, която добавя WebSocket клиент върху генерирания API клиент:

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
    user_id_ws="a-unique-presence-id",  # напр. UUID за тази сесия
    handle_live_event=handle_live_event,
    on_connection_status_change=lambda connected, last_event_time: print(
        f"connected={connected}"
    ),
    region=None,  # задайте "eu" за EU региона
)

# ...по-късно, когато вече не искате актуализации:
result.close()
```

Абонатът изпълнява връзката на фонов daemon нишка, прозрачно се свързва отново с jitter и извлича пропуснатите събития по време на прекъсване от крайната точка на event-log при повторно свързване. Подайте незадължителен `can_see_comments` callback (`List[str] -> Dict[str, str]`, връщащ id-тата, които потребителят НЕ може да вижда), за да филтрирате събития за коментари, които потребителят няма право да разглежда. Задайте `disable_live_commenting=True`, за да направите `subscribe_to_changes` no-op, който връща `None`.

### Чести проблеми

1. **Грешка 401 "missing-api-key"**: Уверете се, че сте задали `config.api_key = {"api_key": "YOUR_KEY"}` преди създаването на DefaultApi инстанцията.
2. **Грешен API клас**: Използвайте `DefaultApi` за сървърни автентикирани заявки, `PublicApi` за клиентски/публични заявки и `ModerationApi` за заявки към дъборда на модератора.
3. **Грешки при импорт**: Уверете се, че импортирате от правилния модул:
   - API клиент: `from client import ...`
   - SSO инструменти: `from sso import ...`
   - Абонаменти на живо: `from pubsub import ...` (изисква екстрата `pubsub`)