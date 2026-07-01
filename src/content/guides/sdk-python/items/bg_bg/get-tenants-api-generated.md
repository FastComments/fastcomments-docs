## Параметри

| Име | Тип | Място | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| meta | string | query | No |  |
| skip | number | query | No |  |

## Отговор

Връща: [`GetTenantsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenants_response.py)

## Пример

[inline-code-attrs-start title = 'get_tenants Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetTenantsOptions
from client.models.get_tenants_response import GetTenantsResponse
from client.rest import ApiException
from pprint import pprint

# Определянето на хоста е по избор и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък със всички поддържани параметри за конфигурация.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиентът трябва да конфигурира параметрите за удостоверяване и упълномощяване
# в съответствие с политиката за сигурност на API сървъра.
# Примери за всеки метод на удостоверяване са предоставени по-долу, използвайте примера, който
# отговаря на вашия случай на използване на удостоверяване.

# Конфигуриране на удостоверяване с API ключ: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Откоментирайте долното, за да зададете префикс (например Bearer) за API ключа, ако е необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    meta = 'meta_example' # str |  (по избор)
    skip = 3.4 # float |  (по избор)

    try:
        api_response = api_instance.get_tenants(tenant_id, GetTenantsOptions(meta=meta, skip=skip))
        print("The response of DefaultApi->get_tenants:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenants: %s\n" % e)
[inline-code-end]