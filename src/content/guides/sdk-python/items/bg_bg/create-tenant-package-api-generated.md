## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |

## Отговор

Връща: [`CreateTenantPackage200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_tenant_package200_response.py)

## Пример

[inline-code-attrs-start title = 'Пример за create_tenant_package'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_tenant_package200_response import CreateTenantPackage200Response
from client.models.create_tenant_package_body import CreateTenantPackageBody
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# Задаването на host е по избор и по подразбиране е https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
# Вижте configuration.py за списък с всички поддържани параметри за конфигурация.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure the authentication and authorization parameters
# Клиентът трябва да конфигурира параметрите за удостоверяване и упълномощаване
# in accordance with the API server security policy.
# в съответствие с политиката за сигурност на API сървъра.
# Examples for each auth method are provided below, use the example that
# Примерите за всеки метод за удостоверяване са по-долу, използвайте примера, който
# satisfies your auth use case.
# отговаря на вашия сценарий за удостоверяване.

# Configure API key authorization: api_key
# Конфигурирайте удостоверяване с API ключ: api_key

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# Премахнете коментара по-долу, за да зададете префикс (напр. Bearer) за API ключ, ако е необходимо

# Enter a context with an instance of the API client
# Влезте в контекст с инстанция на API клиента
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    # Създайте инстанция на API класа
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_tenant_package_body = client.CreateTenantPackageBody() # CreateTenantPackageBody | 

    try:
        api_response = api_instance.create_tenant_package(tenant_id, create_tenant_package_body)
        print("The response of DefaultApi->create_tenant_package:\n")
        # The response of DefaultApi->create_tenant_package:\n
        print(pprint(api_response))
    except Exception as e:
        # Exception when calling DefaultApi->create_tenant_package: %s\n
        print("Exception when calling DefaultApi->create_tenant_package: %s\n" % e)
[inline-code-end]

---