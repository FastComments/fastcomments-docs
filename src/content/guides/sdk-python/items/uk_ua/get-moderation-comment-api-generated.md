## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| commentId | string | path | Так |  |
| includeEmail | boolean | query | Ні |  |
| includeIP | boolean | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_comment_response.py)

## Приклад

[inline-code-attrs-start title = 'get_moderation_comment Example'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_api_comment_response import ModerationAPICommentResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
# Відкрийте контекст з екземпляром API-клієнта
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    # Створіть екземпляр класу API
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    include_email = True # bool |  (необов'язково)
    include_ip = True # bool |  (необов'язково)
    sso = 'sso_example' # str |  (необов'язково)

    try:
        api_response = api_instance.get_moderation_comment(comment_id, include_email=include_email, include_ip=include_ip, sso=sso)
        print("The response of ModerationApi->get_moderation_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_moderation_comment: %s\n" % e)
[inline-code-end]