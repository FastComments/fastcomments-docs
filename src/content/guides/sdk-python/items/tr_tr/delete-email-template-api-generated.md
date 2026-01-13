## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| id | string | path | Evet |  |

## Yanıt

Dönen değer: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_public200_response.py)

## Örnek

[inline-code-attrs-start title = 'delete_email_template Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_public200_response import FlagCommentPublic200Response
from client.rest import ApiException
from pprint import pprint

# Host tanımlaması isteğe bağlıdır ve varsayılan https://fastcomments.com'dur
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# İstemci, kimlik doğrulama ve yetkilendirme parametrelerini
# API sunucusunun güvenlik politikasına uygun olarak yapılandırmalıdır.
# Aşağıda her kimlik doğrulama yöntemi için örnekler verilmiştir; kimlik doğrulama kullanım durumunuza uygun olan örneği kullanın.

# API anahtarı yetkilendirmesini yapılandır: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Gerekirse API anahtarı için önek (ör. Bearer gibi) ayarlamak için aşağıdaki satırın yorumunu kaldırın
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API istemcisi örneği ile bir bağlam (context) oluşturun
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluşturun
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.delete_email_template(tenant_id, id)
        print("The response of DefaultApi->delete_email_template:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_email_template: %s\n" % e)
[inline-code-end]