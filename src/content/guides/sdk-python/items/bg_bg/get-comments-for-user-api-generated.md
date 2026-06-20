## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| userId | string | query | Не |  |
| direction | string | query | Не |  |
| repliesToUserId | string | query | Не |  |
| page | number | query | Не |  |
| includei10n | boolean | query | Не |  |
| locale | string | query | Не |  |
| isCrawler | boolean | query | Не |  |

## Отговор

Връща: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_for_user_response.py)

## Пример

[inline-code-attrs-start title = 'Пример за get_comments_for_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_for_user_response import GetCommentsForUserResponse
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Дефинирането на host е по избор и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък на всички поддържани параметри за конфигурация.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Влезте в контекст с екземпляр на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте екземпляр на API класа
    api_instance = client.PublicApi(api_client)
    user_id = 'user_id_example' # str |  (по избор)
    direction = client.SortDirections() # SortDirections |  (по избор)
    replies_to_user_id = 'replies_to_user_id_example' # str |  (по избор)
    page = 3.4 # float |  (по избор)
    includei10n = True # bool |  (по избор)
    locale = 'locale_example' # str |  (по избор)
    is_crawler = True # bool |  (по избор)

    try:
        api_response = api_instance.get_comments_for_user(user_id=user_id, direction=direction, replies_to_user_id=replies_to_user_id, page=page, includei10n=includei10n, locale=locale, is_crawler=is_crawler)
        print("The response of PublicApi->get_comments_for_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_for_user: %s\n" % e)
[inline-code-end]