## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
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

## Отговор

Връща: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_get_comments_response.py)

## Пример

[inline-code-attrs-start title = 'get_api_comments Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_api_get_comments_response import ModerationAPIGetCommentsResponse
from client.rest import ApiException
from pprint import pprint

# Задаването на хост е по избор и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък на всички поддържани параметри за конфигурация.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Влезте в контекст с инстанция на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте инстанция на класа API
    api_instance = client.ModerationApi(api_client)
    page = 3.4 # float |  (по избор)
    count = 3.4 # float |  (по избор)
    text_search = 'text_search_example' # str |  (по избор)
    by_ip_from_comment = 'by_ip_from_comment_example' # str |  (по избор)
    filters = 'filters_example' # str |  (по избор)
    search_filters = 'search_filters_example' # str |  (по избор)
    sorts = 'sorts_example' # str |  (по избор)
    demo = True # bool |  (по избор)
    sso = 'sso_example' # str |  (по избор)

    try:
        api_response = api_instance.get_api_comments(page=page, count=count, text_search=text_search, by_ip_from_comment=by_ip_from_comment, filters=filters, search_filters=search_filters, sorts=sorts, demo=demo, sso=sso)
        print("The response of ModerationApi->get_api_comments:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_api_comments: %s\n" % e)
[inline-code-end]