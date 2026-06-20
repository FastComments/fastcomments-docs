## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| commentId | string | path | Да |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`PostRemoveCommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/post_remove_comment_response.py)

## Пример

[inline-code-attrs-start title = 'post_remove_comment Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.post_remove_comment_response import PostRemoveCommentResponse
from client.rest import ApiException
from pprint import pprint

# Постављање host-а је опционално и подразумевано је https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    sso = 'sso_example' # str |  (опционо)

    try:
        api_response = api_instance.post_remove_comment(comment_id, sso=sso)
        print("The response of ModerationApi->post_remove_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_remove_comment: %s\n" % e)
[inline-code-end]