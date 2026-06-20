## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| page | number | query | Не |  |
| count | number | query | Не |  |
| text-search | string | query | Не |  |
| byIPFromComment | string | query | Не |  |
| filters | string | query | Не |  |
| searchFilters | string | query | Не |  |
| sorts | string | query | Не |  |
| demo | boolean | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_get_comments_response.py)

## Пример

[inline-code-attrs-start title = 'get_api_comments Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_api_get_comments_response import ModerationAPIGetCommentsResponse
from client.rest import ApiException
from pprint import pprint

# Подешавање host-а је опционо и подразумевано је https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.ModerationApi(api_client)
    page = 3.4 # float |  (опционо)
    count = 3.4 # float |  (опционо)
    text_search = 'text_search_example' # str |  (опционо)
    by_ip_from_comment = 'by_ip_from_comment_example' # str |  (опционо)
    filters = 'filters_example' # str |  (опционо)
    search_filters = 'search_filters_example' # str |  (опционо)
    sorts = 'sorts_example' # str |  (опционо)
    demo = True # bool |  (опционо)
    sso = 'sso_example' # str |  (опционо)

    try:
        api_response = api_instance.get_api_comments(page=page, count=count, text_search=text_search, by_ip_from_comment=by_ip_from_comment, filters=filters, search_filters=search_filters, sorts=sorts, demo=demo, sso=sso)
        print("The response of ModerationApi->get_api_comments:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_api_comments: %s\n" % e)
[inline-code-end]