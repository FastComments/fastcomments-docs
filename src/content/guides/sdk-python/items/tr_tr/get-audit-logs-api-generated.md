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

Döndürür: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_audit_logs200_response.py)

## Örnek

[inline-code-attrs-start title = 'get_audit_logs Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_audit_logs200_response import GetAuditLogs200Response
from client.models.sortdir import SORTDIR
from client.rest import ApiException
from pprint import pprint

# Host tanımlamak isteğe bağlıdır ve varsayılan https://fastcomments.com'dur
# Tüm desteklenen yapılandırma parametreleri listesini görmek için configuration.py'ye bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# İstemci, kimlik doğrulama ve yetkilendirme parametrelerini
# API sunucusunun güvenlik politikasına uygun şekilde yapılandırmalıdır.
# Her kimlik doğrulama yöntemi için örnekler aşağıda verilmiştir; kullanım
# durumunuza uygun olanı kullanın.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Gerekirse API anahtarı için önek (e.g. Bearer) ayarlamak üzere aşağıdaki satırı açıklayın
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API istemcisi örneği ile bir bağlama girin
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