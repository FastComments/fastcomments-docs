## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| limit | number | query | Hayır |  |
| skip | number | query | Hayır |  |
| order | string | query | Hayır |  |
| after | number | query | Hayır |  |
| before | number | query | Hayır |  |

## Yanıt

Döndürür: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_audit_logs_response.py)

## Örnek

[inline-code-attrs-start title = 'get_audit_logs Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_audit_logs_response import GetAuditLogsResponse
from client.models.sortdir import SORTDIR
from client.rest import ApiException
from pprint import pprint

# Sunucu (host) tanımlaması isteğe bağlıdır ve varsayılan https://fastcomments.com'tur
# Tüm desteklenen yapılandırma parametreleri için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# İstemci, API sunucusu güvenlik politikasına uygun olarak kimlik doğrulama ve yetkilendirme parametrelerini yapılandırmalıdır.
# Her kimlik doğrulama yöntemi için örnekler aşağıda verilmiştir; ihtiyaçlarınıza uygun olan örneği kullanın.

# API anahtarı yetkilendirmesini yapılandırın: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Gerekirse API anahtarı için öneki (örn. Bearer) ayarlamak üzere aşağıdakinin yorumunu kaldırın
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API istemcisi örneği ile bir context içine girin
with client.ApiClient(configuration) as api_client:
    # API sınıfından bir örnek oluşturun
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    limit = 3.4 # float |  (isteğe bağlı)
    skip = 3.4 # float |  (isteğe bağlı)
    order = client.SORTDIR() # SORTDIR |  (isteğe bağlı)
    after = 3.4 # float |  (isteğe bağlı)
    before = 3.4 # float |  (isteğe bağlı)

    try:
        api_response = api_instance.get_audit_logs(tenant_id, limit=limit, skip=skip, order=order, after=after, before=before)
        print("The response of DefaultApi->get_audit_logs:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_audit_logs: %s\n" % e)
[inline-code-end]

---