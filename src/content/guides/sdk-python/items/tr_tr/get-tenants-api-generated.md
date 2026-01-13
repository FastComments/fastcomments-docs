## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| meta | string | query | Hayır |  |
| skip | number | query | Hayır |  |

## Yanıt

Döndürür: [`GetTenants200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenants200_response.py)

## Örnek

[inline-code-attrs-start title = 'get_tenants Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenants200_response import GetTenants200Response
from client.rest import ApiException
from pprint import pprint

# Host tanımlamak isteğe bağlıdır ve varsayılan https://fastcomments.com'dur
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# İstemci, kimlik doğrulama ve yetkilendirme parametrelerini yapılandırmalıdır
# API sunucusunun güvenlik politikasına uygun olarak.
# Her yetkilendirme yöntemi için örnekler aşağıda verilmiştir; kullanmak
# istediğiniz yetkilendirme kullanım durumunu karşılayan örneği kullanın.

# API anahtarı yetkilendirmesini yapılandırın: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Gerekirse API anahtarı için önek (örn. Bearer) ayarlamak üzere aşağıdakilerin yorumunu kaldırın
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API istemcisi örneği ile bir bağlam girin
with client.ApiClient(configuration) as api_client:
    # API sınıfından bir örnek oluşturun
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    meta = 'meta_example' # str |  (isteğe bağlı)
    skip = 3.4 # float |  (isteğe bağlı)

    try:
        api_response = api_instance.get_tenants(tenant_id, meta=meta, skip=skip)
        print("The response of DefaultApi->get_tenants:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenants: %s\n" % e)
[inline-code-end]