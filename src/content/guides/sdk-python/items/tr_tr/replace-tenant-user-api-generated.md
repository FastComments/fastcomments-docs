## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| id | string | path | Evet |  |
| updateComments | string | query | Hayır |  |

## Yanıt

Döndürür: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Örnek

[inline-code-attrs-start title = 'replace_tenant_user Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.models.replace_tenant_user_body import ReplaceTenantUserBody
from client.rest import ApiException
from pprint import pprint

# Sunucu tanımlanması isteğe bağlıdır ve varsayılan olarak https://fastcomments.com kullanılır
# Desteklenen tüm yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# İstemci, kimlik doğrulama ve yetkilendirme parametrelerini
# API sunucusunun güvenlik politikasına uygun şekilde yapılandırmalıdır.
# Aşağıda her kimlik doğrulama yöntemi için örnekler verilmiştir; kullanım durumunuza
# uygun olan örneği kullanın.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Gerekirse API anahtarı için önek (örn. Bearer) ayarlamak üzere aşağıdaki yorumu kaldırın
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API istemcisi örneği ile bir bağlam açın
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluşturun
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    replace_tenant_user_body = client.ReplaceTenantUserBody() # ReplaceTenantUserBody | 
    update_comments = 'update_comments_example' # str |  (optional)

    try:
        api_response = api_instance.replace_tenant_user(tenant_id, id, replace_tenant_user_body, update_comments=update_comments)
        print("The response of DefaultApi->replace_tenant_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->replace_tenant_user: %s\n" % e)
[inline-code-end]