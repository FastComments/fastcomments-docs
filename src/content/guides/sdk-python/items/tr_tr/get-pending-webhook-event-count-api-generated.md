## Parametreler

| Ad | Tip | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| commentId | string | query | Hayır |  |
| externalId | string | query | Hayır |  |
| eventType | string | query | Hayır |  |
| type | string | query | Hayır |  |
| domain | string | query | Hayır |  |
| attemptCountGT | number | query | Hayır |  |

## Yanıt

Döndürür: [`GetPendingWebhookEventCount200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_pending_webhook_event_count200_response.py)

## Örnek

[inline-code-attrs-start title = 'get_pending_webhook_event_count Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_pending_webhook_event_count200_response import GetPendingWebhookEventCount200Response
from client.rest import ApiException
from pprint import pprint

# Host tanımlamak isteğe bağlıdır ve varsayılan olarak https://fastcomments.com kullanılır
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# İstemci, kimlik doğrulama ve yetkilendirme parametrelerini yapılandırmalıdır
# API sunucusu güvenlik politikasına uygun olarak.
# Her bir kimlik doğrulama yöntemi için örnekler aşağıda verilmiştir, kullanılacak
# örneği kimlik doğrulama kullanım durumunuza uygun olanıdır.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Gerekirse API anahtarı için önek (ör. Bearer) ayarlamak için aşağıdaki satırı yorumdan çıkarın
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API istemcisi örneği ile bir bağlam açın
with client.ApiClient(configuration) as api_client:
    # API sınıfından bir örnek oluşturun
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str |  (isteğe bağlı)
    external_id = 'external_id_example' # str |  (isteğe bağlı)
    event_type = 'event_type_example' # str |  (isteğe bağlı)
    type = 'type_example' # str |  (isteğe bağlı)
    domain = 'domain_example' # str |  (isteğe bağlı)
    attempt_count_gt = 3.4 # float |  (isteğe bağlı)

    try:
        api_response = api_instance.get_pending_webhook_event_count(tenant_id, comment_id=comment_id, external_id=external_id, event_type=event_type, type=type, domain=domain, attempt_count_gt=attempt_count_gt)
        print("The response of DefaultApi->get_pending_webhook_event_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_pending_webhook_event_count: %s\n" % e)
[inline-code-end]

---