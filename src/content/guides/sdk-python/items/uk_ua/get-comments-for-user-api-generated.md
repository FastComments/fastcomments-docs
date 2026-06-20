## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | Ні |  |
| direction | string | query | Ні |  |
| repliesToUserId | string | query | Ні |  |
| page | number | query | Ні |  |
| includei10n | boolean | query | Ні |  |
| locale | string | query | Ні |  |
| isCrawler | boolean | query | Ні |  |

## Відповідь

Повертає: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_for_user_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад get_comments_for_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_for_user_response import GetCommentsForUserResponse
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Визначення хоста є необов'язковим і за замовчуванням встановлено https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Використовуйте контекст з екземпляром клієнта API
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.PublicApi(api_client)
    user_id = 'user_id_example' # str |  (необов'язковий)
    direction = client.SortDirections() # SortDirections |  (необов'язковий)
    replies_to_user_id = 'replies_to_user_id_example' # str |  (необов'язковий)
    page = 3.4 # float |  (необов'язковий)
    includei10n = True # bool |  (необов'язковий)
    locale = 'locale_example' # str |  (необов'язковий)
    is_crawler = True # bool |  (необов'язковий)

    try:
        api_response = api_instance.get_comments_for_user(user_id=user_id, direction=direction, replies_to_user_id=replies_to_user_id, page=page, includei10n=includei10n, locale=locale, is_crawler=is_crawler)
        print("The response of PublicApi->get_comments_for_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_for_user: %s\n" % e)
[inline-code-end]