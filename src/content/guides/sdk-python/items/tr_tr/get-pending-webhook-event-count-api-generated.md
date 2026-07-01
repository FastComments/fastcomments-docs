## Parametreler

| Ad | Tip | Konum | Gereklidir | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| commentId | string | query | Hayır |  |
| externalId | string | query | Hayır |  |
| eventType | string | query | Hayır |  |
| type | string | query | Hayır |  |
| domain | string | query | Hayır |  |
| attemptCountGT | number | query | Hayır |  |

## Yanıt

Döner: [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_pending_webhook_event_count_response.py)

## Örnek

[inline-code-attrs-start title = 'get_pending_webhook_event_count Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetPendingWebhookEventCountOptions
from client.models.get_pending_webhook_event_count_response import GetPendingWebhookEventCountResponse
from client.rest import ApiException
from pprint import pprint

# Ana bilgisayarı tanımlamak isteğe bağlıdır ve varsayılan olarak https://fastcomments.com adresine yönelir
# configuration.py dosyasında desteklenen tüm yapılandırma parametrelerinin listesini görebilirsiniz.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# İstemcinin, API sunucusunun güvenlik politikasına uygun olarak kimlik doğrulama ve yetkilendirme parametrelerini yapılandırması gerekir.
# Aşağıda her kimlik doğrulama yöntemi için örnekler verilmiştir, kullanım durumunuza uygun örneği kullanın.

# API anahtarı yetkilendirmesini yapılandır: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Gerekirse API anahtarı için önek (örn. Bearer) ayarlamak için aşağıdaki satırın yorumunu kaldırın
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API istemcisi bir örnekle bir bağlam girin
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluşturun
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str |  (optional)
    external_id = 'external_id_example' # str |  (optional)
    event_type = 'event_type_example' # str |  (optional)
    type = 'type_example' # str |  (optional)
    domain = 'domain_example' # str |  (optional)
    attempt_count_gt = 3.4 # float |  (optional)

    try:
        api_response = api_instance.get_pending_webhook_event_count(tenant_id, GetPendingWebhookEventCountOptions(comment_id=comment_id, external_id=external_id, event_type=event_type, type=type, domain=domain, attempt_count_gt=attempt_count_gt))
        print("DefaultApi->get_pending_webhook_event_count yanıtı:\n")
        pprint(api_response)
    except Exception as e:
        print("DefaultApi->get_pending_webhook_event_count çağrılırken istisna oluştu: %s\n" % e)
[inline-code-end]