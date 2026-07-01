## Параметри

| Назва | Тип | Розташування | Обов’язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| commentId | string | path | Yes |  |
| broadcastId | string | query | Yes |  |
| editKey | string | query | No |  |
| sso | string | query | No |  |

## Відповідь

Повертає: [`PublicAPISetCommentTextResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/public_api_set_comment_text_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад set_comment_text'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import SetCommentTextOptions
from client.models.comment_text_update_request import CommentTextUpdateRequest
from client.models.public_api_set_comment_text_response import PublicAPISetCommentTextResponse
from client.rest import ApiException
from pprint import pprint

# Визначення хоста є необов’язковим і за замовчуванням встановлює https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str | 
    comment_text_update_request = client.CommentTextUpdateRequest() # CommentTextUpdateRequest | 
    edit_key = 'edit_key_example' # str |  (необов’язково)
    sso = 'sso_example' # str |  (необов’язково)

    try:
        api_response = api_instance.set_comment_text(tenant_id, comment_id, broadcast_id, comment_text_update_request, SetCommentTextOptions(edit_key=edit_key, sso=sso))
        print("The response of PublicApi->set_comment_text:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->set_comment_text: %s\n" % e)
[inline-code-end]