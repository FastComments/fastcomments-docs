## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Отговор

Връща: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Пример

[inline-code-attrs-start title = 'Пример за delete_tenant_package'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.rest import ApiException
from pprint import pprint

# Задаването на хоста е по избор и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък на всички налични параметри за конфигурация.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиентът трябва да конфигурира параметрите за автентикация и авторизация
# в съответствие с правилата за сигурност на API сървъра.
# Примерите за всеки метод на автентикация са показани по-долу, използвайте примера, който
# отговаря на вашия случай на използване за автентикация.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Премахнете коментара по-долу, за да зададете префикс (e.g. Bearer) за API ключа, ако е необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Влезте в контекст с инстанция на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте инстанция на класа API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.delete_tenant_package(tenant_id, id)
        print("The response of DefaultApi->delete_tenant_package:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_tenant_package: %s\n" % e)
[inline-code-end]