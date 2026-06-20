## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| userId | string | query | Не |  |
| state | number | query | Не |  |
| skip | number | query | Не |  |
| limit | number | query | Не |  |

## Отговор

Връща: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tickets_response.py)

## Пример

[inline-code-attrs-start title = 'Пример за get_tickets'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tickets_response import GetTicketsResponse
from client.rest import ApiException
from pprint import pprint

# Дефинирането на хоста е незадължително и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък на всички поддържани параметри за конфигурация.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиентът трябва да конфигурира параметрите за удостоверяване и упълномощаване
# в съответствие с политиката за сигурност на API сървъра.
# По-долу са предоставени примери за всеки метод на удостоверяване, използвайте примера който
# отговаря на вашия случай на използване за удостоверяване.

# Конфигурирайте удостоверяване с API ключ: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Разкоментирайте по-долу за да зададете префикс (например Bearer) за API ключа, ако е необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Влезте в контекст с инстанция на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте инстанция на API класа
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (незадължително)
    state = 3.4 # float |  (незадължително)
    skip = 3.4 # float |  (незадължително)
    limit = 3.4 # float |  (незадължително)

    try:
        api_response = api_instance.get_tickets(tenant_id, user_id=user_id, state=state, skip=skip, limit=limit)
        print("The response of DefaultApi->get_tickets:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tickets: %s\n" % e)
[inline-code-end]