## Parameters

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| commentId | string | path | Evet |  |
| sso | string | query | Hayır |  |

## Yanıt

Döner: [`ModerationAPIGetLogsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_get_logs_response.py)

## Örnek

[inline-code-attrs-start title = 'get_logs Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_api_get_logs_response import ModerationAPIGetLogsResponse
from client.rest import ApiException
from pprint import pprint

# Host'u tanımlamak isteğe bağlıdır ve varsayılan olarak https://fastcomments.com adresini kullanır
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
    sso = 'sso_example' # str |  (opsiyonel)

    try:
        api_response = api_instance.get_logs(tenant_id, comment_id, sso=sso)
        print("The response of ModerationApi->get_logs:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_logs: %s\n" % e)
[inline-code-end]

---