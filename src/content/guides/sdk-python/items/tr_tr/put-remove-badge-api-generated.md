## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| badgeId | string | sorgu | Evet |  |
| userId | string | sorgu | Hayır |  |
| commentId | string | sorgu | Hayır |  |
| broadcastId | string | sorgu | Hayır |  |
| sso | string | sorgu | Hayır |  |

## Yanıt

Döndürür: [`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/remove_user_badge_response.py)

## Örnek

[inline-code-attrs-start title = 'put_remove_badge Örnek'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.remove_user_badge_response import RemoveUserBadgeResponse
from client.rest import ApiException
from pprint import pprint

# Host tanımlamak isteğe bağlıdır ve varsayılan olarak https://fastcomments.com kullanılır
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Bir API istemcisi örneği ile bir bağlam içine girin
with client.ApiClient(configuration) as api_client:
    # API sınıfından bir örnek oluşturun
    api_instance = client.ModerationApi(api_client)
    badge_id = 'badge_id_example' # str | 
    user_id = 'user_id_example' # str |  (isteğe bağlı)
    comment_id = 'comment_id_example' # str |  (isteğe bağlı)
    broadcast_id = 'broadcast_id_example' # str |  (isteğe bağlı)
    sso = 'sso_example' # str |  (isteğe bağlı)

    try:
        api_response = api_instance.put_remove_badge(badge_id, user_id=user_id, comment_id=comment_id, broadcast_id=broadcast_id, sso=sso)
        print("The response of ModerationApi->put_remove_badge:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->put_remove_badge: %s\n" % e)
[inline-code-end]