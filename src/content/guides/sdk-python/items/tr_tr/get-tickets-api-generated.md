## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| userId | string | query | Hayır |  |
| state | number | query | Hayır |  |
| skip | number | query | Hayır |  |
| limit | number | query | Hayır |  |

## Yanıt

Döndürür: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tickets_response.py)

## Örnek

[inline-code-attrs-start title = 'get_tickets Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tickets_response import GetTicketsResponse
from client.rest import ApiException
from pprint import pprint

# Host belirtmek isteğe bağlıdır ve varsayılan https://fastcomments.com'dur
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# İstemci, kimlik doğrulama ve yetkilendirme parametrelerini yapılandırmalıdır
# API sunucusunun güvenlik politikasına göre.
# Her kimlik doğrulama yöntemi için örnekler aşağıda verilmiştir, kullanım durumunuza uygun olan
# örneği kullanın.

# API anahtarı yetkilendirmesini yapılandırın: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Gerekirse API anahtarı için önek (ör. Bearer) ayarlamak üzere aşağıdaki satırın başındaki yorumu kaldırın
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API istemcisi örneğiyle bir bağlam açın
with client.ApiClient(configuration) as api_client:
    # API sınıfından bir örnek oluşturun
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (isteğe bağlı)
    state = 3.4 # float |  (isteğe bağlı)
    skip = 3.4 # float |  (isteğe bağlı)
    limit = 3.4 # float |  (isteğe bağlı)

    try:
        api_response = api_instance.get_tickets(tenant_id, user_id=user_id, state=state, skip=skip, limit=limit)
        print("The response of DefaultApi->get_tickets:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tickets: %s\n" % e)
[inline-code-end]