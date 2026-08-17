## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|--------------|-------------|------|
| tenantId | string | query | Так |  |
| commentId | string | path | Так |  |
| reviewed | boolean | query | Ні |  |
| broadcastId | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Приклад

[inline-code-attrs-start title = 'post_set_comment_review_status Приклад'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import PostSetCommentReviewStatusOptions
from client.models.api_empty_response import APIEmptyResponse
from client.rest import ApiException
from pprint import pprint

# Визначення хоста є необов'язковим і за замовчуванням встановлює https://fastcomments.com
# Дивіться configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Відкрити контекст з екземпляром клієнта API
with client.ApiClient(configuration) as api_client:
    # Створити екземпляр класу API
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    reviewed = True # bool |  (optional)
    broadcast_id = 'broadcast_id_example' # str |  (optional)
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.post_set_comment_review_status(tenant_id, comment_id, PostSetCommentReviewStatusOptions(reviewed=reviewed, broadcast_id=broadcast_id, sso=sso))
        print("Відповідь ModerationApi->post_set_comment_review_status:\n")
        pprint(api_response)
    except Exception as e:
        print("Виняток під час виклику ModerationApi->post_set_comment_review_status: %s\n" % e)
[inline-code-end]