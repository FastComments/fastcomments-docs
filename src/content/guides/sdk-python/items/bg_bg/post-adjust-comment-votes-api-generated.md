## Parameters

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| commentId | string | path | Да |  |
| broadcastId | string | query | Не |  |
| sso | string | query | Не |  |

## Response

Връща: [`AdjustVotesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/adjust_votes_response.py)

## Пример

[inline-code-attrs-start title = 'post_adjust_comment_votes Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import PostAdjustCommentVotesOptions
from client.models.adjust_comment_votes_params import AdjustCommentVotesParams
from client.models.adjust_votes_response import AdjustVotesResponse
from client.rest import ApiException
from pprint import pprint

# Определянето на хоста е незадължително и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък на всички поддържани параметри за конфигурация.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Въведете контекст с инстанция на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте инстанция на API класа
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    adjust_comment_votes_params = client.AdjustCommentVotesParams() # AdjustCommentVotesParams | 
    broadcast_id = 'broadcast_id_example' # str |  (по избор)
    sso = 'sso_example' # str |  (по избор)

    try:
        api_response = api_instance.post_adjust_comment_votes(tenant_id, comment_id, adjust_comment_votes_params, PostAdjustCommentVotesOptions(broadcast_id=broadcast_id, sso=sso))
        print("The response of ModerationApi->post_adjust_comment_votes:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_adjust_comment_votes: %s\n" % e)
[inline-code-end]