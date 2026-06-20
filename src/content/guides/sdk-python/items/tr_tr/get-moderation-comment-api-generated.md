## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Evet |  |
| includeEmail | boolean | query | Hayır |  |
| includeIP | boolean | query | Hayır |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_comment_response.py)

## Örnek

[inline-code-attrs-start title = 'get_moderation_comment Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_api_comment_response import ModerationAPICommentResponse
from client.rest import ApiException
from pprint import pprint

# Host tanımlamak isteğe bağlıdır ve varsayılan https://fastcomments.com'dur
# Tüm desteklenen yapılandırma parametreleri için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API istemcisinin bir örneği ile bir bağlam açın
with client.ApiClient(configuration) as api_client:
    # API sınıfından bir örnek oluşturun
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    include_email = True # bool |  (isteğe bağlı)
    include_ip = True # bool |  (isteğe bağlı)
    sso = 'sso_example' # str |  (isteğe bağlı)

    try:
        api_response = api_instance.get_moderation_comment(comment_id, include_email=include_email, include_ip=include_ip, sso=sso)
        print("The response of ModerationApi->get_moderation_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_moderation_comment: %s\n" % e)
[inline-code-end]