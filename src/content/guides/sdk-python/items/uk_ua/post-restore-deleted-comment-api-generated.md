## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|--------------|------------|------|
| tenantId | string | query | Так |  |
| commentId | string | path | Так |  |
| broadcastId | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Приклад

[inline-code-attrs-start title = 'post_restore_deleted_comment Приклад'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import PostRestoreDeletedCommentOptions
from client.models.api_empty_response import APIEmptyResponse
from client.rest import ApiException
from pprint import pprint

# Визначення хоста є необов'язковим і за замовчуванням встановлено https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Відкрити контекст з екземпляром API-клієнта
with client.ApiClient(configuration) as api_client:
    # Створити екземпляр класу API
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str |  (необов'язково)
    sso = 'sso_example' # str |  (необов'язково)

    try:
        api_response = api_instance.post_restore_deleted_comment(tenant_id, comment_id, PostRestoreDeletedCommentOptions(broadcast_id=broadcast_id, sso=sso))
        print("The response of ModerationApi->post_restore_deleted_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_restore_deleted_comment: %s\n" % e)
[inline-code-end]

---