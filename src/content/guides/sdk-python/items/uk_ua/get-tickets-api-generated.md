## Параметри

| Назва | Тип | Розташування | Обов’язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| userId | string | query | Ні |  |
| state | number | query | Ні |  |
| skip | number | query | Ні |  |
| limit | number | query | Ні |  |

## Відповідь

Повертає: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tickets_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад get_tickets'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetTicketsOptions
from client.models.get_tickets_response import GetTicketsResponse
from client.rest import ApiException
from pprint import pprint

# Визначення хосту є необов’язковим і за замовчуванням https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
# Клієнт повинен налаштувати параметри автентифікації та авторизації
# у відповідності до політики безпеки API сервера.
# Приклади для кожного методу автентифікації наведені нижче, використайте приклад,
# який відповідає вашому випадку використання автентифікації.

# Налаштування авторизації за допомогою API ключа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для API ключа, якщо потрібно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Введіть контекст з екземпляром клієнта API
with client.ApiClient(configuration) as api_client:
    # Створити екземпляр класу API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (необов’язково)
    state = 3.4 # float |  (необов’язково)
    skip = 3.4 # float |  (необов’язково)
    limit = 3.4 # float |  (необов’язково)

    try:
        api_response = api_instance.get_tickets(tenant_id, GetTicketsOptions(user_id=user_id, state=state, skip=skip, limit=limit))
        print("The response of DefaultApi->get_tickets:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tickets: %s\n" % e)
[inline-code-end]

---