## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| id | string | path | Evet |  |
| updateComments | string | query | Hayır |  |

## Yanıt

Döndürür: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_public200_response.py)

## Örnek

[inline-code-attrs-start title = 'replace_tenant_user Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_public200_response import FlagCommentPublic200Response
from client.models.replace_tenant_user_body import ReplaceTenantUserBody
from client.rest import ApiException
from pprint import pprint

# Host tanımlamak isteğe bağlıdır ve varsayılan https://fastcomments.com'dur
# Desteklenen tüm yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# İstemci, kimlik doğrulama ve yetkilendirme parametrelerini
# API sunucusunun güvenlik politikasına uygun olarak yapılandırmalıdır.
# Her kimlik doğrulama yöntemi için örnekler aşağıda verilmiştir; kullanım
# durumunuza uygun olan örneği kullanın.

# API anahtarı yetkilendirmesini yapılandırın: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Gerekirse API anahtarı için önek (ör. Bearer) ayarlamak üzere aşağıdakinin yorumunu kaldırın
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API istemcisinin bir örneği ile bir bağlam başlatın
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