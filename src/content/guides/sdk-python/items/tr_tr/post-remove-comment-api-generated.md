## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| commentId | string | path | Evet |  |
| broadcastId | string | query | Hayır |  |
| sso | string | query | Hayır |  |

## Response

Döndürür: [`PostRemoveCommentApiResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/post_remove_comment_api_response.py)

## Example

[inline-code-attrs-start title = 'post_remove_comment Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import PostRemoveCommentOptions
from client.models.post_remove_comment_api_response import PostRemoveCommentApiResponse
from client.rest import ApiException
from pprint import pprint

# Host tanımlaması isteğe bağlıdır ve varsayılan olarak https://fastcomments.com adresini kullanır
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
    broadcast_id = 'broadcast_id_example' # str |  (optional)
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.post_remove_comment(tenant_id, comment_id, PostRemoveCommentOptions(broadcast_id=broadcast_id, sso=sso))
        print("The response of ModerationApi->post_remove_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_remove_comment: %s\n" % e)
[inline-code-end]