## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| largeInternalURLSanitized | string | query | Да |  |

## Отговор

Връща: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/gif_get_large_response.py)

## Пример

[inline-code-attrs-start title = 'Пример за get_gif_large'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.gif_get_large_response import GifGetLargeResponse
from client.rest import ApiException
from pprint import pprint

# Задаването на host е по избор и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък на всички поддържани параметри на конфигурацията.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Влезте в контекст с инстанция на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте инстанция на API класа
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    large_internal_url_sanitized = 'large_internal_url_sanitized_example' # str | 

    try:
        api_response = api_instance.get_gif_large(tenant_id, large_internal_url_sanitized)
        print("The response of PublicApi->get_gif_large:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_gif_large: %s\n" % e)
[inline-code-end]