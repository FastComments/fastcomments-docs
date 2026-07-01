## Parametreler

| Ad | Tip | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | query | No |  |
| externalId | string | query | No |  |
| eventType | string | query | No |  |
| type | string | query | No |  |
| domain | string | query | No |  |
| attemptCountGT | number | query | No |  |
| skip | number | query | No |  |

## Yanıt

Döndürür: [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_pending_webhook_events_response.py)

## Örnek

[inline-code-attrs-start title = 'get_pending_webhook_events Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetPendingWebhookEventsOptions
from client.models.get_pending_webhook_events_response import GetPendingWebhookEventsResponse
from client.rest import ApiException
from pprint import pprint

# Host'i tanımlamak isteğe bağlıdır ve varsayılan olarak https://fastcomments.com adresine ayarlıdır.
# Tüm desteklenen yapılandırma parametrelerinin listesini görmek için configuration.py dosyasına bakın.
# İstemci, kimlik doğrulama ve yetkilendirme parametrelerini yapılandırmalıdır.
# API sunucusunun güvenlik politikasına uygun olarak.
# Aşağıda her kimlik doğrulama yöntemi için örnekler sağlanmıştır, şu örneği kullanın
# ihtiyaçlarınıza uygun kimlik doğrulama örneği.
# API anahtarı yetkilendirmesini yapılandır: api_key
# Gerekirse API anahtarı için ön ek (ör. Bearer) ayarlamak için aşağıdaki satırın yorumunu kaldırın
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API istemcisi örneğiyle bir bağlam girin
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluşturun
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str |
    comment_id = 'comment_id_example' # str |  (opsiyonel)
    external_id = 'external_id_example' # str |  (opsiyonel)
    event_type = 'event_type_example' # str |  (opsiyonel)
    type = 'type_example' # str |  (opsiyonel)
    domain = 'domain_example' # str |  (opsiyonel)
    attempt_count_gt = 3.4 # float |  (opsiyonel)
    skip = 3.4 # float |  (opsiyonel)

    try:
        api_response = api_instance.get_pending_webhook_events(tenant_id, GetPendingWebhookEventsOptions(comment_id=comment_id, external_id=external_id, event_type=event_type, type=type, domain=domain, attempt_count_gt=attempt_count_gt, skip=skip))
        print("The response of DefaultApi->get_pending_webhook_events:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_pending_webhook_events: %s\n" % e)
[inline-code-end]