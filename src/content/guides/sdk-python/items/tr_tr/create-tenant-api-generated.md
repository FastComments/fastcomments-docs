## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |

## Yanıt

Döndürür: [`CreateTenantResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_tenant_response.py)

## Örnek

[inline-code-attrs-start title = 'create_tenant Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_tenant_body import CreateTenantBody
from client.models.create_tenant_response import CreateTenantResponse
from client.rest import ApiException
from pprint import pprint

# Sunucuyu tanımlamak isteğe bağlıdır ve varsayılan olarak https://fastcomments.com olarak ayarlanmıştır
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# İstemci, kimlik doğrulama ve yetkilendirme parametrelerini yapılandırmalıdır
# API sunucusunun güvenlik politikasına uygun olarak.
# Her kimlik doğrulama yöntemi için örnekler aşağıda verilmiştir, kullanım durumunuza uygun olanı kullanın.

# API anahtarı yetkilendirmesini yapılandırın: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Gerekirse API anahtarı için önek (örn. Bearer) ayarlamak için aşağıdaki satırı yorumdan çıkarın
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API istemci örneği ile bir bağlama girin
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluşturun
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

---