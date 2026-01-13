### Коришћење аутентификованих API-ја (DefaultApi)

**Важно:** Морате поставити ваш API кључ на Configuration пре прављења аутентификованих захтева. Ако то не урадите, захтеви ће се завршити са грешком 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Креирајте и конфигуришите API клијент
config = Configuration()
config.host = "https://fastcomments.com/api"

# ПОТРЕБНО: Поставите ваш API кључ (нађите га на FastComments контролној табли)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Направите API инстанцу са конфигурисаним клијентом
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
    # Чести проблеми:
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

### Коришћење SSO (Single Sign-On)

SDK садржи алате за генерисање сигурних SSO токена:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Направите податке корисника
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Направите SSO инстанцу са вашим API тајним кључем
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Генеришите SSO токен
sso_token = sso.create_token()

# Користите овај токен у вашем фронтенду или га проследите API позивима
print(f"SSO Token: {sso_token}")
```

За једноставни SSO (мање безбедан, за тестирање):

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

1. **401 "missing-api-key" error**: Уверите се да сте поставили `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` пре него што направите DefaultApi инстанцу.
2. **Погрешна API класа**: Користите `DefaultApi` за серверске аутентификоване захтеве, `PublicApi` за клијентске/јавне захтеве.
3. **Грешке приликом увоза**: Уверите се да увозите из правог модула:
   - API client: `from client import ...`
   - SSO utilities: `from sso import ...`