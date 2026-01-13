## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| commentId | string | path | Evet |  |
| broadcastId | string | query | Evet |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`LockComment200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/lock_comment200_response.py)

## Örnek

[inline-code-attrs-start title = 'un_lock_comment Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.lock_comment200_response import LockComment200Response
from client.rest import ApiException
from pprint import pprint

# Sunucu tanımı isteğe bağlıdır ve varsayılan olarak https://fastcomments.com kullanılır
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API istemcisinin bir örneği ile bir bağlam (context) açın
with client.ApiClient(configuration) as api_client:
    # API sınıfından bir örnek oluşturun
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str | 
    sso = 'sso_example' # str |  (isteğe bağlı)

    try:
        api_response = api_instance.un_lock_comment(tenant_id, comment_id, broadcast_id, sso=sso)
        print("The response of PublicApi->un_lock_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->un_lock_comment: %s\n" % e)
[inline-code-end]