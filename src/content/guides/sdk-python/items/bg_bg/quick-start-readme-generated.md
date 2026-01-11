### Използване на удостоверени API (DefaultApi)

**Важно:** Трябва да зададете вашия API ключ в Configuration преди да правите удостоверени заявки. Ако не го направите, заявките ще върнат грешка 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Създайте и конфигурирайте API клиента
config = Configuration()
config.host = "https://fastcomments.com/api"

# ЗАДЪЛЖИТЕЛНО: Задайте вашия API ключ (вземете го от таблото на FastComments)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Създайте екземпляра на API с конфигурирания клиент
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Сега можете да правите удостоверени извиквания на API
try:
    # Пример: Добавяне на SSO потребител
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
    # Чести грешки:
    # - 401: API ключът липсва или е невалиден
    # - 400: Валидирането на заявката не успя
```

### Използване на публични API (PublicApi)

Публичните крайни точки не изискват удостоверяване:

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

### Използване на SSO (Еднократно влизане)

SDK-то включва помощни средства за генериране на сигурни SSO токени:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Създаване на данни за потребителя
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Създайте SSO екземпляр с вашия API секрет
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Генериране на SSO токен
sso_token = sso.create_token()

# Използвайте този токен във вашия фронтенд или го предайте на API извиквания
print(f"SSO Token: {sso_token}")
```

За прост SSO (по-малко сигурен, за тестване):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    user_id="user-123",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Чести проблеми

1. **401 "missing-api-key" грешка**: Уверете се, че сте задали `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` преди да създадете екземпляра на DefaultApi.
2. **Грешен клас API**: Използвайте `DefaultApi` за удостоверени заявки от страна на сървъра, `PublicApi` за клиентски/публични заявки.
3. **Грешки при импорт**: Уверете се, че импортирате от правилния модул:
   - API клиент: `from client import ...`
   - SSO помощни средства: `from sso import ...`