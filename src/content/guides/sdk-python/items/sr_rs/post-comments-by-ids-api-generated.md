## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| sso | string | query | Не |  |

## Одговор

Враћа: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_child_comments_response.py)

## Пример

[inline-code-attrs-start title = 'post_comments_by_ids Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.comments_by_ids_params import CommentsByIdsParams
from client.models.moderation_api_child_comments_response import ModerationAPIChildCommentsResponse
from client.rest import ApiException
from pprint import pprint

# Постављање host-а је опционо и подразумевано је https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.ModerationApi(api_client)
    comments_by_ids_params = client.CommentsByIdsParams() # CommentsByIdsParams | 
    sso = 'sso_example' # str |  (опционо)

    try:
        api_response = api_instance.post_comments_by_ids(comments_by_ids_params, sso=sso)
        print("The response of ModerationApi->post_comments_by_ids:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_comments_by_ids: %s\n" % e)
[inline-code-end]