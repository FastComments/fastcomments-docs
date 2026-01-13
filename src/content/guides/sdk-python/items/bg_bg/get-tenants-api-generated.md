## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| meta | string | query | Не |  |
| skip | number | query | Не |  |

## Отговор

Връща: [`GetTenants200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenants200_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_tenants'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenants200_response import GetTenants200Response
from client.rest import ApiException
from pprint import pprint

# Дефинирането на host е по избор и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък на всички поддържани параметри за конфигурация.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиентът трябва да конфигурира параметрите за удостоверяване и авторизация
# в съответствие с политиката за сигурност на API сървъра.
# Примерите за всеки метод за удостоверяване са дадени по-долу, използвайте примера, който
# отговаря на вашия случай на използване за удостоверяване.

# Конфигуриране на авторизация с API ключ: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Премахнете коментара отдолу, за да зададете префикс (напр. Bearer) за API ключа, ако е необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Влезте в контекст с екземпляр на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте екземпляр на класа API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    meta = 'meta_example' # str |  (optional)
    skip = 3.4 # float |  (optional)

    try:
        api_response = api_instance.get_tenants(tenant_id, meta=meta, skip=skip)
        print("The response of DefaultApi->get_tenants:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenants: %s\n" % e)
[inline-code-end]

---