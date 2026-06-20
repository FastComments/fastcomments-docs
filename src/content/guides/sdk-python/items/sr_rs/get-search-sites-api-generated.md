## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| value | string | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_site_search_response.py)

## Пример

[inline-code-attrs-start title = 'get_search_sites Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_site_search_response import ModerationSiteSearchResponse
from client.rest import ApiException
from pprint import pprint

# Дефинисање host-а је опционално и подразумевано је https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу класе API
    api_instance = client.ModerationApi(api_client)
    value = 'value_example' # str |  (опционо)
    sso = 'sso_example' # str |  (опционо)

    try:
        api_response = api_instance.get_search_sites(value=value, sso=sso)
        print("The response of ModerationApi->get_search_sites:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_search_sites: %s\n" % e)
[inline-code-end]