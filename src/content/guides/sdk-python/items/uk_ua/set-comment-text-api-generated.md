## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| commentId | string | path | Так |  |
| broadcastId | string | query | Так |  |
| editKey | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`SetCommentText200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/set_comment_text200_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад set_comment_text'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.comment_text_update_request import CommentTextUpdateRequest
from client.models.set_comment_text200_response import SetCommentText200Response
from client.rest import ApiException
from pprint import pprint

# Визначення хоста необов'язкове; за замовчуванням — https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Використайте контекст з екземпляром API-клієнта
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str | 
    comment_text_update_request = client.CommentTextUpdateRequest() # CommentTextUpdateRequest | 
    edit_key = 'edit_key_example' # str |  (необов'язково)
    sso = 'sso_example' # str |  (необов'язково)

    try:
        api_response = api_instance.set_comment_text(tenant_id, comment_id, broadcast_id, comment_text_update_request, edit_key=edit_key, sso=sso)
        print("The response of PublicApi->set_comment_text:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->set_comment_text: %s\n" % e)
[inline-code-end]