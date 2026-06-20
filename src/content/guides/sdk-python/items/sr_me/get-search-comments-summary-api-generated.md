## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| value | string | query | Не |  |
| filters | string | query | Не |  |
| searchFilters | string | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_comment_search_response.py)

## Пример

[inline-code-attrs-start title = 'get_search_comments_summary Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_comment_search_response import ModerationCommentSearchResponse
from client.rest import ApiException
from pprint import pprint

# Постављање host-а је опционално и подразумевано је https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Отворите контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.ModerationApi(api_client)
    value = 'value_example' # str |  (опционално)
    filters = 'filters_example' # str |  (опционално)
    search_filters = 'search_filters_example' # str |  (опционално)
    sso = 'sso_example' # str |  (опционално)

    try:
        api_response = api_instance.get_search_comments_summary(value=value, filters=filters, search_filters=search_filters, sso=sso)
        print("The response of ModerationApi->get_search_comments_summary:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_search_comments_summary: %s\n" % e)
[inline-code-end]