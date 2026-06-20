## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Так |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`PostRemoveCommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/post_remove_comment_response.py)

## Приклад

[inline-code-attrs-start title = 'post_remove_comment Приклад'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.post_remove_comment_response import PostRemoveCommentResponse
from client.rest import ApiException
from pprint import pprint

# Визначення хоста необов'язкове та за замовчуванням встановлено на https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Увійдіть у контекст із екземпляром клієнта API
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    sso = 'sso_example' # str |  (необов'язково)

    try:
        api_response = api_instance.post_remove_comment(comment_id, sso=sso)
        print("The response of ModerationApi->post_remove_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_remove_comment: %s\n" % e)
[inline-code-end]