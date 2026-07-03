### Користење аутентификованих API (DefaultApi)

**Важно:** Морате поставити ваш API кључ у Configuration пре него што направите аутентификоване захтеве. Ако то не урадите, захтеви ће пропасти са грешком 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Креирајте и конфигуришите API клијент
config = Configuration()
config.host = "https://fastcomments.com"

# Потребно: Поставите ваш API кључ (преузмите га са вашег FastComments контролне табле)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

# Креирајте API пример са конфигурисаним клијентом
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Сада можете урадити аутентификоване API позиве
try:
    # Пример: Додајте SSO корисника
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user("YOUR_TENANT_ID", user_data)
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # Чести грешке:
    # - 401: API кључ недостаје или је неважећи
    # - 400: Валидација захтева није успела
```

### Коришћење јавних API (PublicApi)

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

### Коришћење контролне табле за модерирање (ModerationApi)

`ModerationApi` управља контролном таблом за модератори. Методе се позивају у име модератора помоћу прослеђивања `sso` токена:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Пребројајте коментаре који чекају модерирање
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Коришћење SSO (Single Sign-On)

SDK укључује алате за генерисање безбедних SSO токена:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Креирајте податке о кориснику (id, email и username су потребни)
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Потпишите га вашим API тајним (HMAC-SHA256)
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# Генеришите SSO токен који ће се проследити у widget или API позив
sso_token = sso.create_token()

# Користите овај токен у вашој фронтенд апликацији или га проследите у API позиве
print(f"SSO Token: {sso_token}")
```

За прост SSO (мање безбедан, за тестирање):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    username="johndoe",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Уобичајени проблеми

1. **401 "missing-api-key" грешка**: Уверите се да сте поставили `config.api_key = {"api_key": "YOUR_KEY"}` пре креирања DefaultApi инстанце.
2. **Погрешна API класа**: Користите `DefaultApi` за серверске аутентификоване захтеве, `PublicApi` за клијентске/јавне захтеве и `ModerationApi` за захтеве контролне табле модератора.
3. **Грешке при увозу**: Уверите се да увозите из исправног модула:
   - API клијент: `from client import ...`
   - SSO алати: `from sso import ...`