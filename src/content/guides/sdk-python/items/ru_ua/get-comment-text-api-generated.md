## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| commentId | string | path | Да |  |
| editKey | string | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`PublicAPIGetCommentTextResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/public_api_get_comment_text_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_comment_text'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.public_api_get_comment_text_response import PublicAPIGetCommentTextResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
# Enter a context with an instance of the API client
# Create an instance of the API class
tenant_id = 'tenant_id_example' # str | 
comment_id = 'comment_id_example' # str | 
edit_key = 'edit_key_example' # str |  (необязательно)
sso = 'sso_example' # str |  (необязательно)

try:
    api_response = api_instance.get_comment_text(tenant_id, comment_id, edit_key=edit_key, sso=sso)
    print("The response of PublicApi->get_comment_text:\n")
    pprint(api_response)
except Exception as e:
    print("Exception when calling PublicApi->get_comment_text: %s\n" % e)
[inline-code-end]