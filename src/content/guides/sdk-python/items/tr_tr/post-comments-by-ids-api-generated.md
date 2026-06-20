## Parametreler

| İsim | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_child_comments_response.py)

## Örnek

[inline-code-attrs-start title = 'post_comments_by_ids Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.comments_by_ids_params import CommentsByIdsParams
from client.models.moderation_api_child_comments_response import ModerationAPIChildCommentsResponse
from client.rest import ApiException
from pprint import pprint

# Sunucunun tanımlanması isteğe bağlıdır ve varsayılan olarak https://fastcomments.com kullanılır
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API istemcisinin bir örneğiyle bir bağlam açın
with client.ApiClient(configuration) as api_client:
    # API sınıfından bir örnek oluşturun
    api_instance = client.ModerationApi(api_client)
    comments_by_ids_params = client.CommentsByIdsParams() # CommentsByIdsParams | 
    sso = 'sso_example' # str |  (isteğe bağlı)

    try:
        api_response = api_instance.post_comments_by_ids(comments_by_ids_params, sso=sso)
        print("The response of ModerationApi->post_comments_by_ids:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_comments_by_ids: %s\n" % e)
[inline-code-end]