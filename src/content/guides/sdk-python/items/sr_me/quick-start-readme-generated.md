### Коришћење аутентификованих API-ја (DefaultApi)

**Важно:** Морате да подесите ваш API кључ у `Configuration` пре слања аутентификованих захтева. У супротном, захтеви ће пропасти са 401 грешком.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Направите и конфигуришите API клијента
config = Configuration()
config.host = "https://fastcomments.com/api"

# ОБАВЕЗНО: Подесите ваш API кључ (преузмите га из FastComments контролне табле)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Направите API инстанцу са конфигурисаним клијентом
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Сада можете да правите аутентификоване API позиве
try:
    # Пример: Додајте SSO корисника
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
    # - 401: API кључ недостаје или није важећи
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

### Коришћење контролне табле за модерацију (ModerationApi)

`ModerationApi` покреће контролну таблу модератора. Методе се позивају у име модератора прослеђивањем `sso` токена:

```python
from client import ApiClient, Configuration, ModerationApi

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Пребројте коментаре који чекају модерацију
    response = moderation_api.get_count(sso="SSO_TOKEN")
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Коришћење SSO (Single Sign-On)

SDK садржи алате за генерисање безбедних SSO токена:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Креирајте податке о кориснику
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Направите SSO инстанцу са вашим API секретом
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Генеришите SSO токен
sso_token = sso.create_token()

# Искористите овај токен у вашем фронтенду или проследите у API позивима
print(f"SSO Token: {sso_token}")
```

За једноставан SSO (мање сигуран, за тестирање):

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

1. **401 "missing-api-key" грешка**: Уверите се да сте подесили `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` пре креирања DefaultApi инстанце.
2. **Погрешна API класа**: Користите `DefaultApi` за аутентификоване захтеве на серверској страни, `PublicApi` за клијентске/јавне захтеве, и `ModerationApi` за захтеве контролне табле модератора.
3. **Грешке приликом увоза**: Уверите се да увозите из правог модула:
   - API client: `from client import ...`
   - SSO utilities: `from sso import ...`