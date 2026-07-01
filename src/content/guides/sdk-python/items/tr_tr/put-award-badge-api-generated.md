## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| badgeId | string | query | Evet |  |
| userId | string | query | Hayır |  |
| commentId | string | query | Hayır |  |
| broadcastId | string | query | Hayır |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/award_user_badge_response.py)

## Örnek

[inline-code-attrs-start title = 'put_award_badge Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import PutAwardBadgeOptions
from client.models.award_user_badge_response import AwardUserBadgeResponse
from client.rest import ApiException
from pprint import pprint

# Host tanımlamak isteğe bağlıdır ve varsayılan olarak https://fastcomments.com
# Tüm desteklenen yapılandırma parametrelerinin bir listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API istemcisi bir örnek ile bir bağlam girin
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluştur
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    badge_id = 'badge_id_example' # str | 
    user_id = 'user_id_example' # str |  (isteğe bağlı)
    comment_id = 'comment_id_example' # str |  (isteğe bağlı)
    broadcast_id = 'broadcast_id_example' # str |  (isteğe bağlı)
    sso = 'sso_example' # str |  (isteğe bağlı)

    try:
        api_response = api_instance.put_award_badge(tenant_id, badge_id, PutAwardBadgeOptions(user_id=user_id, comment_id=comment_id, broadcast_id=broadcast_id, sso=sso))
        print("ModerationApi->put_award_badge yanıtı:\n")
        pprint(api_response)
    except Exception as e:
        print("ModerationApi->put_award_badge çağrılırken istisna: %s\n" % e)
[inline-code-end]