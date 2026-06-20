## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| id | string | path | Evet |  |

## Yanıt

Döndürür: [`APIGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_user_badge_progress_response.py)

## Örnek

[inline-code-attrs-start title = 'get_user_badge_progress_by_id Örnek'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_get_user_badge_progress_response import APIGetUserBadgeProgressResponse
from client.rest import ApiException
from pprint import pprint

# Sunucu host'unu tanımlamak isteğe bağlıdır ve varsayılan https://fastcomments.com'dur
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# İstemci, kimlik doğrulama ve yetkilendirme parametrelerini yapılandırmalıdır
# API sunucusu güvenlik politikasına uygun olarak.
# Her kimlik doğrulama yöntemi için örnekler aşağıda verilmiştir,
# kullanım durumunuza uyanı kullanın.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Gerekirse API anahtarı için önek (ör. Bearer) ayarlamak üzere aşağıdaki yorum işaretini kaldırın
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API istemcisi örneği ile bir bağlam girin
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluşturun
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_user_badge_progress_by_id(tenant_id, id)
        print("The response of DefaultApi->get_user_badge_progress_by_id:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user_badge_progress_by_id: %s\n" % e)
[inline-code-end]