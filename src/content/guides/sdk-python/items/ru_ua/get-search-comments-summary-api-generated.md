## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| value | string | query | Нет |  |
| filters | string | query | Нет |  |
| searchFilters | string | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_comment_search_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_search_comments_summary'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_comment_search_response import ModerationCommentSearchResponse
from client.rest import ApiException
from pprint import pprint

# Указание host необязательно и по умолчанию — https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Откройте контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.ModerationApi(api_client)
    value = 'value_example' # str |  (необязательно)
    filters = 'filters_example' # str |  (необязательно)
    search_filters = 'search_filters_example' # str |  (необязательно)
    sso = 'sso_example' # str |  (необязательно)

    try:
        api_response = api_instance.get_search_comments_summary(value=value, filters=filters, search_filters=search_filters, sso=sso)
        print("The response of ModerationApi->get_search_comments_summary:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_search_comments_summary: %s\n" % e)
[inline-code-end]