## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Evet |  |
| sso | string | query | Hayır |  |

## Response

Döndürür: [`AdjustVotesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/adjust_votes_response.py)

## Örnek

[inline-code-attrs-start title = 'post_adjust_comment_votes Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.adjust_comment_votes_params import AdjustCommentVotesParams
from client.models.adjust_votes_response import AdjustVotesResponse
from client.rest import ApiException
from pprint import pprint

# Host tanımı isteğe bağlıdır ve varsayılan olarak https://fastcomments.com kullanılır
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API istemcisinin bir örneği ile bir bağlama girin
with client.ApiClient(configuration) as api_client:
    # API sınıfından bir örnek oluşturun
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    adjust_comment_votes_params = client.AdjustCommentVotesParams() # AdjustCommentVotesParams | 
    sso = 'sso_example' # str |  (isteğe bağlı)

    try:
        api_response = api_instance.post_adjust_comment_votes(comment_id, adjust_comment_votes_params, sso=sso)
        print("The response of ModerationApi->post_adjust_comment_votes:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_adjust_comment_votes: %s\n" % e)
[inline-code-end]