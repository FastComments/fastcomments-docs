## Parametreler

| İsim | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| approved | boolean | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Yanıt

Döndürür: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/set_comment_approved_response.py)

## Örnek

[inline-code-attrs-start title = 'post_set_comment_approval_status Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import PostSetCommentApprovalStatusOptions
from client.models.set_comment_approved_response import SetCommentApprovedResponse
from client.rest import ApiException
from pprint import pprint

# Host'i tanımlamak isteğe bağlıdır ve varsayılan olarak https://fastcomments.com'dir
# configuration.py dosyasında desteklenen tüm yapılandırma parametrelerinin bir listesi bulunur.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API istemcisi bir örnek ile bir bağlam içine girin
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluşturun
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    approved = True # bool |  (optional)
    broadcast_id = 'broadcast_id_example' # str |  (optional)
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.post_set_comment_approval_status(tenant_id, comment_id, PostSetCommentApprovalStatusOptions(approved=approved, broadcast_id=broadcast_id, sso=sso))
        print("ModerationApi->post_set_comment_approval_status yanıtı:\n")
        pprint(api_response)
    except Exception as e:
        print("ModerationApi->post_set_comment_approval_status çağrılırken istisna oluştu: %s\n" % e)
[inline-code-end]