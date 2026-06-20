## Параметры

| Имя | Тип | Расположение | Обязательный | Описание |
|------|------|----------|----------|-------------|
| page | number | query | Нет |  |
| count | number | query | Нет |  |
| text-search | string | query | Нет |  |
| byIPFromComment | string | query | Нет |  |
| filters | string | query | Нет |  |
| searchFilters | string | query | Нет |  |
| sorts | string | query | Нет |  |
| demo | boolean | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_get_comments_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_api_comments'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_api_get_comments_response import ModerationAPIGetCommentsResponse
from client.rest import ApiException
from pprint import pprint

# Указание параметра host необязательно и по умолчанию равно https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Войдите в контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.ModerationApi(api_client)
    page = 3.4 # float |  (необязательно)
    count = 3.4 # float |  (необязательно)
    text_search = 'text_search_example' # str |  (необязательно)
    by_ip_from_comment = 'by_ip_from_comment_example' # str |  (необязательно)
    filters = 'filters_example' # str |  (необязательно)
    search_filters = 'search_filters_example' # str |  (необязательно)
    sorts = 'sorts_example' # str |  (необязательно)
    demo = True # bool |  (необязательно)
    sso = 'sso_example' # str |  (необязательно)

    try:
        api_response = api_instance.get_api_comments(page=page, count=count, text_search=text_search, by_ip_from_comment=by_ip_from_comment, filters=filters, search_filters=search_filters, sorts=sorts, demo=demo, sso=sso)
        print("The response of ModerationApi->get_api_comments:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_api_comments: %s\n" % e)
[inline-code-end]