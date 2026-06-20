---
## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| search | string | query | Да |  |
| locale | string | query | Не |  |
| rating | string | query | Не |  |
| page | number | query | Не |  |

## Response

Връща: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_gifs_search_response.py)

## Пример

[inline-code-attrs-start title = 'Пример за get_gifs_search'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_gifs_search_response import GetGifsSearchResponse
from client.rest import ApiException
from pprint import pprint

# Задаването на host е по избор и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък на всички поддържани конфигурационни параметри.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Влезте в контекст с инстанция на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте инстанция на API класа
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    search = 'search_example' # str | 
    locale = 'locale_example' # str |  (по избор)
    rating = 'rating_example' # str |  (по избор)
    page = 3.4 # float |  (по избор)

    try:
        api_response = api_instance.get_gifs_search(tenant_id, search, locale=locale, rating=rating, page=page)
        print("The response of PublicApi->get_gifs_search:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_gifs_search: %s\n" % e)
[inline-code-end]

---