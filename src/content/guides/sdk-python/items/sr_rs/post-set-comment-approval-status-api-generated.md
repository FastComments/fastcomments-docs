## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| commentId | string | path | Да |  |
| approved | boolean | query | Не |  |
| broadcastId | string | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/set_comment_approved_response.py)

## Пример

[inline-code-attrs-start title = 'post_set_comment_approval_status Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import PostSetCommentApprovalStatusOptions
from client.models.set_comment_approved_response import SetCommentApprovedResponse
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
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    approved = True # bool |  (optional)
    broadcast_id = 'broadcast_id_example' # str |  (optional)
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.post_set_comment_approval_status(tenant_id, comment_id, PostSetCommentApprovalStatusOptions(approved=approved, broadcast_id=broadcast_id, sso=sso))
        print("Одговор од ModerationApi->post_set_comment_approval_status:\n")
        pprint(api_response)
    except Exception as e:
        print("Изузетак при позиву ModerationApi->post_set_comment_approval_status: %s\n" % e)
[inline-code-end]