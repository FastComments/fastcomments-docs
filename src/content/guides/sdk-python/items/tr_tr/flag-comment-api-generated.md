## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| userId | string | query | No |  |
| anonUserId | string | query | No |  |

## Yanıt

Döndürür: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_response.py)

## Örnek

[inline-code-attrs-start title = 'flag_comment Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_response import FlagCommentResponse
from client.rest import ApiException
from pprint import pprint

# Sunucu tanımlamak isteğe bağlıdır ve varsayılan https://fastcomments.com'dur
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# İstemci kimlik doğrulama ve yetkilendirme parametrelerini
# API sunucusunun güvenlik politikasına uygun şekilde yapılandırmalıdır.
# Her kimlik doğrulama yöntemi için örnekler aşağıda verilmiştir, kullanım durumunuza uygun olan örneği kullanın.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Gerekirse API anahtarı için öneki (ör. Bearer) ayarlamak için aşağıdaki yorumu kaldırın
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API istemcisi örneği ile bir bağlam açın
with client.ApiClient(configuration) as api_client:
    # API sınıfından bir örnek oluşturun
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    user_id = 'user_id_example' # str |  (isteğe bağlı)
    anon_user_id = 'anon_user_id_example' # str |  (isteğe bağlı)

    try:
        api_response = api_instance.flag_comment(tenant_id, id, user_id=user_id, anon_user_id=anon_user_id)
        print("The response of DefaultApi->flag_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->flag_comment: %s\n" % e)
[inline-code-end]