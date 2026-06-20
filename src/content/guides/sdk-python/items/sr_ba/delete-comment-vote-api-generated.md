## Параметри

| Назив | Тип | Location | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| commentId | string | path | Да |  |
| voteId | string | path | Да |  |
| urlId | string | query | Да |  |
| broadcastId | string | query | Да |  |
| editKey | string | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/vote_delete_response.py)

## Примјер

[inline-code-attrs-start title = 'delete_comment_vote Примјер'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.vote_delete_response import VoteDeleteResponse
from client.rest import ApiException
from pprint import pprint

# Дефинисање host-а је опционално и подразумевано је https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    vote_id = 'vote_id_example' # str | 
    url_id = 'url_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str | 
    edit_key = 'edit_key_example' # str |  (опционо)
    sso = 'sso_example' # str |  (опционо)

    try:
        api_response = api_instance.delete_comment_vote(tenant_id, comment_id, vote_id, url_id, broadcast_id, edit_key=edit_key, sso=sso)
        print("The response of PublicApi->delete_comment_vote:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->delete_comment_vote: %s\n" % e)
[inline-code-end]