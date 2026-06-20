### Коришћење аутентификованих API-ја (DefaultApi)

**Важно:** Морате подесити ваш API кључ у Configuration пре слања аутентификованих захтева. Ако то не урадите, захтеви ће бити одбијени са 401 грешком.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Креирајте и конфигуришите API клијента
config = Configuration()
config.host = "https://fastcomments.com/api"

# ОБАВЕЗНО: Подесите ваш API кључ (преузмите га са вашe FastComments контролне табле)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Креирајте API инстанцу са конфигурисаним клијентом
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Сада можете правити аутентификоване API позиве
try:
    # Пример: Додавање SSO корисника
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
    # Уобичајене грешке:
    # - 401: API кључ недостаје или је неважећи
    # - 400: Валидација захтева није успела
```

### Коришћење јавних API-ја (PublicApi)

Јавни ендпоинти не захтевају аутентификацију:

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

### Коришћење модерацијске контролне табле (ModerationApi)

`ModerationApi` покреће модерацијску контролну таблу. Методе се позивају у име модератора прослеђивањем `sso` токена:

```python
from client import ApiClient, Configuration, ModerationApi

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Броји коментаре који чекају модерацију
    response = moderation_api.get_count(sso="SSO_TOKEN")
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Коришћење SSO (Single Sign-On)

SDK садржи алате за генерисање сигурних SSO токена:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Креирајте податке о кориснику
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Креирајте SSO инстанцу са вашим API секретом
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Генеришите SSO токен
sso_token = sso.create_token()

# Користите овај токен у вашем фронтенду или прослиједите у API позиве
print(f"SSO Token: {sso_token}")
```

За једноставан SSO (мање сигурно, за тестирање):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    user_id="user-123",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Уобичајени проблеми

1. **401 "missing-api-key" error**: Уверите се да сте поставили `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` пре креирања DefaultApi инстанце.
2. **Погрешна API класа**: Користите `DefaultApi` за серверске аутентификоване захтеве, `PublicApi` за клијентске/јавне захтеве, и `ModerationApi` за захтеве модерацијске контролне табле.
3. **Грешке при увозу**: Уверите се да увозите из исправног модула:
   - API клијент: `from client import ...`
   - SSO алати: `from sso import ...`