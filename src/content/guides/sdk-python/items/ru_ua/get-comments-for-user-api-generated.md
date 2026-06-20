## Параметры

| Имя | Тип | Location | Обязательный | Описание |
|------|------|----------|----------|-------------|
| userId | string | query | Нет |  |
| direction | string | query | Нет |  |
| repliesToUserId | string | query | Нет |  |
| page | number | query | Нет |  |
| includei10n | boolean | query | Нет |  |
| locale | string | query | Нет |  |
| isCrawler | boolean | query | Нет |  |

## Ответ

Возвращает: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_for_user_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_comments_for_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_for_user_response import GetCommentsForUserResponse
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Указание хоста необязательно, по умолчанию используется https://fastcomments.com
# Смотрите configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Откройте контекст с экземпляром клиента API
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр API-класса
    api_instance = client.PublicApi(api_client)
    user_id = 'user_id_example' # str |  (optional)
    direction = client.SortDirections() # SortDirections |  (optional)
    replies_to_user_id = 'replies_to_user_id_example' # str |  (optional)
    page = 3.4 # float |  (optional)
    includei10n = True # bool |  (optional)
    locale = 'locale_example' # str |  (optional)
    is_crawler = True # bool |  (optional)

    try:
        api_response = api_instance.get_comments_for_user(user_id=user_id, direction=direction, replies_to_user_id=replies_to_user_id, page=page, includei10n=includei10n, locale=locale, is_crawler=is_crawler)
        print("The response of PublicApi->get_comments_for_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_for_user: %s\n" % e)
[inline-code-end]