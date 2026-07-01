## Parameters

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| userId | string | query | No |  |
| direction | string | query | No |  |
| repliesToUserId | string | query | No |  |
| page | number | query | No |  |
| includei10n | boolean | query | No |  |
| locale | string | query | No |  |
| isCrawler | boolean | query | No |  |

## Ответ

Возвращает: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_for_user_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_comments_for_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetCommentsForUserOptions
from client.models.get_comments_for_user_response import GetCommentsForUserResponse
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Определение хоста является необязательным и по умолчанию https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Войти в контекст с экземпляром клиента API
with client.ApiClient(configuration) as api_client:
    # Создать экземпляр класса API
    api_instance = client.PublicApi(api_client)
    user_id = 'user_id_example' # str |  (необязательно)
    direction = client.SortDirections() # SortDirections |  (необязательно)
    replies_to_user_id = 'replies_to_user_id_example' # str |  (необязательно)
    page = 3.4 # float |  (необязательно)
    includei10n = True # bool |  (необязательно)
    locale = 'locale_example' # str |  (необязательно)
    is_crawler = True # bool |  (необязательно)

    try:
        api_response = api_instance.get_comments_for_user(GetCommentsForUserOptions(user_id=user_id, direction=direction, replies_to_user_id=replies_to_user_id, page=page, includei10n=includei10n, locale=locale, is_crawler=is_crawler))
        print("The response of PublicApi->get_comments_for_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_for_user: %s\n" % e)
[inline-code-end]