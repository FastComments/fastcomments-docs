### Коришћење аутентификованих API‑ја (DefaultApi)

**Важно:** Морате поставити ваш API кључ у Configuration пре него што извршите аутентификоване захтеве. Ако то не урадите, захтеви ће пропасти са грешком 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Креирање и конфигурисање API клијента
config = Configuration()
config.host = "https://fastcomments.com"

# ОБАВЕЗНО: Поставите ваш API кључ (преузмите га са вашег FastComments контролне табле)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

# Креирање API инстанце са конфигурисаним клијентом
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Сада можете извршавати аутентификоване API позиве
try:
    # Пример: Додавање SSO корисника
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user("YOUR_TENANT_ID", user_data)
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # Уобичајене грешке:
    # - 401: API кључ недостаје или је неважећи
    # - 400: Валидација захтева није успела
```

### Коришћење јавних API‑ја (PublicApi)

Јавни крајњи тачке не захтевају аутентификацију:

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

### Коришћење контролне табле за модерацију (ModerationApi)

„ModerationApi“ покреће контролну таблу за модераторе. Методи се позивају у име модератора прослеђивањем `sso` токена:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Бројање коментара који чекају на модерацију
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Коришћење SSO (Single Sign-On)

SDK укључује алате за генерисање безбедних SSO токена:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Креирање података о кориснику (ид, имејл и корисничко име су обавезни)
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Потпишите га вашим API тајном (HMAC‑SHA256)
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# Генерисање SSO токена за прослеђивање у виџет или API позив
sso_token = sso.create_token()

# Користите овај токен у вашој фронтенд апликацији или га проследите у API позиве
print(f"SSO Token: {sso_token}")
```

За једноставни SSO (мање безбедан, за тестирање):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

# Креирање података о кориснику
user_data = SimpleSSOUserData(
    username="johndoe",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Живе претплате (PubSub)

Модул `pubsub` вам омогућава претплату на догађаје коментара у реалном времену (нови коментари, гласови, измене, обавештења, итд.) преко WebSocket‑а, реплицирајући `LiveEventSubscriber` из FastComments Java SDK‑а. Захтева `pubsub` екстра, који додаје WebSocket клијента изнад генерисаног API клијента:

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
    user_id_ws="a-unique-presence-id",  # нпр. UUID за ову сесију
    handle_live_event=handle_live_event,
    on_connection_status_change=lambda connected, last_event_time: print(
        f"connected={connected}"
    ),
    region=None,  # поставити на "eu" за EU регион
)

# ...касније, када више не желите ажурирања:
result.close()
```

### Уобичајени проблеми

1. **401 грешка „missing-api-key“**: Уверите се да сте поставили `config.api_key = {"api_key": "YOUR_KEY"}` пре креирања DefaultApi инстанце.  
2. **Погрешна API класа**: Користите `DefaultApi` за серверске аутентификоване захтеве, `PublicApi` за клијентске/јавне захтеве, и `ModerationApi` за захтеве контролне табле модератора.  
3. **Грешке при увозу**: Уверите се да увозите из исправног модула:  
   - API клијент: `from client import ...`  
   - SSO алати: `from sso import ...`  
   - Живе претплате: `from pubsub import ...` (захтева `pubsub` екстра)