---
## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| skip | number | query | Hayır |  |

## Yanıt

Döndürür: [`GetModeratorsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_moderators_response.py)

## Örnek

[inline-code-attrs-start title = 'get_moderators Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_moderators_response import GetModeratorsResponse
from client.rest import ApiException
from pprint import pprint

# Sunucunun tanımlanması isteğe bağlıdır ve varsayılan olarak https://fastcomments.com kullanılır
# Tüm desteklenen yapılandırma parametreleri için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# İstemci, kimlik doğrulama ve yetkilendirme parametrelerini
# API sunucusunun güvenlik politikasına uygun olarak yapılandırmalıdır.
# Her kimlik doğrulama yöntemi için örnekler aşağıda verilmiştir,
# kullanım senaryonuza uygun olanı kullanın.

# API anahtarı yetkilendirmesini yapılandırın: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Gerekirse API anahtarı için önek (örn. Bearer) ayarlamak üzere aşağıdakinin yorumunu kaldırın
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API istemcisinin bir örneği ile bir bağlam açın
with client.ApiClient(configuration) as api_client:
    # API sınıfından bir örnek oluşturun
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    skip = 3.4 # float |  (isteğe bağlı)

    try:
        api_response = api_instance.get_moderators(tenant_id, skip=skip)
        print("The response of DefaultApi->get_moderators:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_moderators: %s\n" % e)
[inline-code-end]

---