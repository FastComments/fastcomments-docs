## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------------|--------------|----------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| state | number | query | No |  |
| skip | number | query | No |  |
| limit | number | query | No |  |

## Отговор

Връща: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tickets_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_tickets'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetTicketsOptions
from client.models.get_tickets_response import GetTicketsResponse
from client.rest import ApiException
from pprint import pprint

# Определянето на host е незадължително и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък с всички поддържани параметри за конфигурация.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиентът трябва да конфигурира параметрите за автентикация и оторизация
# в съответствие с политиката за сигурност на API сървъра.
# Примери за всеки метод за удостоверяване са предоставени по-долу, използвайте примера,
# който отговаря на вашия случай на използване на удостоверяване.

# Конфигурирайте API ключ за оторизация: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Разкоментирайте по-долу, за да зададете префикс (напр. Bearer) за API ключ, ако е необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Въведете контекст с инстанция на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте инстанция на API класa
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (по избор)
    state = 3.4 # float |  (по избор)
    skip = 3.4 # float |  (по избор)
    limit = 3.4 # float |  (по избор)

    try:
        api_response = api_instance.get_tickets(tenant_id, GetTicketsOptions(user_id=user_id, state=state, skip=skip, limit=limit))
        print("The response of DefaultApi->get_tickets:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tickets: %s\n" % e)
[inline-code-end]