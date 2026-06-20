## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| urlId | string | query | Evet |  |
| broadcastId | string | query | Evet |  |
| sessionId | string | query | Hayır |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`SaveCommentsResponseWithPresence`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/save_comments_response_with_presence.py)

## Örnek

[inline-code-attrs-start title = 'create_comment_public Örnek'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.comment_data import CommentData
from client.models.save_comments_response_with_presence import SaveCommentsResponseWithPresence
from client.rest import ApiException
from pprint import pprint

# Sunucu tanımlaması isteğe bağlıdır ve varsayılan https://fastcomments.com'dur
# Tüm desteklenen yapılandırma parametreleri için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API istemcisinin bir örneği ile bir bağlam açın
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluşturun
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str | 
    comment_data = client.CommentData() # CommentData | 
    session_id = 'session_id_example' # str |  (isteğe bağlı)
    sso = 'sso_example' # str |  (isteğe bağlı)

    try:
        api_response = api_instance.create_comment_public(tenant_id, url_id, broadcast_id, comment_data, session_id=session_id, sso=sso)
        print("The response of PublicApi->create_comment_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->create_comment_public: %s\n" % e)
[inline-code-end]