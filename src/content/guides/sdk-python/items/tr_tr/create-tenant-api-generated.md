## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |

## Yanıt

Döndürür: [`CreateTenant200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_tenant200_response.py)

## Örnek

[inline-code-attrs-start title = 'create_tenant Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_tenant200_response import CreateTenant200Response
from client.models.create_tenant_body import CreateTenantBody
from client.rest import ApiException
from pprint import pprint

# Sunucunun tanımlanması isteğe bağlıdır ve varsayılan olarak https://fastcomments.com kullanılır
# Desteklenen tüm yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# İstemci kimlik doğrulama ve yetkilendirme parametrelerini yapılandırmalıdır
# API sunucusunun güvenlik politikasına uygun olarak.
# Aşağıda her bir kimlik doğrulama yöntemi için örnekler verilmiştir, kullanım durumunuza
# uygun olan örneği kullanın.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Gerekli ise API anahtarı için önek (örn. Bearer) ayarlamak için aşağıdaki yorumu kaldırın
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API istemcisinin bir örneği ile bir bağlam içine girin
with client.ApiClient(configuration) as api_client:
    # API sınıfından bir örnek oluşturun
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_tenant_body = client.CreateTenantBody() # CreateTenantBody | 

    try:
        api_response = api_instance.create_tenant(tenant_id, create_tenant_body)
        print("The response of DefaultApi->create_tenant:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_tenant: %s\n" % e)
[inline-code-end]