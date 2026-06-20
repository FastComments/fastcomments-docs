---
## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Response

Враћа: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Пример

[inline-code-attrs-start title = 'Пример delete_notification_count'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# Постављање host-а је опционо и подразумева https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
# Погледајте configuration.py за листу свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Клијент мора да конфигурише параметре за аутентификацију и ауторизацију
# у складу са безбедносном политиком API сервера.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.
# Испод су дата упутства (пример) за сваки метод аутентификације, користите пример који
# одговара вашем начину аутентификације.

# Configure API key authorization: api_key
# Конфигуришите овлашћење помоћу API кључа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# Откоментаришите испод да бисте подесили префикс (нпр. Bearer) за API кључ, ако је потребно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.delete_notification_count(tenant_id, id)
        print("The response of DefaultApi->delete_notification_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_notification_count: %s\n" % e)
[inline-code-end]

---