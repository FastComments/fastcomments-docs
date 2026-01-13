## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Отговор

Връща: [`CreateTenant200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_tenant200_response.py)

## Пример

[inline-code-attrs-start title = 'Пример за create_tenant'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_tenant200_response import CreateTenant200Response
from client.models.create_tenant_body import CreateTenantBody
from client.rest import ApiException
from pprint import pprint

# Определянето на хоста е по избор и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък на всички поддържани параметри за конфигурация.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиентът трябва да конфигурира параметрите за удостоверяване и авторизация
# в съответствие с политиката за сигурност на API сървъра.
# Примери за всеки метод за удостоверяване са дадени по-долу, използвайте примера, който
# отговаря на вашия случай на използване за удостоверяване.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# Разкоментирайте по-долу, за да зададете префикс (например Bearer) за API ключа, ако е необходимо

# Enter a context with an instance of the API client
# Влезте в контекст с инстанция на API клиента
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    # Създайте инстанция на API класа
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_tenant_body = client.CreateTenantBody() # CreateTenantBody | 

    try:
        api_response = api_instance.create_tenant(tenant_id, create_tenant_body)
        print("The response of DefaultApi->create_tenant:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_tenant: %s\n" % e)
[inline-code-end]