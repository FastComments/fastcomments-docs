## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| direction | string | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Yanıt

Döndürür: [`VoteResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/vote_response.py)

## Örnek

[inline-code-attrs-start title = 'post_vote Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import PostVoteOptions
from client.models.vote_response import VoteResponse
from client.rest import ApiException
from pprint import pprint

# Barındırıcıyı tanımlamak isteğe bağlıdır ve varsayılan olarak https://fastcomments.com adresini kullanır
# configuration.py dosyasında desteklenen tüm yapılandırma parametrelerinin bir listesini bulabilirsiniz.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API istemcisinin bir örneğiyle bir bağlam girin
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluşturun
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    direction = 'direction_example' # str |  (opsiyonel)
    broadcast_id = 'broadcast_id_example' # str |  (opsiyonel)
    sso = 'sso_example' # str |  (opsiyonel)

    try:
        api_response = api_instance.post_vote(tenant_id, comment_id, PostVoteOptions(direction=direction, broadcast_id=broadcast_id, sso=sso))
        print("The response of ModerationApi->post_vote:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_vote: %s\n" % e)
[inline-code-end]