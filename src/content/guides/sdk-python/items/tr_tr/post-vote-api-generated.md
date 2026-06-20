## Parametreler

| Ad | Tür | Konum | Zorunlu | Açıklama |
|------|------|----------|----------|-------------|
| commentId | string | path | Yes |  |
| direction | string | query | No |  |
| sso | string | query | No |  |

## Yanıt

Döndürür: [`VoteResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/vote_response.py)

## Örnek

[inline-code-attrs-start title = 'post_vote Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.vote_response import VoteResponse
from client.rest import ApiException
from pprint import pprint

# Sunucuyu tanımlamak isteğe bağlıdır ve varsayılan https://fastcomments.com'dur
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API istemcisinin bir örneğiyle bir bağlam açın
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluşturun
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    direction = 'direction_example' # str |  (isteğe bağlı)
    sso = 'sso_example' # str |  (isteğe bağlı)

    try:
        api_response = api_instance.post_vote(comment_id, direction=direction, sso=sso)
        print("The response of ModerationApi->post_vote:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_vote: %s\n" % e)
[inline-code-end]