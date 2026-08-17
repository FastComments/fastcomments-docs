## Parameters

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| commentId | string | path | Да |  |
| editKey | string | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Returns: [`PublicAPIGetCommentTextResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/public_api_get_comment_text_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_comment_text'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetCommentTextOptions
from client.models.public_api_get_comment_text_response import PublicAPIGetCommentTextResponse
from client.rest import ApiException
from pprint import pprint

# Дефинисање хоста је опционо и подразумева https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # стр | 
    comment_id = 'comment_id_example' # стр | 
    edit_key = 'edit_key_example' # стр |  (опционо)
    sso = 'sso_example' # стр |  (опционо)

    try:
        api_response = api_instance.get_comment_text(tenant_id, comment_id, GetCommentTextOptions(edit_key=edit_key, sso=sso))
        print("The response of PublicApi->get_comment_text:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comment_text: %s\n" % e)
[inline-code-end]