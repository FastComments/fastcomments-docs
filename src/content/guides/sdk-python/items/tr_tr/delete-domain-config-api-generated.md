## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| domain | string | path | Evet |  |

## Yanıt

Döndürür: [`DeleteDomainConfig200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_domain_config200_response.py)

## Örnek

[inline-code-attrs-start title = 'delete_domain_config Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.delete_domain_config200_response import DeleteDomainConfig200Response
from client.rest import ApiException
from pprint import pprint

# Sunucu (host) tanımlamak isteğe bağlıdır ve varsayılan olarak https://fastcomments.com olarak ayarlanmıştır
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# İstemci, kimlik doğrulama ve yetkilendirme parametrelerini
# API sunucusunun güvenlik politikasına göre yapılandırmalıdır.
# Her kimlik doğrulama yöntemi için aşağıda örnekler verilmiştir, kullanacağınız
# kimlik doğrulama durumuna uyan örneği kullanın.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API istemcisinin bir örneği ile bir bağlama girin
with client.ApiClient(configuration) as api_client:
    # API sınıfından bir örnek oluşturun
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    domain = 'domain_example' # str | 

    try:
        api_response = api_instance.delete_domain_config(tenant_id, domain)
        print("The response of DefaultApi->delete_domain_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_domain_config: %s\n" % e)
[inline-code-end]

---